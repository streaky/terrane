use super::super::{ProjectedType, foreign_aliases};
use super::*;

#[test]
fn generic_foreign_name_is_disambiguated_only_on_collision() {
    let arc_custom = "std::sync::Arc<iced::theme::Custom>";
    let arc_name = instantiated_type_name("Arc", arc_custom);
    let aliases = foreign_aliases(&BTreeMap::from([(arc_custom.to_owned(), arc_name)]));

    assert_eq!(aliases.get(arc_custom).map(String::as_str), Some("Arc"));

    let arc_other = "std::sync::Arc<witness::Other>";
    let aliases = foreign_aliases(&BTreeMap::from([
        (
            arc_custom.to_owned(),
            instantiated_type_name("Arc", arc_custom),
        ),
        (
            arc_other.to_owned(),
            instantiated_type_name("Arc", arc_other),
        ),
    ]));
    assert_eq!(aliases[arc_custom], "Arc-of-iced-theme-Custom");
    assert_eq!(aliases[arc_other], "Arc-of-witness-Other");
}
#[test]
fn instantiated_foreign_names_describe_their_canonical_arguments() {
    assert_eq!(
        instantiated_type_name(
            "RangeInclusive",
            "core::ops::range::RangeInclusive<iced::Degrees>"
        ),
        "RangeInclusive-of-iced-Degrees"
    );
    let degrees = ProjectedType::Foreign {
        rust_path: "iced::Degrees".to_owned(),
        name: "Degrees".to_owned(),
        base_rust_path: "iced::Degrees".to_owned(),
        arguments: Vec::new(),
    };
    assert_eq!(
        instantiated_nominal_name(
            "RangeInclusive",
            "core::ops::range::RangeInclusive<iced::Degrees>",
            &[degrees]
        ),
        "RangeInclusive-of-iced-Degrees"
    );
    let open = instantiated_nominal_name(
        "Query",
        "sqlx_core::query::Query<'q, DB, A>",
        &[
            ProjectedType::Generic("DB".to_owned()),
            ProjectedType::Generic("A".to_owned()),
        ],
    );
    assert!(open.starts_with("Query-"));
    assert!(!open.starts_with("Query-of-"));
    let application = ProjectedType::Opaque {
        anonymous_chain: false,
        bounds: vec![
            "iced_program::Program<State = State, Message = Message, Theme = Theme>".to_owned(),
        ],
    };
    assert_eq!(
        instantiated_nominal_name("Application", "iced::Application<opaque>", &[application]),
        "Application-of-Program-State-Message-Theme"
    );
    assert_eq!(
        instantiated_type_name(
            "Result",
            "core::result::Result<alloc::vec::Vec<iced::Point>, iced::Error>"
        ),
        "Result-of-alloc-vec-Vec-iced-Point-and-iced-Error"
    );
}
