// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: example
fn main() {
    let frame: FrameState;
    let body: FrameBody;
    let value: f64;
    frame = make_frame();
    body = __terrane_raised(
        frame
            .bodies
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: src/main.trn:7:12-7:27 */,
                ),
            ),
        0 /* terrane-site: src/main.trn:7:12-7:27 */,
    );
    value = body.x;
    let _ = &value;
}
// Source: src/model/types.trn
// Namespace: example/model
#[derive(Clone)]
pub struct FrameBody {
    pub x: f64,
}
impl FrameBody {
    pub fn terrane_construct(x: f64) -> Self {
        let mut __terrane_constructed_value = Self { x: 0.0_f64 };
        __terrane_constructed_value.construct(x);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, x: f64) {
        self.x = x;
    }
}
// Source: src/snapshot/frame.trn
// Namespace: example/snapshot
#[derive(Clone)]
pub struct FrameState {
    pub bodies: terrane_collection_support::List<FrameBody>,
}
impl FrameState {
    pub fn terrane_construct(
        bodies: terrane_collection_support::List<FrameBody>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            bodies: terrane_collection_support::List::<FrameBody>::new(Vec::new()),
        };
        __terrane_constructed_value.construct(bodies);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, bodies: terrane_collection_support::List<FrameBody>) {
        self.bodies = bodies;
    }
}
fn make_frame() -> FrameState {
    let mut bodies: terrane_collection_support::List<FrameBody>;
    bodies = terrane_collection_support::List::<FrameBody>::new(Vec::new());
    bodies.append(FrameBody::terrane_construct(7.5));
    return FrameState::terrane_construct(bodies);
}
