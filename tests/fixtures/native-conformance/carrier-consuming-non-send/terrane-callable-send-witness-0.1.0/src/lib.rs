#[derive(Clone)]
pub struct Local { value: std::rc::Rc<u32> }
impl Local {
    pub fn new() -> Self { Self { value: std::rc::Rc::new(7) } }
    pub fn get(&self) -> u32 { *self.value }
}
#[derive(Clone)]
pub struct Event { pub active: bool }
pub fn invoke_mut(mut callback: Box<dyn FnMut(&mut Event)>, value: &mut Event) { callback(value); }
pub fn invoke_once(callback: Box<dyn FnOnce(&Event)>, value: &Event) { callback(value); }
