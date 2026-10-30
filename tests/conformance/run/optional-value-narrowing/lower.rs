// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-value-narrowing
#[derive(Clone)]
pub struct CounterStorage {
    pub total: terrane_int_support::Int,
    pub tag: Option<String>,
    pub items: terrane_collection_support::List<terrane_int_support::Int>,
}
impl CounterStorage {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
            tag: Some(String::from("tag")),
            items: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
        }
    }
    pub fn bump(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = self.total.clone() + amount.clone();
            self.total = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub enum Counter {
    Own(CounterStorage),
    DerivedCounter(DerivedCounter),
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self::Own(CounterStorage::terrane_construct())
    }
    pub fn bump(&mut self, amount: terrane_int_support::Int) {
        match self {
            Self::Own(value) => value.bump(amount),
            Self::DerivedCounter(value) => value.bump(amount),
        }
    }
    pub fn terrane_field_total(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.total,
            Self::DerivedCounter(value) => &value.total,
        }
    }
    pub fn terrane_field_total_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.total,
            Self::DerivedCounter(value) => &mut value.total,
        }
    }
    pub fn terrane_field_tag(&self) -> &Option<String> {
        match self {
            Self::Own(value) => &value.tag,
            Self::DerivedCounter(value) => &value.tag,
        }
    }
    pub fn terrane_field_tag_mut(&mut self) -> &mut Option<String> {
        match self {
            Self::Own(value) => &mut value.tag,
            Self::DerivedCounter(value) => &mut value.tag,
        }
    }
    pub fn terrane_field_items(
        &self,
    ) -> &terrane_collection_support::List<terrane_int_support::Int> {
        match self {
            Self::Own(value) => &value.items,
            Self::DerivedCounter(value) => &value.items,
        }
    }
    pub fn terrane_field_items_mut(
        &mut self,
    ) -> &mut terrane_collection_support::List<terrane_int_support::Int> {
        match self {
            Self::Own(value) => &mut value.items,
            Self::DerivedCounter(value) => &mut value.items,
        }
    }
}
#[derive(Clone)]
pub struct DerivedCounter {
    pub total: terrane_int_support::Int,
    pub tag: Option<String>,
    pub items: terrane_collection_support::List<terrane_int_support::Int>,
    pub extra: terrane_int_support::Int,
}
impl DerivedCounter {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
            tag: Some(String::from("tag")),
            items: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            extra: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn bump(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = self.total.clone() + amount.clone();
            self.total = __terrane_value;
        }
    }
}
fn mutate_counter(value: Option<Counter>) {
    let value = std::sync::Arc::new(TerraneSharedCell::new(value));
    let __trn_5f6f62736572766572: std::sync::Arc<TerraneSharedCell<Option<Counter>>>;
    __trn_5f6f62736572766572 = value.clone();
    let _ = &__trn_5f6f62736572766572;
    if {
        let __terrane_present = (&value)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        {
            let __terrane_owner = value.clone();
            let __terrane_shared_argument_0 = terrane_int_support::Int::from(1_i128);
            let __terrane_call_result = match &mut *__terrane_owner
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .bump(__terrane_shared_argument_0);
            __terrane_call_result
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
            __terrane_result = match &mut * (&value).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_total()
            .clone(); __terrane_result }; __terrane_snapshot })
        );
        {
            let __terrane_owner = value.clone();
            let __terrane_shared_argument_0 = {
                let __terrane_snapshot = {
                    let __terrane_result = match &mut *(&value)
                        .lock()
                        .expect("reference lock poisoned")
                    {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    }
                        .terrane_field_total()
                        .clone();
                    __terrane_result
                };
                __terrane_snapshot
            };
            let __terrane_shared_argument_1 = __terrane_shared_argument_0;
            let __terrane_call_result = match &mut *__terrane_owner
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .bump(__terrane_shared_argument_1);
            __terrane_call_result
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
            __terrane_result = match &mut * (&value).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_total()
            .clone(); __terrane_result }; __terrane_snapshot })
        );
        {
            let __terrane_owner = value.clone();
            let __terrane_value = terrane_int_support::Int::from(10_i128);
            *match &mut *__terrane_owner.lock().expect("reference lock poisoned") {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .terrane_field_total_mut() = __terrane_value;
        }
        {
            let __terrane_owner = value.clone();
            let __terrane_value = {
                let __terrane_snapshot = {
                    let __terrane_result = match &mut *(&value)
                        .lock()
                        .expect("reference lock poisoned")
                    {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    }
                        .terrane_field_total()
                        .clone();
                    __terrane_result
                };
                __terrane_snapshot
            } + terrane_int_support::Int::from(1_i128);
            *match &mut *__terrane_owner.lock().expect("reference lock poisoned") {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .terrane_field_total_mut() = __terrane_value;
        }
        {
            let __terrane_owner = value.clone();
            let __terrane_collection_argument_0 = {
                let __terrane_snapshot = {
                    let __terrane_result = match &mut *(&value)
                        .lock()
                        .expect("reference lock poisoned")
                    {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    }
                        .terrane_field_total()
                        .clone();
                    __terrane_result
                };
                __terrane_snapshot
            };
            match &mut *__terrane_owner.lock().expect("reference lock poisoned") {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .terrane_field_items_mut()
                .append(__terrane_collection_argument_0);
        };
        {
            let __terrane_owner = value.clone();
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &({
                        let __terrane_snapshot = {
                            let __terrane_result = match &mut *(&value)
                                .lock()
                                .expect("reference lock poisoned")
                            {
                                Some(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            }
                                .terrane_field_total()
                                .clone();
                            __terrane_result
                        };
                        __terrane_snapshot
                    } - terrane_int_support::Int::from(11_i128)),
                ),
                0 /* terrane-site: case.trn:24:9-24:56 */,
            );
            let __terrane_value = {
                let __terrane_snapshot = {
                    let __terrane_result = match &mut *(&value)
                        .lock()
                        .expect("reference lock poisoned")
                    {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    }
                        .terrane_field_total()
                        .clone();
                    __terrane_result
                };
                __terrane_snapshot
            } + terrane_int_support::Int::from(1_i128);
            let _ = __terrane_raised(
                match &mut *__terrane_owner.lock().expect("reference lock poisoned") {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .terrane_field_items_mut()
                    .set(__terrane_index, __terrane_value),
                0 /* terrane-site: case.trn:24:9-24:56 */,
            );
        }
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            { let __terrane_result = match &mut * (&value).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_total()
            .clone(); __terrane_result }; __terrane_snapshot }), String::from("/"),
            terrane_scalar_support::scalar_text(&{ let __terrane_owner = &value;
            __terrane_raised(match &* __terrane_owner.lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_items()
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:25:34-25:48 */)), 1 /* terrane-site: case.trn:25:34-25:48 */) }), String::from("/"),
            terrane_scalar_support::scalar_text(&{ let __terrane_owner = &value;
            __terrane_raised(match &* __terrane_owner.lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_items()
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: case.trn:25:55-25:69 */)), 2 /* terrane-site: case.trn:25:55-25:69 */) })
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &value;
            match &* __terrane_owner.lock().expect("reference lock poisoned") {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .terrane_field_tag().is_some() }), terrane_scalar_support::scalar_text(&{ let
            __terrane_owner = &value; match &* __terrane_owner.lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_tag()
            .is_none() })
        );
        {
            let __terrane_owner = value.clone();
            let __terrane_value = None;
            *match &mut *__terrane_owner.lock().expect("reference lock poisoned") {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .terrane_field_tag_mut() = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &value;
            match &* __terrane_owner.lock().expect("reference lock poisoned") {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .terrane_field_tag().is_none() })
        );
    }
}
fn helper() {
    let found: String;
    found = String::from("shadow");
    println!("{}", terrane_scalar_support::scalar_text(&found));
}
fn show(value: Option<i8>) {
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
fn maybe() -> Option<i8> {
    return Some(4);
}
fn missing() -> Option<i8> {
    return None;
}
fn enabled() -> bool {
    return true;
}
fn shared_snapshot(value: Option<String>) -> String {
    let value = std::sync::Arc::new(TerraneSharedCell::new(value));
    let __trn_5f6f776e6572: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    __trn_5f6f776e6572 = value.clone();
    let _ = &__trn_5f6f776e6572;
    if {
        let __terrane_present = (&value)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        return {
            let __terrane_value = match &*(&value)
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone();
            __terrane_value
        };
    }
    return String::from("missing");
}
fn main() {
    let mut value: Option<i8>;
    let other: Option<i8>;
    let returned: Option<i8>;
    let called: Option<i8>;
    let missingvalue: Option<i8>;
    let found: Option<terrane_string_support::TextRange>;
    let sharedvalue: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    let __trn_5f6f62736572766572: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    let snapshot: String;
    let sharednumber: std::sync::Arc<TerraneSharedCell<Option<i8>>>;
    let __trn_5f6e756d6265726f776e6572: std::sync::Arc<TerraneSharedCell<Option<i8>>>;
    let mut late: Option<std::sync::Arc<TerraneSharedCell<Option<String>>>> = None;
    let __trn_5f6c6174656f776e6572: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    let mut latecounter: Option<std::sync::Arc<TerraneSharedCell<Option<Counter>>>> = None;
    let __trn_5f636f756e7465726f776e6572: std::sync::Arc<
        TerraneSharedCell<Option<Counter>>,
    >;
    value = Some(7);
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
        value = None;
        println!("{}", terrane_scalar_support::scalar_text(&true));
    }
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    other = Some(8);
    if (&other).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &other { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    show(Some(9));
    show(None);
    returned = maybe();
    if (&returned).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &returned { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    called = maybe();
    if (&called).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &called { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missingvalue = None;
    if (&missingvalue).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &missingvalue {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missing();
    println!("{}", terrane_scalar_support::scalar_text(&true));
    helper();
    found = {
        let __terrane_receiver_value = &String::from("banana");
        let __terrane_string_argument_0 = &String::from("ana");
        terrane_string_support::find(
            &__terrane_receiver_value,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    if (&found).is_some() {
        if (&found).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") } .text()
                .to_owned())
            );
        }
    }
    sharedvalue = std::sync::Arc::new(
        TerraneSharedCell::new(Some(String::from("retained"))),
    );
    __trn_5f6f62736572766572 = sharedvalue.clone();
    let _ = &__trn_5f6f62736572766572;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_some();
        __terrane_present }), terrane_scalar_support::scalar_text(&{ let
        __terrane_present = (&sharedvalue).lock().expect("reference lock poisoned")
        .is_none(); __terrane_present })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_some();
        __terrane_present }), { let mut __terrane_text = String::new(); { let
        __terrane_result = match &* sharedvalue.lock().expect("reference lock poisoned")
        { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), }; __terrane_result
        } __terrane_text }
    );
    if {
        let __terrane_present = (&sharedvalue)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
            (&sharedvalue).lock().expect("reference lock poisoned").is_none();
            __terrane_present })
        );
        println!(
            "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&match
            &* (&sharedvalue).lock().expect("reference lock poisoned") { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") });
            __terrane_result }, { let __terrane_result =
            terrane_scalar_support::scalar_text(&match &* (&sharedvalue).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }); __terrane_result }
        );
        snapshot = {
            let __terrane_value = match &*(&sharedvalue)
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone();
            __terrane_value
        };
        *sharedvalue.lock().expect("reference lock poisoned") = Some(
            String::from("changed"),
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&snapshot), { let mut
            __terrane_text = String::new(); { let __terrane_result = match &* sharedvalue
            .lock().expect("reference lock poisoned") { Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), };
            __terrane_result } __terrane_text }
        );
    }
    *sharedvalue.lock().expect("reference lock poisoned") = None;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_none();
        __terrane_present })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_some();
        __terrane_present }), terrane_scalar_support::scalar_text(&{ let
        __terrane_present = (&sharedvalue).lock().expect("reference lock poisoned")
        .is_none(); __terrane_present })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&shared_snapshot(Some(String::from("parameter"))))
    );
    sharednumber = std::sync::Arc::new(TerraneSharedCell::new(Some(7)));
    __trn_5f6e756d6265726f776e6572 = sharednumber.clone();
    let _ = &__trn_5f6e756d6265726f776e6572;
    if {
        let __terrane_present = (&sharednumber)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        println!(
            "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&* match
            &* (&sharednumber).lock().expect("reference lock poisoned") { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") });
            __terrane_result }, { let __terrane_result =
            terrane_scalar_support::scalar_text(&* match &* (&sharednumber).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }); __terrane_result }
        );
    }
    if enabled() {
        let _ = late
            .insert(
                std::sync::Arc::new(TerraneSharedCell::new(Some(String::from("late")))),
            );
        __trn_5f6c6174656f776e6572 = late
            .as_ref()
            .expect("flow-proven available binding")
            .clone();
        let _ = &__trn_5f6c6174656f776e6572;
    }
    if {
        let __terrane_present = late
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "late",
                "case.trn",
                97,
                8,
            ))
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        println!(
            "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&match
            &* late.as_ref().unwrap_or_else(| | __terrane_uninitialized_binding("late",
            "case.trn", 98, 16)).lock().expect("reference lock poisoned") { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") });
            __terrane_result }, { let __terrane_result =
            terrane_scalar_support::scalar_text(&match &* late.as_ref().unwrap_or_else(|
            | __terrane_uninitialized_binding("late", "case.trn", 98, 22)).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }); __terrane_result }
        );
    }
    mutate_counter(Some(Counter::terrane_construct()));
    mutate_counter(Some(Counter::DerivedCounter(DerivedCounter::terrane_construct())));
    if enabled() {
        let _ = latecounter
            .insert(
                std::sync::Arc::new(
                    TerraneSharedCell::new(Some(Counter::terrane_construct())),
                ),
            );
        __trn_5f636f756e7465726f776e6572 = latecounter
            .as_ref()
            .expect("flow-proven available binding")
            .clone();
        let _ = &__trn_5f636f756e7465726f776e6572;
    }
    if {
        let __terrane_present = latecounter
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "latecounter",
                "case.trn",
                104,
                8,
            ))
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        {
            let __terrane_owner = latecounter
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "latecounter",
                    "case.trn",
                    105,
                    9,
                ))
                .clone();
            let __terrane_shared_argument_0 = terrane_int_support::Int::from(1_i128);
            let __terrane_call_result = match &mut *__terrane_owner
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .bump(__terrane_shared_argument_0);
            __terrane_call_result
        };
        {
            let __terrane_owner = latecounter
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "latecounter",
                    "case.trn",
                    106,
                    9,
                ))
                .clone();
            let __terrane_value = {
                let __terrane_snapshot = {
                    let __terrane_result = match &mut *latecounter
                        .as_ref()
                        .unwrap_or_else(|| __terrane_uninitialized_binding(
                            "latecounter",
                            "case.trn",
                            106,
                            29,
                        ))
                        .lock()
                        .expect("reference lock poisoned")
                    {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    }
                        .terrane_field_total()
                        .clone();
                    __terrane_result
                };
                __terrane_snapshot
            } + terrane_int_support::Int::from(9_i128);
            *match &mut *__terrane_owner.lock().expect("reference lock poisoned") {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .terrane_field_total_mut() = __terrane_value;
        }
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
            __terrane_result = match &mut * latecounter.as_ref().unwrap_or_else(| |
            __terrane_uninitialized_binding("latecounter", "case.trn", 107, 16)).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_total()
            .clone(); __terrane_result }; __terrane_snapshot }),
            terrane_scalar_support::scalar_text(&{ let __terrane_owner = latecounter
            .as_ref().unwrap_or_else(| | __terrane_uninitialized_binding("latecounter",
            "case.trn", 107, 35)); match &* __terrane_owner.lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .terrane_field_tag()
            .is_some() })
        );
    }
}
