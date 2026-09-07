#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerraneChannelOverflow {
    Block,
    FailSend,
    DropNewest,
    DropOldest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneChannelSendOutcome<T> {
    pub accepted: bool,
    pub closed: bool,
    pub dropped: bool,
    pub rejected_value: Option<T>,
    pub dropped_value: Option<T>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneChannelReceiveOutcome<T> {
    pub available: bool,
    pub closed: bool,
    pub value: Option<T>,
}

struct TerraneChannelState<T> {
    values: std::collections::VecDeque<T>,
    capacity: usize,
    overflow: TerraneChannelOverflow,
    sender_closed: bool,
    receiver_closed: bool,
    next_waiter: usize,
    sender_wakers: std::collections::BTreeMap<usize, std::task::Waker>,
    receiver_wakers: std::collections::BTreeMap<usize, std::task::Waker>,
    rendezvous_values: std::collections::BTreeMap<usize, T>,
    rendezvous_completed: std::collections::BTreeSet<usize>,
}

pub struct TerraneChannelSender<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
}

pub struct TerraneChannelReceiver<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
}

pub struct TerraneChannelPair<T> {
    pub sender: TerraneChannelSender<T>,
    pub receiver: TerraneChannelReceiver<T>,
}

impl<T> TerraneChannelPair<T> {
    pub fn new(capacity: usize, overflow: TerraneChannelOverflow) -> Self {
        let state = std::sync::Arc::new(std::sync::Mutex::new(TerraneChannelState {
            values: std::collections::VecDeque::with_capacity(capacity),
            capacity,
            overflow,
            sender_closed: false,
            receiver_closed: false,
            next_waiter: 0,
            sender_wakers: std::collections::BTreeMap::new(),
            receiver_wakers: std::collections::BTreeMap::new(),
            rendezvous_values: std::collections::BTreeMap::new(),
            rendezvous_completed: std::collections::BTreeSet::new(),
        }));
        Self {
            sender: TerraneChannelSender {
                state: state.clone(),
            },
            receiver: TerraneChannelReceiver { state },
        }
    }
}

impl<T> TerraneChannelSender<T> {
    pub fn send(&self, value: T) -> TerraneChannelSend<T> {
        TerraneChannelSend {
            state: self.state.clone(),
            value: Some(value),
            waiter_id: None,
        }
    }

    pub fn close(self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.sender_closed = true;
        for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
            waker.wake();
        }
    }
}

impl<T> Drop for TerraneChannelSender<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.sender_closed = true;
        for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
            waker.wake();
        }
    }
}

pub struct TerraneChannelSend<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
    value: Option<T>,
    waiter_id: Option<usize>,
}

impl<T: Unpin> std::future::Future for TerraneChannelSend<T> {
    type Output = TerraneChannelSendOutcome<T>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state_ref = self.state.clone();
        let mut state = state_ref.lock().expect("channel state lock poisoned");
        if let Some(waiter_id) = self.waiter_id
            && state.rendezvous_completed.remove(&waiter_id)
        {
            state.sender_wakers.remove(&waiter_id);
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: true,
                closed: false,
                dropped: false,
                rejected_value: None,
                dropped_value: None,
            });
        }
        if state.receiver_closed {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.sender_wakers.remove(&waiter_id);
                if self.value.is_none() {
                    self.value = state.rendezvous_values.remove(&waiter_id);
                }
                state.rendezvous_completed.remove(&waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: false,
                closed: true,
                dropped: false,
                rejected_value: self.value.take(),
                dropped_value: None,
            });
        }
        if state.capacity == 0 {
            let waiter_id = self.waiter_id.unwrap_or_else(|| {
                let waiter_id = state.next_waiter;
                state.next_waiter += 1;
                self.waiter_id = Some(waiter_id);
                waiter_id
            });
            if let Some(value) = self.value.take() {
                state.rendezvous_values.insert(waiter_id, value);
            }
            state
                .sender_wakers
                .insert(waiter_id, context.waker().clone());
            for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                waker.wake();
            }
            return std::task::Poll::Pending;
        }
        if state.values.len() < state.capacity {
            state.values.push_back(self.value.take().expect("send polled after completion"));
            if let Some(waiter_id) = self.waiter_id.take() {
                state.sender_wakers.remove(&waiter_id);
            }
            for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                waker.wake();
            }
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: true,
                closed: false,
                dropped: false,
                rejected_value: None,
                dropped_value: None,
            });
        }
        match state.overflow {
            TerraneChannelOverflow::Block => {
                let waiter_id = self.waiter_id.unwrap_or_else(|| {
                    let waiter_id = state.next_waiter;
                    state.next_waiter += 1;
                    self.waiter_id = Some(waiter_id);
                    waiter_id
                });
                state
                    .sender_wakers
                    .insert(waiter_id, context.waker().clone());
                std::task::Poll::Pending
            }
            TerraneChannelOverflow::FailSend => {
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: false,
                    closed: false,
                    dropped: false,
                    rejected_value: self.value.take(),
                    dropped_value: None,
                })
            }
            TerraneChannelOverflow::DropNewest => {
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: false,
                    closed: false,
                    dropped: true,
                    rejected_value: None,
                    dropped_value: self.value.take(),
                })
            }
            TerraneChannelOverflow::DropOldest => {
                let dropped_value = state.values.pop_front();
                state.values.push_back(self.value.take().expect("send polled after completion"));
                for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                    waker.wake();
                }
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: true,
                    closed: false,
                    dropped: true,
                    rejected_value: None,
                    dropped_value,
                })
            }
        }
    }

}

impl<T> Drop for TerraneChannelSend<T> {
    fn drop(&mut self) {
        if let Some(waiter_id) = self.waiter_id {
            let mut state = self
                .state
                .lock()
                .expect("channel state lock poisoned");
            state.sender_wakers.remove(&waiter_id);
            state.rendezvous_values.remove(&waiter_id);
            state.rendezvous_completed.remove(&waiter_id);
        }
    }
}

impl<T> TerraneChannelReceiver<T> {
    pub fn receive(&self) -> TerraneChannelReceive<T> {
        TerraneChannelReceive {
            state: self.state.clone(),
            waiter_id: None,
        }
    }

    pub fn close(self) -> terrane_collection_support::List<T> {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.receiver_closed = true;
        let remaining =
            terrane_collection_support::List::new(state.values.drain(..).collect());
        for (_, waker) in std::mem::take(&mut state.sender_wakers) {
            waker.wake();
        }
        remaining
    }
}

impl<T> Drop for TerraneChannelReceiver<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.receiver_closed = true;
        state.values.clear();
        for (_, waker) in std::mem::take(&mut state.sender_wakers) {
            waker.wake();
        }
    }
}

pub struct TerraneChannelReceive<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
    waiter_id: Option<usize>,
}

impl<T> std::future::Future for TerraneChannelReceive<T> {
    type Output = TerraneChannelReceiveOutcome<T>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state_ref = self.state.clone();
        let mut state = state_ref.lock().expect("channel state lock poisoned");
        if state.capacity == 0
            && let Some(waiter_id) = state.rendezvous_values.keys().next().copied()
        {
            let value = state
                .rendezvous_values
                .remove(&waiter_id)
                .expect("rendezvous value disappeared");
            state.rendezvous_completed.insert(waiter_id);
            if let Some(waker) = state.sender_wakers.remove(&waiter_id) {
                waker.wake();
            }
            if let Some(receiver_waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&receiver_waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: true,
                closed: false,
                value: Some(value),
            });
        }
        if let Some(value) = state.values.pop_front() {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&waiter_id);
            }
            for (_, waker) in std::mem::take(&mut state.sender_wakers) {
                waker.wake();
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: true,
                closed: false,
                value: Some(value),
            });
        }
        if state.sender_closed {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: false,
                closed: true,
                value: None,
            });
        }
        let waiter_id = self.waiter_id.unwrap_or_else(|| {
            let waiter_id = state.next_waiter;
            state.next_waiter += 1;
            self.waiter_id = Some(waiter_id);
            waiter_id
        });
        state
            .receiver_wakers
            .insert(waiter_id, context.waker().clone());
        for waker in state.sender_wakers.values() {
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }
}

impl<T> Unpin for TerraneChannelReceive<T> {}

impl<T> Drop for TerraneChannelReceive<T> {
    fn drop(&mut self) {
        if let Some(waiter_id) = self.waiter_id {
            self.state
                .lock()
                .expect("channel state lock poisoned")
                .receiver_wakers
                .remove(&waiter_id);
        }
    }
}
