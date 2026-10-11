// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: stable-local-storage
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S367 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S367 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn collisions(
    source_value: terrane_int_support::Int,
    source_value_3: terrane_int_support::Int,
    source_value_2: terrane_int_support::Int,
) {
    let nested: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    println!("{}", terrane_scalar_support::scalar_text(&source_value));
    println!("{}", terrane_scalar_support::scalar_text(&source_value_3));
    println!("{}", terrane_scalar_support::scalar_text(&source_value_2));
    nested = {
        let source_value = source_value.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            let source_value_3: terrane_int_support::Int = terrane_int_support::Int::from(
                7_i128,
            );
            return source_value.clone() + source_value_3.clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&nested()));
}
fn parameter_carrier(
    value: terrane_int_support::Int,
    value_2: terrane_int_support::Int,
    replace: bool,
) {
    let mut value_3: TerraneUnionF0S367 = TerraneUnionF0S367::Arm0(value);
    let captured: std::sync::Arc<dyn Fn() -> () + Send + Sync>;
    if replace {
        value_3 = TerraneUnionF0S367::Arm1(String::from("carrier"));
    }
    captured = {
        let value_3 = value_3.clone();
        let value_2 = value_2.clone();
        std::sync::Arc::new(move || -> () {
            println!("{}", terrane_scalar_support::scalar_text(&value_3));
            println!("{}", terrane_scalar_support::scalar_text(&value_2));
            ()
        })
    };
    captured();
}
fn versions() {
    let value: i64;
    let value_2: i64;
    let before: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let value_3: String;
    let after: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
    let value_4: bool;
    value = 3;
    value_2 = 20;
    before = {
        let value = value.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(value as i128);
        })
    };
    value_3 = String::from("cat");
    after = {
        let value_3 = value_3.clone();
        std::sync::Arc::new(move || -> String {
            return value_3.clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&before()));
    println!("{}", terrane_scalar_support::scalar_text(&after()));
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
    let __terrane_replacement = true;
    drop(value_3);
    value_4 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&value_4));
}
fn builder() {
    let values_2: i64;
    let values: i64;
    let mut values_3: terrane_collection_support::List<i64>;
    let mut index: i64;
    values_2 = 9;
    values = 4;
    println!("{}", terrane_scalar_support::scalar_text(&values));
    values_3 = terrane_collection_support::List::<i64>::new(Vec::new());
    index = 0;
    {
        let __terrane_list_append_0 = values_3.make_unique();
        let __terrane_list_start_0 = index;
        let __terrane_list_end_0 = 3 as i64;
        let __terrane_list_length_0 = (__terrane_list_start_0..__terrane_list_end_0)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_0 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_0 <= __terrane_list_capacity_limit_0 {
            __terrane_list_append_0.reserve(__terrane_list_length_0);
            for __terrane_list_index_0 in __terrane_list_start_0..__terrane_list_end_0 {
                index = __terrane_list_index_0;
                __terrane_list_append_0.push(index);
            }
            index = std::cmp::max(__terrane_list_start_0, __terrane_list_end_0);
        } else {
            __terrane_list_append_0.reserve(__terrane_list_capacity_limit_0);
            while index < 3 {
                __terrane_list_append_0.push(index);
                index = index + 1;
            }
        }
        let _ = &index;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values_3
        .length()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&values_2));
    println!("{}", terrane_scalar_support::scalar_text(&index));
}
fn native_input(input: String) -> terrane_int_support::Int {
    let length: terrane_int_support::Int;
    length = {
        let input = input.clone();
        terrane_int_support::Int::from(input.len() as i64)
    };
    println!("{}", terrane_scalar_support::scalar_text(&input));
    return length.clone();
}
fn main() {
    collisions(
        terrane_int_support::Int::from(10_i128),
        terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(30_i128),
    );
    parameter_carrier(
        terrane_int_support::Int::from(5_i128),
        terrane_int_support::Int::from(8_i128),
        true,
    );
    parameter_carrier(
        terrane_int_support::Int::from(5_i128),
        terrane_int_support::Int::from(8_i128),
        false,
    );
    versions();
    builder();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&native_input(String::from("readable")))
    );
}
