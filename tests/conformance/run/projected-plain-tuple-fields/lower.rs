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
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(plain_sum(terrane_recursive_descriptor_witness::Plain
        { size : match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let
        tuple_value = good.clone(); if tuple_value.length() != 2 { return Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) } }), 0 /* terrane-site: src/main.trn:8:13-8:53 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(plain_sum(terrane_recursive_descriptor_witness::Plain
        { size : match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let
        tuple_value = good.clone(); if tuple_value.length() != 2 { return Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) } }), 1 /* terrane-site: src/main.trn:9:13-9:53 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(plain_sum(terrane_recursive_descriptor_witness::Plain
        { size : match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let
        tuple_value = terrane_collection_support::Tuple:: < u32 > ::new(vec![3, 4]); if
        tuple_value.length() != 2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) } }), 2 /* terrane-site: src/main.trn:10:13-10:62 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(nested_sum(terrane_recursive_descriptor_witness::Nested
        { rows : match | | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_collection_support::List:: < terrane_collection_support::Tuple < u32 >
        > ::new(vec![good.clone()]).into_iter().map(| item | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let tuple_value = item; if tuple_value.length() !=
        2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) }).collect:: < Result <
        Vec < (u32, u32,) >, _ > > () ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, size : match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let __terrane_field_argument : Option <
        terrane_collection_support::Tuple < u32 > > = Some(good.clone());
        __terrane_field_argument } .map(| value | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let tuple_value = value; if tuple_value.length()
        != 2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) }).transpose() ?) } () {
        Ok(value) => value, Err(error) => std::panic::panic_any(error) } .into() }),
        3 /* terrane-site: src/main.trn:11:13-11:76 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(nested_sum(terrane_recursive_descriptor_witness::Nested
        { rows : match | | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_collection_support::List:: < terrane_collection_support::Tuple < u32 >
        > ::new(vec![good]).into_iter().map(| item | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let tuple_value = item; if tuple_value.length() !=
        2 { return Err(crate ::TerraneForeignError(crate
        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (tuple_items.next().expect("validated projected tuple length"), tuple_items
        .next().expect("validated projected tuple length"),) }) }).collect:: < Result <
        Vec < (u32, u32,) >, _ > > () ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, size : None:: < (u32, u32,) > }),
        4 /* terrane-site: src/main.trn:12:13-12:63 */))
    );
    owner = terrane_recursive_descriptor_witness::Plain {
        size: match || -> Result<_, crate::TerraneForeignError> {
            Ok({
                let tuple_value = terrane_collection_support::Tuple::<
                    u32,
                >::new(vec![1, 2]);
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
                    tuple_items.next().expect("validated projected tuple length"),
                    tuple_items.next().expect("validated projected tuple length"),
                )
            })
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        },
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&owner).add(match | |
        -> Result < _, crate ::TerraneForeignError > { Ok({ let tuple_value =
        terrane_collection_support::Tuple:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(5_i128)]); if tuple_value.length() != 2 { return
        Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        "projected tuple length did not match its checked type", crate
        ::TERRANE_NO_SITE))); } let mut tuple_items = tuple_value.into_vec().into_iter();
        (terrane_int_support::coerce:: < u32 > (&tuple_items.next()
        .expect("validated projected tuple length")).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?, terrane_int_support::coerce:: < u32 > (&tuple_items
        .next().expect("validated projected tuple length")).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?,) }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_recursive_descriptor_witness",
        "terrane_recursive_descriptor_witness::Plain::add")) }, 5 /* terrane-site: src/main.trn:14:13-14:37 */))
    );
}
