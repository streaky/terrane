pub fn open_callback<T, F: FnOnce(T) -> T>(value: T, callback: F) -> T {
    callback(value)
}

pub trait Factory<Value> {
    fn create(self) -> Value;
}

impl<F, Value> Factory<Value> for F
where
    F: FnOnce() -> Value,
{
    fn create(self) -> Value {
        self()
    }
}

pub trait Combine<State, Message> {
    fn combine(self, state: State, message: Message) -> State;
}

impl<F, State, Message> Combine<State, Message> for F
where
    F: FnOnce(State, Message) -> State,
{
    fn combine(self, state: State, message: Message) -> State {
        self(state, message)
    }
}

pub fn compose<State, Message>(
    initial: impl Factory<State>,
    message: impl Factory<Message>,
    combine: impl Combine<State, Message>,
) -> State {
    combine.combine(initial.create(), message.create())
}
