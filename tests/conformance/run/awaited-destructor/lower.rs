// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: awaited-destructor
async fn close_native() {
    println!("{}", terrane_scalar_support::scalar_text(&String::from("closed")));
}
#[derive(Clone)]
pub struct Resource {
    __terrane_lifetime: std::sync::Arc<()>,
}
impl Resource {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    #[allow(unused_mut)]
    pub async fn destruct(mut self: &mut Self) {
        __terrane_await(close_native()).await;
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        __terrane_await_destructor(async {
            self.destruct().await;
        });
    }
}
fn main() {
    __terrane_run(async move {
        let value: Resource = Resource::terrane_construct();
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&String::from("body")));
    });
}
