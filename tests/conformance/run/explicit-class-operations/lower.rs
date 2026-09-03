// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: explicit-class-operations
#[derive(Clone)]
pub struct AnimalStorage {}
impl AnimalStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "animal",
            name: "animal",
            kind: "class",
        }
            .name
            .to_owned()
            .clone();
    }
    pub fn terrane_static_create() -> Animal {
        return Animal::terrane_construct();
    }
}
#[derive(Clone)]
pub enum Animal {
    Own(AnimalStorage),
    Dog(Dog),
}
impl Animal {
    pub fn terrane_construct() -> Self {
        Self::Own(AnimalStorage::terrane_construct())
    }
    pub fn class_name(&self) -> String {
        match self {
            Self::Own(value) => value.class_name(),
            Self::Dog(value) => value.class_name(),
        }
    }
}
#[derive(Clone)]
pub struct Dog {}
impl Dog {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "dog",
            name: "dog",
            kind: "class",
        }
            .name
            .to_owned()
            .clone();
    }
    pub fn terrane_static_create() -> Dog {
        return Dog::terrane_construct();
    }
}
pub static TERRANE_STATIC_WIDGET_CURRENT: std::sync::LazyLock<
    std::sync::Mutex<Option<Widget>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
pub static TERRANE_STATIC_WIDGET_CALLS: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct Widget {}
impl Widget {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "widget",
            name: "widget",
            kind: "class",
        }
            .name
            .to_owned()
            .clone();
    }
    pub fn terrane_static_shared() -> Widget {
        if TERRANE_STATIC_WIDGET_CURRENT
            .lock()
            .expect("static field lock poisoned")
            .clone()
            .is_none()
        {
            {
                let __terrane_static_value = Some(Widget::terrane_construct());
                *TERRANE_STATIC_WIDGET_CURRENT
                    .lock()
                    .expect("static field lock poisoned") = __terrane_static_value;
            }
        }
        {
            let __terrane_static_value = TERRANE_STATIC_WIDGET_CALLS
                .lock()
                .expect("static field lock poisoned")
                .clone() + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_WIDGET_CALLS.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        return TERRANE_STATIC_WIDGET_CURRENT
            .lock()
            .expect("static field lock poisoned")
            .clone()
            .expect("semantic optional narrowing");
    }
    pub fn terrane_static_call_count() -> terrane_int_support::Int {
        return TERRANE_STATIC_WIDGET_CALLS
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
}
fn main() {
    let pet: Dog = Dog::terrane_static_create();
    let first: Widget = Widget::terrane_static_shared();
    let second: Widget = Widget::terrane_static_shared();
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&pet.class_name()),
        terrane_scalar_support::scalar_text(&Widget::terrane_static_call_count()),
        terrane_scalar_support::scalar_text(&first.class_name()),
        terrane_scalar_support::scalar_text(&second.class_name())
    );
}
