// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: collection-identity-lifetime
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::from(""),
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
        {
            let __terrane_value = name;
            self.name = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", String::from("drop-"), terrane_scalar_support::scalar_text(&self
            .name)
        );
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
fn release_order() {
    let mut values: terrane_collection_support::List<Marker>;
    let removed: Marker;
    let mut cleared: terrane_collection_support::List<Marker>;
    let original: terrane_collection_support::List<Marker>;
    let mut separated: terrane_collection_support::List<Marker>;
    values = terrane_collection_support::List::<
        Marker,
    >::new(
        vec![
            Marker::terrane_construct(String::from("replace-old")),
            Marker::terrane_construct(String::from("remove-me")),
            Marker::terrane_construct(String::from("destroy-last"))
        ],
    );
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            0 /* terrane-site: case.trn:17:3-17:50 */,
        );
        let __terrane_collection_argument_1 = Marker::terrane_construct(
            String::from("replacement"),
        );
        __terrane_raised(
            values.set(__terrane_collection_argument_0, __terrane_collection_argument_1),
            0 /* terrane-site: case.trn:17:3-17:50 */,
        );
    };
    println!("{}", String::from("after-set"));
    removed = {
        let __terrane_result = {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                ),
                1 /* terrane-site: case.trn:20:22-20:38 */,
            );
            __terrane_raised(
                values.remove(__terrane_index),
                1 /* terrane-site: case.trn:20:22-20:38 */,
            )
        };
        __terrane_result
    };
    println!(
        "{}{}", String::from("after-remove-"),
        terrane_scalar_support::scalar_text(&removed.name)
    );
    println!("{}", String::from("after-block"));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_result = {
                    let __terrane_index = __terrane_raised_completion!(
                        terrane_collection_support::index_from_int(&terrane_int_support::Int::from(9_i128)),
                        2 /* terrane-site: case.trn:24:5-24:21 */
                    );
                    __terrane_raised_completion!(
                        values.remove(__terrane_index), 2 /* terrane-site: case.trn:24:5-24:21 */
                    )
                };
                __terrane_result
            };
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("remove-index"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    cleared = terrane_collection_support::List::<
        Marker,
    >::new(
        vec![
            Marker::terrane_construct(String::from("clear-first")),
            Marker::terrane_construct(String::from("clear-second"))
        ],
    );
    {
        cleared.clear();
    };
    println!("{}", String::from("after-clear"));
    original = terrane_collection_support::List::<
        Marker,
    >::new(vec![Marker::terrane_construct(String::from("cow-original"))]);
    separated = original.clone();
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            3 /* terrane-site: case.trn:33:3-33:57 */,
        );
        let __terrane_collection_argument_1 = Marker::terrane_construct(
            String::from("cow-replacement"),
        );
        __terrane_raised(
            separated
                .set(__terrane_collection_argument_0, __terrane_collection_argument_1),
            3 /* terrane-site: case.trn:33:3-33:57 */,
        );
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(original
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:34:10-34:21 */)), 4 /* terrane-site: case.trn:34:10-34:21 */).name),
        terrane_scalar_support::scalar_text(&__terrane_raised(separated
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:34:28-34:40 */)), 5 /* terrane-site: case.trn:34:28-34:40 */).name)
    );
    println!("{}", String::from("after-cow"));
}
fn main() {
    let left: std::sync::Arc<TerraneSharedCell<Marker>>;
    let right: std::sync::Arc<TerraneSharedCell<Marker>>;
    let left_reference: std::sync::Arc<TerraneSharedCell<Marker>>;
    let same_reference: std::sync::Arc<TerraneSharedCell<Marker>>;
    let other_reference: std::sync::Arc<TerraneSharedCell<Marker>>;
    let left_weak: std::sync::Weak<TerraneSharedCell<Marker>>;
    let references: std::sync::Arc<
        TerraneSharedCell<
            terrane_collection_support::List<std::sync::Arc<TerraneSharedCell<Marker>>>,
        >,
    >;
    let collection_reference: std::sync::Arc<
        TerraneSharedCell<
            terrane_collection_support::List<std::sync::Arc<TerraneSharedCell<Marker>>>,
        >,
    >;
    let collection_alias: std::sync::Arc<
        TerraneSharedCell<
            terrane_collection_support::List<std::sync::Arc<TerraneSharedCell<Marker>>>,
        >,
    >;
    let marker_type: TerraneDescriptor;
    let reference_type: TerraneDescriptor;
    let collection_type: TerraneDescriptor;
    release_order();
    left = std::sync::Arc::new(
        TerraneSharedCell::new(Marker::terrane_construct(String::from("left"))),
    );
    right = std::sync::Arc::new(
        TerraneSharedCell::new(
            {
                let __terrane_value = left
                    .lock()
                    .expect("reference lock poisoned")
                    .clone();
                __terrane_value
            }
                .terrane_separate(),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&false),
        terrane_scalar_support::scalar_text(&false)
    );
    left_reference = left.clone();
    same_reference = left_reference.clone();
    other_reference = right.clone();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_reference; let __terrane_identity_right = &same_reference;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_reference; let __terrane_identity_right = &other_reference;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    left_weak = std::sync::Arc::downgrade(&left_reference);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_weak; let __terrane_identity_right = &left_reference;
        std::ptr::eq(std::sync::Weak::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    references = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                std::sync::Arc<TerraneSharedCell<Marker>>,
            >::new(vec![left_reference.clone(), same_reference.clone()]),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&false));
    collection_reference = references.clone();
    collection_alias = collection_reference.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &collection_reference; let __terrane_identity_right = &collection_alias;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    marker_type = {
        let _ = &{
            let __terrane_value = left.lock().expect("reference lock poisoned").clone();
            __terrane_value
        };
        TerraneDescriptor {
            identity: "/collection-identity-lifetime::marker",
            name: "marker",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[
                TerraneFieldMetadata {
                    name: "name",
                    external_name: "name",
                    defaulted: true,
                    optional: false,
                    secret: false,
                },
            ],
        }
    };
    reference_type = {
        let _ = &left_reference;
        TerraneDescriptor {
            identity: "shared ref marker",
            name: "shared ref marker",
            kind: "type",
            inherently_identity_bearing: true,
            fields: &[],
        }
    };
    collection_type = {
        let _ = &{
            let __terrane_value = references
                .lock()
                .expect("reference lock poisoned")
                .clone();
            __terrane_value
        };
        TerraneDescriptor {
            identity: "/core/collections::list of shared ref marker",
            name: "list of shared ref marker",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&marker_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&reference_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&collection_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&marker_type
        .inherently_identity_bearing)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&reference_type
        .inherently_identity_bearing)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&collection_type
        .inherently_identity_bearing)
    );
}
