// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
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
    let number_terrane_f0_s129: i8;
    let text_terrane_f0_s147: String;
    let numbers_terrane_f0_s173: terrane_collection_support::List<i8>;
    let number_descriptor_terrane_f0_s211: TerraneDescriptor;
    let text_descriptor_terrane_f0_s245: TerraneDescriptor;
    let list_descriptor_terrane_f0_s275: TerraneDescriptor;
    number_terrane_f0_s129 = 7;
    text_terrane_f0_s147 = String::from("terrane");
    numbers_terrane_f0_s173 = terrane_collection_support::List::<
        i8,
    >::new(vec![number_terrane_f0_s129]);
    number_descriptor_terrane_f0_s211 = {
        let _ = &number_terrane_f0_s129;
        TerraneDescriptor {
            identity: "/core/types::int8",
            name: "int8",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    text_descriptor_terrane_f0_s245 = {
        let _ = &text_terrane_f0_s147;
        TerraneDescriptor {
            identity: "/core/types::string",
            name: "string",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    list_descriptor_terrane_f0_s275 = {
        let _ = &numbers_terrane_f0_s173;
        TerraneDescriptor {
            identity: "/core/collections::list of int8",
            name: "list of int8",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&number_descriptor_terrane_f0_s211
        .identity.to_owned()),
        terrane_scalar_support::scalar_text(&number_descriptor_terrane_f0_s211.name
        .to_owned()),
        terrane_scalar_support::scalar_text(&number_descriptor_terrane_f0_s211.kind
        .to_owned())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&text_descriptor_terrane_f0_s245
        .identity.to_owned()),
        terrane_scalar_support::scalar_text(&text_descriptor_terrane_f0_s245.name
        .to_owned()),
        terrane_scalar_support::scalar_text(&text_descriptor_terrane_f0_s245.kind
        .to_owned())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&list_descriptor_terrane_f0_s275
        .identity.to_owned()),
        terrane_scalar_support::scalar_text(&list_descriptor_terrane_f0_s275.name
        .to_owned()),
        terrane_scalar_support::scalar_text(&list_descriptor_terrane_f0_s275.kind
        .to_owned())
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(number_descriptor_terrane_f0_s211
        .fields.len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(text_descriptor_terrane_f0_s245
        .fields.len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(list_descriptor_terrane_f0_s275
        .fields.len() as i128))
    );
}
