use crate::{Capability, CapabilityInner, ResultValue};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, Weak};
use tokio::sync::Notify;

const SIGNAL_NAMES: [&str; terrane_signal_support::SIGNAL_COUNT] =
    ["interrupt", "terminate", "hangup", "quit"];
static BROKER: LazyLock<Mutex<Weak<Broker>>> = LazyLock::new(|| Mutex::new(Weak::new()));

pub(crate) struct Subscription {
    broker: Arc<Broker>,
    id: u64,
    mask: [bool; terrane_signal_support::SIGNAL_COUNT],
    pending: Mutex<VecDeque<ResultValue>>,
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
    fn start() -> Result<Arc<Self>, String> {
        let registration = Arc::new(
            terrane_signal_support::Registration::install()
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
        let mut stale = Vec::new();
        let subscriptions = self
            .subscriptions
            .lock()
            .expect("signal subscription lock poisoned");
        for (id, weak) in subscriptions.iter() {
            let Some(subscription) = weak.upgrade() else {
                stale.push(*id);
                continue;
            };
            if !subscription.active.load(Ordering::Acquire) || !subscription.mask[signal_index] {
                continue;
            }
            let mut pending = subscription
                .pending
                .lock()
                .expect("signal queue lock poisoned");
            if let Some(previous) = pending
                .back_mut()
                .filter(|event| event.detail == SIGNAL_NAMES[signal_index])
            {
                let previous_count = previous.exact_number.parse::<u128>().unwrap_or(u128::MAX);
                let combined = previous_count.saturating_add(u128::from(batch.count));
                previous.exact_number = combined.to_string();
                previous.flag |= batch.overflowed || combined == u128::MAX;
            } else {
                pending.push_back(ResultValue {
                    detail: SIGNAL_NAMES[signal_index].to_owned(),
                    exact_number: batch.count.to_string(),
                    number: i128::from(sequence),
                    flag: batch.overflowed,
                    secondary_exact_number: crate::monotonic_nanos().to_string(),
                    ..ResultValue::default()
                });
            }
            drop(pending);
            subscription.notify.notify_one();
        }
        drop(subscriptions);
        if !stale.is_empty() {
            let mut subscriptions = self
                .subscriptions
                .lock()
                .expect("signal subscription lock poisoned");
            for id in stale {
                subscriptions.remove(&id);
            }
        }
    }

    fn remove(&self, id: u64) {
        self.subscriptions
            .lock()
            .expect("signal subscription lock poisoned")
            .remove(&id);
        if self
            .subscriptions
            .lock()
            .expect("signal subscription lock poisoned")
            .is_empty()
        {
            self.shutdown();
        }
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
    let broker = {
        let mut global = BROKER.lock().expect("signal broker lock poisoned");
        if let Some(existing) = global
            .upgrade()
            .filter(|broker| !broker.stopping.load(Ordering::Acquire))
        {
            existing
        } else {
            let Ok(created) = Broker::start() else {
                return ResultValue::error("failed to initialize process signal broker");
            };
            *global = Arc::downgrade(&created);
            created
        }
    };
    let id = broker.next_subscription.fetch_add(1, Ordering::AcqRel);
    let subscription = Arc::new(Subscription {
        broker: broker.clone(),
        id,
        mask,
        pending: Mutex::new(VecDeque::new()),
        notify: Notify::new(),
        active: AtomicBool::new(true),
    });
    broker
        .subscriptions
        .lock()
        .expect("signal subscription lock poisoned")
        .insert(id, Arc::downgrade(&subscription));
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
        if let Some(event) = subscription
            .pending
            .lock()
            .expect("signal queue lock poisoned")
            .pop_front()
        {
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
        subscription.broker.remove(subscription.id);
        subscription.notify.notify_waiters();
    }
    ResultValue::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::sys::signal::{Signal, raise};

    async fn collect_three(capability: &Capability) -> (u128, Vec<i128>) {
        let mut count = 0_u128;
        let mut sequences = Vec::new();
        while count < 3 {
            let event = next(capability).await;
            count += event
                .exact_number
                .parse::<u128>()
                .expect("exact signal count");
            sequences.push(event.number);
            assert!(!event.flag);
        }
        (count, sequences)
    }

    #[tokio::test]
    async fn broker_fans_out_one_observation_to_every_interested_subscription() {
        let names = vec!["interrupt".to_owned()];
        let first = subscribe(&names).capability.expect("first subscription");
        let second = subscribe(&names).capability.expect("second subscription");
        for _ in 0..3 {
            raise(Signal::SIGINT).expect("raise interrupt");
        }
        let first_events =
            tokio::time::timeout(std::time::Duration::from_secs(1), collect_three(&first))
                .await
                .expect("first subscription notified");
        let second_events =
            tokio::time::timeout(std::time::Duration::from_secs(1), collect_three(&second))
                .await
                .expect("second subscription notified");

        assert_eq!(first_events.0, 3);
        assert_eq!(second_events.0, 3);
        assert_eq!(first_events.1, second_events.1);
        assert!(!close(&first).failed);
        assert!(!close(&second).failed);
    }

    #[tokio::test]
    async fn broker_preserves_exact_coalesced_counts_and_overflow_status() {
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
}
