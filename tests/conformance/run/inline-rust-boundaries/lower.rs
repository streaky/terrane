// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: inline-rust-boundaries
fn hoisted_captures() {
    let mut text: Option<String> = None;
    let mut upper: Option<String> = None;
    let mut flag: Option<bool> = None;
    let mut copied: Option<bool> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = text.insert(String::from("hoisted"));
            let _ = upper
                .insert({
                    let text = text
                        .as_ref()
                        .expect("flow-proven available inline Rust capture")
                        .clone();
                    text.to_uppercase()
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&upper.as_ref()
                .expect("flow-proven available binding"))
            );
            let _ = flag.insert(true);
            let _ = copied
                .insert({
                    let flag = *flag
                        .as_ref()
                        .expect("flow-proven available inline Rust capture");
                    flag
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&copied.as_ref()
                .expect("flow-proven available binding"))
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
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("unexpected"));
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
fn add(
    left: terrane_int_support::Int,
    right: terrane_int_support::Int,
) -> terrane_int_support::Int {
    let result: terrane_int_support::Int;
    result = left + right;
    return result.clone();
}
#[allow(unsafe_code)]
fn inspect_pointer() -> u8 {
    let value: u8;
    value = unsafe { *(&7u8 as *const u8) };
    return value;
}
fn rust_statement() {
    {
        println!("inline statement");
    }
}
fn inspect_cloned_input(input: String) -> terrane_int_support::Int {
    let size: terrane_int_support::Int;
    size = {
        let input = input.clone();
        terrane_int_support::Int::from(input.len() as i64)
    };
    println!("{}", terrane_scalar_support::scalar_text(&input));
    return size.clone();
}
fn borrowed_address(input: &String) -> u64 {
    return input.as_ptr() as u64;
}
fn owned_address(input: String) -> u64 {
    return input.as_ptr() as u64;
}
fn identity(value: String) -> String {
    return value;
}
fn optional_rust(present: bool) -> Option<String> {
    return if present { Some(String::from("optional")) } else { None };
}
fn main() {
    let input: String;
    let address: u64;
    let value: String;
    let copied: String;
    let mut observed: String;
    let mut index: terrane_int_support::Int;
    let callback: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
    let optional: Option<String>;
    let absent: Option<String>;
    let direct: Option<String>;
    let missing: Option<String>;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&add(terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(22_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&inspect_pointer()));
    rust_statement();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&inspect_cloned_input(String::from("copy me")))
    );
    input = String::from("owned handoff");
    address = borrowed_address(&input);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(address == owned_address(input)))
    );
    value = String::from("repeat");
    copied = identity(value.clone());
    observed = {
        let value = value.clone();
        value
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(copied.as_str() == observed
        .as_str()))
    );
    index = terrane_int_support::Int::from(0_i128);
    while index < terrane_int_support::Int::from(2_i128) {
        observed = {
            let value = value.clone();
            value
        };
        println!("{}", terrane_scalar_support::scalar_text(&observed));
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    callback = {
        let value = value.clone();
        std::sync::Arc::new(move || -> String {
            return {
                let value = value.clone();
                value
            };
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
    optional = optional_rust(true);
    if (&optional).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &optional { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    absent = None::<String>;
    println!("{}", terrane_scalar_support::scalar_text(&(&absent).is_none()));
    direct = Some(String::from("direct"));
    if (&direct).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &direct { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missing = optional_rust(false);
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    hoisted_captures();
}
