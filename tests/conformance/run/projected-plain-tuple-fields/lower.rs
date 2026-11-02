// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let first: u32;
    let second: u32;
    let good: terrane_collection_support::Tuple<u32>;
    let owner: Plain;
    first = 1;
    second = 2;
    good = terrane_collection_support::Tuple::<u32>::new(vec![first, second]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 = {
        let __terrane_conversion_input_0 = good.clone(); __terrane_raised(| | -> Result <
        _, crate ::TerraneForeignError > { Ok(terrane_recursive_descriptor_witness::Plain
        { size : { let tuple_value = __terrane_conversion_input_0; if tuple_value
        .length() != 2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) } }) } (),
        0 /* terrane-site: src/main.trn:8:25-8:52 */) };
        __terrane_raised(plain_sum(__terrane_conversion_input_0), 1 /* terrane-site: src/main.trn:8:13-8:53 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 = {
        let __terrane_conversion_input_0 = good.clone(); __terrane_raised(| | -> Result <
        _, crate ::TerraneForeignError > { Ok(terrane_recursive_descriptor_witness::Plain
        { size : { let tuple_value = __terrane_conversion_input_0; if tuple_value
        .length() != 2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) } }) } (),
        2 /* terrane-site: src/main.trn:9:25-9:52 */) };
        __terrane_raised(plain_sum(__terrane_conversion_input_0), 3 /* terrane-site: src/main.trn:9:13-9:53 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 = {
        let __terrane_conversion_input_0 = terrane_collection_support::Tuple:: < u32 >
        ::new(vec![3, 4]); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_recursive_descriptor_witness::Plain { size :
        { let tuple_value = __terrane_conversion_input_0; if tuple_value.length() != 2 {
        return Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) } }) } (),
        4 /* terrane-site: src/main.trn:10:25-10:61 */) };
        __terrane_raised(plain_sum(__terrane_conversion_input_0), 5 /* terrane-site: src/main.trn:10:13-10:62 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 = {
        let __terrane_conversion_input_1 = Some(good.clone()); let
        __terrane_conversion_input_0 = terrane_collection_support::List:: <
        terrane_collection_support::Tuple < u32 > > ::new(vec![good.clone()]);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_recursive_descriptor_witness::Nested { rows :
        __terrane_conversion_input_0.into_iter().map(| item | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let tuple_value = item; if tuple_value.length() !=
        2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) }).collect:: < Result <
        Vec < (u32, u32,) >, _ > > () ?, size : { let __terrane_field_argument : Option <
        terrane_collection_support::Tuple < u32 > > = __terrane_conversion_input_1;
        __terrane_field_argument } .map(| value | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let tuple_value = value; if tuple_value.length()
        != 2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) }).transpose() ? .into()
        }) } (), 6 /* terrane-site: src/main.trn:11:26-11:75 */) };
        __terrane_raised(nested_sum(__terrane_conversion_input_0), 7 /* terrane-site: src/main.trn:11:13-11:76 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 = {
        let __terrane_conversion_input_0 = terrane_collection_support::List:: <
        terrane_collection_support::Tuple < u32 > > ::new(vec![good]); __terrane_raised(|
        | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_recursive_descriptor_witness::Nested { rows :
        __terrane_conversion_input_0.into_iter().map(| item | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let tuple_value = item; if tuple_value.length() !=
        2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) }).collect:: < Result <
        Vec < (u32, u32,) >, _ > > () ?, size : None:: < (u32, u32,) > }) } (),
        8 /* terrane-site: src/main.trn:12:26-12:62 */) };
        __terrane_raised(nested_sum(__terrane_conversion_input_0), 9 /* terrane-site: src/main.trn:12:13-12:63 */) })
    );
    owner = {
        let __terrane_conversion_input_0 = terrane_collection_support::Tuple::<
            u32,
        >::new(vec![1, 2]);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Plain {
                    size: {
                        let tuple_value = __terrane_conversion_input_0;
                        if tuple_value.length() != 2 {
                            return Err(
                                crate::TerraneForeignError(
                                    crate::TerraneError::custom_raised(
                                        crate::TERRANE_DEPENDENCY_ERROR,
                                        "projected tuple length did not match its checked type",
                                        crate::TERRANE_NO_SITE,
                                    ),
                                ),
                            );
                        }
                        let mut tuple_items = tuple_value.into_vec().into_iter();
                        (
                            tuple_items
                                .next()
                                .expect("validated projected tuple length"),
                            tuple_items.next().expect("validated projected tuple length"),
                        )
                    },
                })
            }(),
            10 /* terrane-site: src/main.trn:13:13-13:49 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_collection_support::Tuple:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(5_i128)]); __terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&owner).add({ let tuple_value =
        __terrane_conversion_input_0; if tuple_value.length() != 2 { return Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (terrane_int_support::coerce:: < u32 > (&tuple_items.next()
        .expect("validated projected tuple length")).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?, terrane_int_support::coerce:: < u32 > (&tuple_items
        .next().expect("validated projected tuple length")).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?,) })) })) { Ok(result) => result ?, Err(payload) => return
        Err(crate ::__terrane_dependency_panic(payload,
        "terrane_recursive_descriptor_witness",
        "terrane_recursive_descriptor_witness::Plain::add")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 11 /* terrane-site: src/main.trn:14:13-14:37 */) })
    );
}
