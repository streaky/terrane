use std::sync::{Arc, Mutex, Weak};
use terrane_retained_callback_api::{Event, ListenerAlias, SharedListener, WeakListener};

type Listeners = Mutex<Vec<SharedListener<Event>>>;
#[derive(Clone, Default)]
pub struct Registry { listeners: Arc<Listeners> }
#[derive(Clone)]
pub struct WeakRegistry { listeners: Weak<Listeners> }

impl Registry {
    pub fn new() -> Self { Self::default() }
    pub fn register(&self, listener: ListenerAlias<Event>) -> WeakListener<Event> {
        let token = Arc::downgrade(&listener);
        self.listeners.lock().unwrap().push(listener);
        token
    }
    pub fn remove(&self, token: &WeakListener<Event>) -> bool {
        let mut listeners = self.listeners.lock().unwrap();
        let before = listeners.len();
        listeners.retain(|listener| !Weak::ptr_eq(&Arc::downgrade(listener), token));
        listeners.len() != before
    }
    pub fn emit(&self, event: &Event) {
        // This API explicitly permits reentrancy: no registry lock spans user code.
        // Native APIs with a different contract must not inherit this guarantee.
        let snapshot = self.listeners.lock().unwrap().clone();
        for listener in snapshot { listener.on_event(event); }
    }
    pub fn count(&self) -> usize { self.listeners.lock().unwrap().len() }
    pub fn weak_owner(&self) -> WeakRegistry { WeakRegistry { listeners: Arc::downgrade(&self.listeners) } }
    pub fn token_alive(token: &WeakListener<Event>) -> bool { token.upgrade().is_some() }
    pub fn same_token(left: &WeakListener<Event>, right: &WeakListener<Event>) -> bool { Weak::ptr_eq(left, right) }
    pub fn clone_token(token: &WeakListener<Event>) -> WeakListener<Event> { token.clone() }
    pub fn destroy(self) { drop(self); }
    pub fn snapshot_then_remove_and_deliver(&self, token: &WeakListener<Event>, event: &Event) -> bool {
        let snapshot = self.listeners.lock().unwrap().clone();
        let removed = self.remove(token);
        for listener in snapshot { listener.on_event(event); }
        removed
    }
}
impl WeakRegistry {
    pub fn clear(&self) -> usize {
        let Some(listeners) = self.listeners.upgrade() else { return 0 };
        let mut listeners = listeners.lock().unwrap();
        let count = listeners.len();
        listeners.clear();
        count
    }
}
