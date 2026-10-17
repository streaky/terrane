// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: authored-generic-partial-construction
pub struct Connection {
    pub input: ByteReader,
    pub tag: String,
}
impl Connection {
    pub fn terrane_construct(tag: String) -> Self {
        let mut __terrane_constructed_value = Self {
            input: stdin(),
            tag: String::from(""),
        };
        __terrane_constructed_value.construct(tag);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, tag: String) {
        {
            let __terrane_value = tag;
            self.tag = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!("{}", terrane_scalar_support::scalar_text(&self.tag));
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        self.destruct();
    }
}
#[derive(Clone)]
pub struct Pair<TerraneType54> {
    pub first: Option<TerraneType54>,
    pub second: Option<TerraneType54>,
}
impl<TerraneType54> Pair<TerraneType54> {
    pub fn terrane_construct(
        first: TerraneType54,
        second: TerraneType54,
        fail: bool,
    ) -> Result<Self, TerraneError> {
        let mut __terrane_constructed_value = Self { first: None, second: None };
        __terrane_constructed_value.construct(first, second, fail)?;
        Ok(__terrane_constructed_value)
    }
    pub fn construct(
        &mut self,
        first: TerraneType54,
        second: TerraneType54,
        fail: bool,
    ) -> Result<(), TerraneError> {
        {
            let __terrane_value = first;
            self.first = Some(__terrane_value);
        }
        if fail {
            return Err(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    0 /* terrane-site: case.trn:19:13-19:33 */,
                ),
            );
        }
        {
            let __terrane_value = second;
            self.second = Some(__terrane_value);
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78 {
    pub fn terrane_construct(
        value: terrane_int_support::Int,
        __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value
            .construct(
                value,
                __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        value: terrane_int_support::Int,
        __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = value.clone()
                + __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565.clone();
            self.value = __terrane_value;
        }
    }
}
fn main() {
    let __trn_626f78: TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78;
    let first: Connection;
    let second: Connection;
    let mut value: Option<Pair<Connection>> = None;
    __trn_626f78 = TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78::terrane_construct(
        terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(22_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&__trn_626f78.value));
    first = Connection::terrane_construct(String::from("initialized closed"));
    second = Connection::terrane_construct(String::from("unassigned closed"));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = value
                .insert(
                    __terrane_traced_completion!(
                        Pair::< Connection >::terrane_construct(first, second, true),
                        1 /* terrane-site: case.trn:33:17-33:61 */
                    ),
                );
            let _ = &value;
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
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("construction failed"));
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
