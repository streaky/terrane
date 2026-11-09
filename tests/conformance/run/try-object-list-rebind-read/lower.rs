// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct K {
    pub n: terrane_int_support::Int,
}
impl K {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(1_i128),
        }
    }
}
fn eat(k: terrane_collection_support::List<K>) -> terrane_int_support::Int {
    let _ = &k;
    return terrane_int_support::Int::from(1_i128);
}
fn main() {
    let mut o: Option<terrane_collection_support::List<K>> = None;
    let _ = o
        .insert(
            terrane_collection_support::List::<K>::new(vec![K::terrane_construct()]),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&eat(o.as_ref()
                .expect("flow-proven available binding").clone()))
            );
            let _ = o
                .insert(
                    terrane_collection_support::List::<
                        K,
                    >::new(vec![K::terrane_construct()]),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&2));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(o.as_ref()
        .expect("flow-proven available binding")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:16:12-16:16 */)), 0 /* terrane-site: case.trn:16:12-16:16 */).n)
    );
}
