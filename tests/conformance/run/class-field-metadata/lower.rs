// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: class-field-metadata
#[derive(Clone)]
pub struct ServiceOptions {
    pub internal_name: String,
    pub retry_limit: terrane_int_support::Int,
    pub note: Option<String>,
    pub credential: String,
}
impl ServiceOptions {
    pub fn terrane_construct() -> Self {
        Self {
            internal_name: String::from("primary"),
            retry_limit: terrane_int_support::Int::from(3_i128),
            note: None,
            credential: String::from("hidden"),
        }
    }
}
fn main() {
    let descriptor: TerraneDescriptor = TerraneDescriptor {
        identity: "/class-field-metadata::service-options",
        name: "service-options",
        kind: "class",
        inherently_identity_bearing: false,
        fields: &[
            TerraneFieldMetadata {
                name: "internal-name",
                external_name: "serviceName",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "retry-limit",
                external_name: "retry-limit",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "note",
                external_name: "note",
                defaulted: true,
                optional: true,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "credential",
                external_name: "credential",
                defaulted: true,
                optional: false,
                secret: true,
            },
        ],
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(descriptor
        .fields.len() as i128))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(descriptor.fields
        .iter().map(| field | field.name.to_owned()).collect:: < Vec < String > > ()
        .get(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:12:12-12:37 */)).cloned().ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:12:12-12:37 */))), 0 /* terrane-site: case.trn:12:12-12:37 */)),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&__terrane_raised(descriptor.fields.iter()
        .map(| field | field.external_name.to_owned()).collect:: < Vec < String > > ()
        .get(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:12:44-12:78 */)).cloned().ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:12:44-12:78 */))), 1 /* terrane-site: case.trn:12:44-12:78 */))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.defaulted).collect:: < Vec < bool > > ())
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:13:12-13:41 */)), 2 /* terrane-site: case.trn:13:12-13:41 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.optional).collect:: < Vec < bool > > ())
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        3 /* terrane-site: case.trn:13:43-13:71 */)), 3 /* terrane-site: case.trn:13:43-13:71 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.secret).collect:: < Vec < bool > > ())
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        4 /* terrane-site: case.trn:13:73-13:99 */)), 4 /* terrane-site: case.trn:13:73-13:99 */))
    );
}
