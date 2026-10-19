// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: callable-mode-fields-only
fn noop() {
    return ();
}
async fn increment(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(1_i128);
}
pub struct Operations {
    pub change: TerraneMutableCallable<(), ()>,
    pub finish: TerraneConsumingCallable<(), ()>,
    pub read: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >,
    pub change_async: TerraneMutableCallable<
        (terrane_int_support::Int,),
        std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
    >,
    pub finish_async: TerraneConsumingCallable<
        (terrane_int_support::Int,),
        std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
    >,
}
impl Operations {
    pub fn terrane_construct() -> Self {
        Self {
            change: TerraneMutableCallable::new(move |(): ()| noop()),
            finish: TerraneConsumingCallable::new(move |(): ()| noop()),
            read: std::sync::Arc::new(move |
                argument_0: terrane_int_support::Int,
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                Box::pin(increment(argument_0))
            }),
            change_async: TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                Box::pin(increment(argument_0))
            }),
            finish_async: TerraneConsumingCallable::new(move |
                (argument_0,): (terrane_int_support::Int,),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                Box::pin(increment(argument_0))
            }),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let value: Operations;
        let read: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >;
        let change: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let finish: TerraneConsumingCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let first: terrane_int_support::Int;
        let second: terrane_int_support::Int;
        let third: terrane_int_support::Int;
        value = Operations::terrane_construct();
        value.change.call(());
        read = value.read.clone();
        change = value.change_async.clone();
        finish = value.finish_async;
        first = __terrane_await(read(terrane_int_support::Int::from(1_i128))).await;
        second = __terrane_await(change.call((terrane_int_support::Int::from(2_i128),)))
            .await;
        third = __terrane_await(finish.call((terrane_int_support::Int::from(3_i128),)))
            .await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first),
            terrane_scalar_support::scalar_text(&second),
            terrane_scalar_support::scalar_text(&third)
        );
        println!("{}", String::from("changed"));
        value.finish.call(());
        println!("{}", String::from("finished"));
    });
}
