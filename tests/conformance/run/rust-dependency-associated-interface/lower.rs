// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct IntsStorage {
    pub value: terrane_int_support::Int,
}
impl IntsStorage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(40_i128),
        }
    }
    pub fn first(&self) -> Option<terrane_int_support::Int> {
        return Some(self.value.clone());
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
#[derive(Clone)]
pub enum Ints {
    Own(IntsStorage),
    MoreInts(MoreInts),
}
impl Ints {
    pub fn terrane_construct() -> Self {
        Self::Own(IntsStorage::terrane_construct())
    }
    pub fn first(&self) -> Option<terrane_int_support::Int> {
        match self {
            Self::Own(value) => value.first(),
            Self::MoreInts(value) => value.first(),
        }
    }
    pub fn length(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.length(),
            Self::MoreInts(value) => value.length(),
        }
    }
    pub fn terrane_field_value(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.value,
            Self::MoreInts(value) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.value,
            Self::MoreInts(value) => &mut value.value,
        }
    }
}
impl SequenceProtocol<terrane_int_support::Int> for Ints {
    fn clone_box(&self) -> Box<dyn SequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        Ints::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        Ints::length(&*self)
    }
}
impl From<Ints> for Sequence<terrane_int_support::Int> {
    fn from(value: Ints) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for Ints {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Ints>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Ints>::length(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct MoreInts {
    pub value: terrane_int_support::Int,
}
impl MoreInts {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(40_i128),
        }
    }
    pub fn first(&self) -> Option<terrane_int_support::Int> {
        return Some(self.value.clone());
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl SequenceProtocol<terrane_int_support::Int> for MoreInts {
    fn clone_box(&self) -> Box<dyn SequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        MoreInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        MoreInts::length(&*self)
    }
}
impl From<MoreInts> for Sequence<terrane_int_support::Int> {
    fn from(value: MoreInts) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for MoreInts {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <MoreInts>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <MoreInts>::length(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct NestedValues {}
impl NestedValues {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn first(&self) -> Option<Nested> {
        return None;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
impl SequenceProtocol<Nested> for NestedValues {
    fn clone_box(&self) -> Box<dyn SequenceProtocol<Nested>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SequenceProtocol<Nested>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<Nested> {
        NestedValues::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NestedValues::length(&*self)
    }
}
impl From<NestedValues> for Sequence<Nested> {
    fn from(value: NestedValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for NestedValues {
    type Item = terrane_associated_witness::Nested;
    fn first(&self) -> Option<terrane_associated_witness::Nested> {
        let __terrane_boundary: Result<
            Option<terrane_associated_witness::Nested>,
            crate::TerraneForeignError,
        > = (|| {
            let __terrane_value = <NestedValues>::first(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NestedValues>::length(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct NamedInts {
    pub value: terrane_int_support::Int,
}
impl NamedInts {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(30_i128),
        }
    }
    pub fn first(&self) -> Option<terrane_int_support::Int> {
        return Some(self.value.clone());
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(4_i128);
    }
    pub fn label(&self) -> String {
        return String::from("name");
    }
}
impl NamedSequenceProtocol<terrane_int_support::Int> for NamedInts {
    fn clone_box(&self) -> Box<dyn NamedSequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn NamedSequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        NamedInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NamedInts::length(&*self)
    }
    fn label(&self) -> String {
        NamedInts::label(&*self)
    }
}
impl From<NamedInts> for NamedSequence<terrane_int_support::Int> {
    fn from(value: NamedInts) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::NamedSequence for NamedInts {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NamedInts>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl SequenceProtocol<terrane_int_support::Int> for NamedInts {
    fn clone_box(&self) -> Box<dyn SequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SequenceProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        NamedInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NamedInts::length(&*self)
    }
}
impl From<NamedInts> for Sequence<terrane_int_support::Int> {
    fn from(value: NamedInts) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for NamedInts {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NamedInts>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NamedInts>::length(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct Lists {}
impl Lists {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn first(
        &self,
    ) -> Option<terrane_collection_support::List<terrane_int_support::Int>> {
        return Some(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(7_i128),
                    terrane_int_support::Int::from(8_i128)
                ],
            ),
        );
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
impl SequenceProtocol<terrane_collection_support::List<terrane_int_support::Int>>
for Lists {
    fn clone_box(
        &self,
    ) -> Box<
        dyn SequenceProtocol<terrane_collection_support::List<terrane_int_support::Int>>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn SequenceProtocol<terrane_collection_support::List<terrane_int_support::Int>>,
    > {
        Box::new(self.clone())
    }
    fn first(
        &self,
    ) -> Option<terrane_collection_support::List<terrane_int_support::Int>> {
        Lists::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        Lists::length(&*self)
    }
}
impl From<Lists>
for Sequence<terrane_collection_support::List<terrane_int_support::Int>> {
    fn from(value: Lists) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for Lists {
    type Item = std::vec::Vec<i64>;
    fn first(&self) -> Option<std::vec::Vec<i64>> {
        let __terrane_boundary: Result<
            Option<std::vec::Vec<i64>>,
            crate::TerraneForeignError,
        > = (|| {
            let __terrane_value = <Lists>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            value
                                .into_vec()
                                .into_iter()
                                .map(|item| -> Result<_, crate::TerraneForeignError> {
                                    Ok(
                                        terrane_int_support::coerce::<i64>(&item)
                                            .map_err(|error| crate::TerraneForeignError(
                                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                            ))?,
                                    )
                                })
                                .collect::<Result<std::vec::Vec<i64>, _>>()?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Lists>::length(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn interface_length(
    value: Sequence<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return (&value).length();
}
fn main() {
    let applied: Sequence<terrane_int_support::Int>;
    let applied_named: NamedSequence<terrane_int_support::Int>;
    let applied_items: terrane_collection_support::List<
        Sequence<terrane_int_support::Int>,
    >;
    let maybe_applied: Option<Sequence<terrane_int_support::Int>>;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_associated_witness::sequence_total(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(Ints::terrane_construct()) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::sequence_total")) }, 0 /* terrane-site: src/main.trn:47:13-47:45 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_associated_witness::nested_total(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(NestedValues::terrane_construct()) } () { Ok(value)
        => value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::nested_total")) }, 1 /* terrane-site: src/main.trn:48:13-48:52 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_associated_witness::named_total(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(NamedInts::terrane_construct()) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::named_total")) }, 2 /* terrane-site: src/main.trn:49:13-49:48 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(Ints::terrane_construct()),
        3 /* terrane-site: src/main.trn:50:13-50:43 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(NamedInts::terrane_construct()),
        4 /* terrane-site: src/main.trn:51:13-51:49 */))
    );
    applied = <Sequence<terrane_int_support::Int>>::from(Ints::terrane_construct());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(applied),
        5 /* terrane-site: src/main.trn:53:13-53:34 */))
    );
    applied_named = <NamedSequence<
        terrane_int_support::Int,
    >>::from(NamedInts::terrane_construct());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(applied_named),
        6 /* terrane-site: src/main.trn:55:13-55:40 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&interface_length(< Sequence <
        terrane_int_support::Int > > ::from(Ints::terrane_construct())))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&interface_length(< Sequence <
        terrane_int_support::Int > > ::from(NamedInts::terrane_construct())))
    );
    applied_items = terrane_collection_support::List::<
        Sequence<terrane_int_support::Int>,
    >::new(
        vec![< Sequence < terrane_int_support::Int >>::from(Ints::terrane_construct())],
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(applied_items
        .length()))
    );
    maybe_applied = Some(
        <Sequence<terrane_int_support::Int>>::from(Ints::terrane_construct()),
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&maybe_applied).is_some()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_associated_witness::sequence_total(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(MoreInts::terrane_construct()) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::sequence_total")) }, 7 /* terrane-site: src/main.trn:62:13-62:50 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_associated_witness::list_total(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(Lists::terrane_construct()) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::list_total")) }, 8 /* terrane-site: src/main.trn:63:13-63:42 */))
    );
}
