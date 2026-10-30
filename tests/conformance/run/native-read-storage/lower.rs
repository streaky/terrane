// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Holder {
    pub text: String,
    pub data: Vec<u8>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            text: String::from("source storage"),
            data: Vec::<u8>::from([65, 66, 67]),
        }
    }
}
#[derive(Clone)]
pub struct Inspector {}
impl Inspector {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn marker(&self) -> bool {
        return true;
    }
}
impl InspectProtocol for Inspector {
    fn clone_box(&self) -> Box<dyn InspectProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InspectProtocol> {
        Box::new(self.clone())
    }
    fn marker(&self) -> bool {
        Inspector::marker(&*self)
    }
    fn inspect_text(&self, left: &str, right: &str) -> bool {
        || -> Result<bool, crate::TerraneForeignError> {
            let __terrane_default = <Inspector as witness::Inspect>::inspect_text(
                &*self,
                left,
                right,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn inspect_strings(
        &self,
        left: &std::vec::Vec<String>,
        right: &std::vec::Vec<String>,
    ) -> bool {
        || -> Result<bool, crate::TerraneForeignError> {
            let __terrane_default = <Inspector as witness::Inspect>::inspect_strings(
                &*self,
                left,
                right,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl From<Inspector> for Inspect {
    fn from(value: Inspector) -> Self {
        Self(Box::new(value))
    }
}
impl witness::Inspect for Inspector {
    fn marker(&self) -> bool {
        let __terrane_boundary: Result<bool, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Inspector>::marker(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn read_holder(item: &Holder) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item.text,
        &item.text), 0 /* terrane-site: src/main.trn:15:13-15:44 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_string(&item
        .text, &item.text), 1 /* terrane-site: src/main.trn:16:13-16:46 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&item
        .data, &item.data), 2 /* terrane-site: src/main.trn:17:13-17:45 */))
    );
}
fn main() {
    __terrane_run(async move {
        let text: String;
        let data: Vec<u8>;
        let mut item: Holder;
        let mut values: terrane_collection_support::List<String>;
        let values_copy: terrane_collection_support::List<String>;
        let mut native: Record;
        let copy: Record;
        let reader: std::sync::Arc<dyn Fn(String, String) -> bool + Send + Sync>;
        let shared: std::sync::Arc<TerraneSharedCell<Holder>>;
        let pending;
        let async_reader: std::sync::Arc<
            dyn Fn(
                String,
            ) -> std::pin::Pin<Box<dyn Future<Output = String>>> + Send + Sync,
        >;
        let erased: Inspect;
        let observer: &Inspect;
        let inspect_callback: std::sync::Arc<
            dyn Fn(String, String) -> bool + Send + Sync,
        >;
        let mut word: &String;
        let list_reader: std::sync::Arc<
            dyn Fn(
                terrane_collection_support::List<String>,
                terrane_collection_support::List<String>,
            ) -> bool + Send + Sync,
        >;
        let list_inspector: std::sync::Arc<
            dyn Fn(
                terrane_collection_support::List<String>,
                terrane_collection_support::List<String>,
            ) -> bool + Send + Sync,
        >;
        let mut before: String;
        let pending_value;
        let address: terrane_int_support::Int;
        let pending_list;
        let observed: ReadObservation;
        let mut method_input: String;
        text = String::from("ordinary storage");
        data = Vec::<u8>::from([65, 66, 67]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&text,
            &text), 3 /* terrane-site: src/main.trn:22:13-22:34 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_string(&text,
            &text), 4 /* terrane-site: src/main.trn:23:13-23:36 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&data,
            &data), 5 /* terrane-site: src/main.trn:24:13-24:35 */))
        );
        item = Holder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item
            .text, &item.text), 6 /* terrane-site: src/main.trn:26:13-26:44 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&item
            .data, &item.data), 7 /* terrane-site: src/main.trn:27:13-27:45 */))
        );
        read_holder(&item);
        values = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("indexed storage")]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            8 /* terrane-site: src/main.trn:30:24-30:33 */); let
            __terrane_native_reference_0 = &* __terrane_raised(values
            .get(__terrane_index).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index)),
            8 /* terrane-site: src/main.trn:30:24-30:33 */); let __terrane_index =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: src/main.trn:30:35-30:44 */); let
            __terrane_native_reference_1 = &* __terrane_raised(values
            .get(__terrane_index).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index)),
            9 /* terrane-site: src/main.trn:30:35-30:44 */);
            __terrane_raised(same_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 10 /* terrane-site: src/main.trn:30:13-30:44 */) })
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values).as_vec()), 11 /* terrane-site: src/main.trn:31:13-31:42 */))
        );
        values_copy = values.clone();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values_copy).as_vec()), 12 /* terrane-site: src/main.trn:33:13-33:47 */))
        );
        {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                13 /* terrane-site: src/main.trn:34:5-34:31 */,
            );
            let __terrane_value = String::from("changed list");
            let _ = __terrane_raised(
                values.set(__terrane_index, __terrane_value),
                13 /* terrane-site: src/main.trn:34:5-34:31 */,
            );
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values_copy).as_vec()), 14 /* terrane-site: src/main.trn:35:13-35:47 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            15 /* terrane-site: src/main.trn:36:12-36:21 */)), 15 /* terrane-site: src/main.trn:36:12-36:21 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values_copy
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            16 /* terrane-site: src/main.trn:37:12-37:26 */)), 16 /* terrane-site: src/main.trn:37:12-37:26 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::same_generic(&text, &text))) { Ok(value) => Ok(value), Err(payload)
            => Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::same_generic")) }, 17 /* terrane-site: src/main.trn:38:13-38:37 */))
        );
        native = __terrane_raised(
            terrane_static_trn_5265636f7264_new(),
            18 /* terrane-site: src/main.trn:39:14-39:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&native
            .text, &native.text), 19 /* terrane-site: src/main.trn:40:13-40:48 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&native)
            .same(&native.text))) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Record::same")) },
            20 /* terrane-site: src/main.trn:41:13-41:37 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265636f7264_inspect(&item
            .text, &item.text), 21 /* terrane-site: src/main.trn:42:13-42:50 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = same_after_yield(&item.text, &item.text); async move {
            __terrane_raised_err(__terrane_future. await, 22 /* terrane-site: src/main.trn:43:18-43:56 */) } }). await, 22 /* terrane-site: src/main.trn:43:18-43:56 */))
        );
        copy = native.clone();
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = native.update();
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Record::update",
                        ),
                    )
                }
            },
            23 /* terrane-site: src/main.trn:45:5-45:19 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(owned_text(text
            .clone()), 24 /* terrane-site: src/main.trn:48:13-48:29 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&text));
        {
            let __terrane_value = String::from("changed");
            item.text = __terrane_value;
        }
        println!("{}", terrane_scalar_support::scalar_text(&item.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item
            .text, &item.text), 25 /* terrane-site: src/main.trn:52:13-52:44 */))
        );
        __terrane_raised(
            update_record(&mut native),
            26 /* terrane-site: src/main.trn:53:5-53:26 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        reader = std::sync::Arc::new(move |argument_0: String, argument_1: String| __terrane_raised(
            same_text(&argument_0, &argument_1),
            27 /* terrane-site: src/main.trn:56:14-56:23 */,
        ));
        println!(
            "{}", terrane_scalar_support::scalar_text(&reader(text.clone(), text
            .clone()))
        );
        shared = std::sync::Arc::new(
            TerraneSharedCell::new(Holder::terrane_construct()),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &shared;
            let __terrane_native_read_guard_0 = &* __terrane_owner.lock()
            .expect("shared reference lock poisoned");
            __terrane_raised(same_text(&__terrane_native_read_guard_0.text,
            &__terrane_native_read_guard_0.text), 28 /* terrane-site: src/main.trn:59:13-59:48 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &shared;
            let __terrane_native_read_guard_0 = &* __terrane_owner.lock()
            .expect("shared reference lock poisoned");
            __terrane_raised(same_bytes(&__terrane_native_read_guard_0.data,
            &__terrane_native_read_guard_0.data), 29 /* terrane-site: src/main.trn:60:13-60:49 */) })
        );
        pending = {
            let __terrane_native_value_0 = {
                let __terrane_snapshot = shared
                    .lock()
                    .expect("shared reference lock poisoned")
                    .text
                    .clone();
                __terrane_snapshot
            };
            async move {
                let __terrane_future = echo_after_yield(&__terrane_native_value_0);
                __terrane_raised_err(
                    __terrane_future.await,
                    30 /* terrane-site: src/main.trn:61:15-61:44 */,
                )
            }
        };
        {
            let __terrane_owner = shared.clone();
            let __terrane_value = String::from("replacement before polling");
            __terrane_owner.lock().expect("shared reference lock poisoned").text = __terrane_value;
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(pending)
            . await, 31 /* terrane-site: src/main.trn:63:18-63:25 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = shared
            .lock().expect("shared reference lock poisoned").text.clone();
            __terrane_snapshot })
        );
        async_reader = std::sync::Arc::new(move |
            argument_0: String,
        | -> std::pin::Pin<Box<dyn Future<Output = _>>> {
            Box::pin(async move {
                __terrane_raised(
                    echo_after_yield(&argument_0).await,
                    32 /* terrane-site: src/main.trn:65:20-65:36 */,
                )
            })
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_reader(text
            .clone())). await)
        );
        erased = <Inspect>::from(Inspector::terrane_construct());
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&erased).inspect_text(&text,
            &text))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&erased)
            .inspect_strings((&values).as_vec(), (&values).as_vec()))
        );
        observer = &erased;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&* observer).inspect_text(&text,
            &text))
        );
        inspect_callback = {
            let receiver = erased.clone();
            std::sync::Arc::new(move |argument_0: String, argument_1: String| {
                receiver.inspect_text(&argument_0, &argument_1)
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&inspect_callback(text.clone(),
            text.clone()))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_snapshot_0
            = (* &native.text).clone(); let __terrane_native_input_0 =
            &__terrane_native_snapshot_0; let __terrane_native_input_1 =
            &__terrane_raised(take_text(&mut native), 33 /* terrane-site: src/main.trn:74:38-74:55 */);
            __terrane_raised(join_text(__terrane_native_input_0,
            __terrane_native_input_1), 34 /* terrane-site: src/main.trn:74:13-74:56 */) })
        );
        {
            let __terrane_value = String::from("old receiver");
            native.text = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver =
            (* &native.text).clone(); let __terrane_string_argument_0 =
            &__terrane_raised(take_text(&mut native), 35 /* terrane-site: src/main.trn:76:36-76:53 */); __terrane_string_receiver
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence(&native
            .texts, &native.texts), 36 /* terrane-site: src/main.trn:78:13-78:54 */))
        );
        let __terrane_iterable_0 = &native.texts;
        let mut __terrane_iterator_0 = __terrane_iterable_0.iter();
        loop {
            word = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&* word));
        }
        list_reader = std::sync::Arc::new(move |
            argument_0: terrane_collection_support::List<String>,
            argument_1: terrane_collection_support::List<String>|
        __terrane_raised(
            same_sequence(argument_0.as_vec(), argument_1.as_vec()),
            37 /* terrane-site: src/main.trn:81:19-81:32 */,
        ));
        println!(
            "{}", terrane_scalar_support::scalar_text(&list_reader(values.clone(), values
            .clone()))
        );
        list_inspector = {
            let receiver = erased;
            std::sync::Arc::new(move |
                argument_0: terrane_collection_support::List<String>,
                argument_1: terrane_collection_support::List<String>|
            receiver.inspect_strings(argument_0.as_vec(), argument_1.as_vec()))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&list_inspector(values.clone(),
            values.clone()))
        );
        before = String::from("before replacement");
        pending_value = {
            let __terrane_native_value_0 = before;
            async move {
                let __terrane_future = echo_after_yield(&__terrane_native_value_0);
                __terrane_raised_err(
                    __terrane_future.await,
                    38 /* terrane-site: src/main.trn:86:21-86:45 */,
                )
            }
        };
        before = String::from("after replacement");
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(pending_value)
            . await, 39 /* terrane-site: src/main.trn:88:18-88:31 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&before));
        address = __terrane_raised(
            sequence_address((&values).as_vec()),
            40 /* terrane-site: src/main.trn:90:15-90:39 */,
        );
        pending_list = {
            let __terrane_native_value_0 = values.clone();
            async move {
                let __terrane_future = read_sequence_after_yield(
                    (&__terrane_native_value_0).as_vec(),
                );
                __terrane_raised_err(
                    __terrane_future.await,
                    41 /* terrane-site: src/main.trn:91:20-91:53 */,
                )
            }
        };
        {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                42 /* terrane-site: src/main.trn:92:5-92:38 */,
            );
            let __terrane_value = String::from("after retained read");
            let _ = __terrane_raised(
                values.set(__terrane_index, __terrane_value),
                42 /* terrane-site: src/main.trn:92:5-92:38 */,
            );
        }
        observed = __terrane_traced(
            __terrane_await(pending_list).await,
            43 /* terrane-site: src/main.trn:93:22-93:34 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(observed
            .address as i128) == address))
        );
        println!("{}", terrane_scalar_support::scalar_text(&observed.first));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            44 /* terrane-site: src/main.trn:96:12-96:21 */)), 44 /* terrane-site: src/main.trn:96:12-96:21 */))
        );
        method_input = String::from("before method");
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = { let __terrane_call = (&native)
            .echo_after_yield(&method_input); Box::pin(async move { match crate
            ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(value) =>
            Ok(value), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "witness", "witness::Record::echo_after_yield")) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 45 /* terrane-site: src/main.trn:98:18-98:55 */) } }). await, 45 /* terrane-site: src/main.trn:98:18-98:55 */))
        );
        method_input = String::from("after method");
        println!("{}", terrane_scalar_support::scalar_text(&method_input));
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_native_value_0 = { let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").text.clone(); __terrane_snapshot };
            let __terrane_native_future_receiver = &native; async move { let
            __terrane_future = { let __terrane_call = __terrane_native_future_receiver
            .echo_after_yield(&__terrane_native_value_0); Box::pin(async move { match
            crate ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(value)
            => Ok(value), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "witness", "witness::Record::echo_after_yield")) } }) };
            __terrane_raised_err(__terrane_future. await, 46 /* terrane-site: src/main.trn:102:18-102:54 */) } }). await, 46 /* terrane-site: src/main.trn:102:18-102:54 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
    });
}
