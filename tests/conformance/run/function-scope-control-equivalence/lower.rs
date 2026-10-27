// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: function-scope-control-equivalence
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S441 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S441 {
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
enum TerraneUnionF0S539 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S539 {
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
enum TerraneUnionF0S1017 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S1017 {
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
enum TerraneUnionF0S1289 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S1289 {
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
enum TerraneUnionF0S1487 {
    Arm0(bool),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S1487 {
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
enum TerraneUnionF0S1725 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S1725 {
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
fn straight_replacement() {
    let x: i64;
    let x_2: String;
    x = 3;
    let _ = &x;
    x_2 = String::from("cat");
    println!("{}", terrane_scalar_support::scalar_text(&x_2));
}
fn straight_int() {
    let x: i64;
    x = 3;
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn straight_string() {
    let x: String;
    x = String::from("cat");
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn branched_replacement(choose: bool) {
    let x: i64;
    let x_2: String;
    if choose {
        x = 3;
        let _ = &x;
        x_2 = String::from("cat");
    } else {
        x_2 = String::from("cat");
    }
    println!("{}", terrane_scalar_support::scalar_text(&x_2));
}
fn branched_single(choose: bool) {
    let x: TerraneUnionF0S441;
    if choose {
        x = TerraneUnionF0S441::Arm0(terrane_int_support::Int::from(3_i128));
    } else {
        x = TerraneUnionF0S441::Arm1(String::from("cat"));
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF0S441::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S441::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn loop_replacement(mut count: terrane_int_support::Int) {
    let mut x: TerraneUnionF0S539;
    x = TerraneUnionF0S539::Arm0(terrane_int_support::Int::from(3_i128));
    while count.clone() < terrane_int_support::Int::from(1_i128) {
        x = TerraneUnionF0S539::Arm1(String::from("cat"));
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF0S539::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S539::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn loop_int(mut count: terrane_int_support::Int) {
    let mut x: i64;
    x = 3;
    while count.clone() < terrane_int_support::Int::from(1_i128) {
        x = 3;
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn loop_string(mut count: terrane_int_support::Int) {
    let mut x: String;
    x = String::from("cat");
    while count.clone() < terrane_int_support::Int::from(1_i128) {
        x = String::from("cat");
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn both_branches(choose: bool) -> terrane_int_support::Int {
    let x: terrane_int_support::Int;
    if choose {
        x = terrane_int_support::Int::from(3_i128);
    } else {
        x = terrane_int_support::Int::from(4_i128);
    }
    return x.clone();
}
fn for_join(text: String) {
    let mut last: TerraneUnionF0S1017;
    last = TerraneUnionF0S1017::Arm0(terrane_int_support::Int::from(10_i128));
    let __terrane_iterable_0 = text;
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &__terrane_iterable_0,
    );
    loop {
        last = TerraneUnionF0S1017::Arm1(
            match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            },
        );
        let _ = &last;
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &last {
            TerraneUnionF0S1017::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &last {
        TerraneUnionF0S1017::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S1017::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn control_declared_function(choose: bool) -> terrane_int_support::Int {
    fn helper() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
    if choose {}
    return helper();
}
fn branch_type_use(choose: bool) -> terrane_int_support::Int {
    let x: TerraneUnionF0S1289;
    let result: terrane_int_support::Int;
    if choose {
        x = TerraneUnionF0S1289::Arm0(terrane_int_support::Int::from(3_i128));
        result = match x {
            TerraneUnionF0S1289::Arm0(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        } + terrane_int_support::Int::from(1_i128);
    } else {
        x = TerraneUnionF0S1289::Arm1(String::from("cat"));
        result = terrane_int_support::Int::from(
            terrane_string_support::length(
                &match &x {
                    TerraneUnionF0S1289::Arm1(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
            ) as i128,
        );
    }
    return result.clone();
}
fn nested_join(outer: bool, inner: bool) {
    let x: TerraneUnionF0S1487;
    if outer {
        if inner {
            x = TerraneUnionF0S1487::Arm1(terrane_int_support::Int::from(1_i128));
        } else {
            x = TerraneUnionF0S1487::Arm2(String::from("two"));
        }
    } else {
        x = TerraneUnionF0S1487::Arm0(false);
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF0S1487::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S1487::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S1487::Arm2(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn returned_branch(choose: bool) -> terrane_int_support::Int {
    let x: i64;
    if choose {
        return terrane_int_support::Int::from(9_i128);
    } else {
        x = 3;
    }
    return terrane_int_support::Int::from(x as i128);
}
fn loop_backedge() {
    let mut x: TerraneUnionF0S1725;
    let mut count: terrane_int_support::Int;
    x = TerraneUnionF0S1725::Arm0(terrane_int_support::Int::from(3_i128));
    count = terrane_int_support::Int::from(0_i128);
    while count.clone() < terrane_int_support::Int::from(2_i128) {
        println!(
            "{}", { let mut __terrane_text = String::new(); match &x {
            TerraneUnionF0S1725::Arm0(__terrane_arm) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
            __terrane_text); }, TerraneUnionF0S1725::Arm1(__terrane_arm) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
            __terrane_text); } } __terrane_text }
        );
        x = TerraneUnionF0S1725::Arm1(String::from("cat"));
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF0S1725::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF0S1725::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn main() {
    let mut last: String;
    straight_replacement();
    straight_int();
    straight_string();
    branched_replacement(true);
    branched_replacement(false);
    branched_single(true);
    branched_single(false);
    loop_replacement(terrane_int_support::Int::from(0_i128));
    loop_replacement(terrane_int_support::Int::from(1_i128));
    loop_int(terrane_int_support::Int::from(0_i128));
    loop_int(terrane_int_support::Int::from(1_i128));
    loop_string(terrane_int_support::Int::from(0_i128));
    loop_string(terrane_int_support::Int::from(1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&both_branches(true)));
    println!("{}", terrane_scalar_support::scalar_text(&both_branches(false)));
    last = String::from("before");
    let __terrane_iterable_1 = String::from("ab");
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(
        &__terrane_iterable_1,
    );
    loop {
        last = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &last;
        println!("{}", terrane_scalar_support::scalar_text(&last));
    }
    println!("{}", terrane_scalar_support::scalar_text(&last));
    for_join(String::from(""));
    for_join(String::from("ab"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&control_declared_function(false))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&control_declared_function(true))
    );
    println!("{}", terrane_scalar_support::scalar_text(&branch_type_use(true)));
    println!("{}", terrane_scalar_support::scalar_text(&branch_type_use(false)));
    nested_join(true, true);
    nested_join(true, false);
    nested_join(false, false);
    println!("{}", terrane_scalar_support::scalar_text(&returned_branch(true)));
    println!("{}", terrane_scalar_support::scalar_text(&returned_branch(false)));
    loop_backedge();
}
