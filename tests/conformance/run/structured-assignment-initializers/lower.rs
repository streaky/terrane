// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: structured-assignment-initializers
#[derive(Clone)]
pub struct Item {
    pub value: terrane_int_support::Int,
    pub title: String,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            title: String::from("default"),
        }
    }
}
#[derive(Clone)]
pub struct Holder {
    pub child: Item,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            child: {
                let mut instance = Item::terrane_construct();
                {
                    let __terrane_value = terrane_int_support::Int::from(5_i128);
                    instance.value = __terrane_value;
                }
                instance
            },
        }
    }
}
#[derive(Clone)]
pub struct KeywordFields {
    pub __trn_726566: Option<terrane_int_support::Int>,
    pub not: Option<bool>,
    pub __trn_6d6f7665: Option<String>,
}
impl KeywordFields {
    pub fn terrane_construct() -> Self {
        Self {
            __trn_726566: None,
            not: None,
            __trn_6d6f7665: None,
        }
    }
}
fn make() -> Item {
    return {
        let mut instance = Item::terrane_construct();
        {
            let __terrane_value = terrane_int_support::Int::from(2_i128);
            instance.value = __terrane_value;
        }
        {
            let __terrane_value = String::from("returned");
            instance.title = __terrane_value;
        }
        instance
    };
}
fn main() {
    let wrapped: Holder;
    let first: Item;
    let second: Item;
    let returned: Item;
    let mut keyword: KeywordFields;
    wrapped = Holder::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&wrapped.child.value),
        String::from(":"), terrane_scalar_support::scalar_text(&wrapped.child.title)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_SEEDED.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("seeded", "case.trn", 26, 12)))
    );
    first = {
        let mut instance = Item::terrane_construct();
        {
            let __terrane_value = terrane_int_support::Int::from(3_i128);
            instance.value = __terrane_value;
        }
        instance
    };
    second = {
        let mut instance = Item::terrane_construct();
        {
            let __terrane_value = terrane_int_support::Int::from(4_i128);
            instance.value = __terrane_value;
        }
        instance
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first.value), String::from(":"),
        terrane_scalar_support::scalar_text(&second.value)
    );
    returned = make();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&returned.value),
        String::from(":"), terrane_scalar_support::scalar_text(&returned.title)
    );
    keyword = {
        let mut instance = KeywordFields::terrane_construct();
        {
            let __terrane_value = Some(terrane_int_support::Int::from(2_i128));
            instance.__trn_726566 = __terrane_value;
        }
        instance
    };
    {
        let __terrane_value = Some(true);
        keyword.not = __terrane_value;
    }
    {
        let __terrane_value = Some(String::from("named"));
        keyword.__trn_6d6f7665 = __terrane_value;
    }
    println!(
        "{}{}{}{}{}", { let mut __terrane_text = String::new(); match &keyword
        .__trn_726566 { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text },
        String::from(":"), { let mut __terrane_text = String::new(); match &keyword.not {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text },
        String::from(":"), { let mut __terrane_text = String::new(); match &keyword
        .__trn_6d6f7665 { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
}
