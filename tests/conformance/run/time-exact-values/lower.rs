// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: time-exact-values
fn main() {
    let one: Duration;
    let quarter: Duration;
    let precise: Duration;
    let minimum: Duration;
    let sum: Duration;
    let huge: Duration;
    let huge_scaled: Duration;
    let reduced: Option<Duration>;
    let scaled: Duration;
    let wall: Instant;
    let before: MonotonicInstant;
    let timeout: Deadline;
    let after: MonotonicInstant;
    let dynamic_negative: terrane_int_support::Int;
    one = __terrane_traced(
        Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:6:11-6:31 */,
    );
    quarter = __terrane_traced(
        Duration::terrane_static_milliseconds(terrane_int_support::Int::from(250_i128)),
        1 /* terrane-site: case.trn:7:15-7:42 */,
    );
    precise = __terrane_traced(
        Duration::terrane_static_microseconds(terrane_int_support::Int::from(50_i128)),
        2 /* terrane-site: case.trn:8:15-8:41 */,
    );
    minimum = __terrane_traced(
        Duration::terrane_static_nanoseconds(terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:9:15-9:39 */,
    );
    sum = one.add(quarter.clone());
    huge = __terrane_traced(
        Duration::terrane_static_nanoseconds(
            terrane_int_support::Int::from_decimal(
                "10000000000000000000000000000000000000000",
            ),
        ),
        4 /* terrane-site: case.trn:11:12-11:76 */,
    );
    huge_scaled = __terrane_traced(
        huge.multiply(terrane_int_support::Int::from(3_i128)),
        5 /* terrane-site: case.trn:12:19-12:35 */,
    );
    reduced = sum.subtract.checked(quarter.clone());
    scaled = __terrane_traced(
        precise.multiply(terrane_int_support::Int::from(4_i128)),
        6 /* terrane-site: case.trn:14:14-14:33 */,
    );
    wall = Clock::terrane_static_wall();
    before = Clock::terrane_static_monotonic();
    timeout = Clock::terrane_static_deadline(quarter.clone());
    after = Clock::terrane_static_monotonic();
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&one.seconds),
        terrane_scalar_support::scalar_text(&quarter.nanoseconds),
        terrane_scalar_support::scalar_text(&precise.nanoseconds),
        terrane_scalar_support::scalar_text(&minimum.nanoseconds)
    );
    if (&reduced).is_some() {
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&sum.seconds),
            terrane_scalar_support::scalar_text(&sum.nanoseconds),
            terrane_scalar_support::scalar_text(&match &reduced { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .total_nanoseconds),
            terrane_scalar_support::scalar_text(&scaled.total_nanoseconds)
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&(huge_scaled.total_nanoseconds.clone()
        ==
        terrane_int_support::Int::from_decimal("30000000000000000000000000000000000000000")))
    );
    dynamic_negative = wall.unix_seconds.clone() - wall.unix_seconds.clone()
        - terrane_int_support::Int::from(1_i128);
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(wall.nanoseconds.clone() >=
        terrane_int_support::Int::from(0_i128))),
        terrane_scalar_support::scalar_text(&(wall.nanoseconds.clone() <
        terrane_int_support::Int::from(1000000000_i128))),
        terrane_scalar_support::scalar_text(&(before <= after)),
        terrane_scalar_support::scalar_text(&(__terrane_traced(before
        .duration_until(&after), 7 /* terrane-site: case.trn:24:84-24:112 */)
        .total_nanoseconds.clone() >= terrane_int_support::Int::from(0_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(timeout.expires_at
        .duration_until(&timeout.expires_at), 8 /* terrane-site: case.trn:25:13-25:66 */).total_nanoseconds)
    );
    println!("{}", terrane_scalar_support::scalar_text(&timeout.expired()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(Duration::terrane_static_nanoseconds(dynamic_negative
                .clone()), 9 /* terrane-site: case.trn:28:17-28:56 */) .nanoseconds)
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
                    println!("{}", String::from("invalid"));
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
