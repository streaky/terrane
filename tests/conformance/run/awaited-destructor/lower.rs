// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: awaited-destructor
async fn close_native() {
    __terrane_await(
            terrane_time_sleep_after(
                __terrane_traced(
                        Duration::terrane_static_milliseconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        0 /* terrane-site: case.trn:6:26-6:51 */,
                    )
                    .total_nanoseconds
                    .clone(),
            ),
        )
        .await;
    println!("{}", String::from("closed"));
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
        println!("{}", String::from("destructed"));
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
fn release() {
    let value: std::sync::Arc<TerraneSharedCell<Resource>>;
    let owner: std::sync::Arc<TerraneSharedCell<Resource>>;
    let alias: std::sync::Arc<TerraneSharedCell<Resource>>;
    value = std::sync::Arc::new(TerraneSharedCell::new(Resource::terrane_construct()));
    owner = value.clone();
    alias = owner.clone();
    {
        let _: std::sync::Arc<TerraneSharedCell<Resource>> = alias.clone();
    }
    println!("{}", String::from("body"));
}
fn main() {
    __terrane_run(async move {
        release();
        println!("{}", String::from("after"));
    });
}
