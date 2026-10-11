// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct WorkerImpl {
    pub value: witness::Opaque<String>,
}
impl WorkerImpl {
    pub fn terrane_construct() -> Self {
        Self {
            value: __terrane_raised(
                make_text(),
                0 /* terrane-site: src/main.trn:6:20-6:30 */,
            ),
        }
    }
    pub fn work(&self) -> bool {
        return true;
    }
}
impl witness::Worker for WorkerImpl {
    fn work(&self) -> bool {
        let __terrane_boundary: Result<bool, crate::TerraneForeignError> = (|| {
            let __terrane_value = <WorkerImpl>::work(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn main() {
    let value: WorkerImpl;
    value = WorkerImpl::terrane_construct();
    let _ = &value;
}
