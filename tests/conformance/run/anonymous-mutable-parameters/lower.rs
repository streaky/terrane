// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: anonymous-mutable-parameters
fn main() {
    __terrane_run(async move {
        let shared: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >;
        let total: terrane_int_support::Int;
        let stateful: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let later: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >;
        shared = {
            std::sync::Arc::new(move |
                mut value: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                value = value.clone() + terrane_int_support::Int::from(1_i128);
                return value.clone();
            })
        };
        total = terrane_int_support::Int::from(0_i128);
        stateful = {
            let mut total = total.clone();
            TerraneMutableCallable::new(move |
                (mut value,): (terrane_int_support::Int,),
            | -> terrane_int_support::Int {
                value = value.clone() * terrane_int_support::Int::from(2_i128);
                total = total.clone() + value.clone();
                return total.clone();
            })
        };
        later = {
            std::sync::Arc::new(move |
                mut value: terrane_int_support::Int,
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                Box::pin(async move {
                    value = value.clone() + terrane_int_support::Int::from(3_i128);
                    return value.clone();
                })
            })
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&shared(terrane_int_support::Int::from(4_i128)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&stateful
            .call((terrane_int_support::Int::from(2_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&stateful
            .call((terrane_int_support::Int::from(3_i128),)))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(later(terrane_int_support::Int::from(4_i128)))
            . await)
        );
    });
}
