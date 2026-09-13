// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: implicit-class-field-defaults
pub static TERRANE_STATIC_DEFAULTS_SHARED: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct Defaults {
    pub message: String,
    pub path: String,
    pub id: terrane_int_support::Int,
    pub active: bool,
    pub something: i8,
    pub ratio: f32,
    pub data: Vec<u8>,
    pub note: Option<String>,
    pub items: terrane_collection_support::List<terrane_int_support::Int>,
    pub names: terrane_collection_support::Map<String, terrane_int_support::Int>,
    pub tags: terrane_collection_support::Set<String>,
    pub unordered_names: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >,
    pub unordered_tags: terrane_collection_support::UnorderedSet<String>,
    pub explicit: terrane_int_support::Int,
}
impl Defaults {
    pub fn terrane_construct(message: String) -> Self {
        let mut value = Self {
            message: String::new(),
            path: String::new(),
            id: terrane_int_support::Int::from(0_i128),
            active: false,
            something: 0,
            ratio: 0.0_f32,
            data: Vec::new(),
            note: None,
            items: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            names: terrane_collection_support::Map::<
                String,
                terrane_int_support::Int,
            >::new(Vec::new()),
            tags: terrane_collection_support::Set::<String>::new(Vec::new()),
            unordered_names: terrane_collection_support::UnorderedMap::<
                String,
                terrane_int_support::Int,
            >::new(Vec::new()),
            unordered_tags: terrane_collection_support::UnorderedSet::<
                String,
            >::new(Vec::new()),
            explicit: terrane_int_support::Int::from(7_i128),
        };
        value.construct(message);
        value
    }
    pub fn construct(&mut self, message: String) {
        self.message = message;
    }
}
#[derive(Clone)]
pub struct Plain {
    pub count: terrane_int_support::Int,
    pub text: String,
}
impl Plain {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(0_i128),
            text: String::new(),
        }
    }
}
fn main() {
    let value: Defaults = Defaults::terrane_construct(String::from("ready"));
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&value.message),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.path),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.id),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.active),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.something),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.ratio)
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(value.data.len()
        as i128)), terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.items
        .length())), terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.names
        .length())), terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.tags
        .length())), terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value
        .unordered_names.length())),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value
        .unordered_tags.length()))
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&value.note.is_none()),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.explicit),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&TERRANE_STATIC_DEFAULTS_SHARED.lock()
        .expect("static field lock poisoned").clone())
    );
    let descriptor: TerraneDescriptor = TerraneDescriptor {
        identity: "/implicit-class-field-defaults::defaults",
        name: "defaults",
        kind: "class",
        inherently_identity_bearing: false,
        fields: &[
            TerraneFieldMetadata {
                name: "message",
                external_name: "message",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "path",
                external_name: "path",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "id",
                external_name: "id",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "active",
                external_name: "active",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "something",
                external_name: "something",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "ratio",
                external_name: "ratio",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "data",
                external_name: "data",
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
                name: "items",
                external_name: "items",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "names",
                external_name: "names",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "tags",
                external_name: "tags",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "unordered-names",
                external_name: "unordered-names",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "unordered-tags",
                external_name: "unordered-tags",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "explicit",
                external_name: "explicit",
                defaulted: true,
                optional: false,
                secret: false,
            },
        ],
    };
    let plain_value: Plain = Plain::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&plain_value.count),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&plain_value.text)
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(descriptor
        .fields.len() as i128)), terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.defaulted).collect:: < Vec < bool > > ())
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:38:39-38:68 */)), 0 /* terrane-site: case.trn:38:39-38:68 */)),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.defaulted).collect:: < Vec < bool > > ())
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(13_i128)),
        1 /* terrane-site: case.trn:38:75-38:105 */)), 1 /* terrane-site: case.trn:38:75-38:105 */))
    );
}
