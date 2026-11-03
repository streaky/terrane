#[derive(Clone)]
pub struct First { pub value: u32 }
#[derive(Clone)]
pub struct Second { pub value: u32 }
pub fn take_first(value: First) -> u32 { value.value }
