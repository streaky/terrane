// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TerraneNsAppSocketAddrIdentity2F61707000736F636B65742D61646472 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppSocketAddrIdentity2F61707000736F636B65742D61646472 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
fn main() {
    let address: IpAddr;
    let socket: TerraneNsDepsTokioNetSocketAddrIdentity2F646570732F746F6B696F2F6E657400536F636B657441646472;
    let network: IpNet;
    let payload: IpAddr;
    let borrowed_payload: &IpAddr;
    let mut __trn_5f696e76616c6964: Option<IpAddr> = None;
    let local: TerraneNsAppSocketAddrIdentity2F61707000736F636B65742D61646472;
    address = __terrane_raised(
        terrane_static_trn_497041646472_from_str("192.0.2.44"),
        0 /* terrane-site: src/main.trn:10:22-10:52 */,
    );
    socket = {
        let __terrane_conversion_input_0 = address;
        let __terrane_conversion_input_1 = terrane_int_support::Int::from(3210_i128);
        __terrane_raised(
            terrane_static_trn_54657272616e654e7344657073546f6b696f4e6574536f636b6574416464724964656e7469747932463634363537303733324637343646364236393646324636453635373430303533364636333642363537343431363436343732_new(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            1 /* terrane-site: src/main.trn:11:25-11:55 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&socket).port()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "std",
        "std::net::SocketAddr::port")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from_u128(value as u128)) } } (),
        2 /* terrane-site: src/main.trn:12:13-12:25 */))
    );
    network = __terrane_raised(
        terrane_static_trn_49704e6574_from_str("192.0.2.44/32"),
        3 /* terrane-site: src/main.trn:13:21-13:53 */,
    );
    payload = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&socket).ip()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::net::SocketAddr::ip",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        4 /* terrane-site: src/main.trn:14:22-14:32 */,
    );
    borrowed_payload = &payload;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&network).contains(&* borrowed_payload)) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "ipnet", "ipnet::IpNet::contains")) }; Ok::
        < _, crate ::TerraneForeignError > (value) } } (), 5 /* terrane-site: src/main.trn:16:13-16:47 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = __trn_5f696e76616c6964
                .insert(
                    __terrane_raised_completion!(
                        terrane_static_trn_497041646472_from_str("not-an-address"),
                        6 /* terrane-site: src/main.trn:18:20-18:54 */
                    ),
                );
            let _ = &__trn_5f696e76616c6964;
            println!("{}", String::from("unexpected parse success"));
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("invalid address caught"));
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
    local = TerraneNsAppSocketAddrIdentity2F61707000736F636B65742D61646472::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&local.value));
}
