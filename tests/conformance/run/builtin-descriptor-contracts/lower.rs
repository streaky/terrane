// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneFieldMetadata {
    name: &'static str,
    external_name: &'static str,
    defaulted: bool,
    optional: bool,
    secret: bool,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
    inherently_identity_bearing: bool,
    fields: &'static [TerraneFieldMetadata],
}
// Source: case.trn
// Namespace: builtin-descriptor-contracts
fn main() {
    let number: i8 = 7;
    let text: String = String::from("terrane");
    let numbers: terrane_collection_support::List<i8> = terrane_collection_support::List::<
        i8,
    >::new(vec![number]);
    let number_descriptor: TerraneDescriptor = {
        let _ = &number;
        TerraneDescriptor {
            identity: "/core/types::int8",
            name: "int8",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    let text_descriptor: TerraneDescriptor = {
        let _ = &text;
        TerraneDescriptor {
            identity: "/core/types::string",
            name: "string",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    let list_descriptor: TerraneDescriptor = {
        let _ = &numbers;
        TerraneDescriptor {
            identity: "/core/collections::list of int8",
            name: "list of int8",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&number_descriptor.identity
        .to_owned()), terrane_scalar_support::scalar_text(&number_descriptor.name
        .to_owned()), terrane_scalar_support::scalar_text(&number_descriptor.kind
        .to_owned())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&text_descriptor.identity
        .to_owned()), terrane_scalar_support::scalar_text(&text_descriptor.name
        .to_owned()), terrane_scalar_support::scalar_text(&text_descriptor.kind
        .to_owned())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&list_descriptor.identity
        .to_owned()), terrane_scalar_support::scalar_text(&list_descriptor.name
        .to_owned()), terrane_scalar_support::scalar_text(&list_descriptor.kind
        .to_owned())
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(number_descriptor
        .fields.len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(text_descriptor
        .fields.len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(list_descriptor
        .fields.len() as i128))
    );
}
