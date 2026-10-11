// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: core/annotations.trn
// Namespace: core/annotations
#[derive(Clone)]
pub struct Annotation {
    pub targets: terrane_collection_support::List<AnnotationTarget>,
    pub repeatable: bool,
}
impl Annotation {
    pub fn terrane_construct() -> Self {
        Self {
            targets: terrane_collection_support::List::<
                AnnotationTarget,
            >::new(Vec::new()),
            repeatable: false,
        }
    }
}
#[derive(Clone)]
pub enum AnnotationTarget {
    Class,
    Callable,
    Parameter,
    Field,
}
#[derive(Clone)]
pub struct Descriptor {}
impl Descriptor {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
