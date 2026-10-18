// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: map-key-removal
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
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
fn main() {
    let mut ordered: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let preserved: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let removed: terrane_int_support::Int;
    let absent: Option<terrane_int_support::Int>;
    let mut key: String;
    let mut value: terrane_int_support::Int;
    let mut unordered: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let unordered_preserved: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let mut pair: terrane_collection_support::Entry<String, terrane_int_support::Int>;
    let mut unordered_markers: terrane_collection_support::UnorderedMap<String, Marker>;
    let mut unique: terrane_collection_support::Map<String, Marker>;
    let mut shared: terrane_collection_support::Map<String, Marker>;
    let shared_alias: terrane_collection_support::Map<String, Marker>;
    let removed_marker: Marker;
    ordered = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128)),
            terrane_collection_support::Entry::new(String::from("third"),
            terrane_int_support::Int::from(3_i128))
        ],
    );
    preserved = ordered.clone();
    removed = {
        let __terrane_result = {
            let __terrane_key = String::from("second");
            __terrane_raised(
                ordered.remove(&__terrane_key),
                0 /* terrane-site: case.trn:15:17-15:41 */,
            )
        };
        __terrane_result
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&removed),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordered
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(preserved
        .length()))
    );
    absent = {
        let __terrane_result = {
            let __terrane_key = String::from("missing");
            ordered.remove_checked(&__terrane_key)
        };
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&absent.is_none()),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordered
        .length()))
    );
    {
        let __terrane_collection_argument_0 = String::from("second");
        let __terrane_collection_argument_1 = terrane_int_support::Int::from(4_i128);
        ordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_0 = {
        let __terrane_result = ordered.keys();
        __terrane_result
    };
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        key = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&key));
    }
    let __terrane_iterable_1 = ordered.clone();
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let __terrane_item_1 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_1.key;
        let _ = &key;
        value = __terrane_item_1.value;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
    }
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_collection_argument_0 = String::from("missing");
                let _ = __terrane_raised_completion!(
                    ordered.remove(&__terrane_collection_argument_0),
                    1 /* terrane-site: case.trn:25:5-25:30 */
                );
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
                    && __terrane_error_0.kind == TerraneErrorKind::MissingKey
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("missing"));
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
    unordered = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128)),
            terrane_collection_support::Entry::new(String::from("third"),
            terrane_int_support::Int::from(3_i128))
        ],
    );
    unordered_preserved = unordered.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        __terrane_key = String::from("second"); __terrane_raised(unordered
        .remove(&__terrane_key), 2 /* terrane-site: case.trn:31:11-31:37 */) };
        __terrane_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        __terrane_key = String::from("missing"); unordered.remove_checked(&__terrane_key)
        }; __terrane_result } .is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(unordered
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(unordered_preserved
        .length()))
    );
    let __terrane_iterable_2 = {
        let __terrane_result = unordered.entries();
        __terrane_result
    };
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        pair = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&pair.key.clone()),
            terrane_scalar_support::scalar_text(&pair.value.clone())
        );
    }
    {
        let __terrane_collection_argument_0 = String::from("second");
        let __terrane_collection_argument_1 = terrane_int_support::Int::from(4_i128);
        unordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_3 = {
        let __terrane_result = unordered.entries();
        __terrane_result
    };
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_3,
    );
    loop {
        pair = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &pair;
        println!(
            "{}{}{}", String::from("reinserted-"),
            terrane_scalar_support::scalar_text(&pair.key.clone()),
            terrane_scalar_support::scalar_text(&pair.value.clone())
        );
    }
    unordered_markers = terrane_collection_support::UnorderedMap::<
        String,
        Marker,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            Marker::terrane_construct(String::from("unordered")))
        ],
    );
    {
        let __terrane_collection_argument_0 = String::from("item");
        let _ = __terrane_raised(
            unordered_markers.remove(&__terrane_collection_argument_0),
            3 /* terrane-site: case.trn:40:3-40:35 */,
        );
    };
    println!("{}", String::from("after-unordered"));
    unique = terrane_collection_support::Map::<
        String,
        Marker,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            Marker::terrane_construct(String::from("unique")))
        ],
    );
    {
        let __terrane_collection_argument_0 = String::from("item");
        let _ = __terrane_raised(
            unique.remove(&__terrane_collection_argument_0),
            4 /* terrane-site: case.trn:44:3-44:24 */,
        );
    };
    println!("{}", String::from("after-unique"));
    shared = terrane_collection_support::Map::<
        String,
        Marker,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            Marker::terrane_construct(String::from("shared")))
        ],
    );
    shared_alias = shared.clone();
    removed_marker = {
        let __terrane_result = {
            let __terrane_key = String::from("item");
            __terrane_raised(
                shared.remove(&__terrane_key),
                5 /* terrane-site: case.trn:49:29-49:50 */,
            )
        };
        __terrane_result
    };
    println!(
        "{}{}", String::from("removed-"),
        terrane_scalar_support::scalar_text(&removed_marker.name)
    );
    println!(
        "{}{}", String::from("alias-"),
        terrane_scalar_support::scalar_text(&__terrane_raised(shared_alias
        .get_or_error(&String::from("item")), 6 /* terrane-site: case.trn:51:20-51:40 */).name)
    );
}
