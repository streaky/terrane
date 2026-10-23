// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: function-scope-boundaries
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S183<'a> {
    Arm0(&'a terrane_int_support::Int),
    Arm1(&'a String),
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S424 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S424 {
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
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S442 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S442 {
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
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S897 {
    Arm0(bool),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S897 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm2(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S1273 {
    Arm0(bool),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S1273 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm2(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub enum Choice {
    Number(terrane_int_support::Int),
    Label(String),
}
fn borrow_match(item: Choice) -> terrane_int_support::Int {
    let value: TerraneUnionF0S183;
    let value_2: &String;
    let result: terrane_int_support::Int;
    match &item {
        Choice::Number(__terrane_pattern_162_value) => {
            value = TerraneUnionF0S183::Arm0(__terrane_pattern_162_value);
            result = (*match &value {
                TerraneUnionF0S183::Arm0(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            })
                .clone();
        }
        Choice::Label(__terrane_pattern_238_value) => {
            value_2 = __terrane_pattern_238_value;
            println!("{}", terrane_scalar_support::scalar_text(&* value_2));
            result = terrane_int_support::Int::from(
                terrane_string_support::length(&value_2.clone()) as i128,
            );
        }
    }
    return result.clone();
}
fn move_match(item: Choice) {
    let value: TerraneUnionF0S424;
    let value_2: String;
    let result: TerraneUnionF0S442;
    let __terrane_match_value_379 = item;
    match __terrane_match_value_379 {
        Choice::Number(__terrane_pattern_403_value) => {
            value = TerraneUnionF0S424::Arm0(__terrane_pattern_403_value);
            result = TerraneUnionF0S442::Arm0(
                match value {
                    TerraneUnionF0S424::Arm0(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
            );
        }
        Choice::Label(__terrane_pattern_469_value) => {
            value_2 = __terrane_pattern_469_value;
            result = TerraneUnionF0S442::Arm1(value_2);
        }
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &result {
        TerraneUnionF0S442::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S442::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn closure_boundary() -> terrane_int_support::Int {
    let outer: i64;
    let hidden: i64;
    let read_outer: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let nested: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    outer = 40;
    hidden = 9;
    read_outer = {
        let outer = outer.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(outer as i128);
        })
    };
    nested = {
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            let hidden: i64 = 7;
            return terrane_int_support::Int::from(hidden as i128);
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&read_outer()));
    nested();
    println!("{}", terrane_scalar_support::scalar_text(&hidden));
    return terrane_int_support::Int::from(hidden as i128);
}
fn finite_unions(choose_first: bool, choose_second: bool) {
    let mut value: TerraneUnionF0S897;
    if choose_first {
        value = TerraneUnionF0S897::Arm1(terrane_int_support::Int::from(1_i128));
    } else {
        value = TerraneUnionF0S897::Arm2(String::from("two"));
    }
    if choose_second {
        if matches!(&value, TerraneUnionF0S897::Arm1(_)) {
            value = TerraneUnionF0S897::Arm0(true);
        } else {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(terrane_string_support::length(&match
                &value { TerraneUnionF0S897::Arm2(value) => value, _ =>
                unreachable!("flow-proven storage refinement") }) as i128))
            );
        }
    } else {
        value = TerraneUnionF0S897::Arm1(terrane_int_support::Int::from(3_i128));
    }
    if matches!(&value, TerraneUnionF0S897::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match value {
            TerraneUnionF0S897::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            terrane_int_support::Int::from(4_i128)))
        );
    } else if matches!(&value, TerraneUnionF0S897::Arm2(_)) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_string_support::length(&match
            &value { TerraneUnionF0S897::Arm2(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }) as i128))
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value {
            TerraneUnionF0S897::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
}
fn loop_union(mut count: terrane_int_support::Int) {
    let mut value: TerraneUnionF0S1273;
    value = TerraneUnionF0S1273::Arm1(terrane_int_support::Int::from(10_i128));
    while count.clone() < terrane_int_support::Int::from(3_i128) {
        if count.clone() == terrane_int_support::Int::from(0_i128) {
            value = TerraneUnionF0S1273::Arm2(String::from("loop"));
        } else {
            value = TerraneUnionF0S1273::Arm0(true);
        }
        count = count.clone() + terrane_int_support::Int::from(1_i128);
        if count.clone() == terrane_int_support::Int::from(2_i128) {
            break;
        }
    }
    if matches!(&value, TerraneUnionF0S1273::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match value {
            TerraneUnionF0S1273::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            terrane_int_support::Int::from(1_i128)))
        );
    } else if matches!(&value, TerraneUnionF0S1273::Arm2(_)) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_string_support::length(&match
            &value { TerraneUnionF0S1273::Arm2(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }) as i128))
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value {
            TerraneUnionF0S1273::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
}
fn main() {
    let number: Choice;
    let other_number: Choice;
    let other_label: Choice;
    let movable_label: Choice;
    number = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(5_i128);
        Choice::Number(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&borrow_match(number.clone())));
    other_number = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(6_i128);
        Choice::Number(__terrane_enum_payload_0)
    };
    move_match(other_number);
    other_label = {
        let __terrane_enum_payload_0 = String::from("borrowed");
        Choice::Label(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&borrow_match(other_label)));
    movable_label = {
        let __terrane_enum_payload_0 = String::from("moved");
        Choice::Label(__terrane_enum_payload_0)
    };
    move_match(movable_label);
    println!("{}", terrane_scalar_support::scalar_text(&closure_boundary()));
    finite_unions(true, true);
    finite_unions(true, false);
    finite_unions(false, true);
    finite_unions(false, false);
    loop_union(terrane_int_support::Int::from(0_i128));
    loop_union(terrane_int_support::Int::from(1_i128));
}
