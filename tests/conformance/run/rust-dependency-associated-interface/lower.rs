// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
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
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<terrane_int_support::Int>
for Ints {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        Ints::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        Ints::length(&*self)
    }
}
impl From<Ints>
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
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
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<terrane_int_support::Int>
for MoreInts {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        MoreInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        MoreInts::length(&*self)
    }
}
impl From<MoreInts>
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
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
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<Nested> for NestedValues {
    fn clone_box(
        &self,
    ) -> Box<dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<Nested>> {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<Nested>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<Nested> {
        NestedValues::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NestedValues::length(&*self)
    }
}
impl From<NestedValues> for TerraneNs4Deps26TerraneAssociatedWitnessSequence<Nested> {
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
impl TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
    terrane_int_support::Int,
> for NamedInts {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
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
impl From<NamedInts>
for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<terrane_int_support::Int> {
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
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<terrane_int_support::Int>
for NamedInts {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        NamedInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NamedInts::length(&*self)
    }
}
impl From<NamedInts>
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
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
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
    terrane_collection_support::List<terrane_int_support::Int>,
> for Lists {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_collection_support::List<terrane_int_support::Int>,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_collection_support::List<terrane_int_support::Int>,
        >,
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
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<
    terrane_collection_support::List<terrane_int_support::Int>,
> {
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
    value: TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return value.length();
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(sequence_total(Ints::terrane_construct()),
        0 /* terrane-site: src/main.trn:47:13-47:45 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(nested_total(NestedValues::terrane_construct()),
        1 /* terrane-site: src/main.trn:48:13-48:52 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(named_total(NamedInts::terrane_construct()),
        2 /* terrane-site: src/main.trn:49:13-49:48 */))
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
    let applied: TerraneNs4Deps26TerraneAssociatedWitnessSequence<
        terrane_int_support::Int,
    > = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
        terrane_int_support::Int,
    >>::from(Ints::terrane_construct());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(applied),
        5 /* terrane-site: src/main.trn:53:13-53:34 */))
    );
    let applied_named: TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
        terrane_int_support::Int,
    > = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
        terrane_int_support::Int,
    >>::from(NamedInts::terrane_construct());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(applied_named),
        6 /* terrane-site: src/main.trn:55:13-55:40 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&interface_length(<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence < terrane_int_support::Int > >
        ::from(Ints::terrane_construct())))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&interface_length(<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence < terrane_int_support::Int > >
        ::from(NamedInts::terrane_construct())))
    );
    let applied_items: terrane_collection_support::List<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int>,
    > = terrane_collection_support::List::<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int>,
    >::new(
        vec![
            < TerraneNs4Deps26TerraneAssociatedWitnessSequence < terrane_int_support::Int
            >>::from(Ints::terrane_construct())
        ],
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(applied_items
        .length()))
    );
    let maybe_applied: Option<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int>,
    > = Some(
        <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
            terrane_int_support::Int,
        >>::from(Ints::terrane_construct()),
    );
    println!("{}", terrane_scalar_support::scalar_text(&maybe_applied.is_some()));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(sequence_total(MoreInts::terrane_construct()),
        7 /* terrane-site: src/main.trn:62:13-62:50 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(list_total(Lists::terrane_construct()),
        8 /* terrane-site: src/main.trn:63:13-63:42 */))
    );
}
// Source: <terrane>/projected/deps/terrane-associated-witness.trn
// Namespace: deps/terrane-associated-witness
pub trait TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            TerraneAssociated,
        >,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            TerraneAssociated,
        >,
    >;
    fn first(&self) -> Option<TerraneAssociated>;
    fn length(&self) -> terrane_int_support::Int;
    fn label(&self) -> String;
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone
for Box<
    dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<TerraneAssociated>,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>(
    Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            TerraneAssociated,
        >,
    >,
);
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<TerraneAssociated> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<TerraneAssociated> {
    pub fn first(&self) -> Option<TerraneAssociated> {
        self.0.first()
    }
    pub fn length(&self) -> terrane_int_support::Int {
        self.0.length()
    }
    pub fn label(&self) -> String {
        self.0.label()
    }
}
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<terrane_int_support::Int> {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
                terrane_int_support::Int,
            >>::first(&*self);
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
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
                terrane_int_support::Int,
            >>::length(&*self);
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
impl terrane_associated_witness::NamedSequence
for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<terrane_int_support::Int> {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
                terrane_int_support::Int,
            >>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>,
    >;
    fn first(&self) -> Option<TerraneAssociated>;
    fn length(&self) -> terrane_int_support::Int;
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone
for Box<
    dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNs4Deps26TerraneAssociatedWitnessSequence<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>(
    Box<dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>>,
);
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone for TerraneNs4Deps26TerraneAssociatedWitnessSequence<TerraneAssociated> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> TerraneNs4Deps26TerraneAssociatedWitnessSequence<TerraneAssociated> {
    pub fn first(&self) -> Option<TerraneAssociated> {
        self.0.first()
    }
    pub fn length(&self) -> terrane_int_support::Int {
        self.0.length()
    }
}
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<
    terrane_collection_support::List<terrane_int_support::Int>,
> {
    type Item = std::vec::Vec<i64>;
    fn first(&self) -> Option<std::vec::Vec<i64>> {
        let __terrane_boundary: Result<
            Option<std::vec::Vec<i64>>,
            crate::TerraneForeignError,
        > = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                terrane_collection_support::List<terrane_int_support::Int>,
            >>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            value
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
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                terrane_collection_support::List<terrane_int_support::Int>,
            >>::length(&*self);
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
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<Nested> {
    type Item = terrane_associated_witness::Nested;
    fn first(&self) -> Option<terrane_associated_witness::Nested> {
        let __terrane_boundary: Result<
            Option<terrane_associated_witness::Nested>,
            crate::TerraneForeignError,
        > = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                Nested,
            >>::first(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                Nested,
            >>::length(&*self);
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
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                terrane_int_support::Int,
            >>::first(&*self);
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
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                terrane_int_support::Int,
            >>::length(&*self);
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
pub use terrane_associated_witness::Nested;
pub fn erased_total<
    TerraneBoxed0: terrane_associated_witness::Sequence<Item = i64> + 'static,
>(value: TerraneBoxed0) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = Box::new(value);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::erased_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::erased_total",
                ),
            )
        }
    }
}
pub fn list_total<T: terrane_associated_witness::Sequence<Item = std::vec::Vec<i64>>>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::list_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::list_total",
                ),
            )
        }
    }
}
pub fn named_total<T: terrane_associated_witness::NamedSequence<Item = i64>>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::named_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::named_total",
                ),
            )
        }
    }
}
pub fn nested_total<
    T: terrane_associated_witness::Sequence<Item = terrane_associated_witness::Nested>,
>(value: T) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::nested_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::nested_total",
                ),
            )
        }
    }
}
pub fn sequence_total<T: terrane_associated_witness::Sequence<Item = i64>>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::sequence_total(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::sequence_total",
                ),
            )
        }
    }
}
