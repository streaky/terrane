// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-class-reentry
#[derive(Clone)]
pub struct Inner {
    pub value: terrane_int_support::Int,
}
impl Inner {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn bump(&mut self) {
        {
            let __terrane_value = {
                let __terrane_snapshot = {
                    let __terrane_result = TERRANE_STATIC_HOLDER_SLOT
                        .handle()
                        .lock()
                        .expect("static field lock poisoned")
                        .value
                        .clone();
                    __terrane_result
                };
                __terrane_snapshot
            } + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
    }
}
pub static TERRANE_STATIC_HOLDER_SLOT: std::sync::LazyLock<TerraneStaticObject<Inner>> = std::sync::LazyLock::new(||
TerraneStaticObject::new(Inner::terrane_construct()));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    {
        let __terrane_owner_1 = TERRANE_STATIC_HOLDER_SLOT.handle();
        let __terrane_call_result = __terrane_owner_1
            .lock()
            .expect("static field lock poisoned")
            .bump();
        __terrane_call_result
    };
}
