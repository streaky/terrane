pub fn same_text(left: &str, right: &str) -> bool {
    std::ptr::eq(left.as_ptr(), right.as_ptr()) && left == right
}
pub fn same_string(left: &String, right: &String) -> bool {
    std::ptr::eq(left, right)
}
pub fn same_bytes(left: &Vec<u8>, right: &Vec<u8>) -> bool {
    std::ptr::eq(left.as_ptr(), right.as_ptr()) && left == right
}
pub fn owned_text(mut text: String) -> String {
    text.push('!');
    text
}
#[derive(Clone)]
pub struct Record {
    pub text: String,
    pub data: Vec<u8>,
    pub texts: Vec<String>,
}
impl Record {
    pub fn new() -> Self {
        Self {
            text: String::from("native storage"),
            data: vec![65, 66, 67],
            texts: vec![String::from("native list")],
        }
    }
    pub fn same(&self, text: &str) -> bool {
        same_text(&self.text, text)
    }
    pub fn update(&mut self) {
        self.text.push('!');
        self.data[0] = 90;
    }
    pub fn inspect(text: &str, other: &str) -> bool {
        same_text(text, other)
    }
    pub async fn echo_after_yield(&self, text: &str) -> String {
        same_after_yield(text, text).await;
        format!("{}/{text}", self.text)
    }
}
pub async fn same_after_yield(left: &str, right: &str) -> bool {
    let mut yielded = false;
    std::future::poll_fn(|cx| {
        if yielded {
            std::task::Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }).await;
    same_text(left, right)
}
pub fn same_sequence(left: &[String], right: &[String]) -> bool {
    std::ptr::eq(left.as_ptr(), right.as_ptr())
}
pub fn same_generic<T: AsRef<str>>(left: &T, right: &T) -> bool {
    same_text(left.as_ref(), right.as_ref())
}
pub fn update_record(record: &mut Record) {
    record.update();
}
pub async fn echo_after_yield(text: &str) -> String {
    same_after_yield(text, text).await;
    text.to_owned()
}
pub trait Inspect: Send + Sync {
    fn marker(&self) -> bool;
    fn inspect_text(&self, left: &str, right: &str) -> bool {
        same_text(left, right)
    }
    fn inspect_strings(&self, left: &[String], right: &[String]) -> bool {
        same_sequence(left, right)
    }
}
pub fn take_text(record: &mut Record) -> String {
    std::mem::replace(&mut record.text, String::from("replacement"))
}
pub fn join_text(left: &str, right: &str) -> String {
    format!("{left}|{right}")
}
pub fn join_four(first: &str, second: &str, third: &str, fourth: &str) -> String {
    format!("{first}|{second}|{third}|{fourth}")
}
#[derive(Clone)]
pub struct ReadObservation {
    pub address: u64,
    pub first: String,
}
pub fn sequence_address(items: &[String]) -> u64 {
    items.as_ptr() as u64
}
pub async fn read_sequence_after_yield(items: &[String]) -> ReadObservation {
    same_after_yield("", "").await;
    ReadObservation {
        address: sequence_address(items),
        first: items[0].clone(),
    }
}
