// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneFieldMetadata {
    name: &'static str,
    external_name: &'static str,
    defaulted: bool,
    optional: bool,
    secret: bool,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
    inherently_identity_bearing: bool,
    fields: &'static [TerraneFieldMetadata],
}
// Source: case.trn
// Namespace: static-class-singleton
pub static TERRANE_STATIC_WIDGET_CURRENT: std::sync::LazyLock<
    std::sync::Mutex<Option<Widget>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
#[derive(Clone)]
pub struct Widget {}
impl Widget {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "/static-class-singleton::widget",
            name: "widget",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[],
        }
            .name
            .to_owned();
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
        return TERRANE_STATIC_WIDGET_CURRENT
            .lock()
            .expect("static field lock poisoned")
            .clone()
            .expect("semantic optional narrowing");
    }
}
fn main() {
    let first: Widget = Widget::terrane_static_shared();
    let second: Widget = Widget::terrane_static_shared();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first.class_name()),
        terrane_scalar_support::scalar_text(&second.class_name())
    );
}
