// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: function-scope-resource-joins
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S881 {
    Arm0(Marker),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S881 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
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
enum TerraneUnionF0S1151 {
    Arm0(Marker),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S1151 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        self.name = name;
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("drop ")),
            terrane_scalar_support::scalar_text(&self.name)
        );
    }
    pub fn consume_name(self) -> String {
        return self.name.clone();
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn replace_old() {
    let mut value: Marker;
    value = Marker::terrane_construct(String::from("old"));
    value = Marker::terrane_construct(
        format!(
            "{}{}", terrane_scalar_support::scalar_text(&value.name),
            terrane_scalar_support::scalar_text(&String::from("-replacement"))
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.name));
}
fn moved_replacement() {
    let mut value: Marker;
    value = Marker::terrane_construct(String::from("moved-once"));
    let __terrane_replacement = value;
    value = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&value.name));
}
fn conditional_resource(choose: bool) {
    let item: Marker;
    let item_2: String;
    if choose {
        item = Marker::terrane_construct(String::from("branch"));
        let _ = &item;
    } else {
        item = Marker::terrane_construct(String::from("other-branch"));
        let _ = &item;
    }
    let __terrane_replacement = String::from("replaced-without-reading");
    drop(item);
    item_2 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&item_2));
}
fn union_resource(choose: bool) {
    let mut item: TerraneUnionF0S881;
    if choose {
        item = TerraneUnionF0S881::Arm0(
            Marker::terrane_construct(String::from("union-resource")),
        );
    } else {
        item = TerraneUnionF0S881::Arm1(terrane_int_support::Int::from(12_i128));
    }
    if matches!(&item, TerraneUnionF0S881::Arm1(_)) {
        item = TerraneUnionF0S881::Arm1(
            match &item {
                TerraneUnionF0S881::Arm1(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone() + terrane_int_support::Int::from(1_i128),
        );
    } else {
        item = TerraneUnionF0S881::Arm2(String::from("union-replacement"));
    }
    println!("{}", terrane_scalar_support::scalar_text(&item));
}
fn move_union_resource(choose: bool) -> String {
    let item: TerraneUnionF0S1151;
    if choose {
        item = TerraneUnionF0S1151::Arm0(
            Marker::terrane_construct(String::from("consumed")),
        );
    } else {
        item = TerraneUnionF0S1151::Arm1(String::from("plain"));
    }
    if matches!(&item, TerraneUnionF0S1151::Arm1(_)) {
        return match item {
            TerraneUnionF0S1151::Arm1(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    } else {
        return match item {
            TerraneUnionF0S1151::Arm0(value) => value,
            _ => unreachable!("semantic type refinement"),
        }
            .consume_name();
    }
}
fn main() {
    conditional_resource(true);
    conditional_resource(false);
    replace_old();
    moved_replacement();
    union_resource(true);
    union_resource(false);
    println!("{}", terrane_scalar_support::scalar_text(&move_union_resource(true)));
    println!("{}", terrane_scalar_support::scalar_text(&move_union_resource(false)));
    println!("{}", terrane_scalar_support::scalar_text(&String::from("end")));
}
