// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-class-singleton
pub static TERRANE_STATIC_WIDGET_CURRENT: std::sync::LazyLock<
    TerraneSharedCell<Option<Widget>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(None));
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
        if {
            let __terrane_value = TERRANE_STATIC_WIDGET_CURRENT
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        }
            .is_none()
        {
            {
                let __terrane_static_value = Some(Widget::terrane_construct());
                *TERRANE_STATIC_WIDGET_CURRENT
                    .lock()
                    .expect("static field lock poisoned") = __terrane_static_value;
            }
        }
        return {
            let __terrane_value = TERRANE_STATIC_WIDGET_CURRENT
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        }
            .as_ref()
            .expect("semantic optional narrowing")
            .clone();
    }
}
fn main() {
    let first: Widget;
    let second: Widget;
    first = Widget::terrane_static_shared();
    second = Widget::terrane_static_shared();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first.class_name()),
        terrane_scalar_support::scalar_text(&second.class_name())
    );
}
