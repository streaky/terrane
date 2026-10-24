// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-call-throw-inference
fn make(
    value: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, TerraneError> {
    let d: Duration;
    d = __terrane_traced_err(
        Duration::terrane_static_seconds(value.clone()),
        0 /* terrane-site: case.trn:5:9-5:33 */,
    )?;
    return Ok(d.total_nanoseconds.clone());
}
#[derive(Clone)]
pub struct Maker {}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_build(
        value: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        let d: Duration;
        d = __terrane_traced_err(
            Duration::terrane_static_seconds(value.clone()),
            1 /* terrane-site: case.trn:10:13-10:37 */,
        )?;
        return Ok(d.total_nanoseconds.clone());
    }
    pub fn terrane_static_accept(
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return value.clone();
    }
}
fn argument() -> Result<terrane_int_support::Int, TerraneError> {
    return Ok(
        __terrane_traced_err(
            make(terrane_int_support::Int::from(-1_i128)),
            2 /* terrane-site: case.trn:17:12-17:20 */,
        )?,
    );
}
fn forward_argument() -> Result<terrane_int_support::Int, TerraneError> {
    return Ok(
        Maker::terrane_static_accept(
            __terrane_traced_err(
                argument(),
                3 /* terrane-site: case.trn:20:28-20:37 */,
            )?,
        ),
    );
}
fn main() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(make(terrane_int_support::Int::from(-
                1_i128)), 4 /* terrane-site: case.trn:24:17-24:25 */))
            );
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught-free"));
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(Maker::terrane_static_build(terrane_int_support::Int::from(-
                1_i128)), 5 /* terrane-site: case.trn:28:17-28:33 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("caught-static"));
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
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(forward_argument(),
                6 /* terrane-site: case.trn:32:17-32:34 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("caught-argument"));
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(Maker::terrane_static_build(terrane_int_support::Int::from(2_i128)),
        7 /* terrane-site: case.trn:35:13-35:28 */))
    );
}
