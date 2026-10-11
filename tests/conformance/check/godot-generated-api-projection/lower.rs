// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn vector_length(x: f32, y: f32) -> f32 {
    let position: Vector2;
    position = __terrane_raised(
        terrane_static_trn_566563746f7232_new(x, y),
        0 /* terrane-site: src/main.trn:7:16-7:34 */,
    );
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| position.length()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "godot",
                        "godot::builtin::Vector2::length",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:8:12-8:28 */,
    );
}
#[allow(dead_code)]
fn accepts_generated_class(node: Node2D) {
    let _ = &node;
}
#[allow(dead_code)]
fn generated_position(node: &Node2D) -> Vector2 {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&node).get_position()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "godot",
                        "godot::classes::Node2D::get_position",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:13:12-13:30 */,
    );
}
fn main() {
    let length: f64;
    length = vector_length(3.0_f32, 4.0_f32) as f64;
    let _ = &length;
}
