// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let callback: std::sync::Arc<dyn Fn(&Event) -> bool + Send + Sync>;
    let incoming: Event;
    let change: TerraneOwnedMutableInputsM<MutableEvent, bool>;
    let mut current: MutableEvent;
    callback = {
        std::sync::Arc::new(move |incoming_2: &Event| -> bool {
            return {
                let __terrane_snapshot = terrane_int_support::Int::from(
                        i128::from(*&incoming_2.value),
                    )
                    .clone();
                __terrane_snapshot
            } == terrane_int_support::Int::from(7_i128);
        })
    };
    incoming = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(7_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(optional_callback_api::Event {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            0 /* terrane-site: src/main.trn:8:16-8:41 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        Some(callback.clone());
        __terrane_raised(invoke_optional_direct(__terrane_conversion_input_0, &incoming),
        1 /* terrane-site: src/main.trn:9:13-9:55 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        None; __terrane_raised(invoke_optional_direct(__terrane_conversion_input_0,
        &incoming), 2 /* terrane-site: src/main.trn:10:13-10:51 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        Some(callback.clone());
        __terrane_raised(invoke_optional_alias(__terrane_conversion_input_0, &incoming),
        3 /* terrane-site: src/main.trn:11:13-11:54 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        None; __terrane_raised(invoke_optional_alias(__terrane_conversion_input_0,
        &incoming), 4 /* terrane-site: src/main.trn:12:13-12:50 */) })
    );
    change = {
        TerraneOwnedMutableInputsM::new(move |
            (current_2,): (&mut MutableEvent,),
        | -> bool {
            {
                let __terrane_value: bool = !{
                    let __terrane_snapshot = current_2.active;
                    __terrane_snapshot
                };
                current_2.active = __terrane_value;
            }
            return {
                let __terrane_snapshot = current_2.active;
                __terrane_snapshot
            };
        })
    };
    current = {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(optional_callback_api::MutableEvent {
                    active: __terrane_conversion_input_0,
                })
            }(),
            5 /* terrane-site: src/main.trn:16:15-16:53 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        Some(change); __terrane_raised(invoke_optional_box(__terrane_conversion_input_0,
        &mut current), 6 /* terrane-site: src/main.trn:17:13-17:54 */) })
    );
    println!("{}", terrane_scalar_support::scalar_text(&current.active));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        None; __terrane_raised(invoke_optional_box(__terrane_conversion_input_0, &mut
        current), 7 /* terrane-site: src/main.trn:19:13-19:47 */) })
    );
    println!("{}", terrane_scalar_support::scalar_text(&current.active));
}
