// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn selected_index() -> terrane_int_support::Int {
    println!("{}", String::from("index evaluated"));
    return terrane_int_support::Int::from(0_i128);
}
fn main() {
    let fields: Vec<String>;
    fields = {
        let __terrane_receiver_value_0 = &String::from("value");
        let __terrane_string_argument_0 = ",";
        terrane_string_support::split(
            &__terrane_receiver_value_0,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!({
                let __terrane_receiver = &fields; let __terrane_index =
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
                0 /* terrane-site: src/main.trn:12:16-12:25 */); __terrane_receiver
                .get(__terrane_index).cloned().ok_or_else(| |
                terrane_collection_support::IndexError::from_usize(__terrane_index)) },
                0 /* terrane-site: src/main.trn:12:16-12:25 */))
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
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!({
                        let __terrane_receiver = &fields; let __terrane_index =
                        __terrane_raised_completion!(terrane_collection_support::index_from_int(&selected_index()),
                        1 /* terrane-site: src/main.trn:14:16-14:41 */);
                        __terrane_receiver.get(__terrane_index).cloned().ok_or_else(| |
                        terrane_collection_support::IndexError::from_usize(__terrane_index))
                        }, 1 /* terrane-site: src/main.trn:14:16-14:41 */))
                    );
                    let __terrane_completion_1: TerraneCompletion<()> = (|| {
                        let __terrane_try_1: TerraneCompletion<()> = (|| {
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&__terrane_raised_completion!({
                                let __terrane_receiver = &fields; let __terrane_index =
                                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(-
                                1_i128)), 2 /* terrane-site: src/main.trn:16:20-16:31 */); __terrane_receiver
                                .get(__terrane_index).cloned().ok_or_else(| |
                                terrane_collection_support::IndexError::from_usize(__terrane_index))
                                }, 2 /* terrane-site: src/main.trn:16:20-16:31 */))
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_try_1 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_1) => {
                                let mut __terrane_handled_1 = false;
                                if !__terrane_handled_1
                                    && __terrane_error_1.kind == TerraneErrorKind::IndexError
                                {
                                    __terrane_handled_1 = true;
                                    println!("{}", String::from("invalid index caught"));
                                }
                                if !__terrane_handled_1 {
                                    return TerraneCompletion::Error(__terrane_error_1);
                                }
                            }
                        }
                        TerraneCompletion::Normal
                    })();
                    match __terrane_completion_1 {
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Error(error) => {
                            return TerraneCompletion::Error(error);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    }
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
