// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: trait-field-defaults
#[derive(Clone)]
pub struct Payload {
    pub value: terrane_int_support::Int,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(9_i128),
        }
    }
}
#[derive(Clone)]
pub struct User {
    pub slot: terrane_int_support::Int,
    pub payload: Payload,
}
impl User {
    pub fn terrane_construct() -> Self {
        Self {
            slot: terrane_int_support::Int::from(0_i128),
            payload: Payload::terrane_construct(),
        }
    }
}
fn main() {
    let value: User;
    let descriptor: TerraneDescriptor;
    value = User::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&value.slot), String::from(":"),
        terrane_scalar_support::scalar_text(&value.payload.value)
    );
    descriptor = TerraneDescriptor {
        identity: "/trait-field-defaults::user",
        name: "user",
        kind: "class",
        inherently_identity_bearing: false,
        fields: &[
            TerraneFieldMetadata {
                name: "slot",
                external_name: "slot",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "payload",
                external_name: "payload",
                defaulted: true,
                optional: false,
                secret: false,
            },
        ],
    };
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(descriptor
        .fields.len() as i128)), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:17:39-17:68 */)), 0 /* terrane-site: case.trn:17:39-17:68 */) }), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_1 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:17:75-17:104 */)), 1 /* terrane-site: case.trn:17:75-17:104 */) })
    );
}
