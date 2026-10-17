// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/app/main.trn
// Namespace: annotation-codec-consumer
fn expect_invalid(input: String) -> Result<(), TerraneError> {
    let parsed: DocumentResult;
    let decoded: AccountDecoded;
    parsed = parse_json(input, default_json_options());
    if parsed.failed {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                0 /* terrane-site: src/app/main.trn:12:9-12:29 */,
            ),
        );
    }
    decoded = decode_account(parsed.value.clone());
    if !decoded.failed {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                1 /* terrane-site: src/app/main.trn:15:9-15:29 */,
            ),
        );
    }
    Ok(())
}
fn main() {
    let parsed: DocumentResult;
    let decoded: AccountDecoded;
    let value: Option<Account>;
    let encoded: AccountEncoded;
    let document: Option<DocumentValue>;
    let text: DocumentResult;
    let defaults_input: DocumentResult;
    let defaults: AccountDecoded;
    let default_value: Option<Account>;
    let nickname: Option<String>;
    let mut invalid_value: Account;
    let invalid_encoded: AccountEncoded;
    let mut profile_value: Profile;
    let profile_encoded: ProfileEncoded;
    let profile_document: Option<DocumentValue>;
    let profile_decoded: ProfileDecoded;
    let profile_roundtrip: Option<Profile>;
    parsed = parse_json(
        String::from(
            "{\"accountId\":7,\"label\":\"A\",\"active\":false,\"age\":null,\"nickname\":null}",
        ),
        default_json_options(),
    );
    if parsed.failed {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                2 /* terrane-site: src/app/main.trn:20:9-20:29 */,
            ),
        );
    }
    decoded = decode_account(parsed.value.clone());
    if decoded.failed {
        println!("{}", terrane_scalar_support::scalar_text(&decoded.message));
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                3 /* terrane-site: src/app/main.trn:24:9-24:29 */,
            ),
        );
    }
    value = decoded.value.clone();
    if value.is_some() {
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&match &value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") } .account_id),
            terrane_scalar_support::scalar_text(&match &value { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .display_name),
            terrane_scalar_support::scalar_text(&match &value { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .active),
            terrane_scalar_support::scalar_text(&match &value { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .age.is_none()),
            terrane_scalar_support::scalar_text(&match &value { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .nickname.is_none())
        );
        encoded = encode_account(
            match &value {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone(),
        );
        if encoded.failed {
            __terrane_uncaught(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    4 /* terrane-site: src/app/main.trn:30:13-30:33 */,
                ),
            );
        }
        document = encoded.value.clone();
        if document.is_some() {
            text = canonical_json(
                match document {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
            );
            println!("{}", terrane_scalar_support::scalar_text(&text.value.encoded));
        } else {
            __terrane_uncaught(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    5 /* terrane-site: src/app/main.trn:36:13-36:33 */,
                ),
            );
        }
    } else {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                6 /* terrane-site: src/app/main.trn:38:9-38:29 */,
            ),
        );
    }
    defaults_input = parse_json(
        String::from("{\"accountId\":8}"),
        default_json_options(),
    );
    defaults = decode_account(defaults_input.value.clone());
    if defaults.failed {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                7 /* terrane-site: src/app/main.trn:42:9-42:29 */,
            ),
        );
    }
    default_value = defaults.value.clone();
    if default_value.is_some() {
        nickname = match &default_value {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .nickname
            .clone();
        if nickname.is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&match &default_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                } .display_name), terrane_scalar_support::scalar_text(&match
                &default_value { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") } .active),
                terrane_scalar_support::scalar_text(&match &nickname { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") })
            );
        } else {
            __terrane_uncaught(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    8 /* terrane-site: src/app/main.trn:49:13-49:33 */,
                ),
            );
        }
    } else {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                9 /* terrane-site: src/app/main.trn:51:9-51:29 */,
            ),
        );
    }
    __terrane_traced(
        expect_invalid(String::from("{}")),
        10 /* terrane-site: src/app/main.trn:52:5-52:25 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":\"bad\"}")),
        11 /* terrane-site: src/app/main.trn:53:5-53:42 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":0}")),
        12 /* terrane-site: src/app/main.trn:54:5-54:38 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":101}")),
        13 /* terrane-site: src/app/main.trn:55:5-55:40 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":999999999999}")),
        14 /* terrane-site: src/app/main.trn:56:5-56:49 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":7,\"secret-note\":\"injected\"}")),
        15 /* terrane-site: src/app/main.trn:57:5-57:63 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":7,\"credential\":\"injected\"}")),
        16 /* terrane-site: src/app/main.trn:58:5-58:62 */,
    );
    __terrane_traced(
        expect_invalid(String::from("{\"accountId\":7,\"active\":\"false\"}")),
        17 /* terrane-site: src/app/main.trn:59:5-59:55 */,
    );
    invalid_value = Account::terrane_construct();
    invalid_value.account_id = 0;
    invalid_encoded = encode_account(invalid_value);
    if !invalid_encoded.failed {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                18 /* terrane-site: src/app/main.trn:64:9-64:29 */,
            ),
        );
    }
    println!("{}", String::from("invalid inputs rejected"));
    profile_value = Profile::terrane_construct();
    profile_value.handle = String::from("works");
    profile_encoded = encode_profile(profile_value);
    if profile_encoded.failed {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                19 /* terrane-site: src/app/main.trn:70:9-70:29 */,
            ),
        );
    }
    profile_document = profile_encoded.value.clone();
    if profile_document.is_some() {
        profile_decoded = decode_profile(
            match profile_document {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            },
        );
        if profile_decoded.failed {
            __terrane_uncaught(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    20 /* terrane-site: src/app/main.trn:75:13-75:33 */,
                ),
            );
        }
        profile_roundtrip = profile_decoded.value.clone();
        if profile_roundtrip.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &profile_roundtrip {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                } .handle)
            );
        } else {
            __terrane_uncaught(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    21 /* terrane-site: src/app/main.trn:80:13-80:33 */,
                ),
            );
        }
    } else {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                22 /* terrane-site: src/app/main.trn:82:9-82:29 */,
            ),
        );
    }
}
// Source: src/codec-schema/main.trn
// Namespace: annotation-codec
#[derive(Clone)]
pub struct ExternalName {
    pub value: String,
}
impl ExternalName {
    pub fn terrane_construct() -> Self {
        Self { value: String::new() }
    }
}
#[derive(Clone)]
pub struct Required {}
impl Required {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
#[derive(Clone)]
pub struct Secret {}
impl Secret {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
#[derive(Clone)]
pub struct Minimum {
    pub value: terrane_int_support::Int,
}
impl Minimum {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct Maximum {
    pub value: terrane_int_support::Int,
}
impl Maximum {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct Description {
    pub value: String,
}
impl Description {
    pub fn terrane_construct() -> Self {
        Self { value: String::new() }
    }
}
// Source: src/model/main.trn
// Namespace: codec-model
#[derive(Clone)]
pub struct Account {
    pub account_id: i16,
    pub display_name: String,
    pub active: bool,
    pub age: Option<i8>,
    pub nickname: Option<String>,
    pub secret_note: String,
    pub credential: String,
}
impl Account {
    pub fn terrane_construct() -> Self {
        Self {
            account_id: 0,
            display_name: String::from("defaulted"),
            active: true,
            age: None,
            nickname: Some(String::from("fallback")),
            secret_note: String::from("do not serialize"),
            credential: String::from("do not leak"),
        }
    }
}
#[derive(Clone)]
pub struct Profile {
    pub handle: String,
}
impl Profile {
    pub fn terrane_construct() -> Self {
        Self { handle: String::new() }
    }
}
// Source: .trn/generated/codec/typed-codec-_2fcodec-model_3a_3aaccount.trn
// Namespace: generated/annotation-codec/codec-model/account
#[derive(Clone)]
pub struct AccountDecoded {
    pub failed: bool,
    pub message: String,
    pub value: Option<Account>,
}
impl AccountDecoded {
    pub fn terrane_construct() -> Self {
        Self {
            failed: false,
            message: String::from(""),
            value: None,
        }
    }
}
#[derive(Clone)]
pub struct AccountEncoded {
    pub failed: bool,
    pub message: String,
    pub value: Option<DocumentValue>,
}
impl AccountEncoded {
    pub fn terrane_construct() -> Self {
        Self {
            failed: false,
            message: String::from(""),
            value: None,
        }
    }
}
fn fail_decoded_terrane_generated_annotation_codec_codec_model_account(
    message: String,
) -> AccountDecoded {
    let mut result: AccountDecoded;
    result = AccountDecoded::terrane_construct();
    result.failed = true;
    result.message = message;
    return result;
}
fn fail_encoded_terrane_generated_annotation_codec_codec_model_account(
    message: String,
) -> AccountEncoded {
    let mut result: AccountEncoded;
    result = AccountEncoded::terrane_construct();
    result.failed = true;
    result.message = message;
    return result;
}
fn decode_account(document: DocumentValue) -> AccountDecoded {
    let key_count: terrane_int_support::Int;
    let mut index: terrane_int_support::Int;
    let mut key: String;
    let mut value: Account;
    let field_slot0: DocumentResult;
    let mut parsed_value0: Option<i16> = None;
    let field_slot1: DocumentResult;
    let field_slot2: DocumentResult;
    let field_slot3: DocumentResult;
    let mut parsed_value3: Option<i8> = None;
    let field_slot4: DocumentResult;
    let mut result: AccountDecoded;
    if document.kind.as_str() != "map" {
        return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
            String::from("codec expects an object"),
        );
    }
    key_count = document.length();
    index = terrane_int_support::Int::from(0_i128);
    while index.clone() < key_count.clone() {
        key = document.key(index.clone());
        if key.as_str() != "accountId" && key.as_str() != "label"
            && key.as_str() != "active" && key.as_str() != "age"
            && key.as_str() != "nickname"
        {
            return fail_decoded_terrane_generated_annotation_codec_codec_model_account({
                let mut __terrane_text = String::with_capacity(31);
                __terrane_text.push_str("unknown or inaccessible field: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key,
                    &mut __terrane_text,
                );
                __terrane_text
            });
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    value = Account::terrane_construct();
    field_slot0 = document.field(String::from("accountId"));
    if field_slot0.failed {
        return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
            String::from("missing required field: accountId"),
        );
    }
    if !field_slot0.failed {
        if field_slot0.value.kind.as_str() != "integer" {
            return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                String::from("wrong document type for accountId"),
            );
        }
        let __terrane_completion_0: TerraneCompletion<AccountDecoded> = (|| {
            let __terrane_try_0: TerraneCompletion<AccountDecoded> = (|| {
                let _ = parsed_value0
                    .insert(
                        __terrane_raised_completion!(
                            terrane_int_support::coerce:: < i16 >
                            (&__terrane_raised_completion!(terrane_int_support::parse_radix(&field_slot0
                            .value.scalar, &10), 23 /* terrane-site: .trn/generated/codec/typed-codec-_2fcodec-model_3a_3aaccount.trn:46:36-46:70 */)),
                            24 /* terrane-site: .trn/generated/codec/typed-codec-_2fcodec-model_3a_3aaccount.trn:46:35-46:71 */
                        ),
                    );
                if *parsed_value0.as_ref().expect("flow-proven available binding") < 1 {
                    return TerraneCompletion::Return(
                        fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                            String::from("accountId violates minimum"),
                        ),
                    );
                }
                if *parsed_value0.as_ref().expect("flow-proven available binding") > 100
                {
                    return TerraneCompletion::Return(
                        fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                            String::from("accountId violates maximum"),
                        ),
                    );
                }
                value.account_id = *parsed_value0
                    .as_ref()
                    .expect("flow-proven available binding");
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        return TerraneCompletion::Return(
                            fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                                String::from("integer out of range for accountId"),
                            ),
                        );
                    }
                    if !__terrane_handled_0
                        && __terrane_error_0.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_0 = true;
                        return TerraneCompletion::Return(
                            fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                                String::from("integer out of range for accountId"),
                            ),
                        );
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    field_slot1 = document.field(String::from("label"));
    if !field_slot1.failed {
        if field_slot1.value.kind.as_str() != "string" {
            return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                String::from("wrong document type for label"),
            );
        }
        value.display_name = field_slot1.value.scalar.clone();
    }
    field_slot2 = document.field(String::from("active"));
    if !field_slot2.failed {
        if field_slot2.value.kind.as_str() != "bool" {
            return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                String::from("wrong document type for active"),
            );
        }
        value.active = field_slot2.value.scalar.as_str() == "true";
    }
    field_slot3 = document.field(String::from("age"));
    if !field_slot3.failed {
        if field_slot3.value.kind.as_str() == "none" {
            value.age = None;
        } else {
            if field_slot3.value.kind.as_str() != "integer" {
                return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                    String::from("wrong document type for age"),
                );
            }
            let __terrane_completion_1: TerraneCompletion<AccountDecoded> = (|| {
                let __terrane_try_1: TerraneCompletion<AccountDecoded> = (|| {
                    let _ = parsed_value3
                        .insert(
                            __terrane_raised_completion!(
                                terrane_int_support::coerce:: < i8 >
                                (&__terrane_raised_completion!(terrane_int_support::parse_radix(&field_slot3
                                .value.scalar, &10), 25 /* terrane-site: .trn/generated/codec/typed-codec-_2fcodec-model_3a_3aaccount.trn:74:39-74:73 */)),
                                26 /* terrane-site: .trn/generated/codec/typed-codec-_2fcodec-model_3a_3aaccount.trn:74:38-74:74 */
                            ),
                        );
                    value.age = Some(
                        *parsed_value3.as_ref().expect("flow-proven available binding"),
                    );
                    TerraneCompletion::Normal
                })();
                match __terrane_try_1 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_1) => {
                        let mut __terrane_handled_1 = false;
                        if !__terrane_handled_1
                            && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                        {
                            __terrane_handled_1 = true;
                            return TerraneCompletion::Return(
                                fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                                    String::from("integer out of range for age"),
                                ),
                            );
                        }
                        if !__terrane_handled_1
                            && __terrane_error_1.kind
                                == TerraneErrorKind::IntegerConversionOverflow
                        {
                            __terrane_handled_1 = true;
                            return TerraneCompletion::Return(
                                fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                                    String::from("integer out of range for age"),
                                ),
                            );
                        }
                        if !__terrane_handled_1 {
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            match __terrane_completion_1 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
        }
    }
    field_slot4 = document.field(String::from("nickname"));
    if !field_slot4.failed {
        if field_slot4.value.kind.as_str() == "none" {
            value.nickname = None;
        } else {
            if field_slot4.value.kind.as_str() != "string" {
                return fail_decoded_terrane_generated_annotation_codec_codec_model_account(
                    String::from("wrong document type for nickname"),
                );
            }
            value.nickname = Some(field_slot4.value.scalar.clone());
        }
    }
    result = AccountDecoded::terrane_construct();
    result.value = Some(value);
    return result;
}
fn encode_account(value: Account) -> AccountEncoded {
    let mut entries: DocumentMapEntries;
    let encoded_value0: DocumentResult;
    let value_field3: Option<i8>;
    let encoded_value3: DocumentResult;
    let value_field4: Option<String>;
    let document: DocumentResult;
    let mut result: AccountEncoded;
    entries = DocumentMapEntries::terrane_construct();
    if value.account_id < 1 {
        return fail_encoded_terrane_generated_annotation_codec_codec_model_account(
            String::from("accountId violates minimum"),
        );
    }
    if value.account_id > 100 {
        return fail_encoded_terrane_generated_annotation_codec_codec_model_account(
            String::from("accountId violates maximum"),
        );
    }
    encoded_value0 = make_document_integer({
        let mut __terrane_text = String::with_capacity(0);
        __terrane_text.push_str("");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &value.account_id,
            &mut __terrane_text,
        );
        __terrane_text
    });
    if encoded_value0.failed {
        return fail_encoded_terrane_generated_annotation_codec_codec_model_account(
            encoded_value0.message.clone(),
        );
    }
    entries.append(String::from("accountId"), encoded_value0.value.clone());
    entries
        .append(String::from("label"), make_document_string(value.display_name.clone()));
    entries.append(String::from("active"), make_document_bool(value.active));
    value_field3 = value.age;
    if value_field3.is_some() {
        encoded_value3 = make_document_integer({
            let mut __terrane_text = String::with_capacity(0);
            __terrane_text.push_str("");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &*match &value_field3 {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
                &mut __terrane_text,
            );
            __terrane_text
        });
        if encoded_value3.failed {
            return fail_encoded_terrane_generated_annotation_codec_codec_model_account(
                encoded_value3.message.clone(),
            );
        }
        entries.append(String::from("age"), encoded_value3.value.clone());
    } else {
        entries.append(String::from("age"), make_document_none());
    }
    value_field4 = value.nickname.clone();
    if value_field4.is_some() {
        entries
            .append(
                String::from("nickname"),
                make_document_string(
                    match value_field4 {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    },
                ),
            );
    } else {
        entries.append(String::from("nickname"), make_document_none());
    }
    document = make_document_map(entries);
    if document.failed {
        return fail_encoded_terrane_generated_annotation_codec_codec_model_account(
            document.message.clone(),
        );
    }
    result = AccountEncoded::terrane_construct();
    result.value = Some(document.value.clone());
    return result;
}
// Source: .trn/generated/profile/typed-codec-_2fcodec-model_3a_3aprofile.trn
// Namespace: generated/annotation-codec/codec-model/profile
#[derive(Clone)]
pub struct ProfileDecoded {
    pub failed: bool,
    pub message: String,
    pub value: Option<Profile>,
}
impl ProfileDecoded {
    pub fn terrane_construct() -> Self {
        Self {
            failed: false,
            message: String::from(""),
            value: None,
        }
    }
}
#[derive(Clone)]
pub struct ProfileEncoded {
    pub failed: bool,
    pub message: String,
    pub value: Option<DocumentValue>,
}
impl ProfileEncoded {
    pub fn terrane_construct() -> Self {
        Self {
            failed: false,
            message: String::from(""),
            value: None,
        }
    }
}
fn fail_decoded_terrane_generated_annotation_codec_codec_model_profile(
    message: String,
) -> ProfileDecoded {
    let mut result: ProfileDecoded;
    result = ProfileDecoded::terrane_construct();
    result.failed = true;
    result.message = message;
    return result;
}
fn fail_encoded_terrane_generated_annotation_codec_codec_model_profile(
    message: String,
) -> ProfileEncoded {
    let mut result: ProfileEncoded;
    result = ProfileEncoded::terrane_construct();
    result.failed = true;
    result.message = message;
    return result;
}
fn decode_profile(document: DocumentValue) -> ProfileDecoded {
    let key_count: terrane_int_support::Int;
    let mut index: terrane_int_support::Int;
    let mut key: String;
    let mut value: Profile;
    let field_slot0: DocumentResult;
    let mut result: ProfileDecoded;
    if document.kind.as_str() != "map" {
        return fail_decoded_terrane_generated_annotation_codec_codec_model_profile(
            String::from("codec expects an object"),
        );
    }
    key_count = document.length();
    index = terrane_int_support::Int::from(0_i128);
    while index.clone() < key_count.clone() {
        key = document.key(index.clone());
        if key.as_str() != "handle" {
            return fail_decoded_terrane_generated_annotation_codec_codec_model_profile({
                let mut __terrane_text = String::with_capacity(31);
                __terrane_text.push_str("unknown or inaccessible field: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key,
                    &mut __terrane_text,
                );
                __terrane_text
            });
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    value = Profile::terrane_construct();
    field_slot0 = document.field(String::from("handle"));
    if !field_slot0.failed {
        if field_slot0.value.kind.as_str() != "string" {
            return fail_decoded_terrane_generated_annotation_codec_codec_model_profile(
                String::from("wrong document type for handle"),
            );
        }
        value.handle = field_slot0.value.scalar.clone();
    }
    result = ProfileDecoded::terrane_construct();
    result.value = Some(value);
    return result;
}
fn encode_profile(value: Profile) -> ProfileEncoded {
    let mut entries: DocumentMapEntries;
    let document: DocumentResult;
    let mut result: ProfileEncoded;
    entries = DocumentMapEntries::terrane_construct();
    entries.append(String::from("handle"), make_document_string(value.handle.clone()));
    document = make_document_map(entries);
    if document.failed {
        return fail_encoded_terrane_generated_annotation_codec_codec_model_profile(
            document.message.clone(),
        );
    }
    result = ProfileEncoded::terrane_construct();
    result.value = Some(document.value.clone());
    return result;
}
