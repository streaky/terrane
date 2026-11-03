pub struct Descriptor<'a> { pub text: &'a str, pub number: u32 }
impl Descriptor<'_> {
    pub fn mutable_view(&mut self) { self.number += 1; }
    pub fn escaping_view(&self) -> &u32 { &self.number }
}
pub struct Ticket;
pub struct UniqueDescriptor<'a> { pub ticket: Ticket, pub text: &'a str }
impl UniqueDescriptor<'_> { pub fn inspect(&self) -> String { self.text.to_owned() } }
pub fn ticket() -> Ticket { Ticket }
