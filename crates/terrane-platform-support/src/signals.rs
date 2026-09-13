use crate::{Capability, CapabilityInner, ResultValue};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, Weak};
use tokio::sync::Notify;

const SIGNAL_NAMES: [&str; terrane_signal_support::SIGNAL_COUNT] =
    ["interrupt", "terminate", "hangup", "quit"];
static BROKER: LazyLock<Mutex<Weak<Broker>>> = LazyLock::new(|| Mutex::new(Weak::new()));
#[cfg(test)]
type DispatchPause = (Arc<std::sync::Barrier>, Arc<std::sync::Barrier>);
#[cfg(test)]
static DISPATCH_PAUSE: LazyLock<Mutex<Option<DispatchPause>>> = LazyLock::new(|| Mutex::new(None));

pub(crate) struct Subscription {
    broker: Arc<Broker>,
    id: u64,
    mask: [bool; terrane_signal_support::SIGNAL_COUNT],
    pending: Mutex<[Option<ResultValue>; terrane_signal_support::SIGNAL_COUNT]>,
    notify: Notify,
    active: AtomicBool,
}

struct Broker {
    registration: Mutex<Option<Arc<terrane_signal_support::Registration>>>,
    subscriptions: Mutex<HashMap<u64, Weak<Subscription>>>,
    worker: Mutex<Option<std::thread::JoinHandle<()>>>,
    stopping: AtomicBool,
    next_subscription: AtomicU64,
    next_sequence: AtomicU64,
}

impl Broker {
    fn start(mask: [bool; terrane_signal_support::SIGNAL_COUNT]) -> Result<Arc<Self>, String> {
        let registration = Arc::new(
            terrane_signal_support::Registration::install(mask)
                .map_err(|error| format!("failed to install process signal broker: {error}"))?,
        );
        let broker = Arc::new(Self {
            registration: Mutex::new(Some(registration.clone())),
            subscriptions: Mutex::new(HashMap::new()),
            worker: Mutex::new(None),
            stopping: AtomicBool::new(false),
            next_subscription: AtomicU64::new(1),
            next_sequence: AtomicU64::new(1),
        });
        let broker_for_worker = broker.clone();
        let worker = std::thread::Builder::new()
            .name("terrane-signal-broker".to_owned())
            .spawn(move || broker_for_worker.run(&registration))
            .map_err(|error| format!("failed to start process signal broker: {error}"))?;
        *broker.worker.lock().expect("signal worker lock poisoned") = Some(worker);
        Ok(broker)
    }

    fn run(&self, registration: &terrane_signal_support::Registration) {
        while !self.stopping.load(Ordering::Acquire) {
            let Ok(batches) = registration.wait() else {
                break;
            };
            if self.stopping.load(Ordering::Acquire) {
                break;
            }
            for (index, batch) in batches.into_iter().enumerate() {
                if batch.count != 0 {
                    self.dispatch(index, batch);
                }
            }
        }
    }

    fn dispatch(&self, signal_index: usize, batch: terrane_signal_support::SignalBatch) {
        let sequence = self.next_sequence.fetch_add(1, Ordering::AcqRel);
        let observed = crate::monotonic_nanos().to_string();
        let subscriptions = {
            let mut registered = self
                .subscriptions
                .lock()
                .expect("signal subscription lock poisoned");
            let mut active = Vec::with_capacity(registered.len());
            registered.retain(|_, weak| {
                let Some(subscription) = weak.upgrade() else {
                    return false;
                };
                active.push(subscription);
                true
            });
            active
        };
        #[cfg(test)]
        if let Some((entered, resume)) = DISPATCH_PAUSE
            .lock()
            .expect("dispatch pause lock poisoned")
            .clone()
        {
            entered.wait();
            resume.wait();
        }
        for subscription in subscriptions {
            if !subscription.active.load(Ordering::Acquire) || !subscription.mask[signal_index] {
                continue;
            }
            let mut pending = subscription
                .pending
                .lock()
                .expect("signal queue lock poisoned");
            if let Some(previous) = pending[signal_index].as_mut() {
                let previous_count = previous.exact_number.parse::<u128>().unwrap_or(u128::MAX);
                let combined = previous_count.saturating_add(u128::from(batch.count));
                previous.exact_number = combined.to_string();
                previous.flag |= batch.overflowed || combined == u128::MAX;
            } else {
                pending[signal_index] = Some(ResultValue {
                    detail: SIGNAL_NAMES[signal_index].to_owned(),
                    exact_number: batch.count.to_string(),
                    number: i128::from(sequence),
                    flag: batch.overflowed,
                    secondary_exact_number: observed.clone(),
                    ..ResultValue::default()
                });
            }
            drop(pending);
            subscription.notify.notify_one();
        }
    }

    fn selected_mask(&self) -> [bool; terrane_signal_support::SIGNAL_COUNT] {
        let subscriptions = self
            .subscriptions
            .lock()
            .expect("signal subscription lock poisoned")
            .values()
            .filter_map(Weak::upgrade)
            .collect::<Vec<_>>();
        let mut selected = [false; terrane_signal_support::SIGNAL_COUNT];
        for subscription in subscriptions {
            if subscription.active.load(Ordering::Acquire) {
                for (target, source) in selected.iter_mut().zip(subscription.mask) {
                    *target |= source;
                }
            }
        }
        selected
    }

    fn reconfigure(
        &self,
        mask: [bool; terrane_signal_support::SIGNAL_COUNT],
    ) -> Result<(), String> {
        self.registration
            .lock()
            .expect("signal registration lock poisoned")
            .as_ref()
            .expect("active broker registration")
            .reconfigure(mask)
            .map_err(|error| format!("failed to update process signal handlers: {error}"))
    }

    fn remove(&self, id: u64) -> Result<(), String> {
        let mut global = BROKER.lock().expect("signal broker lock poisoned");
        let is_empty = {
            let mut subscriptions = self
                .subscriptions
                .lock()
                .expect("signal subscription lock poisoned");
            subscriptions.remove(&id);
            subscriptions.is_empty()
        };
        let restored = self.reconfigure(self.selected_mask());
        if is_empty {
            self.shutdown();
            *global = Weak::new();
        }
        restored
    }
    fn is_worker_thread(&self) -> bool {
        self.worker
            .lock()
            .expect("signal worker lock poisoned")
            .as_ref()
            .is_some_and(|worker| worker.thread().id() == std::thread::current().id())
    }

    fn shutdown(&self) {
        if self.stopping.swap(true, Ordering::AcqRel) {
            return;
        }
        if let Some(registration) = self
            .registration
            .lock()
            .expect("signal registration lock poisoned")
            .as_ref()
        {
            registration.wake();
        }
        // Joining is deliberately synchronous: `wake` makes this a bounded hand-off, and exact
        // disposition restoration must finish before another subscription can install handlers.
        if let Some(worker) = self
            .worker
            .lock()
            .expect("signal worker lock poisoned")
            .take()
        {
            let _ = worker.join();
        }
        self.registration
            .lock()
            .expect("signal registration lock poisoned")
            .take();
    }
}

impl Drop for Broker {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if !self.active.swap(false, Ordering::AcqRel) {
            return;
        }
        let broker = self.broker.clone();
        let id = self.id;
        if broker.is_worker_thread() {
            let fallback = broker.clone();
            let spawned = std::thread::Builder::new()
                .name("terrane-signal-cleanup".to_owned())
                .spawn(move || {
                    if let Err(error) = broker.remove(id) {
                        eprintln!("{error}");
                    }
                });
            if let Err(error) = spawned {
                eprintln!("failed to start process signal cleanup: {error}");
                fallback.stopping.store(true, Ordering::Release);
                if let Some(registration) = fallback
                    .registration
                    .lock()
                    .expect("signal registration lock poisoned")
                    .as_ref()
                {
                    registration.wake();
                }
                let mut global = BROKER.lock().expect("signal broker lock poisoned");
                if global
                    .upgrade()
                    .is_some_and(|current| Arc::ptr_eq(&current, &fallback))
                {
                    *global = Weak::new();
                }
            }
        } else if let Err(error) = broker.remove(id) {
            eprintln!("{error}");
        }
        self.notify.notify_waiters();
    }
}

pub(crate) fn subscribe(names: &[String]) -> ResultValue {
    let mut mask = [false; terrane_signal_support::SIGNAL_COUNT];
    for name in names {
        let Some(index) = SIGNAL_NAMES.iter().position(|candidate| candidate == name) else {
            return ResultValue::error(format!("unsupported process signal `{name}`"));
        };
        mask[index] = true;
    }
    if !mask.into_iter().any(|selected| selected) {
        return ResultValue::error("a process signal subscription requires at least one signal");
    }
    let mut global = BROKER.lock().expect("signal broker lock poisoned");
    let broker = if let Some(existing) = global
        .upgrade()
        .filter(|broker| !broker.stopping.load(Ordering::Acquire))
    {
        let mut combined = existing.selected_mask();
        for (target, source) in combined.iter_mut().zip(mask) {
            *target |= source;
        }
        if let Err(error) = existing.reconfigure(combined) {
            return ResultValue::error(error);
        }
        existing
    } else {
        let Ok(created) = Broker::start(mask) else {
            return ResultValue::error("failed to initialize process signal broker");
        };
        *global = Arc::downgrade(&created);
        created
    };
    let id = broker.next_subscription.fetch_add(1, Ordering::AcqRel);
    let subscription = Arc::new(Subscription {
        broker: broker.clone(),
        id,
        mask,
        pending: Mutex::new(std::array::from_fn(|_| None)),
        notify: Notify::new(),
        active: AtomicBool::new(true),
    });
    broker
        .subscriptions
        .lock()
        .expect("signal subscription lock poisoned")
        .insert(id, Arc::downgrade(&subscription));
    drop(global);
    ResultValue {
        capability: Some(Capability(Arc::new(CapabilityInner::SignalSubscription(
            subscription,
        )))),
        ..ResultValue::default()
    }
}

pub(crate) async fn next(capability: &Capability) -> ResultValue {
    let CapabilityInner::SignalSubscription(subscription) = capability.0.as_ref() else {
        return ResultValue::error("capability is not a process signal subscription");
    };
    loop {
        let notified = subscription.notify.notified();
        let event = {
            let mut pending = subscription
                .pending
                .lock()
                .expect("signal queue lock poisoned");
            pending
                .iter()
                .enumerate()
                .filter_map(|(index, event)| event.as_ref().map(|event| (index, event.number)))
                .min_by_key(|(_, sequence)| *sequence)
                .and_then(|(index, _)| pending[index].take())
        };
        if let Some(event) = event {
            return event;
        }
        if !subscription.active.load(Ordering::Acquire) {
            return ResultValue::error("process signal subscription is closed");
        }
        notified.await;
    }
}

pub(crate) fn close(capability: &Capability) -> ResultValue {
    let CapabilityInner::SignalSubscription(subscription) = capability.0.as_ref() else {
        return ResultValue::error("capability is not a process signal subscription");
    };
    if subscription.active.swap(false, Ordering::AcqRel) {
        let removed = subscription.broker.remove(subscription.id);
        subscription.notify.notify_waiters();
        if let Err(error) = removed {
            return ResultValue::error(error);
        }
    }
    ResultValue::default()
}
#[cfg(test)]
mod tests {
    static TEST_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
        std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));
    use super::*;

    async fn collect_three(capability: &Capability) -> u128 {
        let mut count = 0_u128;
        while count < 3 {
            let event = next(capability).await;
            count += event
                .exact_number
                .parse::<u128>()
                .expect("exact signal count");
            assert!(!event.flag);
        }
        count
    }

    #[tokio::test]
    async fn broker_fans_out_one_observation_to_every_interested_subscription() {
        let _test_lock = TEST_LOCK.lock().await;
        let names = vec!["interrupt".to_owned()];
        let first = subscribe(&names).capability.expect("first subscription");
        let second = subscribe(&names).capability.expect("second subscription");
        let CapabilityInner::SignalSubscription(state) = first.0.as_ref() else {
            panic!("signal subscription capability");
        };
        state.broker.dispatch(
            0,
            terrane_signal_support::SignalBatch {
                count: 3,
                overflowed: false,
            },
        );
        let first_events =
            tokio::time::timeout(std::time::Duration::from_secs(1), collect_three(&first))
                .await
                .expect("first subscription notified");
        let second_events =
            tokio::time::timeout(std::time::Duration::from_secs(1), collect_three(&second))
                .await
                .expect("second subscription notified");

        assert_eq!(first_events, 3);
        assert_eq!(second_events, 3);
        assert!(!close(&first).failed);
        assert!(!close(&second).failed);
    }

    #[tokio::test]
    async fn broker_preserves_exact_coalesced_counts_and_overflow_status() {
        let _test_lock = TEST_LOCK.lock().await;
        let subscription = subscribe(&["terminate".to_owned()])
            .capability
            .expect("subscription");
        let CapabilityInner::SignalSubscription(state) = subscription.0.as_ref() else {
            panic!("signal subscription capability");
        };
        state.broker.dispatch(
            1,
            terrane_signal_support::SignalBatch {
                count: u64::MAX,
                overflowed: true,
            },
        );
        state.broker.dispatch(
            1,
            terrane_signal_support::SignalBatch {
                count: 1,
                overflowed: false,
            },
        );
        let event = next(&subscription).await;
        assert_eq!(event.exact_number, (u128::from(u64::MAX) + 1).to_string());
        assert!(event.flag);
        assert!(!close(&subscription).failed);
    }
    #[tokio::test]
    async fn pending_storage_is_one_coalesced_slot_per_signal_kind() {
        let _test_lock = TEST_LOCK.lock().await;
        let subscription = subscribe(&SIGNAL_NAMES.map(str::to_owned))
            .capability
            .expect("subscription");
        let CapabilityInner::SignalSubscription(state) = subscription.0.as_ref() else {
            panic!("signal subscription capability");
        };
        for _ in 0..1_000 {
            for index in [2, 0, 3, 1] {
                state.broker.dispatch(
                    index,
                    terrane_signal_support::SignalBatch {
                        count: 1,
                        overflowed: false,
                    },
                );
            }
        }
        let mut details = Vec::new();
        for _ in 0..terrane_signal_support::SIGNAL_COUNT {
            let event = next(&subscription).await;
            assert_eq!(event.exact_number, "1000");
            assert!(!event.flag);
            details.push(event.detail);
        }
        assert_eq!(details, ["hangup", "interrupt", "quit", "terminate"]);

        state.broker.dispatch(
            0,
            terrane_signal_support::SignalBatch {
                count: 1,
                overflowed: false,
            },
        );
        let reset = next(&subscription).await;
        assert_eq!(reset.exact_number, "1");
        assert!(!reset.flag);
        assert!(!close(&subscription).failed);
    }
    #[tokio::test]
    async fn dropping_the_last_capability_during_dispatch_cannot_deadlock_the_worker() {
        let _test_lock = TEST_LOCK.lock().await;
        let subscription = subscribe(&["interrupt".to_owned()])
            .capability
            .expect("subscription");
        let entered = Arc::new(std::sync::Barrier::new(2));
        let resume = Arc::new(std::sync::Barrier::new(2));
        *DISPATCH_PAUSE.lock().expect("dispatch pause lock poisoned") =
            Some((entered.clone(), resume.clone()));

        let status = std::process::Command::new("kill")
            .args(["-INT", &std::process::id().to_string()])
            .status()
            .expect("send interrupt to test process");
        assert!(status.success());
        entered.wait();
        drop(subscription);
        resume.wait();
        *DISPATCH_PAUSE.lock().expect("dispatch pause lock poisoned") = None;

        for _ in 0..100 {
            if BROKER
                .lock()
                .expect("signal broker lock poisoned")
                .upgrade()
                .is_none()
            {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("dropped subscription did not stop its broker");
    }
}
