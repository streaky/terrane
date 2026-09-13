// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: collection-identity-lifetime
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut value = Self {
            name: String::from(""),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        value.construct(name);
        value
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
            "{}{}", terrane_scalar_support::scalar_text(&String::from("drop-")),
            terrane_scalar_support::scalar_text(&self.name)
        );
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
fn release_order() {
    let mut values: terrane_collection_support::List<Marker> = terrane_collection_support::List::<
        Marker,
    >::new(
        vec![
            Marker::terrane_construct(String::from("replace-old")),
            Marker::terrane_construct(String::from("remove-me")),
            Marker::terrane_construct(String::from("destroy-last"))
        ],
    );
    __terrane_raised(
        values
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: case.trn:18:3-18:50 */,
                ),
                Marker::terrane_construct(String::from("replacement")),
            ),
        0 /* terrane-site: case.trn:18:3-18:50 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-set")));
    if true {
        let removed: Marker = __terrane_raised(
            values
                .remove(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(1_i128),
                        ),
                        1 /* terrane-site: case.trn:21:22-21:38 */,
                    ),
                ),
            1 /* terrane-site: case.trn:21:22-21:38 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("after-remove-")),
            terrane_scalar_support::scalar_text(&removed.name)
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-block")));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                values
                .remove(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(9_i128)),
                2 /* terrane-site: case.trn:25:5-25:21 */)), 2 /* terrane-site: case.trn:25:5-25:21 */
            );
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
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("remove-index"))
                    );
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
    let mut cleared: terrane_collection_support::List<Marker> = terrane_collection_support::List::<
        Marker,
    >::new(
        vec![
            Marker::terrane_construct(String::from("clear-first")),
            Marker::terrane_construct(String::from("clear-second"))
        ],
    );
    cleared.clear();
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-clear")));
    let original: terrane_collection_support::List<Marker> = terrane_collection_support::List::<
        Marker,
    >::new(vec![Marker::terrane_construct(String::from("cow-original"))]);
    let mut separated: terrane_collection_support::List<Marker> = original.clone();
    __terrane_raised(
        separated
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    3 /* terrane-site: case.trn:34:3-34:57 */,
                ),
                Marker::terrane_construct(String::from("cow-replacement")),
            ),
        3 /* terrane-site: case.trn:34:3-34:57 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(original
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:35:10-35:21 */)), 4 /* terrane-site: case.trn:35:10-35:21 */).name),
        terrane_scalar_support::scalar_text(&__terrane_raised(separated
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:35:28-35:40 */)), 5 /* terrane-site: case.trn:35:28-35:40 */).name)
    );
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-cow")));
}
fn main() {
    release_order();
    let left: std::sync::Arc<std::sync::Mutex<Marker>> = std::sync::Arc::new(
        std::sync::Mutex::new(Marker::terrane_construct(String::from("left"))),
    );
    let right: std::sync::Arc<std::sync::Mutex<Marker>> = std::sync::Arc::new(
        std::sync::Mutex::new(
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
        "{}{}", terrane_scalar_support::scalar_text(&{ false }),
        terrane_scalar_support::scalar_text(&{ false })
    );
    let left_reference: std::sync::Arc<std::sync::Mutex<Marker>> = left.clone();
    let same_reference: std::sync::Arc<std::sync::Mutex<Marker>> = left_reference
        .clone();
    let other_reference: std::sync::Arc<std::sync::Mutex<Marker>> = right.clone();
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
    let left_weak: std::sync::Weak<std::sync::Mutex<Marker>> = std::sync::Arc::downgrade(
        &left_reference,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_weak; let __terrane_identity_right = &left_reference;
        std::ptr::eq(std::sync::Weak::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    let references: std::sync::Arc<
        std::sync::Mutex<
            terrane_collection_support::List<std::sync::Arc<std::sync::Mutex<Marker>>>,
        >,
    > = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                std::sync::Arc<std::sync::Mutex<Marker>>,
            >::new(vec![left_reference.clone(), same_reference.clone()]),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&{ false }));
    let collection_reference: std::sync::Arc<
        std::sync::Mutex<
            terrane_collection_support::List<std::sync::Arc<std::sync::Mutex<Marker>>>,
        >,
    > = references.clone();
    let collection_alias: std::sync::Arc<
        std::sync::Mutex<
            terrane_collection_support::List<std::sync::Arc<std::sync::Mutex<Marker>>>,
        >,
    > = collection_reference.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &collection_reference; let __terrane_identity_right = &collection_alias;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    let marker_type: TerraneDescriptor = {
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
    let reference_type: TerraneDescriptor = {
        let _ = &left_reference;
        TerraneDescriptor {
            identity: "shared ref marker",
            name: "shared ref marker",
            kind: "type",
            inherently_identity_bearing: true,
            fields: &[],
        }
    };
    let collection_type: TerraneDescriptor = {
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
