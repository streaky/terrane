// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct OptionalAddress {
    pub address: Option<IpAddr>,
}
impl OptionalAddress {
    pub fn terrane_construct(address: Option<IpAddr>) -> Self {
        let mut __terrane_constructed_value = Self { address: None };
        __terrane_constructed_value.construct(address);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, address: Option<IpAddr>) {
        {
            let __terrane_value: Option<IpAddr> = address;
            self.address = __terrane_value;
        }
    }
    pub fn peer(&self) -> Option<String> {
        let address: Option<IpAddr>;
        address = self.address.clone();
        if (&address).is_some() {
            return Some(
                render_address(
                    *match &address {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    },
                ),
            );
        }
        return None;
    }
}
fn render_address(address: IpAddr) -> String {
    let formatted: String;
    formatted = address.to_string();
    return formatted;
}
fn main() {
    let t: Token;
    let read: std::sync::Arc<dyn Fn() -> Result<u64, TerraneError> + Send + Sync>;
    let view: OptionalAddress;
    let found: Option<String>;
    let empty: OptionalAddress;
    let optional: Option<IpAddr>;
    t = __terrane_raised(
        terrane_static_trn_546f6b656e_new(),
        0 /* terrane-site: src/main.trn:24:9-24:20 */,
    );
    read = {
        let t = t.clone();
        std::sync::Arc::new(move || -> Result<u64, TerraneError> {
            return Ok(
                __terrane_raised_err(
                    terrane_int_support::coerce::<
                        u64,
                    >(
                        &__terrane_raised_err(
                            || -> Result<_, crate::TerraneForeignError> {
                                {
                                    let value = match std::panic::catch_unwind(
                                        std::panic::AssertUnwindSafe(|| -> Result<
                                            _,
                                            crate::TerraneForeignError,
                                        > { Ok(t.clone().value()) }),
                                    ) {
                                        Ok(result) => result?,
                                        Err(payload) => {
                                            return Err(
                                                crate::__terrane_dependency_panic(
                                                    payload,
                                                    "terrane_query_witness",
                                                    "terrane_query_witness::Token::value",
                                                ),
                                            );
                                        }
                                    };
                                    Ok::<
                                        _,
                                        crate::TerraneForeignError,
                                    >(terrane_int_support::Int::from_u128(value as u128))
                                }
                            }(),
                            1 /* terrane-site: src/main.trn:26:16-26:24 */,
                        )?,
                    ),
                    1 /* terrane-site: src/main.trn:26:16-26:24 */,
                )?,
            );
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read(),
        2 /* terrane-site: src/main.trn:27:13-27:18 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read(),
        3 /* terrane-site: src/main.trn:28:13-28:18 */))
    );
    view = OptionalAddress::terrane_construct(
        Some(
            __terrane_raised(
                sample_address(),
                4 /* terrane-site: src/main.trn:29:40-29:55 */,
            ),
        ),
    );
    found = view.peer();
    if (&found).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    empty = OptionalAddress::terrane_construct(None);
    println!("{}", terrane_scalar_support::scalar_text(&empty.peer().is_none()));
    optional = Some(
        __terrane_raised(
            sample_address(),
            5 /* terrane-site: src/main.trn:35:29-35:44 */,
        ),
    );
    if (&optional).is_some() {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_address(match
            &optional { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }, match &optional {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }),
            6 /* terrane-site: src/main.trn:38:17-38:49 */))
        );
    }
}
