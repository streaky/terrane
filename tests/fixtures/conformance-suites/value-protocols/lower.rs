// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: tests/integration/adaptive-fixed-comparison/case.trn
// Namespace: adaptive-fixed-comparison
#[derive(Clone)]
pub struct FixedCount {
    pub expected: i64,
}
impl FixedCount {
    pub fn terrane_construct() -> Self {
        Self { expected: 3 }
    }
}
fn test_scenario_terrane_adaptive_fixed_comparison() {
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let equal: i64;
    let lower: i64;
    let higher: i64;
    let negative: i64;
    let samples: terrane_collection_support::List<FixedCount>;
    let mut sample: FixedCount;
    let beyond: terrane_int_support::Int;
    let maximum: i64;
    let large: terrane_int_support::Int;
    let current: Option<terrane_int_support::Int>;
    let mut bound: terrane_int_support::Int;
    let mut step: terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    equal = 3;
    lower = 2;
    higher = 4;
    negative = -1;
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) == terrane_int_support::Int::from(equal as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(equal as
        i128) == terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) != terrane_int_support::Int::from(lower as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(lower as
        i128) != terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) > terrane_int_support::Int::from(lower as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(lower as
        i128) < terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) >= terrane_int_support::Int::from(equal as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(equal as
        i128) <= terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) < terrane_int_support::Int::from(higher as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(higher as
        i128) > terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) <= terrane_int_support::Int::from(equal as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(equal as
        i128) >= terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) > terrane_int_support::Int::from(negative as i128))),
        String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(negative as
        i128) < terrane_int_support::Int::from(values.length())))
    );
    samples = terrane_collection_support::List::<
        FixedCount,
    >::new(vec![FixedCount::terrane_construct()]);
    let __terrane_iterable_0 = samples;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        sample = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}",
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
            .length()) != terrane_int_support::Int::from(sample.expected as i128))),
            String::from(","),
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
            .length()) == terrane_int_support::Int::from(sample.expected as i128)))
        );
    }
    beyond = terrane_int_support::Int::from(9223372036854775808_i128);
    maximum = 9223372036854775807;
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(beyond >
        terrane_int_support::Int::from(maximum as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(maximum as
        i128) < beyond))
    );
    large = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211456",
    );
    current = Some(large.clone());
    bound = large.clone();
    let __terrane_iterable_1 = values;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        step = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if (&current).is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&(* match &current {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                } >= bound)), String::from(","),
                terrane_scalar_support::scalar_text(&(bound == * match &current {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                }))
            );
            bound = &bound + &step;
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&large));
}
// Source: tests/integration/adaptive-union-field/case.trn
// Namespace: adaptive-union-field
#[derive(Clone)]
pub struct WorkItem {
    pub estimate: Option<terrane_int_support::Int>,
}
impl WorkItem {
    pub fn terrane_construct(estimate: Option<terrane_int_support::Int>) -> Self {
        let mut __terrane_constructed_value = Self { estimate: None };
        __terrane_constructed_value.construct(estimate);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, estimate: Option<terrane_int_support::Int>) {
        {
            let __terrane_value: Option<terrane_int_support::Int> = estimate;
            self.estimate = __terrane_value;
        }
    }
    pub fn estimate_units(&self) -> Option<terrane_int_support::Int> {
        return self.estimate.clone();
    }
    pub fn next_estimate(&self) -> Option<terrane_int_support::Int> {
        let current: Option<terrane_int_support::Int>;
        current = self.estimate.clone();
        if (&current).is_some() {
            return Some(
                &*match &current {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                } + &terrane_int_support::Int::from(1_i128),
            );
        }
        return None;
    }
}
fn test_scenario_terrane_adaptive_union_field() {
    let item: WorkItem;
    let estimate: Option<terrane_int_support::Int>;
    let next: Option<terrane_int_support::Int>;
    item = WorkItem::terrane_construct(Some(terrane_int_support::Int::from(4_i128)));
    estimate = item.estimate_units();
    if (&estimate).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &estimate { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    next = item.next_estimate();
    if (&next).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &next { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
// Source: tests/integration/borrowed-scalar-comparisons/case.trn
// Namespace: borrowed-scalar-comparisons
fn greater(value: &terrane_int_support::Int) -> bool {
    return *value > terrane_int_support::Int::from(1_i128);
}
fn less(value: &terrane_int_support::Int) -> bool {
    return terrane_int_support::Int::from(1_i128) < *value;
}
fn compare(left: &terrane_int_support::Int, right: &terrane_int_support::Int) -> bool {
    return *left > *right;
}
fn local_compare(value: terrane_int_support::Int) -> bool {
    let borrowed: &terrane_int_support::Int;
    borrowed = &value;
    return *borrowed > terrane_int_support::Int::from(2_i128);
}
fn mutable_compare(mut value: terrane_int_support::Int) -> bool {
    let borrowed: &mut terrane_int_support::Int;
    borrowed = &mut value;
    return *borrowed < terrane_int_support::Int::from(10_i128);
}
fn fixed_width(value: &u8) -> bool {
    return *value > 1;
}
fn shared_compare(
    value: std::sync::Arc<TerraneSharedCell<terrane_int_support::Int>>,
) -> bool {
    return {
        let __terrane_value = value
            .lock()
            .expect("shared reference lock poisoned")
            .clone();
        __terrane_value
    } > terrane_int_support::Int::from(1_i128);
}
fn pipeline_result() -> terrane_int_support::Int {
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let predicate: std::sync::Arc<
        dyn Fn(&terrane_int_support::Int) -> bool + Send + Sync,
    >;
    let result: terrane_collection_support::List<terrane_int_support::Int>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    predicate = {
        std::sync::Arc::new(move |value: &terrane_int_support::Int| -> bool {
            return *value > terrane_int_support::Int::from(1_i128);
        })
    };
    result = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = predicate;
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1(__terrane_pipeline_item) {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    return terrane_int_support::Int::from(result.length());
}
fn test_scenario_terrane_borrowed_scalar_comparisons() {
    let value: terrane_int_support::Int;
    let other: terrane_int_support::Int;
    let small: u8;
    value = terrane_int_support::Int::from(5_i128);
    other = terrane_int_support::Int::from(3_i128);
    println!("{}", terrane_scalar_support::scalar_text(&greater(&value)));
    println!("{}", terrane_scalar_support::scalar_text(&less(&value)));
    println!("{}", terrane_scalar_support::scalar_text(&compare(&value, &other)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&local_compare(terrane_int_support::Int::from(4_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&mutable_compare(terrane_int_support::Int::from(4_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&shared_compare(std::sync::Arc::new(TerraneSharedCell::new(value))))
    );
    small = 3;
    println!("{}", terrane_scalar_support::scalar_text(&fixed_width(&small)));
    println!("{}", terrane_scalar_support::scalar_text(&pipeline_result()));
}
// Source: tests/integration/bounded-arithmetic-families/case.trn
// Namespace: bounded-arithmetic-families
fn test_scenario_terrane_bounded_arithmetic_families() {
    let small: i8;
    let wrapped: i8;
    let overflowed: terrane_int_support::OverflowResult<i8>;
    let pair: terrane_int_support::DivRemResult<terrane_int_support::Int>;
    let exact: i64;
    let negated: terrane_int_support::OverflowResult<i8>;
    let mut count: i8;
    let sub_overflow: terrane_int_support::OverflowResult<i8>;
    let mul_overflow: terrane_int_support::OverflowResult<i8>;
    let minimum: i8;
    let div_overflow: terrane_int_support::OverflowResult<i8>;
    let rem_overflow: terrane_int_support::OverflowResult<i8>;
    small = 120;
    wrapped = terrane_int_support::fixed_addition_wrap(small, 10);
    println!("{}", terrane_scalar_support::scalar_text(&wrapped));
    overflowed = terrane_int_support::fixed_addition_overflowing(small, 10);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflowed.value),
        terrane_scalar_support::scalar_text(&overflowed.overflowed)
    );
    pair = __terrane_raised(
        terrane_int_support::Int::from(-7_i128)
            .div_rem(&terrane_int_support::Int::from(3_i128)),

            0 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:8:10-8:26 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&pair.quotient),
        terrane_scalar_support::scalar_text(&pair.remainder)
    );
    exact = 5;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(exact
        as i128) * terrane_int_support::Int::from(9_i128)))
    );
    terrane_int_support::fixed_subtraction_checked(small, 20);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_saturate(small,
        2))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division(small,
        3), 1 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:14:11-14:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder(small,
        7), 2 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:14:30-14:48 */))
    );
    negated = terrane_int_support::fixed_negation_overflowing(small);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negated.value),
        terrane_scalar_support::scalar_text(&negated.overflowed)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_shift_left_wrap(small,
        &1), 3 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:17:11-17:35 */))
    );
    __terrane_raised(
        terrane_int_support::fixed_shift_right_checked(small, &2),

            4 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:18:3-18:31 */,
    );
    count = 1;
    count = __terrane_raised(
        terrane_int_support::fixed_addition(count, 1),

            5 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:20:3-20:10 */,
    );
    count = __terrane_raised(
        terrane_int_support::fixed_subtraction(count, 1),

            6 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:21:3-21:10 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&count));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_addition_checked(small,
        10).is_none()),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_addition_saturate(small,
        10))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_wrap(small,
        - 20)),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_saturate(small,
        - 20))
    );
    sub_overflow = terrane_int_support::fixed_subtraction_overflowing(small, -20);
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&sub_overflow.value),
        terrane_scalar_support::scalar_text(&sub_overflow.overflowed),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_checked(small,
        - 20).is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_wrap(small,
        2)),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_checked(small,
        2).is_none())
    );
    mul_overflow = terrane_int_support::fixed_multiplication_overflowing(small, 2);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mul_overflow.value),
        terrane_scalar_support::scalar_text(&mul_overflow.overflowed)
    );
    minimum = -128;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_wrap(minimum,
        - 1), 7 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:31:11-31:34 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_saturate(minimum,
        - 1), 8 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:31:38-31:65 */))
    );
    div_overflow = __terrane_raised(
        terrane_int_support::fixed_division_overflowing(minimum, -1),

            9 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:32:18-32:48 */,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&div_overflow.value),
        terrane_scalar_support::scalar_text(&div_overflow.overflowed),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_checked(minimum,
        - 1), 10 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:33:56-33:82 */).is_none())
    );
    rem_overflow = __terrane_raised(
        terrane_int_support::fixed_remainder_overflowing(minimum, -1),

            11 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:34:18-34:51 */,
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_wrap(minimum,
        - 1), 12 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:35:11-35:37 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_saturate(minimum,
        - 1), 13 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:35:41-35:71 */))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&rem_overflow.value),
        terrane_scalar_support::scalar_text(&rem_overflow.overflowed),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_checked(minimum,
        - 1), 14 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:36:56-36:85 */).is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:37:11-37:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&terrane_int_support::Int::from(3_i128)), 16 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:37:30-37:48 */))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .euclidean_div(&- terrane_int_support::Int::from(3_i128)), 17 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:38:11-38:25 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .modulo(&- terrane_int_support::Int::from(3_i128)), 18 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:38:29-38:46 */))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&- terrane_int_support::Int::from(3_i128)),
        19 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:39:11-39:27 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&- terrane_int_support::Int::from(3_i128)), 20 /* terrane-site: tests/integration/bounded-arithmetic-families/case.trn:39:31-39:50 */))
    );
}
// Source: tests/integration/bounded-class-field-capture/case.trn
// Namespace: bounded-class-field-capture
pub trait TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564(
    Box<
        dyn TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol
for TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565,
>
for TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedClassFieldCaptureBIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650042<
    TerraneType54: TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub x: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
> TerraneNsBoundedClassFieldCaptureBIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650042<
    TerraneType54,
> {
    pub fn terrane_construct(x: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            x: None,
        };
        __terrane_constructed_value.construct(x);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, x: TerraneType54) {
        {
            let __terrane_value = x;
            self.x = Some(__terrane_value);
        }
    }
}
fn go_terrane_bounded_class_field_capture<
    TerraneType54: TerraneNsBoundedClassFieldCaptureNamedIdentity2F626F756E6465642D636C6173732D6669656C642D63617074757265006E616D6564Protocol,
>(
    b: TerraneNsBoundedClassFieldCaptureBIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650042<
        TerraneType54,
    >,
) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let b = b;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return b.x.as_ref().expect("required field initialized").name();
        })
    };
    return f();
}
fn test_scenario_terrane_bounded_class_field_capture() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_class_field_capture(TerraneNsBoundedClassFieldCaptureBIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650042::
        <
        TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565
        >
        ::terrane_construct(TerraneNsBoundedClassFieldCaptureValueIdentity2F626F756E6465642D636C6173732D6669656C642D636170747572650076616C7565::terrane_construct())))
    );
}
// Source: tests/integration/bounded-list-capture/case.trn
// Namespace: bounded-list-capture
pub trait TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564(
    Box<
        dyn TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol
for TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565,
>
for TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_list_capture<
    TerraneType54: TerraneNsBoundedListCaptureNamedIdentity2F626F756E6465642D6C6973742D63617074757265006E616D6564Protocol,
>(items: terrane_collection_support::List<TerraneType54>) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let items = items;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(items.length());
        })
    };
    return f();
}
fn test_scenario_terrane_bounded_list_capture() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_list_capture(terrane_collection_support::List::
        <
        TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565
        >
        ::new(vec![TerraneNsBoundedListCaptureValueIdentity2F626F756E6465642D6C6973742D636170747572650076616C7565::terrane_construct()])))
    );
}
// Source: tests/integration/bounded-local-shared-alias/case.trn
// Namespace: bounded-local-shared-alias
pub trait TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564(
    Box<
        dyn TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol
for TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565,
>
for TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564 {
    fn from(
        value: TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_local_shared_alias<
    TerraneType54: TerraneNsBoundedLocalSharedAliasNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C696173006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let h: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    g = {
        let item = item;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
    h = g.clone();
    return h();
}
fn test_scenario_terrane_bounded_local_shared_alias() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_local_shared_alias(TerraneNsBoundedLocalSharedAliasValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D616C6961730076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/bounded-local-shared-list/case.trn
// Namespace: bounded-local-shared-list
pub trait TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564(
    Box<
        dyn TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol
for TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565,
>
for TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564 {
    fn from(
        value: TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_local_shared_list<
    TerraneType54: TerraneNsBoundedLocalSharedListNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C697374006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let mut fs: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    fs = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = {
            let item = item;
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return item.name();
            })
        };
        fs.append(__terrane_collection_argument_0);
    };
    return terrane_int_support::Int::from(fs.length());
}
fn test_scenario_terrane_bounded_local_shared_list() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_local_shared_list(TerraneNsBoundedLocalSharedListValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D6C6973740076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/bounded-local-shared-reuse/case.trn
// Namespace: bounded-local-shared-reuse
pub trait TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564(
    Box<
        dyn TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol
for TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565,
>
for TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564 {
    fn from(
        value: TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_local_shared_reuse<
    TerraneType54: TerraneNsBoundedLocalSharedReuseNamedIdentity2F626F756E6465642D6C6F63616C2D7368617265642D7265757365006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    g = {
        let item = item;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
    return g() + g();
}
fn test_scenario_terrane_bounded_local_shared_reuse() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_local_shared_reuse(TerraneNsBoundedLocalSharedReuseValueIdentity2F626F756E6465642D6C6F63616C2D7368617265642D72657573650076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/bounded-optional-capture/case.trn
// Namespace: bounded-optional-capture
pub trait TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564(
    Box<
        dyn TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol
for TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565,
>
for TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_optional_capture<
    TerraneType54: TerraneNsBoundedOptionalCaptureNamedIdentity2F626F756E6465642D6F7074696F6E616C2D63617074757265006E616D6564Protocol,
>(item: Option<TerraneType54>) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let item = item;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            if (&item).is_some() {
                return terrane_int_support::Int::from(1_i128);
            }
            return terrane_int_support::Int::from(0_i128);
        })
    };
    return f();
}
fn test_scenario_terrane_bounded_optional_capture() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_optional_capture(Some(TerraneNsBoundedOptionalCaptureValueIdentity2F626F756E6465642D6F7074696F6E616C2D636170747572650076616C7565::terrane_construct())))
    );
}
// Source: tests/integration/bounded-owner-capture/case.trn
// Namespace: bounded-owner-capture
pub trait TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564(
    Box<
        dyn TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol
for TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565,
>
for TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Container<
    TerraneType54: TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub item: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsBoundedOwnerCaptureNamedIdentity2F626F756E6465642D6F776E65722D63617074757265006E616D6564Protocol,
> Container<TerraneType54> {
    pub fn terrane_construct(item: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            item: None,
        };
        __terrane_constructed_value.construct(item);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, item: TerraneType54) {
        {
            let __terrane_value = item;
            self.item = Some(__terrane_value);
        }
    }
    pub fn read(mut self) -> terrane_int_support::Int {
        let item: TerraneType54;
        let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
        item = self.item.take().expect("required field initialized");
        f = {
            let item = item;
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return item.name();
            })
        };
        return f();
    }
}
fn test_scenario_terrane_bounded_owner_capture() {
    let c: Container<
        TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565,
    >;
    c = Container::<
        TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565,
    >::terrane_construct(
        TerraneNsBoundedOwnerCaptureValueIdentity2F626F756E6465642D6F776E65722D636170747572650076616C7565::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&c.read()));
}
// Source: tests/integration/bounded-shared-capture/case.trn
// Namespace: bounded-shared-capture
pub trait TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564(
    Box<
        dyn TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol
for TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565,
>
for TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_shared_capture<
    TerraneType54: TerraneNsBoundedSharedCaptureNamedIdentity2F626F756E6465642D7368617265642D63617074757265006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let item = item;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
    return f();
}
fn test_scenario_terrane_bounded_shared_capture() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_shared_capture(TerraneNsBoundedSharedCaptureValueIdentity2F626F756E6465642D7368617265642D636170747572650076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/bounded-two-interfaces/case.trn
// Namespace: bounded-two-interfaces
pub trait AProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn AProtocol>;
    fn separate_box(&self) -> Box<dyn AProtocol>;
    fn n(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn AProtocol> {
    fn clone(&self) -> Self {
        AProtocol::clone_box(&**self)
    }
}
pub struct A(Box<dyn AProtocol>);
impl Clone for A {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl A {
    pub fn n(&self) -> terrane_int_support::Int {
        self.0.n()
    }
}
pub trait TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol,
    >;
    fn m(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062(
    Box<
        dyn TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol,
    >,
);
impl Clone
for TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062 {
    pub fn m(&self) -> terrane_int_support::Int {
        self.0.m()
    }
}
#[derive(Clone)]
pub struct V {}
impl V {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn n(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn m(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl AProtocol for V {
    fn clone_box(&self) -> Box<dyn AProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AProtocol> {
        Box::new(self.clone())
    }
    fn n(&self) -> terrane_int_support::Int {
        V::n(&*self)
    }
}
impl From<V> for A {
    fn from(value: V) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol
for V {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062Protocol,
    > {
        Box::new(self.clone())
    }
    fn m(&self) -> terrane_int_support::Int {
        V::m(&*self)
    }
}
impl From<V>
for TerraneNsBoundedTwoInterfacesBIdentity2F626F756E6465642D74776F2D696E74657266616365730062 {
    fn from(value: V) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_bounded_two_interfaces<TerraneType54: AProtocol>(
    item: TerraneType54,
) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let item = item;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.n();
        })
    };
    return f();
}
fn test_scenario_terrane_bounded_two_interfaces() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_bounded_two_interfaces(V::terrane_construct()))
    );
}
// Source: tests/integration/byte-index-and-slice/case.trn
// Namespace: byte-index-and-slice
fn test_scenario_terrane_byte_index_and_slice() -> Result<(), TerraneError> {
    let data: Vec<u8>;
    let middle: Vec<u8>;
    let stepped: Vec<u8>;
    let empty: Vec<u8>;
    let __trn_66696e616c: Vec<u8>;
    let mut failure: Option<TerraneError> = None;
    data = {
        let __terrane_receiver_value_0 = &String::from("A👍");
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(
            &__terrane_receiver_value_0,
            __terrane_string_argument_0,
        )
    };
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&data,
        __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        21 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:8:10-8:17 */) ?),
        21 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:8:10-8:17 */) ?),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&data,
        __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        22 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:8:19-8:26 */) ?),
        22 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:8:19-8:26 */) ?)
    );
    middle = __terrane_raised_err(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised_err(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(1_i64),
                ),

                    23 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:9:23-9:34 */,
            )?,
        ),

            24 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:9:18-9:35 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&middle,
        __terrane_string_argument_0), 25 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:10:11-10:30 */) ? })
    );
    stepped = __terrane_raised_err(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised_err(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(2_i128),
                ),

                    26 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:11:24-11:38 */,
            )?,
        ),

            27 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:11:19-11:39 */,
    )?;
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&stepped,
        __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        28 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:12:10-12:20 */) ?),
        28 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:12:10-12:20 */) ?),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&stepped,
        __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        29 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:12:22-12:32 */) ?),
        29 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:12:22-12:32 */) ?),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&stepped,
        __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        30 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:12:34-12:44 */) ?),
        30 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:12:34-12:44 */) ?)
    );
    empty = __terrane_raised_err(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised_err(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(data.len() as i128),
                    terrane_int_support::Int::from(data.len() as i128),
                    terrane_int_support::Int::from(1_i64),
                ),

                    31 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:13:22-13:53 */,
            )?,
        ),

            32 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:13:17-13:54 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&(empty.len() as i128)));
    __trn_66696e616c = __terrane_raised_err(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised_err(
                terrane_collection_support::Range::through(
                    terrane_int_support::Int::from(4_i128),
                    terrane_int_support::Int::from(4_i128),
                    terrane_int_support::Int::from(1_i64),
                ),

                    33 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:15:22-15:41 */,
            )?,
        ),

            34 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:15:17-15:42 */,
    )?;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&__trn_66696e616c,
        __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        35 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:16:10-16:18 */) ?),
        35 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:16:10-16:18 */) ?)
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_collection_support::byte_at(&data,
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(data
                .len() as i128)), 36 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:18:12-18:29 */)),
                36 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:18:12-18:29 */))
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
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("index"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(__terrane_raised_completion!(terrane_collection_support::byte_slice(&data,
                &__terrane_raised_completion!(terrane_collection_support::Range::new(terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(6_i128),
                terrane_int_support::Int::from(1_i64)), 37 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:22:17-22:28 */)),
                38 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:22:12-22:29 */) .len() as
                i128))
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
                    && __terrane_error_1.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("slice"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_collection_support::byte_at(&data,
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(-
                1_i128)), 39 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:26:12-26:21 */)),
                39 /* terrane-site: tests/integration/byte-index-and-slice/case.trn:26:12-26:21 */))
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
                    && __terrane_error_2.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_2 = true;
                    let _ = failure.insert(__terrane_error_2.clone());
                    {
                        let __terrane_error_text_1 = failure
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_1);
                    };
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/bytes-and-legacy-digests/case.trn
// Namespace: bytes-and-legacy-digests
fn test_scenario_terrane_bytes_and_legacy_digests() -> Result<(), TerraneError> {
    let octets: terrane_collection_support::List<u8>;
    let data: Vec<u8>;
    let empty_octets: terrane_collection_support::List<u8>;
    let empty: Vec<u8>;
    let first: DigestResult;
    let second: DigestResult;
    octets = terrane_collection_support::List::<u8>::new(vec![65, 66, 67]);
    data = bytes_from_octets(octets);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&data,
        __terrane_string_argument_0), 40 /* terrane-site: tests/integration/bytes-and-legacy-digests/case.trn:12:11-12:28 */) ? }),
        terrane_scalar_support::scalar_text(&(data.len() as i128))
    );
    empty_octets = terrane_collection_support::List::<u8>::new(vec![]);
    empty = bytes_from_octets(empty_octets);
    println!("{}", terrane_scalar_support::scalar_text(&(empty.len() as i128)));
    first = digest_bytes_terrane_core_random(sha1(), Vec::<u8>::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(first.value.value.clone()))
    );
    second = digest_bytes_terrane_core_random(md5(), Vec::<u8>::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(second.value.value
        .clone()))
    );
    Ok(())
}
// Source: tests/integration/bytes-variadic-concat/case.trn
// Namespace: bytes-variadic-concat
fn test_scenario_terrane_bytes_variadic_concat() -> Result<(), TerraneError> {
    let combined: Vec<u8>;
    let unchanged: Vec<u8>;
    combined = {
        let mut bytes = Vec::<u8>::from([97]);
        let part_0: Vec<u8> = Vec::<u8>::from([]);
        let part_1: Vec<u8> = Vec::<u8>::from([98, 99]);
        let part_2: Vec<u8> = Vec::<u8>::from([100]);
        let additional = match [part_0.len(), part_1.len(), part_2.len()]
            .into_iter()
            .try_fold(0usize, usize::checked_add)
        {
            Some(length) => length,
            None => std::process::abort(),
        };
        if bytes.try_reserve(additional).is_err() {
            std::process::abort();
        }
        bytes.extend(part_0);
        bytes.extend(part_1);
        bytes.extend(part_2);
        bytes
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&combined,
        __terrane_string_argument_0), 41 /* terrane-site: tests/integration/bytes-variadic-concat/case.trn:7:11-7:32 */) ? }),
        terrane_scalar_support::scalar_text(&(combined.len() as i128))
    );
    unchanged = {
        let mut bytes = Vec::<u8>::from([122]);
        let additional = match [].into_iter().try_fold(0usize, usize::checked_add) {
            Some(length) => length,
            None => std::process::abort(),
        };
        if bytes.try_reserve(additional).is_err() {
            std::process::abort();
        }
        bytes
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&unchanged,
        __terrane_string_argument_0), 42 /* terrane-site: tests/integration/bytes-variadic-concat/case.trn:9:11-9:33 */) ? })
    );
    Ok(())
}
// Source: tests/integration/bytes-views-encoding/case.trn
// Namespace: bytes-views-encoding
fn test_scenario_terrane_bytes_views_encoding() -> Result<(), TerraneError> {
    let text: String;
    let mut byte_count: terrane_int_support::Int;
    let mut byte: u8;
    let mut scalar_count: terrane_int_support::Int;
    let mut scalar: String;
    let mut grapheme_count: terrane_int_support::Int;
    let mut grapheme: String;
    let encoded: Vec<u8>;
    let decoded: String;
    let utf16le_text: String;
    let utf16be_text: String;
    let utf32le_text: String;
    let utf32be_text: String;
    let owned: Vec<u8>;
    let wide: Vec<u8>;
    let invalid: Vec<u8>;
    let mut error: Option<TerraneError> = None;
    text = String::from("e\u{301}");
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text) as
        i128)), terrane_scalar_support::scalar_text(&(text.len() as i128)),
        terrane_scalar_support::scalar_text(&(text.chars().count() as i128)),
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text) as
        i128))
    );
    byte_count = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_0 = &text;
        let mut __terrane_iterator_0 = __terrane_iterable_0.bytes();
        loop {
            byte = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if byte == byte {
                byte_count = &byte_count + &terrane_int_support::Int::from(1_i128);
            }
        }
    }
    scalar_count = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_1 = &text;
        let mut __terrane_iterator_1 = __terrane_iterable_1
            .chars()
            .map(|value| value.to_string());
        loop {
            scalar = match __terrane_iterator_1.next() {
                Some(item) => item,
                None => break,
            };
            if scalar.as_str() == scalar.as_str() {
                scalar_count = &scalar_count + &terrane_int_support::Int::from(1_i128);
            }
        }
    }
    grapheme_count = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_2 = &text;
        let mut __terrane_iterator_2 = terrane_string_support::graphemes(
            &__terrane_iterable_2,
        );
        loop {
            grapheme = match __terrane_iterator_2.next() {
                Some(item) => item,
                None => break,
            };
            if grapheme.as_str() == grapheme.as_str() {
                grapheme_count = &grapheme_count
                    + &terrane_int_support::Int::from(1_i128);
            }
        }
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&byte_count),
        terrane_scalar_support::scalar_text(&scalar_count),
        terrane_scalar_support::scalar_text(&grapheme_count)
    );
    encoded = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(&text, __terrane_string_argument_0)
    };
    decoded = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(
            terrane_string_support::decode(&encoded, __terrane_string_argument_0),

                43 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:21:20-21:40 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&decoded));
    utf16le_text = {
        let __terrane_receiver_value_0 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Le;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Le;
        __terrane_raised_err(
            terrane_string_support::decode(
                &__terrane_receiver_value_0,
                __terrane_string_argument_0,
            ),

                44 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:23:25-23:65 */,
        )?
    };
    utf16be_text = {
        let __terrane_receiver_value_1 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Be;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Be;
        __terrane_raised_err(
            terrane_string_support::decode(
                &__terrane_receiver_value_1,
                __terrane_string_argument_0,
            ),

                45 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:24:25-24:65 */,
        )?
    };
    utf32le_text = {
        let __terrane_receiver_value_2 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Le;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Le;
        __terrane_raised_err(
            terrane_string_support::decode(
                &__terrane_receiver_value_2,
                __terrane_string_argument_0,
            ),

                46 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:25:25-25:65 */,
        )?
    };
    utf32be_text = {
        let __terrane_receiver_value_3 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Be;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Be;
        __terrane_raised_err(
            terrane_string_support::decode(
                &__terrane_receiver_value_3,
                __terrane_string_argument_0,
            ),

                47 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:26:25-26:65 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&utf16le_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf16be_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf32le_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf32be_text));
    owned = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(&text, __terrane_string_argument_0)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_decode_receiver_4 =
        owned; let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode_owned(__terrane_decode_receiver_4,
        __terrane_string_argument_0), 48 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:32:11-32:36 */) ? })
    );
    wide = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Le;
        terrane_string_support::encode(&text, __terrane_string_argument_0)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_decode_receiver_5 =
        wide; let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf16Le;
        __terrane_raised_err(terrane_string_support::decode_owned(__terrane_decode_receiver_5,
        __terrane_string_argument_0), 49 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:34:11-34:39 */) ? })
    );
    invalid = Vec::<u8>::from([97, 255]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_decode_receiver_6 = invalid; let __terrane_string_argument_0 =
                terrane_string_support::Encoding::Utf8;
                __terrane_raised_completion!(terrane_string_support::decode_owned(__terrane_decode_receiver_6,
                __terrane_string_argument_0), 50 /* terrane-site: tests/integration/bytes-views-encoding/case.trn:37:13-37:40 */) })
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
                    && __terrane_error_0.kind == TerraneErrorKind::DecodeError
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_7 = error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_7);
                    };
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/call-callable-alias/case.trn
// Namespace: call-callable-alias
fn look_terrane_call_callable_alias(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn test_scenario_terrane_call_callable_alias() {
    let f: std::sync::Arc<dyn Fn(&String, String) -> String + Send + Sync>;
    let o: String;
    f = std::sync::Arc::new(look_terrane_call_callable_alias);
    o = String::from("x");
    println!("{}", terrane_scalar_support::scalar_text(&f(&o, o.clone())));
}
// Source: tests/integration/call-list-borrow-contents/case.trn
// Namespace: call-list-borrow-contents
fn look_terrane_call_list_borrow_contents(
    a: &terrane_collection_support::List<terrane_int_support::Int>,
    b: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        a
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        51 /* terrane-site: tests/integration/call-list-borrow-contents/case.trn:4:12-4:16 */,
                ),
            ),

            51 /* terrane-site: tests/integration/call-list-borrow-contents/case.trn:4:12-4:16 */,
    )
        + __terrane_raised(
            b
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),

                            52 /* terrane-site: tests/integration/call-list-borrow-contents/case.trn:4:19-4:23 */,
                    ),
                ),

                52 /* terrane-site: tests/integration/call-list-borrow-contents/case.trn:4:19-4:23 */,
        );
}
fn test_scenario_terrane_call_list_borrow_contents() {
    let o: terrane_collection_support::List<terrane_int_support::Int>;
    o = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(3_i128)]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_call_list_borrow_contents(&o, o
        .clone()))
    );
}
// Source: tests/integration/call-string-alias-borrow/case.trn
// Namespace: call-string-alias-borrow
fn look_terrane_call_string_alias_borrow(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn test_scenario_terrane_call_string_alias_borrow() {
    let o: String;
    let alias: &String;
    o = String::from("x");
    alias = &o;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_call_string_alias_borrow(alias,
        o.clone()))
    );
}
// Source: tests/integration/call-string-different-owners/case.trn
// Namespace: call-string-different-owners
fn look_terrane_call_string_different_owners(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn test_scenario_terrane_call_string_different_owners() {
    let o: String;
    let p: String;
    o = String::from("x");
    p = String::from("y");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_call_string_different_owners(&o,
        p))
    );
}
// Source: tests/integration/call-string-field-borrow/case.trn
// Namespace: call-string-field-borrow
fn sample_range() -> terrane_string_support::TextRange {
    let mut matches: terrane_collection_support::List<terrane_string_support::TextRange>;
    let mut hit: terrane_string_support::TextRange;
    matches = terrane_collection_support::List::<
        terrane_string_support::TextRange,
    >::new(vec![]);
    let __terrane_iterable_0 = {
        let __terrane_receiver_value_0 = &String::from("éx");
        let __terrane_string_argument_0 = "x";
        terrane_string_support::find_all(
            &__terrane_receiver_value_0,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_0,
    );
    {
        let __terrane_list_append_0 = matches.make_unique();
        loop {
            hit = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            {
                let __terrane_collection_argument_0 = hit;
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
        }
    }
    return __terrane_raised(
        matches
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        53 /* terrane-site: tests/integration/call-string-field-borrow/case.trn:7:12-7:22 */,
                ),
            ),

            53 /* terrane-site: tests/integration/call-string-field-borrow/case.trn:7:12-7:22 */,
    );
}
#[derive(Clone)]
pub struct TerraneNsCallStringFieldBorrowBoxIdentity2F63616C6C2D737472696E672D6669656C642D626F72726F7700626F78 {
    pub value: String,
    pub items: terrane_collection_support::List<String>,
    pub name: String,
    pub hit: terrane_string_support::TextRange,
}
impl TerraneNsCallStringFieldBorrowBoxIdentity2F63616C6C2D737472696E672D6669656C642D626F72726F7700626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            value: String::from("x"),
            items: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("before"), String::from("second")]),
            name: String::from("é"),
            hit: sample_range(),
        }
    }
}
fn observe_terrane_call_string_field_borrow(
    b: &TerraneNsCallStringFieldBorrowBoxIdentity2F63616C6C2D737472696E672D6669656C642D626F72726F7700626F78,
) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* b)
        .items.length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&(* b)
        .name) as i128))
    );
    println!("{}", terrane_scalar_support::scalar_text(&((* b).name.len() as i128)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* b)
        .hit.scalar_start() as i128))
    );
}
fn snapshot(
    b: &TerraneNsCallStringFieldBorrowBoxIdentity2F63616C6C2D737472696E672D6669656C642D626F72726F7700626F78,
) -> terrane_collection_support::List<String> {
    return {
        let __terrane_snapshot = b.items.clone();
        __terrane_snapshot
    };
}
fn look_terrane_call_string_field_borrow(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn test_scenario_terrane_call_string_field_borrow() {
    let mut o: TerraneNsCallStringFieldBorrowBoxIdentity2F63616C6C2D737472696E672D6669656C642D626F72726F7700626F78;
    let saved: terrane_collection_support::List<String>;
    o = TerraneNsCallStringFieldBorrowBoxIdentity2F63616C6C2D737472696E672D6669656C642D626F72726F7700626F78::terrane_construct();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_call_string_field_borrow(&o
        .value, o.value.clone()))
    );
    observe_terrane_call_string_field_borrow(&o);
    saved = snapshot(&o);
    {
        let __terrane_value: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("after")]);
        o.items = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(saved
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        54 /* terrane-site: tests/integration/call-string-field-borrow/case.trn:28:12-28:20 */)),
        54 /* terrane-site: tests/integration/call-string-field-borrow/case.trn:28:12-28:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(o.items
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        55 /* terrane-site: tests/integration/call-string-field-borrow/case.trn:29:12-29:22 */)),
        55 /* terrane-site: tests/integration/call-string-field-borrow/case.trn:29:12-29:22 */))
    );
}
// Source: tests/integration/call-string-later-read/case.trn
// Namespace: call-string-later-read
fn look_terrane_call_string_later_read(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn test_scenario_terrane_call_string_later_read() {
    let o: String;
    o = String::from("x");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_call_string_later_read(&o, o
        .clone()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&o));
}
// Source: tests/integration/call-string-named-borrow/case.trn
// Namespace: call-string-named-borrow
fn look_terrane_call_string_named_borrow(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn test_scenario_terrane_call_string_named_borrow() {
    let o: String;
    o = String::from("x");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_call_string_named_borrow(&o, o
        .clone()))
    );
}
// Source: tests/integration/call-typed-callable/case.trn
// Namespace: call-typed-callable
fn look_terrane_call_typed_callable(a: &String, b: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&*a, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&b, &mut __terrane_text);
        __terrane_text
    };
}
fn apply(
    f: std::sync::Arc<dyn Fn(&String, String) -> String + Send + Sync>,
    o: String,
) -> String {
    return f(&o, o.clone());
}
fn test_scenario_terrane_call_typed_callable() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply(std::sync::Arc::new(look_terrane_call_typed_callable),
        String::from("x")))
    );
}
// Source: tests/integration/callable-exact-result-control/case.trn
// Namespace: callable-exact-result-control
fn source() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
fn test_scenario_terrane_callable_exact_result_control() {
    let exact: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    exact = std::sync::Arc::new(source);
    println!("{}", terrane_scalar_support::scalar_text(&exact()));
}
// Source: tests/integration/callable-field-alias/case.trn
// Namespace: callable-field-alias
#[derive(Clone)]
pub struct TerraneNsCallableFieldAliasHolderIdentity2F63616C6C61626C652D6669656C642D616C69617300686F6C646572 {
    pub f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
}
impl TerraneNsCallableFieldAliasHolderIdentity2F63616C6C61626C652D6669656C642D616C69617300686F6C646572 {
    pub fn terrane_construct(
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn() -> terrane_int_support::Int + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
    pub fn method(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(6_i128);
    }
}
fn test_scenario_terrane_callable_field_alias() {
    let h: TerraneNsCallableFieldAliasHolderIdentity2F63616C6C61626C652D6669656C642D616C69617300686F6C646572;
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let m: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    h = TerraneNsCallableFieldAliasHolderIdentity2F63616C6C61626C652D6669656C642D616C69617300686F6C646572::terrane_construct({
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(5_i128);
        })
    });
    g = h.f.as_ref().expect("required field initialized").clone();
    println!("{}", terrane_scalar_support::scalar_text(&g()));
    m = {
        let receiver = h;
        std::sync::Arc::new(move || receiver.method())
    };
    println!("{}", terrane_scalar_support::scalar_text(&m()));
}
// Source: tests/integration/callable-field-direct/case.trn
// Namespace: callable-field-direct
#[derive(Clone)]
pub struct TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572Storage {
    pub f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
}
impl TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572Storage {
    pub fn terrane_construct(
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn() -> terrane_int_support::Int + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572 {
    Own(
        TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572Storage,
    ),
    Derived(Derived),
}
impl TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572 {
    pub fn terrane_construct(
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) -> Self {
        Self::Own(
            TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572Storage::terrane_construct(
                f,
            ),
        )
    }
    pub fn terrane_field_f(
        &self,
    ) -> &std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync> {
        match self {
            Self::Own(value) => value.f.as_ref().expect("required field initialized"),
            Self::Derived(value) => value.f.as_ref().expect("required field initialized"),
        }
    }
    pub fn terrane_field_f_mut(
        &mut self,
    ) -> &mut std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync> {
        match self {
            Self::Own(value) => value.f.as_mut().expect("required field initialized"),
            Self::Derived(value) => value.f.as_mut().expect("required field initialized"),
        }
    }
    pub fn terrane_field_f_slot_mut(
        &mut self,
    ) -> &mut Option<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    > {
        match self {
            Self::Own(value) => &mut value.f,
            Self::Derived(value) => &mut value.f,
        }
    }
}
#[derive(Clone)]
pub struct Derived {
    pub f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
}
impl Derived {
    pub fn terrane_construct(
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn() -> terrane_int_support::Int + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_callable_field_direct() {
    let h: TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572;
    let child: Derived;
    h = TerraneNsCallableFieldDirectHolderIdentity2F63616C6C61626C652D6669656C642D64697265637400686F6C646572::terrane_construct({
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(5_i128);
        })
    });
    println!("{}", terrane_scalar_support::scalar_text(&h.terrane_field_f() ()));
    child = Derived::terrane_construct({
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(7_i128);
        })
    });
    println!(
        "{}", terrane_scalar_support::scalar_text(&child.f.as_ref()
        .expect("required field initialized") ())
    );
}
// Source: tests/integration/callable-field-reference/case.trn
// Namespace: callable-field-reference
#[derive(Clone)]
pub struct TerraneNsCallableFieldReferenceHolderIdentity2F63616C6C61626C652D6669656C642D7265666572656E636500686F6C646572 {
    pub f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
}
impl TerraneNsCallableFieldReferenceHolderIdentity2F63616C6C61626C652D6669656C642D7265666572656E636500686F6C646572 {
    pub fn terrane_construct(
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn() -> terrane_int_support::Int + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
}
fn invoke_terrane_callable_field_reference(
    h: &TerraneNsCallableFieldReferenceHolderIdentity2F63616C6C61626C652D6669656C642D7265666572656E636500686F6C646572,
) -> terrane_int_support::Int {
    return {
        let __terrane_snapshot = h
            .f
            .as_ref()
            .expect("required field initialized")
            .clone();
        __terrane_snapshot
    }();
}
fn test_scenario_terrane_callable_field_reference() {
    let h: TerraneNsCallableFieldReferenceHolderIdentity2F63616C6C61626C652D6669656C642D7265666572656E636500686F6C646572;
    h = TerraneNsCallableFieldReferenceHolderIdentity2F63616C6C61626C652D6669656C642D7265666572656E636500686F6C646572::terrane_construct({
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(5_i128);
        })
    });
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&invoke_terrane_callable_field_reference(&h))
    );
}
// Source: tests/integration/callable-field-this/case.trn
// Namespace: callable-field-this
#[derive(Clone)]
pub struct TerraneNsCallableFieldThisHolderIdentity2F63616C6C61626C652D6669656C642D7468697300686F6C646572 {
    pub f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
}
impl TerraneNsCallableFieldThisHolderIdentity2F63616C6C61626C652D6669656C642D7468697300686F6C646572 {
    pub fn terrane_construct(
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn() -> terrane_int_support::Int + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
    pub fn invoke(&self) -> terrane_int_support::Int {
        return self.f.as_ref().expect("required field initialized")();
    }
}
fn test_scenario_terrane_callable_field_this() {
    let h: TerraneNsCallableFieldThisHolderIdentity2F63616C6C61626C652D6669656C642D7468697300686F6C646572;
    h = TerraneNsCallableFieldThisHolderIdentity2F63616C6C61626C652D6669656C642D7468697300686F6C646572::terrane_construct({
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(5_i128);
        })
    });
    println!("{}", terrane_scalar_support::scalar_text(&h.invoke()));
}
// Source: tests/integration/canonical-union-alias/case.trn
// Namespace: canonical-union-alias
fn accept(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn test_scenario_terrane_canonical_union_alias() {
    let value: i64;
    value = 7;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&accept(terrane_int_support::Int::from(value
        as i128)))
    );
}
// Source: tests/integration/checked-coercion/case.trn
// Namespace: checked-coercion
static __TERRANE_F32_SHARED: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(100_i128));
fn convert(item: terrane_int_support::Int) -> Result<i8, TerraneError> {
    let mut result: i8;
    result = 0;
    if item > terrane_int_support::Int::from(0_i128) {
        result = __terrane_raised_err(
            terrane_int_support::coerce::<i8>(&item),

                56 /* terrane-site: tests/integration/checked-coercion/case.trn:9:14-9:18 */,
        )?;
    }
    return Ok(result);
}
fn helper() {
    println!("{}", String::from("helper"));
}
fn test_values_terrane_checked_coercion() -> Result<(), TerraneError> {
    let value: i64;
    let within: i64;
    let coerced: i8;
    let renamed_checked: Option<i8>;
    let shared_coerced: i8;
    let parameter_coerced: i8;
    let checked: Option<i8>;
    let absent: bool;
    let present: bool;
    let shadow_safe: i8;
    helper();
    value = 300;
    within = 100;
    coerced = __terrane_raised_err(
        terrane_int_support::coerce::<i8>(&within),
        57 /* terrane-site: tests/integration/checked-coercion/case.trn:19:13-19:19 */,
    )?;
    renamed_checked = terrane_int_support::checked_coerce::<i8>(&within);
    shared_coerced = __terrane_raised_err(
        terrane_int_support::coerce::<i8>(&*__TERRANE_F32_SHARED),
        58 /* terrane-site: tests/integration/checked-coercion/case.trn:21:20-21:33 */,
    )?;
    parameter_coerced = __terrane_traced_err(
        convert(terrane_int_support::Int::from(within as i128)),
        59 /* terrane-site: tests/integration/checked-coercion/case.trn:22:23-22:38 */,
    )?;
    checked = terrane_int_support::checked_coerce::<i8>(&value);
    absent = checked.is_none();
    present = checked.is_some();
    shadow_safe = terrane_int_support::saturating_coerce::<i8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&coerced));
    println!("{}", terrane_scalar_support::scalar_text(&renamed_checked.is_some()));
    println!("{}", terrane_scalar_support::scalar_text(&shared_coerced));
    println!("{}", terrane_scalar_support::scalar_text(&parameter_coerced));
    println!("{}", terrane_scalar_support::scalar_text(&absent));
    println!("{}", terrane_scalar_support::scalar_text(&present));
    println!("{}", terrane_scalar_support::scalar_text(&shadow_safe));
    Ok(())
}
// Source: tests/integration/collection-identity-lifetime/case.trn
// Namespace: collection-identity-lifetime
#[derive(Clone)]
pub struct TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572 {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::from(""),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value: String = name;
            self.name = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", String::from("drop-"), terrane_scalar_support::scalar_text(&self
            .name)
        );
    }
}
impl Drop
for TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn release_order() {
    let mut values: terrane_collection_support::List<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >;
    let removed: TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572;
    let mut cleared: terrane_collection_support::List<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >;
    let original: terrane_collection_support::List<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >;
    let mut separated: terrane_collection_support::List<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >;
    values = terrane_collection_support::List::<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >::new(
        vec![
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(String::from("replace-old")),
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(String::from("remove-me")),
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(String::from("destroy-last"))
        ],
    );
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                60 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:17:3-17:50 */,
        );
        let __terrane_collection_argument_1 = TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(
            String::from("replacement"),
        );
        __terrane_raised(
            values.set(__terrane_collection_argument_0, __terrane_collection_argument_1),

                60 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:17:3-17:50 */,
        );
    };
    println!("{}", String::from("after-set"));
    removed = {
        let __terrane_result = {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                ),

                    61 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:20:22-20:38 */,
            );
            __terrane_raised(
                values.remove(__terrane_index),

                    61 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:20:22-20:38 */,
            )
        };
        __terrane_result
    };
    println!(
        "{}{}", String::from("after-remove-"),
        terrane_scalar_support::scalar_text(&removed.name)
    );
    println!("{}", String::from("after-block"));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_result = {
                    let __terrane_index = __terrane_raised_completion!(
                        terrane_collection_support::index_from_int(&terrane_int_support::Int::from(9_i128)),
                        62 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:24:5-24:21 */
                    );
                    __terrane_raised_completion!(
                        values.remove(__terrane_index), 62 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:24:5-24:21 */
                    )
                };
                __terrane_result
            };
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
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("remove-index"));
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
    cleared = terrane_collection_support::List::<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >::new(
        vec![
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(String::from("clear-first")),
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(String::from("clear-second"))
        ],
    );
    {
        cleared.clear();
    };
    println!("{}", String::from("after-clear"));
    original = terrane_collection_support::List::<
        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
    >::new(
        vec![
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(String::from("cow-original"))
        ],
    );
    separated = original.clone();
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                63 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:33:3-33:57 */,
        );
        let __terrane_collection_argument_1 = TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(
            String::from("cow-replacement"),
        );
        __terrane_raised(
            separated
                .set(__terrane_collection_argument_0, __terrane_collection_argument_1),

                63 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:33:3-33:57 */,
        );
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(original
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        64 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:34:10-34:21 */)),
        64 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:34:10-34:21 */).name),
        terrane_scalar_support::scalar_text(&__terrane_raised(separated
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        65 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:34:28-34:40 */)),
        65 /* terrane-site: tests/integration/collection-identity-lifetime/case.trn:34:28-34:40 */).name)
    );
    println!("{}", String::from("after-cow"));
}
fn test_scenario_terrane_collection_identity_lifetime() {
    let left: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
        >,
    >;
    let right: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
        >,
    >;
    let left_reference: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
        >,
    >;
    let same_reference: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
        >,
    >;
    let other_reference: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
        >,
    >;
    let left_weak: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
        >,
    >;
    let references: std::sync::Arc<
        TerraneSharedCell<
            terrane_collection_support::List<
                std::sync::Arc<
                    TerraneSharedCell<
                        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
                    >,
                >,
            >,
        >,
    >;
    let collection_reference: std::sync::Arc<
        TerraneSharedCell<
            terrane_collection_support::List<
                std::sync::Arc<
                    TerraneSharedCell<
                        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
                    >,
                >,
            >,
        >,
    >;
    let collection_alias: std::sync::Arc<
        TerraneSharedCell<
            terrane_collection_support::List<
                std::sync::Arc<
                    TerraneSharedCell<
                        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
                    >,
                >,
            >,
        >,
    >;
    let marker_type: TerraneDescriptor;
    let reference_type: TerraneDescriptor;
    let collection_type: TerraneDescriptor;
    release_order();
    left = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572::terrane_construct(
                String::from("left"),
            ),
        ),
    );
    right = std::sync::Arc::new(
        TerraneSharedCell::new(
            {
                let __terrane_value = left
                    .lock()
                    .expect("reference lock poisoned")
                    .clone();
                __terrane_value
            }
                .terrane_separate(),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&false),
        terrane_scalar_support::scalar_text(&false)
    );
    left_reference = left.clone();
    same_reference = left_reference.clone();
    other_reference = right.clone();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_reference; let __terrane_identity_right = &same_reference;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_reference; let __terrane_identity_right = &other_reference;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    left_weak = std::sync::Arc::downgrade(&left_reference);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_weak; let __terrane_identity_right = &left_reference;
        std::ptr::eq(std::sync::Weak::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    references = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                std::sync::Arc<
                    TerraneSharedCell<
                        TerraneNsCollectionIdentityLifetimeMarkerIdentity2F636F6C6C656374696F6E2D6964656E746974792D6C69666574696D65006D61726B6572,
                    >,
                >,
            >::new(vec![left_reference.clone(), same_reference.clone()]),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&false));
    collection_reference = references.clone();
    collection_alias = collection_reference.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &collection_reference; let __terrane_identity_right = &collection_alias;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    marker_type = {
        let _ = &{
            let __terrane_value = left.lock().expect("reference lock poisoned").clone();
            __terrane_value
        };
        TerraneDescriptor {
            identity: "/collection-identity-lifetime::marker",
            name: "marker",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[
                TerraneFieldMetadata {
                    name: "name",
                    external_name: "name",
                    defaulted: true,
                    optional: false,
                    secret: false,
                },
            ],
        }
    };
    reference_type = {
        let _ = &left_reference;
        TerraneDescriptor {
            identity: "shared ref marker",
            name: "shared ref marker",
            kind: "type",
            inherently_identity_bearing: true,
            fields: &[],
        }
    };
    collection_type = {
        let _ = &{
            let __terrane_value = references
                .lock()
                .expect("reference lock poisoned")
                .clone();
            __terrane_value
        };
        TerraneDescriptor {
            identity: "/core/collections::list of shared ref marker",
            name: "list of shared ref marker",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&marker_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&reference_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&collection_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&marker_type
        .inherently_identity_bearing)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&reference_type
        .inherently_identity_bearing)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&collection_type
        .inherently_identity_bearing)
    );
}
// Source: tests/integration/collection-lookup-errors/case.trn
// Namespace: collection-lookup-errors
fn test_scenario_terrane_collection_lookup_errors() {
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let checked_index: Option<terrane_int_support::Int>;
    let values_by_key: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let checked_key: Option<terrane_int_support::Int>;
    let present_key: Option<terrane_int_support::Int>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(values
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
                66 /* terrane-site: tests/integration/collection-lookup-errors/case.trn:7:12-7:21 */)),
                66 /* terrane-site: tests/integration/collection-lookup-errors/case.trn:7:12-7:21 */))
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
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("index"));
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
    checked_index = terrane_collection_support::index_from_int(
            &terrane_int_support::Int::from(2_i128),
        )
        .ok()
        .and_then(|index| values.get(index).cloned());
    println!("{}", terrane_scalar_support::scalar_text(&(&checked_index).is_none()));
    values_by_key = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("present"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(values_by_key
                .get_or_error(&String::from("absent")), 67 /* terrane-site: tests/integration/collection-lookup-errors/case.trn:14:12-14:35 */))
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
                    && __terrane_error_1.kind == TerraneErrorKind::MissingKey
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("missing"));
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
    checked_key = values_by_key.get(&String::from("absent")).cloned();
    present_key = values_by_key.get(&String::from("present")).cloned();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(&checked_key).is_none()),
        terrane_scalar_support::scalar_text(&(&present_key).is_some())
    );
}
// Source: tests/integration/collection-property-get-call/case.trn
// Namespace: collection-property-get-call
fn test_scenario_terrane_collection_property_get_call() {
    let values: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut pair: terrane_collection_support::Entry<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    values = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128)]))
        ],
    );
    let __terrane_iterable_0 = {
        let __terrane_result = values.entries();
        __terrane_result
    };
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        pair = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
            .and_then(| index | pair.value.clone().get_or_error(index)),
            68 /* terrane-site: tests/integration/collection-property-get-call/case.trn:7:13-7:30 */))
        );
    }
}
// Source: tests/integration/collection-property-method-calls/case.trn
// Namespace: collection-property-method-calls
fn test_scenario_terrane_collection_property_method_calls() {
    let values: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut pair: terrane_collection_support::Entry<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut appended: terrane_collection_support::List<terrane_int_support::Int>;
    let mut sorted: terrane_collection_support::List<terrane_int_support::Int>;
    values = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128)]))
        ],
    );
    let __terrane_iterable_0 = {
        let __terrane_result = values.entries();
        __terrane_result
    };
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        pair = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        appended = {
            let __terrane_result = {
                let __terrane_value = terrane_int_support::Int::from(2_i128);
                let collection = &mut pair.value.clone();
                collection.append(__terrane_value);
                collection.clone()
            };
            __terrane_result
        };
        sorted = {
            let __terrane_result = {
                let collection = &mut pair.value.clone();
                collection.sort_by(|left, right| left.cmp(right));
                collection.clone()
            };
            __terrane_result
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
            &pair.key.clone(); let __terrane_string_argument_0 = "al";
            __terrane_receiver_value_0.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(appended
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(appended
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
            69 /* terrane-site: tests/integration/collection-property-method-calls/case.trn:10:29-10:40 */)),
            69 /* terrane-site: tests/integration/collection-property-method-calls/case.trn:10:29-10:40 */))
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(sorted
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            70 /* terrane-site: tests/integration/collection-property-method-calls/case.trn:11:12-11:21 */)),
            70 /* terrane-site: tests/integration/collection-property-method-calls/case.trn:11:12-11:21 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(sorted
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            71 /* terrane-site: tests/integration/collection-property-method-calls/case.trn:11:23-11:32 */)),
            71 /* terrane-site: tests/integration/collection-property-method-calls/case.trn:11:23-11:32 */))
        );
    }
}
// Source: tests/integration/collections-value-semantics/case.trn
// Namespace: collections-value-semantics
fn test_scenario_terrane_collections_value_semantics() {
    let original: terrane_collection_support::List<terrane_int_support::Int>;
    let mut independent: terrane_collection_support::List<terrane_int_support::Int>;
    let mut ordered: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let mut pair: terrane_collection_support::Entry<String, terrane_int_support::Int>;
    let mut unique: terrane_collection_support::Set<String>;
    let mut value: String;
    let pair_2: terrane_collection_support::Tuple<String>;
    let explicit: terrane_collection_support::Entry<String, terrane_int_support::Int>;
    let numbers: terrane_collection_support::Range;
    let mut number: terrane_int_support::Int;
    let inclusive: terrane_collection_support::Range;
    let mut empty_count: terrane_int_support::Int;
    let empty: terrane_collection_support::Range;
    let mut ignored: terrane_int_support::Int;
    let mut deterministic_map: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let mut pair_3: terrane_collection_support::Entry<String, terrane_int_support::Int>;
    let mut deterministic_set: terrane_collection_support::UnorderedSet<String>;
    let mut empty_list: terrane_collection_support::List<terrane_int_support::Int>;
    let mut empty_map: terrane_collection_support::Map<terrane_int_support::Int, String>;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let arbitrary: terrane_collection_support::Map<terrane_int_support::Int, String>;
    original = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    independent = original.clone();
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(3_i128);
        independent.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(1_i128),
            ),

                72 /* terrane-site: tests/integration/collections-value-semantics/case.trn:8:3-8:21 */,
        );
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        let _ = __terrane_raised(
            independent.set(__terrane_index, __terrane_value),

                72 /* terrane-site: tests/integration/collections-value-semantics/case.trn:8:3-8:21 */,
        );
    }
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(independent
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(independent
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        73 /* terrane-site: tests/integration/collections-value-semantics/case.trn:9:47-9:61 */)),
        73 /* terrane-site: tests/integration/collections-value-semantics/case.trn:9:47-9:61 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(independent
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        74 /* terrane-site: tests/integration/collections-value-semantics/case.trn:9:63-9:77 */)),
        74 /* terrane-site: tests/integration/collections-value-semantics/case.trn:9:63-9:77 */))
    );
    ordered = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    {
        let __terrane_collection_argument_0 = String::from("third");
        let __terrane_collection_argument_1 = terrane_int_support::Int::from(3_i128);
        ordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    {
        let __terrane_key = String::from("second");
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        ordered.set(__terrane_key, __terrane_value);
    }
    let __terrane_iterable_0 = ordered.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        pair = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&pair.key.clone()),
            terrane_scalar_support::scalar_text(&pair.value.clone())
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("second")), 75 /* terrane-site: tests/integration/collections-value-semantics/case.trn:15:10-15:27 */))
    );
    unique = terrane_collection_support::Set::<
        String,
    >::new(vec![String::from("b"), String::from("a"), String::from("b")]);
    {
        let __terrane_collection_argument_0 = String::from("c");
        unique.add(__terrane_collection_argument_0);
    };
    let __terrane_iterable_1 = unique;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    pair_2 = terrane_collection_support::Tuple::<
        String,
    >::new(vec![String::from("left"), String::from("right")]);
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(pair_2
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(pair_2
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        76 /* terrane-site: tests/integration/collections-value-semantics/case.trn:21:23-21:30 */)),
        76 /* terrane-site: tests/integration/collections-value-semantics/case.trn:21:23-21:30 */))
    );
    explicit = terrane_collection_support::Entry::<
        String,
        terrane_int_support::Int,
    >::new(String::from("key"), terrane_int_support::Int::from(7_i128));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&explicit.key.clone()),
        terrane_scalar_support::scalar_text(&explicit.value.clone())
    );
    numbers = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i64),
        ),

            77 /* terrane-site: tests/integration/collections-value-semantics/case.trn:24:13-24:24 */,
    );
    let __terrane_iterable_2 = numbers;
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        number = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    inclusive = __terrane_raised(
        terrane_collection_support::Range::through(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(-1_i128),
        ),

            78 /* terrane-site: tests/integration/collections-value-semantics/case.trn:27:15-27:38 */,
    );
    let __terrane_iterable_3 = inclusive;
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_3,
    );
    loop {
        number = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    empty_count = terrane_int_support::Int::from(0_i128);
    empty = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(-1_i128),
        ),

            79 /* terrane-site: tests/integration/collections-value-semantics/case.trn:31:11-31:26 */,
    );
    let __terrane_iterable_4 = empty;
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_4,
    );
    loop {
        ignored = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &ignored;
        empty_count = empty_count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&empty_count));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                terrane_collection_support::Range::new(terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(0_i128)), 80 /* terrane-site: tests/integration/collections-value-semantics/case.trn:36:5-36:19 */
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
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("zero"));
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
    deterministic_map = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    {
        let __terrane_key = String::from("second");
        let __terrane_value = terrane_int_support::Int::from(3_i128);
        deterministic_map.set(__terrane_key, __terrane_value);
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_map
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(deterministic_map
        .get_or_error(&String::from("second")), 81 /* terrane-site: tests/integration/collections-value-semantics/case.trn:41:36-41:63 */))
    );
    let __terrane_iterable_5 = deterministic_map;
    let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_5,
    );
    loop {
        pair_3 = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&pair_3.key.clone()));
    }
    deterministic_set = terrane_collection_support::UnorderedSet::<
        String,
    >::new(vec![String::from("x"), String::from("y")]);
    {
        let __terrane_collection_argument_0 = String::from("z");
        deterministic_set.add(__terrane_collection_argument_0);
    };
    {
        let __terrane_result = {
            let __terrane_value = String::from("x");
            deterministic_set.remove(&__terrane_value)
        };
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        __terrane_value = String::from("y"); deterministic_set.contains(&__terrane_value)
        }; __terrane_result })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_set
        .length()))
    );
    let __terrane_iterable_6 = deterministic_set;
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_6,
    );
    loop {
        value = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    empty_list = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(5_i128);
        empty_list.append(__terrane_collection_argument_0);
    };
    empty_map = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(Vec::new());
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(1_i128);
        let __terrane_collection_argument_1 = String::from("one");
        empty_map.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(9_i128)])
        ],
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty_list
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(empty_map
        .get_or_error(&terrane_int_support::Int::from(1_i128)), 82 /* terrane-site: tests/integration/collections-value-semantics/case.trn:56:29-56:41 */)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested
        .length()))
    );
    arbitrary = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(
        vec![
            terrane_collection_support::Entry::< terrane_int_support::Int, String
            >::new(terrane_int_support::Int::from(2_i128), String::from("two")),
            terrane_collection_support::Entry::< terrane_int_support::Int, String
            >::new(terrane_int_support::Int::from(3_i128), String::from("three"))
        ],
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(arbitrary
        .get_or_error(&terrane_int_support::Int::from(2_i128)), 83 /* terrane-site: tests/integration/collections-value-semantics/case.trn:58:10-58:22 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(arbitrary
        .get_or_error(&terrane_int_support::Int::from(3_i128)), 84 /* terrane-site: tests/integration/collections-value-semantics/case.trn:58:24-58:36 */))
    );
}
// Source: tests/integration/complete-float-surface/case.trn
// Namespace: complete-float-surface
fn exercise32() {
    let zero: f32;
    let negative_zero: f32;
    let one: f32;
    let two: f32;
    let three: f32;
    let four: f32;
    let negative_one: f32;
    let negative_two: f32;
    let negative_quarter: f32;
    let fractional: f32;
    let twelve: f32;
    let thousand: f32;
    let overflow_input: f32;
    let underflow_input: f32;
    let eight: f32;
    let decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let not_a_number: f32;
    let infinity: f32;
    let clamped_zero: f32;
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let wide_four: f64;
    let floating_exponent: f64;
    let nan_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let infinity_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let negative_zero_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let negative_infinity: f32;
    let large_exponent: f32;
    let large: f32;
    let descriptor: TerraneDescriptor;
    zero = 0.0_f32;
    negative_zero = -0.0_f32;
    one = 1.0_f32;
    two = 2.0_f32;
    three = 3.0_f32;
    four = 4.0_f32;
    negative_one = -1.0_f32;
    negative_two = -2.0_f32;
    negative_quarter = -0.25_f32;
    fractional = -1.25_f32;
    twelve = 12.0_f32;
    thousand = 1000.0_f32;
    overflow_input = 128.0_f32;
    underflow_input = -150.0_f32;
    eight = 8.0_f32;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = eight
        .cbrt(); __terrane_result } == two)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = three.hypot(four); __terrane_result } == 5.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = four
        .powf(0.5_f32); __terrane_result } == two)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = two.powi(10);
        __terrane_result } == 1024.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .exp2(); __terrane_result } == eight)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.exp_m1(); __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .ln_1p(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log2(); __terrane_result } == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = thousand
        .log10(); __terrane_result } == three)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log(two); __terrane_result } == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .tan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = zero.asin(); __terrane_result } == zero)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = one.acos();
        __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .atan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.atan2(negative_one); __terrane_result } > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one
        .copysign(negative_zero); __terrane_result } == negative_one)),
        terrane_scalar_support::scalar_text(&negative_zero.is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_value : f32 = two; let terrane_lower : f32 = 3.0_f32; let terrane_upper :
        f32 = 4.0_f32; if terrane_value.is_nan() { terrane_value } else if terrane_lower
        .is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN }
        else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } == three)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = fractional; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_up(); __terrane_result } == { let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_down(); __terrane_result } == negative_one * { let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) }))
    );
    decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(twelve);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa ==
        0.75_f32)), terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(decomposition.mantissa, decomposition
        .exponent); __terrane_result } == twelve))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f32::RADIX)) } ==
        terrane_int_support::Int::from(2_i128))), terrane_scalar_support::scalar_text(&({
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f32::MANTISSA_DIGITS)) } ==
        terrane_int_support::Int::from(24_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::EPSILON } > zero)),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::MIN_POSITIVE } .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) } >
        zero)), terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::MIN } < zero)),
        terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::MAX } > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = two.asin();
        __terrane_result } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        __terrane_result = negative_two.powf(0.5_f32); __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        overflow_input.exp2(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = underflow_input
        .exp2(); __terrane_result } == zero))
    );
    not_a_number = {
        let __terrane_result = two.asin();
        __terrane_result
    };
    infinity = one / zero;
    clamped_zero = {
        let __terrane_result = {
            let terrane_value: f32 = negative_zero;
            let terrane_lower: f32 = zero;
            let terrane_upper: f32 = one;
            if terrane_value.is_nan() {
                terrane_value
            } else if terrane_lower.is_nan() || terrane_upper.is_nan()
                || terrane_lower > terrane_upper
            {
                f32::NAN
            } else {
                let terrane_lowered = if terrane_value == 0.0 && terrane_lower == 0.0 {
                    if terrane_value.is_sign_positive()
                        || terrane_lower.is_sign_positive()
                    {
                        0.0
                    } else {
                        -0.0
                    }
                } else {
                    terrane_value.max(terrane_lower)
                };
                if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                    if terrane_lowered.is_sign_negative()
                        || terrane_upper.is_sign_negative()
                    {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    terrane_lowered.min(terrane_upper)
                }
            }
        };
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f32 = not_a_number; let terrane_lower : f32 = zero; let
        terrane_upper : f32 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f32::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let terrane_value
        : f32 = one; let terrane_lower : f32 = not_a_number; let terrane_upper : f32 =
        one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        not_a_number.hypot(infinity); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = infinity.next_up();
        __terrane_result } .is_infinite())
    );
    subnormal_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32({
            let _ = &TerraneDescriptor {
                identity: "float32",
                name: "float32",
                kind: "type",
                inherently_identity_bearing: false,
                fields: &[],
            };
            f32::from_bits(1)
        });
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(subnormal_decomposition.mantissa,
        subnormal_decomposition.exponent); __terrane_result } == { let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) }))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        terrane_scalar_support::scale_binary_f32({ let _ = &TerraneDescriptor { identity
        : "float32", name : "float32", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f32::MAX }, 1); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32({ let _ = &TerraneDescriptor { identity
        : "float32", name : "float32", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f32::from_bits(1) }, - 1); __terrane_result } == zero))
    );
    wide_four = 4.0;
    floating_exponent = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .hypot({ let source_value = wide_four; let converted = source_value as f32; if
        converted as f64 == source_value { converted } else {
        __terrane_raised(Err(terrane_int_support::ArithmeticError::conversion_overflow(&source_value,
        "float64", "float32", "the floating value is not exactly representable")),
        85 /* terrane-site: tests/integration/complete-float-surface/case.trn:50:29-50:38 */) } });
        __terrane_result } == 5.0_f32)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = terrane_scalar_support::scale_binary_f32(one,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent), 86 /* terrane-site: tests/integration/complete-float-surface/case.trn:50:67-50:84 */));
        __terrane_result } == four))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f32 = negative_one; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f32 = one; let terrane_lower : f32 = two; let terrane_upper : f32
        = one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    nan_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(not_a_number);
        __terrane_result
    };
    infinity_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(infinity);
        __terrane_result
    };
    negative_zero_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(negative_zero);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition.mantissa
        .is_nan()), terrane_scalar_support::scalar_text(&(nan_decomposition.exponent ==
        0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&infinity_decomposition.mantissa
        .is_infinite()), terrane_scalar_support::scalar_text(&(infinity_decomposition
        .exponent == 0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_zero_decomposition.mantissa
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition.exponent == 0))
    );
    negative_infinity = negative_one / zero;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f32 = infinity; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f32 = negative_infinity; let terrane_fraction =
        terrane_receiver.fract(); if terrane_fraction == 0.0 { 0.0_f32
        .copysign(terrane_receiver) } else { terrane_fraction } }; __terrane_result }
        .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        negative_infinity.next_down(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = negative_infinity
        .next_down(); __terrane_result } .is_sign_negative())
    );
    large_exponent = 120.0_f32;
    large = {
        let __terrane_result = large_exponent.exp2();
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(large, - 250); __terrane_result } == {
        let __terrane_result = terrane_scalar_support::scale_binary_f32(one, - 130);
        __terrane_result }))
    );
    descriptor = TerraneDescriptor {
        identity: "float32",
        name: "float32",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ = &descriptor; f32::EPSILON }
        == { let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::EPSILON }))
    );
}
fn exercise64() {
    let zero: f64;
    let negative_zero: f64;
    let one: f64;
    let two: f64;
    let three: f64;
    let four: f64;
    let eight: f64;
    let negative_one: f64;
    let negative_two: f64;
    let negative_quarter: f64;
    let fractional: f64;
    let twelve: f64;
    let thousand: f64;
    let overflow_input: f64;
    let underflow_input: f64;
    let decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let not_a_number: f64;
    let infinity: f64;
    let clamped_zero: f64;
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let narrow_four: f32;
    let floating_exponent: f64;
    let nan_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let infinity_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let negative_zero_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let negative_infinity: f64;
    let large_exponent: f64;
    let large: f64;
    let descriptor: TerraneDescriptor;
    zero = 0.0;
    negative_zero = -0.0_f64;
    one = 1.0;
    two = 2.0;
    three = 3.0;
    four = 4.0;
    eight = 8.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = eight
        .cbrt(); __terrane_result } == two)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = three.hypot(four); __terrane_result } == 5.0))
    );
    negative_one = -1.0_f64;
    negative_two = -2.0_f64;
    negative_quarter = -0.25_f64;
    fractional = -1.25_f64;
    twelve = 12.0;
    thousand = 1000.0;
    overflow_input = 1024.0;
    underflow_input = -1075.0_f64;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = four
        .powf(0.5); __terrane_result } == two)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = two.powi(10); __terrane_result } == 1024.0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .exp2(); __terrane_result } == eight)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.exp_m1(); __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .ln_1p(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log2(); __terrane_result } == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = thousand
        .log10(); __terrane_result } == three)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log(two); __terrane_result } == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .tan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = zero.asin(); __terrane_result } == zero)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = one.acos();
        __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .atan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.atan2(negative_one); __terrane_result } > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one
        .copysign(negative_zero); __terrane_result } == negative_one)),
        terrane_scalar_support::scalar_text(&negative_zero.is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_value : f64 = two; let terrane_lower : f64 = 3.0; let terrane_upper : f64
        = 4.0; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } == three)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = fractional; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_up(); __terrane_result } == { let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_down(); __terrane_result } == negative_one * { let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) }))
    );
    decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(twelve);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa == 0.75)),
        terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(decomposition.mantissa, decomposition
        .exponent); __terrane_result } == twelve))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f64::RADIX)) } ==
        terrane_int_support::Int::from(2_i128))), terrane_scalar_support::scalar_text(&({
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f64::MANTISSA_DIGITS)) } ==
        terrane_int_support::Int::from(53_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::EPSILON } > zero)),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::MIN_POSITIVE } .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) } >
        zero)), terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::MIN } < zero)),
        terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::MAX } > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = two.asin();
        __terrane_result } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        __terrane_result = negative_two.powf(0.5); __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        overflow_input.exp2(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = underflow_input
        .exp2(); __terrane_result } == zero))
    );
    not_a_number = {
        let __terrane_result = two.asin();
        __terrane_result
    };
    infinity = one / zero;
    clamped_zero = {
        let __terrane_result = {
            let terrane_value: f64 = negative_zero;
            let terrane_lower: f64 = zero;
            let terrane_upper: f64 = one;
            if terrane_value.is_nan() {
                terrane_value
            } else if terrane_lower.is_nan() || terrane_upper.is_nan()
                || terrane_lower > terrane_upper
            {
                f64::NAN
            } else {
                let terrane_lowered = if terrane_value == 0.0 && terrane_lower == 0.0 {
                    if terrane_value.is_sign_positive()
                        || terrane_lower.is_sign_positive()
                    {
                        0.0
                    } else {
                        -0.0
                    }
                } else {
                    terrane_value.max(terrane_lower)
                };
                if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                    if terrane_lowered.is_sign_negative()
                        || terrane_upper.is_sign_negative()
                    {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    terrane_lowered.min(terrane_upper)
                }
            }
        };
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f64 = not_a_number; let terrane_lower : f64 = zero; let
        terrane_upper : f64 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f64::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let terrane_value
        : f64 = one; let terrane_lower : f64 = not_a_number; let terrane_upper : f64 =
        one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        not_a_number.hypot(infinity); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = infinity.next_up();
        __terrane_result } .is_infinite())
    );
    subnormal_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64({
            let _ = &TerraneDescriptor {
                identity: "float64",
                name: "float64",
                kind: "type",
                inherently_identity_bearing: false,
                fields: &[],
            };
            f64::from_bits(1)
        });
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(subnormal_decomposition.mantissa,
        subnormal_decomposition.exponent); __terrane_result } == { let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) }))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        terrane_scalar_support::scale_binary_f64({ let _ = &TerraneDescriptor { identity
        : "float64", name : "float64", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f64::MAX }, 1); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64({ let _ = &TerraneDescriptor { identity
        : "float64", name : "float64", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f64::from_bits(1) }, - 1); __terrane_result } == zero))
    );
    narrow_four = 4.0_f32;
    floating_exponent = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .hypot(narrow_four as f64); __terrane_result } == 5.0)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(one,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent), 87 /* terrane-site: tests/integration/complete-float-surface/case.trn:115:69-115:86 */));
        __terrane_result } == four))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f64 = negative_one; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f64 = one; let terrane_lower : f64 = two; let terrane_upper : f64
        = one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    nan_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(not_a_number);
        __terrane_result
    };
    infinity_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(infinity);
        __terrane_result
    };
    negative_zero_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(negative_zero);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition.mantissa
        .is_nan()), terrane_scalar_support::scalar_text(&(nan_decomposition.exponent ==
        0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&infinity_decomposition.mantissa
        .is_infinite()), terrane_scalar_support::scalar_text(&(infinity_decomposition
        .exponent == 0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_zero_decomposition.mantissa
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition.exponent == 0))
    );
    negative_infinity = negative_one / zero;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f64 = infinity; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f64 = negative_infinity; let terrane_fraction =
        terrane_receiver.fract(); if terrane_fraction == 0.0 { 0.0_f64
        .copysign(terrane_receiver) } else { terrane_fraction } }; __terrane_result }
        .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        negative_infinity.next_down(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = negative_infinity
        .next_down(); __terrane_result } .is_sign_negative())
    );
    large_exponent = 1000.0;
    large = {
        let __terrane_result = large_exponent.exp2();
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(large, - 2000); __terrane_result } == {
        let __terrane_result = terrane_scalar_support::scale_binary_f64(one, - 1000);
        __terrane_result }))
    );
    descriptor = TerraneDescriptor {
        identity: "float64",
        name: "float64",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ = &descriptor; f64::EPSILON }
        == { let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::EPSILON }))
    );
}
fn test_scenario_terrane_complete_float_surface() {
    exercise32();
    exercise64();
}
// Source: tests/integration/concrete-consuming-local-capture/case.trn
// Namespace: concrete-consuming-local-capture
pub trait TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564(
    Box<
        dyn TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol
for TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565,
>
for TerraneNsConcreteConsumingLocalCaptureNamedIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_concrete_consuming_local_capture(
    item: TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565,
) -> terrane_int_support::Int {
    let g: TerraneConsumingCallable<(), terrane_int_support::Int>;
    g = {
        let item = item.clone();
        TerraneConsumingCallable::new(move |(): ()| -> terrane_int_support::Int {
            return item.name();
        })
    };
    return g.call(());
}
fn test_scenario_terrane_concrete_consuming_local_capture() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_concrete_consuming_local_capture(TerraneNsConcreteConsumingLocalCaptureValueIdentity2F636F6E63726574652D636F6E73756D696E672D6C6F63616C2D636170747572650076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/concrete-returned-shared-capture/case.trn
// Namespace: concrete-returned-shared-capture
pub trait TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564(
    Box<
        dyn TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol
for TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565,
>
for TerraneNsConcreteReturnedSharedCaptureNamedIdentity2F636F6E63726574652D72657475726E65642D7368617265642D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn mk(
    item: TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565,
) -> std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync> {
    return {
        let item = item.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
}
fn test_scenario_terrane_concrete_returned_shared_capture() {
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    g = mk(
        TerraneNsConcreteReturnedSharedCaptureValueIdentity2F636F6E63726574652D72657475726E65642D7368617265642D636170747572650076616C7565::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&g()));
}
// Source: tests/integration/concrete-two-capture-class/case.trn
// Namespace: concrete-two-capture-class
pub trait TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564(
    Box<
        dyn TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol
for TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565,
>
for TerraneNsConcreteTwoCaptureClassNamedIdentity2F636F6E63726574652D74776F2D636170747572652D636C617373006E616D6564 {
    fn from(
        value: TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_concrete_two_capture_class(
    item: TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565,
) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let item = item.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
    g = {
        let item = item.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
    return f() + g();
}
fn test_scenario_terrane_concrete_two_capture_class() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_concrete_two_capture_class(TerraneNsConcreteTwoCaptureClassValueIdentity2F636F6E63726574652D74776F2D636170747572652D636C6173730076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/concrete-two-capture-string/case.trn
// Namespace: concrete-two-capture-string
fn go_terrane_concrete_two_capture_string(s: String) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let s = s.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(
                terrane_string_support::length(&s) as i128,
            );
        })
    };
    g = {
        let s = s.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(
                terrane_string_support::length(&s) as i128,
            );
        })
    };
    return f() + g();
}
fn test_scenario_terrane_concrete_two_capture_string() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_concrete_two_capture_string(String::from("ab")))
    );
}
// Source: tests/integration/consuming-callable-field-take/case.trn
// Namespace: consuming-callable-field-take
pub struct TerraneNsConsumingCallableFieldTakeHolderIdentity2F636F6E73756D696E672D63616C6C61626C652D6669656C642D74616B6500686F6C646572 {
    pub f: Option<TerraneConsumingCallable<(), terrane_int_support::Int>>,
}
impl TerraneNsConsumingCallableFieldTakeHolderIdentity2F636F6E73756D696E672D63616C6C61626C652D6669656C642D74616B6500686F6C646572 {
    pub fn terrane_construct(
        f: TerraneConsumingCallable<(), terrane_int_support::Int>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: TerraneConsumingCallable<(), terrane_int_support::Int>,
    ) {
        {
            let __terrane_value: TerraneConsumingCallable<
                (),
                terrane_int_support::Int,
            > = f;
            self.f = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_consuming_callable_field_take() {
    let mut h: TerraneNsConsumingCallableFieldTakeHolderIdentity2F636F6E73756D696E672D63616C6C61626C652D6669656C642D74616B6500686F6C646572;
    let g: TerraneConsumingCallable<(), terrane_int_support::Int>;
    h = TerraneNsConsumingCallableFieldTakeHolderIdentity2F636F6E73756D696E672D63616C6C61626C652D6669656C642D74616B6500686F6C646572::terrane_construct({
        TerraneConsumingCallable::new(move |(): ()| -> terrane_int_support::Int {
            return terrane_int_support::Int::from(5_i128);
        })
    });
    g = h.f.take().expect("required field initialized");
    println!("{}", terrane_scalar_support::scalar_text(&g.call(())));
}
// Source: tests/integration/consuming-method-reference-snapshot/case.trn
// Namespace: consuming-method-reference-snapshot
#[derive(Clone)]
pub struct TerraneNsConsumingMethodReferenceSnapshotHolderIdentity2F636F6E73756D696E672D6D6574686F642D7265666572656E63652D736E617073686F7400686F6C646572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsConsumingMethodReferenceSnapshotHolderIdentity2F636F6E73756D696E672D6D6574686F642D7265666572656E63652D736E617073686F7400686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(5_i128),
        }
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
fn test_scenario_terrane_consuming_method_reference_snapshot() {
    let h: TerraneNsConsumingMethodReferenceSnapshotHolderIdentity2F636F6E73756D696E672D6D6574686F642D7265666572656E63652D736E617073686F7400686F6C646572;
    let callback: TerraneConsumingCallable<(), terrane_int_support::Int>;
    h = TerraneNsConsumingMethodReferenceSnapshotHolderIdentity2F636F6E73756D696E672D6D6574686F642D7265666572656E63652D736E617073686F7400686F6C646572::terrane_construct();
    callback = {
        let receiver = h;
        TerraneConsumingCallable::new(move |(): ()| receiver.finish())
    };
    println!("{}", terrane_scalar_support::scalar_text(&callback.call(())));
}
// Source: tests/integration/contextual-constant-arithmetic/case.trn
// Namespace: contextual-constant-arithmetic
fn third() -> Result<f32, TerraneError> {
    return Ok(0.33333334_f32);
}
fn bounded() -> i32 {
    return 1;
}
fn test_scenario_terrane_contextual_constant_arithmetic() -> Result<(), TerraneError> {
    let reduced: i8;
    let integral: i64;
    let quotient: i64;
    let ratio: f32;
    let rounded: terrane_int_support::Int;
    reduced = 100;
    integral = 4;
    quotient = 0;
    ratio = 0.33333334_f32;
    rounded = {
        let __terrane_result = __terrane_raised_err(
            terrane_int_support::rounded_f32(
                ratio,
                terrane_int_support::FloatRounding::TiesEven,
            ),

                88 /* terrane-site: tests/integration/contextual-constant-arithmetic/case.trn:14:17-14:29 */,
        )?;
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&reduced));
    println!("{}", terrane_scalar_support::scalar_text(&integral));
    println!("{}", terrane_scalar_support::scalar_text(&quotient));
    println!("{}", terrane_scalar_support::scalar_text(&ratio));
    println!("{}", terrane_scalar_support::scalar_text(&rounded));
    println!("{}", terrane_scalar_support::scalar_text(&true));
    println!("{}", terrane_scalar_support::scalar_text(&false));
    println!("{}", terrane_scalar_support::scalar_text(&bounded()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(third(),
        89 /* terrane-site: tests/integration/contextual-constant-arithmetic/case.trn:23:11-23:17 */) ?)
    );
    Ok(())
}
// Source: tests/integration/contextual-floating-literals/case.trn
// Namespace: contextual-floating-literals
fn test_scenario_terrane_contextual_floating_literals() {
    let hexadecimal: f32;
    let hexadecimal64: f64;
    let whole: f64;
    hexadecimal = 255.0_f32;
    hexadecimal64 = 5.0_f64;
    whole = 9007199254740992.0_f64;
    println!("{}", terrane_scalar_support::scalar_text(&hexadecimal));
    println!("{}", terrane_scalar_support::scalar_text(&hexadecimal64));
    println!("{}", terrane_scalar_support::scalar_text(&whole));
}
// Source: tests/integration/core-regex/case.trn
// Namespace: core-regex
fn first_match(
    pattern: RegexPattern,
    text: String,
) -> Result<Option<terrane_string_support::TextRange>, TerraneError> {
    return Ok(
        __terrane_traced_err(
            pattern.__trn_6d61746368(AsRef::<str>::as_ref(&text)),
            90 /* terrane-site: tests/integration/core-regex/case.trn:7:10-7:29 */,
        )?,
    );
}
fn scalar_end(hit: &terrane_string_support::TextRange) -> terrane_int_support::Int {
    return terrane_int_support::Int::from((*hit).scalar_end() as i128);
}
fn matched_text(hit: &terrane_string_support::TextRange) -> String {
    return (*hit).text().to_owned();
}
fn item_count(
    items: &terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from((*items).length());
}
fn grapheme_count(text: &String) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(terrane_string_support::length(&text) as i128);
}
fn test_scenario_terrane_core_regex() -> Result<(), TerraneError> {
    let pattern: RegexPattern;
    let mut source: String;
    let first: Option<terrane_string_support::TextRange>;
    let found: Option<RegexCaptures>;
    let full: Option<terrane_string_support::TextRange>;
    let start: terrane_int_support::Int;
    let end: terrane_int_support::Int;
    let boundaries: terrane_collection_support::List<terrane_int_support::Int>;
    let call: Option<terrane_string_support::TextRange>;
    let empty: Option<terrane_string_support::TextRange>;
    let mut error: Option<TerraneError> = None;
    let lookaround: RegexPattern;
    let response: Option<RegexCaptures>;
    let status: Option<terrane_string_support::TextRange>;
    let repeated: RegexPattern;
    let positional: RegexPattern;
    let expensive: RegexPattern;
    let input: String;
    let mut error_2: Option<TerraneError> = None;
    let sharp_s: RegexPattern;
    let scalar: RegexPattern;
    let decomposed: String;
    pattern = __terrane_traced_err(
        compile("([0-9]+)?(?<call>[a-z]+)()\\(", true),
        91 /* terrane-site: tests/integration/core-regex/case.trn:22:13-22:75 */,
    )?;
    source = String::from("é OPEN(");
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(pattern
        .matches(AsRef:: < str > ::as_ref(&source)), 92 /* terrane-site: tests/integration/core-regex/case.trn:24:11-24:34 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(pattern
        .matches("é ?"), 93 /* terrane-site: tests/integration/core-regex/case.trn:25:11-25:33 */) ?)
    );
    first = __terrane_traced_err(
        first_match(pattern.clone(), source.clone()),
        94 /* terrane-site: tests/integration/core-regex/case.trn:26:27-26:55 */,
    )?;
    if (&first).is_some() {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(match
            &first { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .byte_start() as i128))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(match
            &first { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .scalar_start() as i128))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&matched_text(match &first {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }))
        );
    }
    found = __terrane_traced_err(
        pattern.captures(AsRef::<str>::as_ref(&source)),
        95 /* terrane-site: tests/integration/core-regex/case.trn:31:11-31:35 */,
    )?;
    if (&found).is_some() {
        source = String::from("reassigned");
        full = match &found {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .group(terrane_int_support::Int::from(0_i128));
        if (&full).is_some() {
            start = terrane_int_support::Int::from(
                match &full {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .byte_start() as i128,
            );
            end = terrane_int_support::Int::from(
                match &full {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .byte_end() as i128,
            );
            boundaries = terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![start.clone(), end.clone()]);
            if item_count(&boundaries) != terrane_int_support::Int::from(2_i128) {
                println!("{}", String::from("incorrect boundary count"));
            }
            println!("{}", terrane_scalar_support::scalar_text(&end));
            println!("{}", terrane_scalar_support::scalar_text(&(&end - &start)));
            println!(
                "{}", terrane_scalar_support::scalar_text(&scalar_end(match &full {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                }))
            );
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(match
                &full { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") } .grapheme_end() as i128)
                + terrane_int_support::Int::from(1_i128)))
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") }
            .group(terrane_int_support::Int::from(1_i128)).is_none())
        );
        call = match &found {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .named_group("call");
        if (&call).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &call { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") } .text()
                .to_owned())
            );
        }
        empty = match &found {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .group(terrane_int_support::Int::from(3_i128));
        if (&empty).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&(match &empty { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") } .text()
                .to_owned().as_str() == ""))
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") }
            .group(terrane_int_support::Int::from(- 1_i128)).is_none())
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") }
            .named_group("unknown").is_none())
        );
        println!("{}", terrane_scalar_support::scalar_text(&source));
    }
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let _: RegexPattern = __terrane_traced_completion!(
                    compile("[", false), 96 /* terrane-site: tests/integration/core-regex/case.trn:56:9-56:21 */
                );
            }
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
                    let _ = error.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&{ let
                        __terrane_error_text_0 = error.as_ref()
                        .expect("flow-proven available binding").message();
                        __terrane_error_text_0 != "" })
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    lookaround = __terrane_traced_err(
        compile("(?<=HTTP/1\\.1 )(?<status>[0-9]+)(?= OK)", false),
        97 /* terrane-site: tests/integration/core-regex/case.trn:60:16-60:66 */,
    )?;
    response = __terrane_traced_err(
        lookaround.captures("read: HTTP/1.1 200 OK"),
        98 /* terrane-site: tests/integration/core-regex/case.trn:61:14-61:58 */,
    )?;
    if (&response).is_some() {
        status = match &response {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .named_group("status");
        if (&status).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &status { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") } .text()
                .to_owned())
            );
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(match
                &status { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") } .byte_start() as i128))
            );
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(lookaround
        .matches("HTTP/1.1 200 Failed"), 99 /* terrane-site: tests/integration/core-regex/case.trn:67:11-67:52 */) ?)
    );
    repeated = __terrane_traced_err(
        compile("\\A(?<word>[a-z]+) \\k<word>\\z", false),
        100 /* terrane-site: tests/integration/core-regex/case.trn:69:14-69:53 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(repeated
        .matches("word word"), 101 /* terrane-site: tests/integration/core-regex/case.trn:70:11-70:40 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(repeated
        .matches("word other"), 102 /* terrane-site: tests/integration/core-regex/case.trn:71:11-71:41 */) ?)
    );
    positional = __terrane_traced_err(
        compile("\\A([a-z]+) \\1\\z", false),
        103 /* terrane-site: tests/integration/core-regex/case.trn:72:16-72:42 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(positional
        .matches("word word"), 104 /* terrane-site: tests/integration/core-regex/case.trn:73:11-73:42 */) ?)
    );
    expensive = __terrane_traced_err(
        compile("\\A(a|aa)+\\1b\\z", false),
        105 /* terrane-site: tests/integration/core-regex/case.trn:75:15-75:40 */,
    )?;
    input = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaacb");
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            {
                let _: bool = __terrane_traced_completion!(
                    expensive.matches(AsRef:: < str > ::as_ref(&input)),
                    106 /* terrane-site: tests/integration/core-regex/case.trn:78:9-78:33 */
                );
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_1 = true;
                    let _ = error_2.insert(__terrane_error_1.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&{ let
                        __terrane_error_text_1 = error_2.as_ref()
                        .expect("flow-proven available binding").message();
                        __terrane_error_text_1 != "" })
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            {
                let _: Option<terrane_string_support::TextRange> = __terrane_traced_completion!(
                    expensive.__trn_6d61746368(AsRef:: < str > ::as_ref(&input)),
                    107 /* terrane-site: tests/integration/core-regex/case.trn:82:9-82:31 */
                );
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("match limit"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            {
                let _: Option<RegexCaptures> = __terrane_traced_completion!(
                    expensive.captures(AsRef:: < str > ::as_ref(&input)),
                    108 /* terrane-site: tests/integration/core-regex/case.trn:86:9-86:34 */
                );
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && __terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_3 = true;
                    println!("{}", String::from("captures limit"));
                }
                if !__terrane_handled_3 {
                    return TerraneCompletion::Error(__terrane_error_3);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_3 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(expensive
        .matches("aab"), 109 /* terrane-site: tests/integration/core-regex/case.trn:89:11-89:35 */) ?)
    );
    sharp_s = __terrane_traced_err(
        compile("\\Astraße\\z", true),
        110 /* terrane-site: tests/integration/core-regex/case.trn:91:13-91:57 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(sharp_s
        .matches("STRASSE"), 111 /* terrane-site: tests/integration/core-regex/case.trn:92:11-92:37 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_receiver_value_2 =
        &String::from("straße");
        terrane_string_support::case_fold(&__terrane_receiver_value_2) } .as_str() == {
        let __terrane_receiver_value_3 = &String::from("STRASSE");
        terrane_string_support::case_fold(&__terrane_receiver_value_3) } .as_str()))
    );
    scalar = __terrane_traced_err(
        compile("\\A.\\z", false),
        112 /* terrane-site: tests/integration/core-regex/case.trn:94:12-94:28 */,
    )?;
    decomposed = String::from("e\u{301}");
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(scalar
        .matches(AsRef:: < str > ::as_ref(&decomposed)), 113 /* terrane-site: tests/integration/core-regex/case.trn:96:11-96:37 */) ?)
    );
    println!("{}", terrane_scalar_support::scalar_text(&grapheme_count(&decomposed)));
    Ok(())
}
// Source: tests/integration/declared-union-reaching-arm/case.trn
// Namespace: declared-union-reaching-arm
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF48S209 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF48S209 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn text(value: String) -> String {
    return value;
}
fn number_terrane_declared_union_reaching_arm(
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return &value + &terrane_int_support::Int::from(1_i128);
}
fn observe_terrane_declared_union_reaching_arm(flag: bool) {
    let mut value: TerraneUnionF48S209;
    let alias: String;
    let values: terrane_collection_support::List<String>;
    value = TerraneUnionF48S209::Arm1(String::from("alpha"));
    if flag {
        value = TerraneUnionF48S209::Arm0(terrane_int_support::Int::from(4_i128));
    }
    if matches!(&&value, TerraneUnionF48S209::Arm1(_)) {
        alias = match &value {
            TerraneUnionF48S209::Arm1(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .clone();
        values = terrane_collection_support::List::<
            String,
        >::new(
            vec![
                match value { TerraneUnionF48S209::Arm1(value) => value, _ =>
                unreachable!("flow-proven storage refinement") }
            ],
        );
        println!("{}", terrane_scalar_support::scalar_text(&text(alias)));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            114 /* terrane-site: tests/integration/declared-union-reaching-arm/case.trn:15:16-15:25 */)),
            114 /* terrane-site: tests/integration/declared-union-reaching-arm/case.trn:15:16-15:25 */))
        );
    } else {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&number_terrane_declared_union_reaching_arm(match
            value { TerraneUnionF48S209::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }))
        );
    }
    value = TerraneUnionF48S209::Arm0(terrane_int_support::Int::from(8_i128));
    if matches!(&&value, TerraneUnionF48S209::Arm0(_)) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&number_terrane_declared_union_reaching_arm(match
            value { TerraneUnionF48S209::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }))
        );
    }
}
fn test_scenario_terrane_declared_union_reaching_arm() {
    observe_terrane_declared_union_reaching_arm(false);
    observe_terrane_declared_union_reaching_arm(true);
}
// Source: tests/integration/destination-empty-collections/case.trn
// Namespace: destination-empty-collections
fn empty_list() -> terrane_collection_support::List<terrane_int_support::Int> {
    return terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]);
}
fn generic_empty<TerraneType54>() -> terrane_collection_support::List<TerraneType54> {
    return terrane_collection_support::List::<TerraneType54>::new(vec![]);
}
fn generic_count<TerraneType54>(
    values: terrane_collection_support::List<TerraneType54>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(values.length());
}
#[derive(Clone)]
pub struct GenericHolder<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub values: terrane_collection_support::List<TerraneType54>,
}
impl<TerraneType54> GenericHolder<TerraneType54> {
    pub fn terrane_construct(
        values: terrane_collection_support::List<TerraneType54>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            values: terrane_collection_support::List::<TerraneType54>::new(Vec::new()),
        };
        __terrane_constructed_value.construct(values);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        values: terrane_collection_support::List<TerraneType54>,
    ) {
        {
            let __terrane_value = values;
            self.values = __terrane_value;
        }
    }
    pub fn count(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(self.values.length());
    }
}
fn consume_generic<TerraneType54>(
    values: terrane_collection_support::List<TerraneType54>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(values.length());
}
fn consume_terrane_destination_empty_collections(
    values: terrane_collection_support::List<terrane_int_support::Int>,
) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
fn consume_optional(
    values: Option<terrane_collection_support::List<terrane_int_support::Int>>,
) {
    println!("{}", terrane_scalar_support::scalar_text(&(&values).is_none()));
}
#[derive(Clone)]
pub struct RequiredHolder {
    pub values: terrane_collection_support::List<terrane_int_support::Int>,
}
impl RequiredHolder {
    pub fn terrane_construct(
        values: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            values: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
        };
        __terrane_constructed_value.construct(values);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        values: terrane_collection_support::List<terrane_int_support::Int>,
    ) {
        {
            let __terrane_value: terrane_collection_support::List<
                terrane_int_support::Int,
            > = values;
            self.values = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsDestinationEmptyCollectionsHolderIdentity2F64657374696E6174696F6E2D656D7074792D636F6C6C656374696F6E7300486F6C646572 {
    pub values: Option<terrane_collection_support::List<terrane_int_support::Int>>,
}
impl TerraneNsDestinationEmptyCollectionsHolderIdentity2F64657374696E6174696F6E2D656D7074792D636F6C6C656374696F6E7300486F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {
            values: Some(
                terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]),
            ),
        }
    }
}
fn test_scenario_terrane_destination_empty_collections() {
    let returned: terrane_collection_support::List<terrane_int_support::Int>;
    let mut generic_returned: terrane_collection_support::List<terrane_int_support::Int>;
    let holder: TerraneNsDestinationEmptyCollectionsHolderIdentity2F64657374696E6174696F6E2D656D7074792D636F6C6C656374696F6E7300486F6C646572;
    let required: RequiredHolder;
    let generic_holder: GenericHolder<terrane_int_support::Int>;
    returned = empty_list();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length()))
    );
    generic_returned = generic_empty::<terrane_int_support::Int>();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(generic_returned
        .length()))
    );
    generic_returned = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(generic_returned
        .length()))
    );
    consume_terrane_destination_empty_collections(
        terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]),
    );
    consume_terrane_destination_empty_collections(
        terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]),
    );
    consume_optional(
        Some(terrane_collection_support::List::<terrane_int_support::Int>::new(vec![])),
    );
    consume_optional(None);
    holder = TerraneNsDestinationEmptyCollectionsHolderIdentity2F64657374696E6174696F6E2D656D7074792D636F6C6C656374696F6E7300486F6C646572::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&(&holder.values).is_none()));
    required = RequiredHolder::terrane_construct(
        terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(required
        .values.length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&generic_count(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&generic_count:: < String >
        (terrane_collection_support::List:: < String > ::new(vec![String::from("a"),
        String::from("b")])))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&consume_generic:: <
        terrane_int_support::Int > (terrane_collection_support::List:: <
        terrane_int_support::Int > ::new(vec![])))
    );
    generic_holder = GenericHolder::<
        terrane_int_support::Int,
    >::terrane_construct(
        terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&generic_holder.count()));
}
// Source: tests/integration/exact-constant-membership/case.trn
// Namespace: exact-constant-membership
fn test_scenario_terrane_exact_constant_membership() {
    println!("{}", terrane_scalar_support::scalar_text(&true));
    println!("{}", terrane_scalar_support::scalar_text(&false));
    println!("{}", terrane_scalar_support::scalar_text(&true));
}
// Source: tests/integration/exact-float-narrowing/case.trn
// Namespace: exact-float-narrowing
fn test_scenario_terrane_exact_float_narrowing() {
    let source: f64;
    let target: f32;
    source = 1.5;
    target = {
        let source_value = source;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),

                    115 /* terrane-site: tests/integration/exact-float-narrowing/case.trn:4:20-4:26 */,
            )
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&target));
}
// Source: tests/integration/explicit-numeric-coercions/case.trn
// Namespace: explicit-numeric-coercions
fn divide_terrane_explicit_numeric_coercions(numerator: f64, denominator: f64) -> f64 {
    return numerator / denominator;
}
fn test_values_terrane_explicit_numeric_coercions() -> Result<(), TerraneError> {
    let quantity: i64;
    let rounded64: f64;
    let rounded32: f32;
    let signed: i8;
    let signed_float: f32;
    let maximum: u128;
    let maximum_float: f64;
    let precise: f64;
    let narrowed: f32;
    let widened: f64;
    let infinity64: f64;
    let infinity32: f32;
    let nan64: f64;
    let nan32: f32;
    let checked_integer: Option<f32>;
    let base: f64;
    let too_large: f64;
    let checked_float: Option<f32>;
    let mut impossible: Option<f32> = None;
    quantity = 9007199254740993;
    rounded64 = __terrane_raised_err(
        terrane_int_support::coerce_to_f64(&quantity),

            116 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:10:23-10:38 */,
    )?;
    rounded32 = __terrane_raised_err(
        terrane_int_support::coerce_to_f32(&quantity),

            117 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:11:23-11:38 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&rounded64));
    println!("{}", terrane_scalar_support::scalar_text(&rounded32));
    signed = -7;
    signed_float = signed as f32;
    maximum = 340282366920938463463374607431768211455;
    maximum_float = maximum as f64;
    println!("{}", terrane_scalar_support::scalar_text(&signed_float));
    println!("{}", terrane_scalar_support::scalar_text(&maximum_float));
    precise = 16777217.0;
    narrowed = __terrane_raised_err(
        terrane_int_support::coerce_f64_to_f32(precise),

            118 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:23:22-23:36 */,
    )?;
    widened = narrowed as f64;
    println!("{}", terrane_scalar_support::scalar_text(&narrowed));
    println!("{}", terrane_scalar_support::scalar_text(&widened));
    infinity64 = divide_terrane_explicit_numeric_coercions(1.0, 0.0);
    infinity32 = __terrane_raised_err(
        terrane_int_support::coerce_f64_to_f32(infinity64),

            119 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:29:24-29:41 */,
    )?;
    nan64 = divide_terrane_explicit_numeric_coercions(0.0, 0.0);
    nan32 = __terrane_raised_err(
        terrane_int_support::coerce_f64_to_f32(nan64),

            120 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:31:19-31:31 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    println!("{}", terrane_scalar_support::scalar_text(&nan32.is_nan()));
    checked_integer = terrane_int_support::coerce_fixed_to_f32(maximum).ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_integer.is_none()));
    base = 65536.0;
    too_large = base * base * base * base * base * base * base * base;
    checked_float = terrane_int_support::coerce_f64_to_f32(too_large).ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_float.is_none()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = impossible
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::coerce_fixed_to_f32(maximum),
                        121 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:44:26-44:40 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&impossible.as_ref()
                .expect("flow-proven available binding"))
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
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("integer overflow caught"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            let _ = impossible
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::coerce_f64_to_f32(too_large),
                        122 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:50:26-50:42 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&impossible.as_ref()
                .expect("flow-proven available binding"))
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
                    && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("float overflow caught"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/fixed-integer-float-destination/case.trn
// Namespace: fixed-integer-float-destination
fn test_scenario_terrane_fixed_integer_float_destination() {
    let signed64: i64;
    let signed64_float: f64;
    let unsigned64: u64;
    let unsigned64_float: f64;
    let signed128: i128;
    let signed128_float: f64;
    let unsigned128: u128;
    let unsigned128_float: f64;
    let signed32: i32;
    let signed32_float: f32;
    let largest_float32_integer: u128;
    let largest_float32: f32;
    let mut inexact64: Option<u64> = None;
    let mut rejected64: Option<f64> = None;
    let mut inexact32: Option<u128> = None;
    let mut rejected32: Option<f32> = None;
    signed64 = 9007199254740992;
    signed64_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(signed64),

            123 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:6:28-6:36 */,
    );
    unsigned64 = 9223372036854775808;
    unsigned64_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(unsigned64),

            124 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:8:30-8:40 */,
    );
    signed128 = -170141183460469231731687303715884105728;
    signed128_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(signed128),

            125 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:10:29-10:38 */,
    );
    unsigned128 = 170141183460469231731687303715884105728;
    unsigned128_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(unsigned128),

            126 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:12:31-12:42 */,
    );
    signed32 = 16777216;
    signed32_float = __terrane_raised(
        terrane_int_support::exact_fixed_f32(signed32),

            127 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:14:28-14:36 */,
    );
    largest_float32_integer = 340282346638528859811704183484516925440;
    largest_float32 = __terrane_raised(
        terrane_int_support::exact_fixed_f32(largest_float32_integer),

            128 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:16:29-16:52 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&signed64_float));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned64_float));
    println!("{}", terrane_scalar_support::scalar_text(&signed128_float));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned128_float));
    println!("{}", terrane_scalar_support::scalar_text(&signed32_float));
    println!("{}", terrane_scalar_support::scalar_text(&largest_float32));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = inexact64.insert(18446744073709551615);
            let _ = rejected64
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::exact_fixed_f64(* inexact64.as_ref()
                        .expect("flow-proven available binding")),
                        129 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:27:26-27:35 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&rejected64.as_ref()
                .expect("flow-proven available binding"))
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
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("inexact float64 caught"));
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
            let _ = inexact32.insert(340282366920938463463374607431768211455);
            let _ = rejected32
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::exact_fixed_f32(* inexact32.as_ref()
                        .expect("flow-proven available binding")),
                        130 /* terrane-site: tests/integration/fixed-integer-float-destination/case.trn:34:26-34:35 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&rejected32.as_ref()
                .expect("flow-proven available binding"))
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
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("inexact float32 caught"));
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
// Source: tests/integration/float-narrowing-special-values/case.trn
// Namespace: float-narrowing-special-values
fn test_scenario_terrane_float_narrowing_special_values() {
    let zero: f64;
    let one: f64;
    let negative_one: f64;
    let negative_zero: f64;
    let maximum: f64;
    let positive_infinity: f64;
    let negative_infinity: f64;
    let narrowed_zero: f32;
    let narrowed_positive: f32;
    let narrowed_negative: f32;
    let narrowed_maximum: f32;
    zero = 0.0;
    one = 1.0;
    negative_one = -1.0_f64;
    negative_zero = -0.0_f64;
    maximum = 340282346638528859811704183484516925440.0;
    positive_infinity = one / zero;
    negative_infinity = negative_one / zero;
    narrowed_zero = {
        let source_value = negative_zero;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),

                    131 /* terrane-site: tests/integration/float-narrowing-special-values/case.trn:10:27-10:40 */,
            )
        }
    };
    narrowed_positive = {
        let source_value = positive_infinity;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),

                    132 /* terrane-site: tests/integration/float-narrowing-special-values/case.trn:11:31-11:48 */,
            )
        }
    };
    narrowed_negative = {
        let source_value = negative_infinity;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),

                    133 /* terrane-site: tests/integration/float-narrowing-special-values/case.trn:12:31-12:48 */,
            )
        }
    };
    narrowed_maximum = {
        let source_value = maximum;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),

                    134 /* terrane-site: tests/integration/float-narrowing-special-values/case.trn:13:30-13:37 */,
            )
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_zero));
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_positive));
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_negative));
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_maximum));
}
// Source: tests/integration/foundational-float-math/case.trn
// Namespace: foundational-float-math
fn test_scenario_terrane_foundational_float_math() {
    let zero32: f32;
    let one32: f32;
    let nine32: f32;
    let root32: std::sync::Arc<dyn Fn() -> f32 + Send + Sync>;
    let pair32: terrane_collection_support::Tuple<f32>;
    let zero64: f64;
    let one64: f64;
    let nine64: f64;
    let pair64: terrane_collection_support::Tuple<f64>;
    let negative: f64;
    let not_a_number: f64;
    let negative_zero: f64;
    let reciprocal: f64;
    let negative_infinity: f64;
    let negative32: f32;
    let low32: f32;
    let high32: f32;
    let not_a_number32: f32;
    let infinity32: f32;
    let low64: f64;
    let high64: f64;
    let minimum64: std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync>;
    let maximum64: std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync>;
    let multiply_add64: std::sync::Arc<dyn Fn(f64, f64) -> f64 + Send + Sync>;
    let minimum_zero: f64;
    let maximum_zero: f64;
    let multiplicand: f64;
    let multiplier: f64;
    let addend: f64;
    let fused: f64;
    let unfused: f64;
    let multiplicand32: f32;
    let multiplier32: f32;
    let addend32: f32;
    let fused32: f32;
    let unfused32: f32;
    zero32 = 0.0_f32;
    one32 = 1.0_f32;
    nine32 = 9.0_f32;
    root32 = {
        let receiver = nine32;
        std::sync::Arc::new(move || receiver.sqrt())
    };
    println!("{}", terrane_scalar_support::scalar_text(&(root32() == 3.0_f32)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero32
        .sin(); __terrane_result } == 0.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero32
        .cos(); __terrane_result } == 1.0_f32))
    );
    pair32 = {
        let __terrane_result = {
            let terrane_sine_cosine = zero32.sin_cos();
            terrane_collection_support::Tuple::new(
                vec![terrane_sine_cosine.0, terrane_sine_cosine.1],
            )
        };
        __terrane_result
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(pair32
        .length()) == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair32
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        135 /* terrane-site: tests/integration/foundational-float-math/case.trn:13:30-13:39 */)),
        135 /* terrane-site: tests/integration/foundational-float-math/case.trn:13:30-13:39 */) == 0.0_f32)),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair32
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        136 /* terrane-site: tests/integration/foundational-float-math/case.trn:13:48-13:57 */)),
        136 /* terrane-site: tests/integration/foundational-float-math/case.trn:13:48-13:57 */) == 1.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one32.ln();
        __terrane_result } == 0.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero32
        .exp(); __terrane_result } == 1.0_f32))
    );
    zero64 = 0.0;
    one64 = 1.0;
    nine64 = 9.0;
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = nine64
        .sqrt(); __terrane_result } == 3.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero64
        .sin(); __terrane_result } == 0.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero64
        .cos(); __terrane_result } == 1.0))
    );
    pair64 = {
        let __terrane_result = {
            let terrane_sine_cosine = zero64.sin_cos();
            terrane_collection_support::Tuple::new(
                vec![terrane_sine_cosine.0, terrane_sine_cosine.1],
            )
        };
        __terrane_result
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(pair64
        .length()) == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair64
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        137 /* terrane-site: tests/integration/foundational-float-math/case.trn:24:30-24:39 */)),
        137 /* terrane-site: tests/integration/foundational-float-math/case.trn:24:30-24:39 */) == 0.0)),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair64
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        138 /* terrane-site: tests/integration/foundational-float-math/case.trn:24:48-24:57 */)),
        138 /* terrane-site: tests/integration/foundational-float-math/case.trn:24:48-24:57 */) == 1.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one64.ln();
        __terrane_result } == 0.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero64
        .exp(); __terrane_result } == 1.0))
    );
    negative = -1.0_f64;
    not_a_number = {
        let __terrane_result = negative.sqrt();
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(not_a_number != not_a_number)));
    negative_zero = -0.0_f64;
    reciprocal = 1.0
        / {
            let __terrane_result = negative_zero.sqrt();
            __terrane_result
        };
    println!("{}", terrane_scalar_support::scalar_text(&(reciprocal < 0.0)));
    negative_infinity = {
        let __terrane_result = zero64.ln();
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(negative_infinity < 0.0)));
    negative32 = -3.0_f32;
    low32 = 2.0_f32;
    high32 = 5.0_f32;
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = negative32
        .abs(); __terrane_result } == 3.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = high32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } ==
        2.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = high32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } ==
        5.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = low32
        .mul_add(high32, 1.0_f32); __terrane_result } == 11.0_f32))
    );
    println!("{}", terrane_scalar_support::scalar_text(&low32.is_finite()));
    not_a_number32 = {
        let __terrane_result = negative32.sqrt();
        __terrane_result
    };
    infinity32 = 1.0_f32 / zero32;
    println!("{}", terrane_scalar_support::scalar_text(&not_a_number32.is_nan()));
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    low64 = 2.0;
    high64 = 5.0;
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = negative
        .abs(); __terrane_result } == 1.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = high64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } == 2.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = high64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } == 5.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = low64
        .mul_add(high64, 1.0); __terrane_result } == 11.0))
    );
    minimum64 = {
        let receiver = low64;
        std::sync::Arc::new(move |argument_0: f64| {
            let terrane_receiver: f64 = receiver;
            let terrane_argument: f64 = argument_0;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_negative()
                    || terrane_argument.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_receiver.min(terrane_argument)
            }
        })
    };
    maximum64 = {
        let receiver = low64;
        std::sync::Arc::new(move |argument_0: f64| {
            let terrane_receiver: f64 = receiver;
            let terrane_argument: f64 = argument_0;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_positive()
                    || terrane_argument.is_sign_positive()
                {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_receiver.max(terrane_argument)
            }
        })
    };
    multiply_add64 = {
        let receiver = low64;
        std::sync::Arc::new(move |argument_0: f64, argument_1: f64| {
            receiver.mul_add(argument_0, argument_1)
        })
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(minimum64(high64) == 2.0)),
        terrane_scalar_support::scalar_text(&(maximum64(high64) == 5.0)),
        terrane_scalar_support::scalar_text(&(multiply_add64(high64, 1.0) == 11.0))
    );
    println!("{}", terrane_scalar_support::scalar_text(&low64.is_finite()));
    println!("{}", terrane_scalar_support::scalar_text(&not_a_number.is_nan()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&negative_infinity.is_infinite())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = not_a_number; let terrane_argument : f64 = low64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } ==
        low64))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = not_a_number; let terrane_argument : f64 = high64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } ==
        high64))
    );
    minimum_zero = {
        let __terrane_result = {
            let terrane_receiver: f64 = negative_zero;
            let terrane_argument: f64 = zero64;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_negative()
                    || terrane_argument.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_receiver.min(terrane_argument)
            }
        };
        __terrane_result
    };
    maximum_zero = {
        let __terrane_result = {
            let terrane_receiver: f64 = negative_zero;
            let terrane_argument: f64 = zero64;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_positive()
                    || terrane_argument.is_sign_positive()
                {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_receiver.max(terrane_argument)
            }
        };
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(1.0 / minimum_zero < 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&(1.0 / maximum_zero > 0.0)));
    multiplicand = 1.0000000000000002;
    multiplier = 1.0000000000000002;
    addend = -1.0000000000000004_f64;
    fused = {
        let __terrane_result = multiplicand.mul_add(multiplier, addend);
        __terrane_result
    };
    unfused = multiplicand * multiplier + addend;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(fused ==
        0.00000000000000000000000000000004930380657631323784))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(unfused == 0.0)));
    multiplicand32 = 1.0000001_f32;
    multiplier32 = 1.0000001_f32;
    addend32 = -1.0000002_f32;
    fused32 = {
        let __terrane_result = multiplicand32.mul_add(multiplier32, addend32);
        __terrane_result
    };
    unfused32 = multiplicand32 * multiplier32 + addend32;
    println!("{}", terrane_scalar_support::scalar_text(&(fused32 == 1.4210855e-14_f32)));
    println!("{}", terrane_scalar_support::scalar_text(&(unfused32 == 0.0_f32)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = 0.0_f32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } ==
        0.0_f32)), terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = 7.0_f32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } ==
        7.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = 0.0; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } == 0.0)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = 7.0; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } == 7.0))
    );
}
// Source: tests/integration/function-value-name-collisions/case.trn
// Namespace: function-value-name-collisions
fn test_scenario_terrane_function_value_name_collisions() -> Result<(), TerraneError> {
    let left: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let right: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let value: String;
    let parsed: Option<terrane_int_support::Int>;
    left = std::sync::Arc::new(selected_terrane_function_value_left);
    right = std::sync::Arc::new(selected_terrane_function_value_right);
    __terrane_traced_err(
        assert(left() == terrane_int_support::Int::from(11_i128)),

            139 /* terrane-site: tests/integration/function-value-name-collisions/case.trn:12:5-12:26 */,
    )?;
    __terrane_traced_err(
        assert(right() == terrane_int_support::Int::from(22_i128)),

            140 /* terrane-site: tests/integration/function-value-name-collisions/case.trn:13:5-13:27 */,
    )?;
    value = String::from("value");
    __terrane_traced_err(
        assert(
            parse_selected_terrane_function_value_left(value.clone())
                == terrane_int_support::Int::from(11_i128),
        ),

            141 /* terrane-site: tests/integration/function-value-name-collisions/case.trn:15:5-15:44 */,
    )?;
    parsed = Some(parse_selected_terrane_function_value_right(value));
    if (&parsed).is_some() {
        __terrane_traced_err(
            assert(
                *match &parsed {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                } == terrane_int_support::Int::from(22_i128),
            ),

                142 /* terrane-site: tests/integration/function-value-name-collisions/case.trn:18:9-18:29 */,
        )?;
    } else {
        __terrane_traced_err(
            assert(false),

                143 /* terrane-site: tests/integration/function-value-name-collisions/case.trn:20:9-20:22 */,
        )?;
    }
    Ok(())
}
// Source: tests/integration/function-value-name-collisions/left.trn
// Namespace: function-value-left
fn selected_terrane_function_value_left() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(11_i128);
}
fn parse_selected_terrane_function_value_left(
    source: String,
) -> terrane_int_support::Int {
    let _ = &source;
    return terrane_int_support::Int::from(11_i128);
}
// Source: tests/integration/function-value-name-collisions/right.trn
// Namespace: function-value-right
fn selected_terrane_function_value_right() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(22_i128);
}
fn parse_selected_terrane_function_value_right(
    source: String,
) -> terrane_int_support::Int {
    let _ = &source;
    return terrane_int_support::Int::from(22_i128);
}
// Source: tests/integration/generic-optional-owned-return/case.trn
// Namespace: generic-optional-owned-return
fn choose_terrane_generic_optional_owned_return<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    if value.is_none() {
        return fallback;
    }
    return match value {
        Some(value) => value,
        _ => unreachable!("flow-proven storage refinement"),
    };
}
fn choose_present<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    if (&value).is_some() {
        return match value {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    return fallback;
}
fn return_for<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let items: terrane_collection_support::List<terrane_int_support::Int>;
    let mut item: terrane_int_support::Int;
    items = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_0 = items;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        if (&value).is_some() {
            return match value {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            };
        }
    }
    return fallback;
}
fn return_while<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let mut index: terrane_int_support::Int;
    index = terrane_int_support::Int::from(0_i128);
    while index < terrane_int_support::Int::from(1_i128) {
        index = index.clone() + terrane_int_support::Int::from(1_i128);
        if !value.is_none() {
            return match value {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            };
        }
    }
    return fallback;
}
fn return_for_membership<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let items: terrane_collection_support::List<terrane_int_support::Int>;
    let item: terrane_int_support::Int;
    items = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_1 = items;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        item = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        if value.is_none() {
            return fallback;
        }
        return match value {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    return fallback;
}
fn preserve_after_loop<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let items: terrane_collection_support::List<terrane_int_support::Int>;
    let mut item: terrane_int_support::Int;
    items = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_2 = items;
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        __terrane_iterable_2.as_slice(),
    );
    loop {
        item = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if (&value).is_some() {
            println!("{}", terrane_scalar_support::scalar_text(&item));
        }
    }
    return choose_terrane_generic_optional_owned_return(value, fallback);
}
fn return_finally<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let mut item: Option<terrane_int_support::Int> = None;
    let mut __terrane_completion_0: TerraneCompletion<TerraneType54> = (|| {
        let __terrane_try_0: TerraneCompletion<TerraneType54> = (|| {
            let __terrane_iterable_3 = terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(2_i128)
                ],
            );
            let mut __terrane_iterator_3 = terrane_collection_support::slice_iterator(
                __terrane_iterable_3.as_slice(),
            );
            loop {
                let _ = item
                    .insert(
                        match __terrane_iterator_3.next() {
                            terrane_collection_support::IterationStep::Item(item) => item,
                            terrane_collection_support::IterationStep::End => break,
                        },
                    );
                let _ = &item;
                if (&value).is_some() {
                    return TerraneCompletion::Return(
                        match value {
                            Some(value) => value,
                            _ => unreachable!("flow-proven storage refinement"),
                        },
                    );
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<TerraneType54> = (|| {
        println!("{}", String::from("finished"));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return fallback;
}
fn return_finally_clone(value: Option<String>, fallback: String) -> String {
    let mut item: Option<terrane_int_support::Int> = None;
    let mut __terrane_completion_1: TerraneCompletion<String> = (|| {
        let __terrane_try_1: TerraneCompletion<String> = (|| {
            let __terrane_iterable_4 = terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(2_i128)
                ],
            );
            let mut __terrane_iterator_4 = terrane_collection_support::slice_iterator(
                __terrane_iterable_4.as_slice(),
            );
            loop {
                let _ = item
                    .insert(
                        match __terrane_iterator_4.next() {
                            terrane_collection_support::IterationStep::Item(item) => item,
                            terrane_collection_support::IterationStep::End => break,
                        },
                    );
                let _ = &item;
                if (&value).is_some() {
                    return TerraneCompletion::Return(
                        match &value {
                            Some(value) => value,
                            _ => unreachable!("flow-proven storage refinement"),
                        }
                            .clone(),
                    );
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_1: TerraneCompletion<String> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&value.is_none()));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return fallback;
}
fn read_before_return<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let mut item: terrane_int_support::Int;
    let __terrane_iterable_5 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut __terrane_iterator_5 = terrane_collection_support::slice_iterator(
        __terrane_iterable_5.as_slice(),
    );
    loop {
        item = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if item == terrane_int_support::Int::from(2_i128) {
            if (&value).is_some() {
                return match value {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                };
            }
        }
        println!("{}", terrane_scalar_support::scalar_text(&value.is_none()));
    }
    return fallback;
}
fn read_before_return_continue<TerraneType54>(
    value: Option<TerraneType54>,
    fallback: TerraneType54,
) -> TerraneType54 {
    let mut item: terrane_int_support::Int;
    let __terrane_iterable_6 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    let mut __terrane_iterator_6 = terrane_collection_support::slice_iterator(
        __terrane_iterable_6.as_slice(),
    );
    loop {
        item = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value.is_none()));
        if item == terrane_int_support::Int::from(1_i128) {
            continue;
        }
        if item == terrane_int_support::Int::from(3_i128) {
            if (&value).is_some() {
                return match value {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                };
            }
        }
    }
    return fallback;
}
fn closure_parameter<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    let callback: std::sync::Arc<dyn Fn(TerraneType54) -> TerraneType54 + Send + Sync>;
    callback = {
        std::sync::Arc::new(move |input: TerraneType54| -> TerraneType54 {
            let item: terrane_int_support::Int;
            let __terrane_iterable_7 = terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]);
            let mut __terrane_iterator_7 = terrane_collection_support::slice_iterator(
                __terrane_iterable_7.as_slice(),
            );
            loop {
                item = match __terrane_iterator_7.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                let _ = &item;
                return input;
            }
            return input;
        })
    };
    return callback(value);
}
fn combine(left: String, right: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(&left, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&right, &mut __terrane_text);
        __terrane_text
    };
}
fn return_expression(value: String) -> String {
    return combine(
        value.clone(),
        {
            let mut __terrane_text = String::with_capacity(1);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &value,
                &mut __terrane_text,
            );
            __terrane_text.push_str("!");
            __terrane_text
        },
    );
}
fn test_scenario_terrane_generic_optional_owned_return() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&choose_terrane_generic_optional_owned_return(Some(terrane_int_support::Int::from(7_i128)),
        terrane_int_support::Int::from(9_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&choose_terrane_generic_optional_owned_return(None,
        terrane_int_support::Int::from(9_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&choose_present(Some(String::from("present")),
        String::from("fallback")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&choose_present(None,
        String::from("fallback")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&choose_terrane_generic_optional_owned_return::
        < terrane_int_support::Int > (Some(terrane_int_support::Int::from(11_i128)),
        terrane_int_support::Int::from(12_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(return_for(Some(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)])), terrane_collection_support::List:: <
        terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(8_i128)]))
        .length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(return_while(Some(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(5_i128)])), terrane_collection_support::List:: <
        terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(9_i128)]))
        .length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(return_for_membership(Some(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(6_i128),
        terrane_int_support::Int::from(7_i128)])), terrane_collection_support::List:: <
        terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(9_i128)]))
        .length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&preserve_after_loop(Some(String::from("kept")),
        String::from("fallback")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&return_finally(Some(String::from("generic-present")),
        String::from("generic-fallback")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&return_finally:: < String > (None,
        String::from("generic-fallback")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&return_finally_clone(Some(String::from("clone-present")),
        String::from("clone-fallback")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&return_finally_clone(None,
        String::from("clone-fallback")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(read_before_return(Some(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(20_i128)])),
        terrane_collection_support::List:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(30_i128)])).length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(read_before_return(None,
        terrane_collection_support::List:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(40_i128)])).length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(read_before_return_continue(Some(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(50_i128)])),
        terrane_collection_support::List:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(60_i128)])).length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&closure_parameter(String::from("parameter")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&return_expression(String::from("left")))
    );
}
// Source: tests/integration/generic-owned-parameter/case.trn
// Namespace: generic-owned-parameter
fn own<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    let callback: TerraneConsumingCallable<(TerraneType54,), TerraneType54>;
    callback = {
        TerraneConsumingCallable::new(move |(input,): (TerraneType54,)| -> TerraneType54 {
            return input;
        })
    };
    return callback.call((value,));
}
fn test_scenario_terrane_generic_owned_parameter() {
    println!("{}", terrane_scalar_support::scalar_text(&own(String::from("owned"))));
}
// Source: tests/integration/generic-return-borrowed-twice/case.trn
// Namespace: generic-return-borrowed-twice
fn both<TerraneType54>(
    a: &TerraneType54,
    b: &TerraneType54,
) -> terrane_int_support::Int {
    let _ = (&a, &b);
    return terrane_int_support::Int::from(7_i128);
}
fn f_terrane_generic_return_borrowed_twice<TerraneType54>(
    value: TerraneType54,
) -> terrane_int_support::Int {
    let item: terrane_int_support::Int;
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        return both(&value, &value);
    }
    return terrane_int_support::Int::from(0_i128);
}
fn test_scenario_terrane_generic_return_borrowed_twice() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_generic_return_borrowed_twice(String::from("safe")))
    );
}
// Source: tests/integration/generic-return-owned-once/case.trn
// Namespace: generic-return-owned-once
fn take<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    return value;
}
fn f_terrane_generic_return_owned_once<TerraneType54>(
    value: TerraneType54,
) -> TerraneType54 {
    let item: terrane_int_support::Int;
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        return take(value);
    }
    return value;
}
fn test_scenario_terrane_generic_return_owned_once() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_generic_return_owned_once(String::from("safe")))
    );
}
// Source: tests/integration/initialized-async-callable-field/case.trn
// Namespace: initialized-async-callable-field
#[derive(Clone)]
pub struct TerraneNsInitializedAsyncCallableFieldFailureIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C64006661696C757265 {
    pub message: String,
}
impl TerraneNsInitializedAsyncCallableFieldFailureIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C64006661696C757265 {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("async field failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
async fn safe() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
async fn bad_terrane_initialized_async_callable_field() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsInitializedAsyncCallableFieldFailureIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C64006661696C757265::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(2)),
            message,
            rendered,

                144 /* terrane-site: tests/integration/initialized-async-callable-field/case.trn:10:5-10:28 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsInitializedAsyncCallableFieldHolderIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572 {
    pub f: std::sync::Arc<
        dyn Fn(
        ) -> std::pin::Pin<
                Box<
                    dyn Future<
                        Output = Result<terrane_int_support::Int, TerraneError>,
                    > + Send,
                >,
            > + Send + Sync,
    >,
}
impl TerraneNsInitializedAsyncCallableFieldHolderIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {
            f: std::sync::Arc::new(move || -> std::pin::Pin<
                Box<dyn Future<Output = _> + Send>,
            > { Box::pin(async move { Ok(safe().await) }) }),
        }
    }
}
async fn test_scenario_terrane_initialized_async_callable_field() -> Result<
    (),
    TerraneError,
> {
    let mut h: TerraneNsInitializedAsyncCallableFieldHolderIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572;
    let mut error: Option<TerraneError> = None;
    h = TerraneNsInitializedAsyncCallableFieldHolderIdentity2F696E697469616C697A65642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572::terrane_construct();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(__terrane_await((h.f)
        ()). await, 145 /* terrane-site: tests/integration/initialized-async-callable-field/case.trn:15:19-15:25 */) ?)
    );
    {
        let __terrane_value: std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<
                        dyn Future<
                            Output = Result<terrane_int_support::Int, TerraneError>,
                        > + Send,
                    >,
                > + Send + Sync,
        > = std::sync::Arc::new(move || -> std::pin::Pin<
            Box<dyn Future<Output = _> + Send>,
        > { Box::pin(bad_terrane_initialized_async_callable_field()) });
        h.f = __terrane_value;
    }
    let __terrane_completion_0: TerraneCompletion<()> = async {
        let __terrane_try_0: TerraneCompletion<()> = async {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await((h
                .f) ()). await, 146 /* terrane-site: tests/integration/initialized-async-callable-field/case.trn:18:23-18:29 */))
            );
            TerraneCompletion::Normal
        }
            .await;
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
                    let _ = error.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_0 = error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_0);
                    };
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    }
        .await;
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/initialized-callable-field-alias/case.trn
// Namespace: initialized-callable-field-alias
fn first_terrane_initialized_callable_field_alias() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(5_i128);
}
fn second_terrane_initialized_callable_field_alias() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
#[derive(Clone)]
pub struct TerraneNsInitializedCallableFieldAliasHolderIdentity2F696E697469616C697A65642D63616C6C61626C652D6669656C642D616C69617300686F6C646572 {
    pub f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
}
impl TerraneNsInitializedCallableFieldAliasHolderIdentity2F696E697469616C697A65642D63616C6C61626C652D6669656C642D616C69617300686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {
            f: std::sync::Arc::new(first_terrane_initialized_callable_field_alias),
        }
    }
}
fn test_scenario_terrane_initialized_callable_field_alias() {
    let mut h: TerraneNsInitializedCallableFieldAliasHolderIdentity2F696E697469616C697A65642D63616C6C61626C652D6669656C642D616C69617300686F6C646572;
    let alias: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    h = TerraneNsInitializedCallableFieldAliasHolderIdentity2F696E697469616C697A65642D63616C6C61626C652D6669656C642D616C69617300686F6C646572::terrane_construct();
    alias = h.f;
    {
        let __terrane_value: std::sync::Arc<
            dyn Fn() -> terrane_int_support::Int + Send + Sync,
        > = std::sync::Arc::new(second_terrane_initialized_callable_field_alias);
        h.f = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&alias()));
    println!("{}", terrane_scalar_support::scalar_text(&(h.f) ()));
}
// Source: tests/integration/integer-coercions/case.trn
// Namespace: integer-coercions
fn test_values_terrane_integer_coercions() -> Result<(), TerraneError> {
    let value: i64;
    let same: terrane_int_support::Int;
    let exact: i8;
    let wrapped: u8;
    let saturated: u8;
    value = 300;
    same = terrane_int_support::Int::from(value as i128);
    exact = __terrane_raised_err(
        terrane_int_support::coerce::<i8>(&120),
        147 /* terrane-site: tests/integration/integer-coercions/case.trn:6:11-6:14 */,
    )?;
    wrapped = terrane_int_support::wrapping_coerce::<
        u8,
    >(
        &(terrane_int_support::Int::from(value as i128)
            + terrane_int_support::Int::from(0_i128)),
    );
    saturated = terrane_int_support::saturating_coerce::<u8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&exact));
    println!("{}", terrane_scalar_support::scalar_text(&wrapped));
    println!("{}", terrane_scalar_support::scalar_text(&saturated));
    println!("{}", terrane_scalar_support::scalar_text(&same));
    Ok(())
}
// Source: tests/integration/integer-euclidean/case.trn
// Namespace: integer-euclidean
fn test_scenario_terrane_integer_euclidean() -> Result<(), TerraneError> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&terrane_int_support::Int::from(3_i128)),
        148 /* terrane-site: tests/integration/integer-euclidean/case.trn:4:10-4:16 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::Int::from(7_i128)
        .euclidean_div(&terrane_int_support::Int::from(- 3_i128)),
        149 /* terrane-site: tests/integration/integer-euclidean/case.trn:5:10-5:16 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::Int::from(-
        7_i128).modulo(&terrane_int_support::Int::from(3_i128)), 150 /* terrane-site: tests/integration/integer-euclidean/case.trn:6:10-6:16 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::Int::from(7_i128)
        .modulo(&terrane_int_support::Int::from(- 3_i128)), 151 /* terrane-site: tests/integration/integer-euclidean/case.trn:7:10-7:16 */) ?)
    );
    Ok(())
}
// Source: tests/integration/integer-move/case.trn
// Namespace: integer-move
static __TERRANE_F67_BASE: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(41_i128));
fn constant_operands(local: terrane_int_support::Int) -> Result<(), TerraneError> {
    __terrane_traced_err(
        assert(
            &*__TERRANE_F67_BASE + &terrane_int_support::Int::from(1_i128)
                == terrane_int_support::Int::from(42_i128)
                && &local + &*__TERRANE_F67_BASE
                    == terrane_int_support::Int::from(43_i128)
                && &*__TERRANE_F67_BASE + &*__TERRANE_F67_BASE
                    == terrane_int_support::Int::from(82_i128),
        ),
        152 /* terrane-site: tests/integration/integer-move/case.trn:8:5-8:72 */,
    )?;
    __terrane_traced_err(
        assert(
            &*__TERRANE_F67_BASE - &local == terrane_int_support::Int::from(39_i128)
                && &local - &*__TERRANE_F67_BASE
                    == terrane_int_support::Int::from(-39_i128)
                && &*__TERRANE_F67_BASE - &*__TERRANE_F67_BASE
                    == terrane_int_support::Int::from(0_i128),
        ),
        153 /* terrane-site: tests/integration/integer-move/case.trn:9:5-9:76 */,
    )?;
    __terrane_traced_err(
        assert(
            &*__TERRANE_F67_BASE * &local == terrane_int_support::Int::from(82_i128)
                && &local * &*__TERRANE_F67_BASE
                    == terrane_int_support::Int::from(82_i128)
                && &*__TERRANE_F67_BASE * &*__TERRANE_F67_BASE
                    == terrane_int_support::Int::from(1681_i128),
        ),
        154 /* terrane-site: tests/integration/integer-move/case.trn:10:5-10:78 */,
    )?;
    __terrane_traced_err(
        assert(
            *__TERRANE_F67_BASE == terrane_int_support::Int::from(41_i128)
                && *__TERRANE_F67_BASE < terrane_int_support::Int::from(42_i128)
                && !(*__TERRANE_F67_BASE == local) && local < *__TERRANE_F67_BASE,
        ),
        155 /* terrane-site: tests/integration/integer-move/case.trn:11:5-11:78 */,
    )?;
    __terrane_traced_err(
        assert(!(*__TERRANE_F67_BASE < local)),
        156 /* terrane-site: tests/integration/integer-move/case.trn:12:5-12:31 */,
    )?;
    Ok(())
}
fn print_bitwise(x: terrane_int_support::Int, y: terrane_int_support::Int) {
    println!("{}", terrane_scalar_support::scalar_text(&(&x & &y)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(&* __TERRANE_F67_BASE &
        &terrane_int_support::Int::from(3_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&x & &* __TERRANE_F67_BASE)));
    println!("{}", terrane_scalar_support::scalar_text(&(&x | &y)));
    println!("{}", terrane_scalar_support::scalar_text(&(&x ^ &y)));
}
fn compact() {
    let owned: i64;
    let transferred: terrane_int_support::Int;
    owned = 1;
    transferred = terrane_int_support::Int::from(owned as i128);
    println!("{}", terrane_scalar_support::scalar_text(&transferred));
}
fn reassigned() {
    let mut owned: terrane_int_support::Int;
    let transferred: terrane_int_support::Int;
    owned = terrane_int_support::Int::from(1_i128);
    transferred = owned;
    owned = terrane_int_support::Int::from(2_i128);
    println!("{}", terrane_scalar_support::scalar_text(&transferred));
    println!("{}", terrane_scalar_support::scalar_text(&owned));
}
fn large() -> Result<(), TerraneError> {
    let mut owned: terrane_int_support::Int;
    let transferred: terrane_int_support::Int;
    let sum: terrane_int_support::Int;
    let difference: terrane_int_support::Int;
    let product: terrane_int_support::Int;
    owned = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211456",
    );
    transferred = owned;
    owned = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211457",
    );
    sum = &transferred + &owned;
    difference = &owned - &transferred;
    product = &transferred * &owned;
    __terrane_traced_err(
        assert(
            sum
                == terrane_int_support::Int::from_decimal(
                    "680564733841876926926749214863536422913",
                ),
        ),
        157 /* terrane-site: tests/integration/integer-move/case.trn:40:5-40:59 */,
    )?;
    __terrane_traced_err(
        assert(difference == terrane_int_support::Int::from(1_i128)),
        158 /* terrane-site: tests/integration/integer-move/case.trn:41:5-41:28 */,
    )?;
    __terrane_traced_err(
        assert(
            product
                == terrane_int_support::Int::from_decimal(
                    "115792089237316195423570985008687907853610267032561502502920958615344897851392",
                ),
        ),
        159 /* terrane-site: tests/integration/integer-move/case.trn:42:5-42:102 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&transferred));
    println!("{}", terrane_scalar_support::scalar_text(&owned));
    Ok(())
}
fn narrowed() {
    let owned: Option<terrane_int_support::Int>;
    let transferred: terrane_int_support::Int;
    owned = Some(terrane_int_support::Int::from(3_i128));
    if (&owned).is_some() {
        transferred = owned.expect("semantic optional narrowing");
        println!("{}", terrane_scalar_support::scalar_text(&transferred));
    }
}
fn from_parameter(owned: terrane_int_support::Int) -> terrane_int_support::Int {
    return owned;
}
fn test_scenario_terrane_integer_move() -> Result<(), TerraneError> {
    __terrane_traced_err(
        constant_operands(terrane_int_support::Int::from(2_i128)),
        160 /* terrane-site: tests/integration/integer-move/case.trn:56:5-56:25 */,
    )?;
    print_bitwise(
        terrane_int_support::Int::from(6_i128),
        terrane_int_support::Int::from(3_i128),
    );
    compact();
    reassigned();
    __terrane_traced_err(
        large(),
        161 /* terrane-site: tests/integration/integer-move/case.trn:60:5-60:11 */,
    )?;
    narrowed();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&from_parameter(terrane_int_support::Int::from(5_i128)))
    );
    Ok(())
}
// Source: tests/integration/interpolation-key-reference-destinations/case.trn
// Namespace: interpolation-key-reference-destinations
fn key_length(value: &String) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        terrane_string_support::length(&*value) as i128,
    );
}
fn forward_key<'a>(value: &'a String) -> &'a String {
    return value;
}
fn generic_reference<'a, TerraneType54>(value: &'a TerraneType54) -> &'a TerraneType54 {
    return value;
}
#[derive(Clone)]
pub struct ReferenceKey {}
impl ReferenceKey {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn interpolate(&self, key: &str) -> String {
        let key = key.to_owned();
        let direct: terrane_int_support::Int;
        let alias: &String;
        let forwarded: &String;
        let generic: &String;
        let callback: std::sync::Arc<
            dyn Fn(&String) -> terrane_int_support::Int + Send + Sync,
        >;
        let maybe: Option<&String>;
        direct = key_length(&key);
        alias = &key;
        forwarded = forward_key(&key);
        generic = generic_reference(&key);
        callback = std::sync::Arc::new(key_length);
        maybe = Some(alias);
        if (&maybe).is_some() {
            return {
                let mut __terrane_interpolated_0 = String::with_capacity(10);
                __terrane_interpolated_0.push_str("refs:");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &direct,
                    &mut __terrane_interpolated_0,
                );
                __terrane_interpolated_0.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key_length(alias),
                    &mut __terrane_interpolated_0,
                );
                __terrane_interpolated_0.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key_length(forwarded),
                    &mut __terrane_interpolated_0,
                );
                __terrane_interpolated_0.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key_length(generic),
                    &mut __terrane_interpolated_0,
                );
                __terrane_interpolated_0.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &callback(&key),
                    &mut __terrane_interpolated_0,
                );
                __terrane_interpolated_0.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key_length(
                        *match &maybe {
                            Some(value) => value,
                            _ => unreachable!("flow-proven storage refinement"),
                        },
                    ),
                    &mut __terrane_interpolated_0,
                );
                __terrane_interpolated_0
            };
        }
        return String::from("missing");
    }
}
impl InterpolatableProtocol for ReferenceKey {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(ReferenceKey::interpolate(&*self, key))
    }
}
impl From<ReferenceKey> for Interpolatable {
    fn from(value: ReferenceKey) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct SharedKey {}
impl SharedKey {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn interpolate(&self, key: &str) -> String {
        let key = key.to_owned();
        let key = std::sync::Arc::new(TerraneSharedCell::new(key));
        let shared: std::sync::Arc<TerraneSharedCell<String>>;
        let observer: std::sync::Weak<TerraneSharedCell<String>>;
        shared = key.clone();
        observer = std::sync::Arc::downgrade(&key.clone());
        return {
            let mut __terrane_interpolated_1 = String::with_capacity(9);
            __terrane_interpolated_1.push_str("shared:");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &*shared.lock().expect("shared reference lock poisoned"),
                &mut __terrane_interpolated_1,
            );
            __terrane_interpolated_1.push_str(":");
            {
                let __terrane_owner = observer.upgrade().expect("reference expired");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &*__terrane_owner.lock().expect("reference lock poisoned"),
                    &mut __terrane_interpolated_1,
                );
            }
            __terrane_interpolated_1.push_str(":");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &*key.lock().expect("reference lock poisoned"),
                &mut __terrane_interpolated_1,
            );
            __terrane_interpolated_1
        };
    }
}
impl InterpolatableProtocol for SharedKey {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(SharedKey::interpolate(&*self, key))
    }
}
impl From<SharedKey> for Interpolatable {
    fn from(value: SharedKey) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct BorrowedKey {}
impl BorrowedKey {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn interpolate(&self, key: &str) -> String {
        let mut owned: String;
        let callback: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
        if key == "snapshot" {
            owned = key.to_owned();
            owned = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &owned,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("!");
                __terrane_text
            };
            return {
                let mut __terrane_interpolated_2 = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &owned,
                    &mut __terrane_interpolated_2,
                );
                __terrane_interpolated_2.push_str("|");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &key,
                    &mut __terrane_interpolated_2,
                );
                __terrane_interpolated_2
            };
        }
        if key == "return" {
            return key.to_owned();
        }
        if key == "single" {
            return key.to_owned();
        }
        if key == "capture" {
            callback = {
                let key = key.to_owned();
                std::sync::Arc::new(move || -> String {
                    return key.clone();
                })
            };
            return {
                let mut __terrane_text = String::with_capacity(0);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &callback(),
                    &mut __terrane_text,
                );
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &callback(),
                    &mut __terrane_text,
                );
                __terrane_text
            };
        }
        return {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &{
                    let mut __terrane_interpolated_3 = String::with_capacity(2);
                    __terrane_interpolated_3.push_str("[");
                    terrane_scalar_support::ScalarDisplay::write_scalar(
                        &key,
                        &mut __terrane_interpolated_3,
                    );
                    __terrane_interpolated_3.push_str("]");
                    __terrane_interpolated_3
                },
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &{
                    let mut __terrane_text = String::with_capacity(1);
                    terrane_scalar_support::ScalarDisplay::write_scalar(
                        &key,
                        &mut __terrane_text,
                    );
                    __terrane_text.push_str("!");
                    __terrane_text
                },
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &{
                    let mut __terrane_text = String::with_capacity(1);
                    terrane_scalar_support::ScalarDisplay::write_scalar(
                        &key,
                        &mut __terrane_text,
                    );
                    __terrane_text.push_str("?");
                    __terrane_text
                },
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &{
                    let mut __terrane_interpolated_4 = String::with_capacity(2);
                    __terrane_interpolated_4.push_str("[");
                    terrane_scalar_support::ScalarDisplay::write_scalar(
                        &key,
                        &mut __terrane_interpolated_4,
                    );
                    __terrane_interpolated_4.push_str("]");
                    __terrane_interpolated_4
                },
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &{
                    let __terrane_separator = &key;
                    let mut __terrane_text = String::with_capacity(2);
                    __terrane_text.push_str("x");
                    __terrane_text.push_str(__terrane_separator);
                    __terrane_text.push_str("y");
                    __terrane_text
                },
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
}
impl InterpolatableProtocol for BorrowedKey {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(BorrowedKey::interpolate(&*self, key))
    }
}
impl From<BorrowedKey> for Interpolatable {
    fn from(value: BorrowedKey) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_interpolation_key_reference_destinations() -> Result<
    (),
    TerraneError,
> {
    let value: ReferenceKey;
    let bound: std::sync::Arc<dyn Fn(String) -> String + Send + Sync>;
    let shared: SharedKey;
    let borrowed: BorrowedKey;
    value = ReferenceKey::terrane_construct();
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_5 = String::with_capacity(0);
                {
                    let __terrane_rendered = value.interpolate("overview");
                    if __terrane_interpolated_5.capacity() == 0 {
                        __terrane_interpolated_5 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_5.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_5
            },
            String::from("refs:8:8:8:8:8:8"),
        ),

            162 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:51:5-51:67 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_6 = String::with_capacity(0);
                {
                    let __terrane_rendered = value.interpolate("");
                    if __terrane_interpolated_6.capacity() == 0 {
                        __terrane_interpolated_6 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_6.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_6
            },
            String::from("refs:0:0:0:0:0:0"),
        ),

            163 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:52:5-52:55 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(value.interpolate("key"), String::from("refs:3:3:3:3:3:3")),

            164 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:53:5-53:72 */,
    )?;
    bound = {
        let receiver = value;
        std::sync::Arc::new(move |argument_0: String| receiver.interpolate(&argument_0))
    };
    __terrane_traced_err(
        assert_equal_string(
            bound(String::from("key")),
            String::from("refs:3:3:3:3:3:3"),
        ),

            165 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:55:5-55:60 */,
    )?;
    shared = SharedKey::terrane_construct();
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_7 = String::with_capacity(0);
                {
                    let __terrane_rendered = shared.interpolate("overview");
                    if __terrane_interpolated_7.capacity() == 0 {
                        __terrane_interpolated_7 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_7.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_7
            },
            String::from("shared:overview:overview:overview"),
        ),

            166 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:57:5-57:85 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_8 = String::with_capacity(0);
                {
                    let __terrane_rendered = shared.interpolate("");
                    if __terrane_interpolated_8.capacity() == 0 {
                        __terrane_interpolated_8 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_8.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_8
            },
            String::from("shared:::"),
        ),

            167 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:58:5-58:49 */,
    )?;
    borrowed = BorrowedKey::terrane_construct();
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_9 = String::with_capacity(0);
                {
                    let __terrane_rendered = borrowed.interpolate("overview");
                    if __terrane_interpolated_9.capacity() == 0 {
                        __terrane_interpolated_9 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_9.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_9
            },
            String::from("[overview]overview!overview?[overview]xoverviewy"),
        ),

            168 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:60:5-60:102 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_10 = String::with_capacity(0);
                {
                    let __terrane_rendered = borrowed.interpolate("");
                    if __terrane_interpolated_10.capacity() == 0 {
                        __terrane_interpolated_10 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_10.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_10
            },
            String::from("[]!?[]xy"),
        ),

            169 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:61:5-61:50 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            borrowed.interpolate("é{a}\\z"),
            String::from("[é{a}\\z]é{a}\\z!é{a}\\z?[é{a}\\z]xé{a}\\zy"),
        ),

            170 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:62:5-62:100 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            borrowed.interpolate("snapshot"),
            String::from("snapshot!|snapshot"),
        ),

            171 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:63:5-63:82 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(borrowed.interpolate("return"), String::from("return")),

            172 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:64:5-64:68 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(borrowed.interpolate("single"), String::from("single")),

            173 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:65:5-65:68 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            borrowed.interpolate("capture"),
            String::from("capturecapture"),
        ),

            174 /* terrane-site: tests/integration/interpolation-key-reference-destinations/case.trn:66:5-66:77 */,
    )?;
    Ok(())
}
// Source: tests/integration/iterator-protocol/case.trn
// Namespace: iterator-protocol
fn test_scenario_terrane_iterator_protocol() {
    let mut values: terrane_collection_support::Iterator<()>;
    let mut value: ();
    let mut exhausted: terrane_collection_support::Iterator<terrane_int_support::Int>;
    let text: String;
    let mut grapheme: String;
    values = terrane_collection_support::Iterator::<()>::new(vec![(), ()]);
    let mut __terrane_iterator_0 = &mut values;
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    exhausted = terrane_collection_support::Iterator::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    {
        let __terrane_result = {
            let _ = exhausted.next();
        };
        __terrane_result
    };
    {
        let __terrane_result = {
            let _ = exhausted.next();
        };
        __terrane_result
    };
    {
        let __terrane_result = {
            let _ = exhausted.next();
        };
        __terrane_result
    };
    println!("{}", String::from("end"));
    text = String::from("A👍🏽");
    {
        let __terrane_iterable_1 = &text;
        let mut __terrane_iterator_1 = terrane_string_support::graphemes(
            &__terrane_iterable_1,
        );
        loop {
            grapheme = match __terrane_iterator_1.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&grapheme));
        }
    }
}
// Source: tests/integration/layout-calls-async/case.trn
// Namespace: layout-calls-async
async fn deferred(value: String) -> String {
    return value;
}
#[derive(Clone)]
pub struct ScheduledMessage {
    pub text: String,
}
impl ScheduledMessage {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value: String = text;
            self.text = __terrane_value;
        }
    }
    pub fn with_text(&self, text: String) -> ScheduledMessage {
        return ScheduledMessage::terrane_construct(text);
    }
    pub async fn wait(&self) -> String {
        return self.text.clone();
    }
    pub async fn __terrane_shared_async_wait(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> String {
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .text
                .clone();
            __terrane_snapshot
        };
    }
}
async fn test_scenario_terrane_layout_calls_async() {
    let text_result: String;
    let message: ScheduledMessage;
    let ready: String;
    text_result = __terrane_await(
            deferred({
                let __terrane_receiver_value_0 = &{
                    let __terrane_receiver_value_1 = &String::from("  ready  ");
                    terrane_string_support::trim(&__terrane_receiver_value_1)
                };
                terrane_string_support::upper(&__terrane_receiver_value_0)
            }),
        )
        .await;
    println!("{}", terrane_scalar_support::scalar_text(&text_result));
    message = ScheduledMessage::terrane_construct(String::from(""));
    ready = __terrane_await((&message.with_text(String::from("async-chain"))).wait())
        .await;
    println!("{}", terrane_scalar_support::scalar_text(&ready));
}
// Source: tests/integration/list-ref-then-owned/case.trn
// Namespace: list-ref-then-owned
fn look_terrane_list_ref_then_owned(
    a: &terrane_collection_support::List<terrane_int_support::Int>,
    b: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let _ = (&a, &b);
    return terrane_int_support::Int::from(1_i128);
}
fn f_terrane_list_ref_then_owned(
    o: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return look_terrane_list_ref_then_owned(&o, o.clone());
}
fn test_scenario_terrane_list_ref_then_owned() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_list_ref_then_owned(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
}
// Source: tests/integration/list-ref-then-owned-local/case.trn
// Namespace: list-ref-then-owned-local
fn look_terrane_list_ref_then_owned_local(
    a: &terrane_collection_support::List<terrane_int_support::Int>,
    b: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let _ = (&a, &b);
    return terrane_int_support::Int::from(1_i128);
}
fn test_scenario_terrane_list_ref_then_owned_local() {
    let o: terrane_collection_support::List<terrane_int_support::Int>;
    o = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&look_terrane_list_ref_then_owned_local(&o, o
        .clone()))
    );
}
// Source: tests/integration/local-collection-capture/case.trn
// Namespace: local-collection-capture
pub trait TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564(
    Box<
        dyn TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(7_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol
for TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565,
>
for TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn local<
    TerraneType54: TerraneNsLocalCollectionCaptureNamedIdentity2F6C6F63616C2D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let mut callbacks: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    let callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    callbacks = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = {
            let item = item;
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return item.name();
            })
        };
        callbacks.append(__terrane_collection_argument_0);
    };
    callback = __terrane_raised(
        callbacks
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        175 /* terrane-site: tests/integration/local-collection-capture/case.trn:13:16-13:28 */,
                ),
            ),

            175 /* terrane-site: tests/integration/local-collection-capture/case.trn:13:16-13:28 */,
    );
    return callback();
}
fn store(
    callbacks: &mut terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >,
    item: TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565,
) {
    {
        let __terrane_collection_argument_0 = {
            let item = item.clone();
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return item.name();
            })
        };
        callbacks.append(__terrane_collection_argument_0);
    };
}
fn test_scenario_terrane_local_collection_capture() {
    let mut callbacks: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    let callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&local(TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565::terrane_construct()))
    );
    callbacks = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >::new(vec![]);
    store(
        &mut callbacks,
        TerraneNsLocalCollectionCaptureValueIdentity2F6C6F63616C2D636F6C6C656374696F6E2D636170747572650076616C7565::terrane_construct(),
    );
    callback = __terrane_raised(
        callbacks
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        176 /* terrane-site: tests/integration/local-collection-capture/case.trn:22:16-22:28 */,
                ),
            ),

            176 /* terrane-site: tests/integration/local-collection-capture/case.trn:22:16-22:28 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
}
// Source: tests/integration/local-loop-callback-capture/case.trn
// Namespace: local-loop-callback-capture
pub trait TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564(
    Box<
        dyn TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol
for TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565,
>
for TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_local_loop_callback_capture<
    TerraneType54: TerraneNsLocalLoopCallbackCaptureNamedIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D63617074757265006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let mut l: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    let mut t: terrane_int_support::Int;
    let mut cb: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    l = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = {
            let item = item;
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return item.name();
            })
        };
        l.append(__terrane_collection_argument_0);
    };
    t = terrane_int_support::Int::from(0_i128);
    let __terrane_iterable_0 = l;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        cb = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        t = t.clone() + cb();
    }
    return t.clone();
}
fn test_scenario_terrane_local_loop_callback_capture() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_local_loop_callback_capture(TerraneNsLocalLoopCallbackCaptureValueIdentity2F6C6F63616C2D6C6F6F702D63616C6C6261636B2D636170747572650076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/local-scalar-union/case.trn
// Namespace: local-scalar-union
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF75S76 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF75S76 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn choose_terrane_local_scalar_union(flag: bool) {
    let value: TerraneUnionF75S76;
    if flag {
        value = TerraneUnionF75S76::Arm0(terrane_int_support::Int::from(1_i128));
    } else {
        value = TerraneUnionF75S76::Arm1(String::from("text"));
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &value {
        TerraneUnionF75S76::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF75S76::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn test_scenario_terrane_local_scalar_union() {
    choose_terrane_local_scalar_union(true);
    choose_terrane_local_scalar_union(false);
}
// Source: tests/integration/local-shared-discarded-argument/case.trn
// Namespace: local-shared-discarded-argument
pub trait TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564(
    Box<
        dyn TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565 {}
impl TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
impl TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol
for TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565,
>
for TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564 {
    fn from(
        value: TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn discard<TerraneType46>(
    input: TerraneType46,
) -> std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync> {
    let _ = &input;
    return {
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(7_i128);
        })
    };
}
fn run<
    TerraneType54: TerraneNsLocalSharedDiscardedArgumentNamedIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E74006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let g: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        let item = item;
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return item.name();
        })
    };
    g = discard(f.clone());
    return g();
}
fn test_scenario_terrane_local_shared_discarded_argument() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&run(TerraneNsLocalSharedDiscardedArgumentValueIdentity2F6C6F63616C2D7368617265642D6469736361726465642D617267756D656E740076616C7565::terrane_construct()))
    );
}
// Source: tests/integration/local-union-alias-carrier/case.trn
// Namespace: local-union-alias-carrier
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF77S68 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF77S68 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF77S135 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF77S135 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF77S170 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF77S170 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn observe_terrane_local_union_alias_carrier(flag: bool) {
    let mut original: TerraneUnionF77S68;
    let explicit: TerraneUnionF77S135;
    let inferred: TerraneUnionF77S170;
    original = TerraneUnionF77S68::Arm1(String::from("alpha"));
    if flag {
        original = TerraneUnionF77S68::Arm0(terrane_int_support::Int::from(4_i128));
    }
    explicit = match (&original).clone() {
        TerraneUnionF77S68::Arm0(value) => TerraneUnionF77S135::Arm0(value),
        TerraneUnionF77S68::Arm1(value) => TerraneUnionF77S135::Arm1(value),
    };
    inferred = match (&original).clone() {
        TerraneUnionF77S68::Arm0(value) => TerraneUnionF77S170::Arm0(value),
        TerraneUnionF77S68::Arm1(value) => TerraneUnionF77S170::Arm1(value),
    };
    if matches!(&&explicit, TerraneUnionF77S135::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &explicit {
            TerraneUnionF77S135::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&* match &explicit {
            TerraneUnionF77S135::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            &terrane_int_support::Int::from(1_i128)))
        );
    }
    if matches!(&&inferred, TerraneUnionF77S170::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &inferred {
            TerraneUnionF77S170::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&* match &inferred {
            TerraneUnionF77S170::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            &terrane_int_support::Int::from(2_i128)))
        );
    }
}
fn test_scenario_terrane_local_union_alias_carrier() {
    observe_terrane_local_union_alias_carrier(false);
    observe_terrane_local_union_alias_carrier(true);
}
// Source: tests/integration/local-union-compatible-boundary/case.trn
// Namespace: local-union-compatible-boundary
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF78S91 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF78S91 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn observe_terrane_local_union_compatible_boundary(flag: bool) {
    let value: TerraneUnionF78S91;
    if flag {
        value = TerraneUnionF78S91::Arm0(terrane_int_support::Int::from(1_i128));
    } else {
        value = TerraneUnionF78S91::Arm1(String::from("text"));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&&value,
        TerraneUnionF78S91::Arm1(_)))
    );
    if matches!(&&value, TerraneUnionF78S91::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &value {
            TerraneUnionF78S91::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &value {
            TerraneUnionF78S91::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
}
fn test_scenario_terrane_local_union_compatible_boundary() {
    observe_terrane_local_union_compatible_boundary(true);
    observe_terrane_local_union_compatible_boundary(false);
}
// Source: tests/integration/logical-comparisons/case.trn
// Namespace: logical-comparisons
fn test_scenario_terrane_logical_comparisons() {
    let x: i64;
    let y: i64;
    x = 5;
    y = 9;
    println!("{}", terrane_scalar_support::scalar_text(&(x > 1 &&y > 2)));
}
// Source: tests/integration/lossy-float-narrowing/case.trn
// Namespace: lossy-float-narrowing
fn divide_terrane_lossy_float_narrowing(numerator: f64, denominator: f64) -> f64 {
    return numerator / denominator;
}
fn test_scenario_terrane_lossy_float_narrowing() {
    let rounded: f32;
    let negative_zero: f64;
    let narrowed_negative_zero: f32;
    let overflow_input: f64;
    let overflow: f32;
    let underflow_input: f64;
    let underflow: f32;
    let nan_input: f64;
    let nan: f32;
    rounded = 16777217.0 as f32;
    negative_zero = -0.0_f64;
    narrowed_negative_zero = negative_zero as f32;
    overflow_input = {
        let __terrane_result = terrane_scalar_support::scale_binary_f32(
            {
                let _ = &TerraneDescriptor {
                    identity: "float32",
                    name: "float32",
                    kind: "type",
                    inherently_identity_bearing: false,
                    fields: &[],
                };
                f32::MAX
            },
            1,
        );
        __terrane_result
    } as f64;
    overflow = overflow_input as f32;
    underflow_input = {
        let _ = &TerraneDescriptor {
            identity: "float64",
            name: "float64",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        };
        f64::from_bits(1)
    };
    underflow = underflow_input as f32;
    nan_input = divide_terrane_lossy_float_narrowing(0.0, 0.0);
    nan = nan_input as f32;
    println!("{}", terrane_scalar_support::scalar_text(&rounded));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(narrowed_negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&narrowed_negative_zero.is_sign_negative())
    );
    println!("{}", terrane_scalar_support::scalar_text(&overflow.is_infinite()));
    println!("{}", terrane_scalar_support::scalar_text(&(underflow == 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&nan.is_nan()));
}
// Source: tests/integration/map-entry-destructuring/case.trn
// Namespace: map-entry-destructuring
fn test_scenario_terrane_map_entry_destructuring() {
    let ordered: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let mut key: String;
    let mut value: terrane_int_support::Int;
    let deterministic: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    ordered = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    let __terrane_iterable_0 = ordered.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_0.key;
        value = __terrane_item_0.value;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
        key = String::from("seen");
        value = value.clone() + terrane_int_support::Int::from(1_i128);
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("first")), 177 /* terrane-site: tests/integration/map-entry-destructuring/case.trn:11:10-11:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("second")), 178 /* terrane-site: tests/integration/map-entry-destructuring/case.trn:11:28-11:45 */))
    );
    deterministic = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("only"),
            terrane_int_support::Int::from(7_i128))
        ],
    );
    let __terrane_iterable_1 = deterministic;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let __terrane_item_1 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_1.key;
        value = __terrane_item_1.value;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
    }
}
// Source: tests/integration/map-key-removal/case.trn
// Namespace: map-key-removal
#[derive(Clone)]
pub struct TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572 {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value: String = name;
            self.name = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", String::from("drop-"), terrane_scalar_support::scalar_text(&self
            .name)
        );
    }
}
impl Drop
for TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn test_scenario_terrane_map_key_removal() -> Result<(), TerraneError> {
    let mut ordered: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let preserved: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let removed: terrane_int_support::Int;
    let absent: Option<terrane_int_support::Int>;
    let mut key: String;
    let mut value: terrane_int_support::Int;
    let mut unordered: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let unordered_preserved: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let mut pair: terrane_collection_support::Entry<String, terrane_int_support::Int>;
    let mut unordered_markers: terrane_collection_support::UnorderedMap<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >;
    let mut unique: terrane_collection_support::Map<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >;
    let mut shared: terrane_collection_support::Map<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >;
    let shared_alias: terrane_collection_support::Map<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >;
    let removed_marker: TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572;
    ordered = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128)),
            terrane_collection_support::Entry::new(String::from("third"),
            terrane_int_support::Int::from(3_i128))
        ],
    );
    preserved = ordered.clone();
    removed = {
        let __terrane_result = {
            let __terrane_key = String::from("second");
            __terrane_raised_err(
                ordered.remove(&__terrane_key),

                    179 /* terrane-site: tests/integration/map-key-removal/case.trn:15:17-15:41 */,
            )?
        };
        __terrane_result
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&removed),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordered
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(preserved
        .length()))
    );
    absent = {
        let __terrane_result = {
            let __terrane_key = String::from("missing");
            ordered.remove_checked(&__terrane_key)
        };
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(&absent).is_none()),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordered
        .length()))
    );
    {
        let __terrane_collection_argument_0 = String::from("second");
        let __terrane_collection_argument_1 = terrane_int_support::Int::from(4_i128);
        ordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_0 = {
        let __terrane_result = ordered.keys();
        __terrane_result
    };
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        key = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&key));
    }
    let __terrane_iterable_1 = ordered.clone();
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let __terrane_item_1 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_1.key;
        value = __terrane_item_1.value;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
    }
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_collection_argument_0 = String::from("missing");
                let _ = __terrane_raised_completion!(
                    ordered.remove(&__terrane_collection_argument_0),
                    180 /* terrane-site: tests/integration/map-key-removal/case.trn:25:5-25:30 */
                );
            };
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
                    && __terrane_error_0.kind == TerraneErrorKind::MissingKey
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("missing"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    unordered = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128)),
            terrane_collection_support::Entry::new(String::from("third"),
            terrane_int_support::Int::from(3_i128))
        ],
    );
    unordered_preserved = unordered.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        __terrane_key = String::from("second"); __terrane_raised_err(unordered
        .remove(&__terrane_key), 181 /* terrane-site: tests/integration/map-key-removal/case.trn:31:11-31:37 */) ? }; __terrane_result
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        __terrane_key = String::from("missing"); unordered.remove_checked(&__terrane_key)
        }; __terrane_result } .is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(unordered
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(unordered_preserved
        .length()))
    );
    let __terrane_iterable_2 = {
        let __terrane_result = unordered.entries();
        __terrane_result
    };
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        __terrane_iterable_2.as_slice(),
    );
    loop {
        pair = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&pair.key.clone()),
            terrane_scalar_support::scalar_text(&pair.value.clone())
        );
    }
    {
        let __terrane_collection_argument_0 = String::from("second");
        let __terrane_collection_argument_1 = terrane_int_support::Int::from(4_i128);
        unordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_3 = {
        let __terrane_result = unordered.entries();
        __terrane_result
    };
    let mut __terrane_iterator_3 = terrane_collection_support::slice_iterator(
        __terrane_iterable_3.as_slice(),
    );
    loop {
        pair = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", String::from("reinserted-"),
            terrane_scalar_support::scalar_text(&pair.key.clone()),
            terrane_scalar_support::scalar_text(&pair.value.clone())
        );
    }
    unordered_markers = terrane_collection_support::UnorderedMap::<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572::terrane_construct(String::from("unordered")))
        ],
    );
    {
        let __terrane_collection_argument_0 = String::from("item");
        let _ = __terrane_raised_err(
            unordered_markers.remove(&__terrane_collection_argument_0),

                182 /* terrane-site: tests/integration/map-key-removal/case.trn:40:3-40:35 */,
        )?;
    };
    println!("{}", String::from("after-unordered"));
    unique = terrane_collection_support::Map::<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572::terrane_construct(String::from("unique")))
        ],
    );
    {
        let __terrane_collection_argument_0 = String::from("item");
        let _ = __terrane_raised_err(
            unique.remove(&__terrane_collection_argument_0),

                183 /* terrane-site: tests/integration/map-key-removal/case.trn:44:3-44:24 */,
        )?;
    };
    println!("{}", String::from("after-unique"));
    shared = terrane_collection_support::Map::<
        String,
        TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            TerraneNsMapKeyRemovalMarkerIdentity2F6D61702D6B65792D72656D6F76616C006D61726B6572::terrane_construct(String::from("shared")))
        ],
    );
    shared_alias = shared.clone();
    removed_marker = {
        let __terrane_result = {
            let __terrane_key = String::from("item");
            __terrane_raised_err(
                shared.remove(&__terrane_key),

                    184 /* terrane-site: tests/integration/map-key-removal/case.trn:49:29-49:50 */,
            )?
        };
        __terrane_result
    };
    println!(
        "{}{}", String::from("removed-"),
        terrane_scalar_support::scalar_text(&removed_marker.name)
    );
    println!(
        "{}{}", String::from("alias-"),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(shared_alias
        .get_or_error(&String::from("item")), 185 /* terrane-site: tests/integration/map-key-removal/case.trn:51:20-51:40 */) ? .name)
    );
    Ok(())
}
// Source: tests/integration/mixed-width-arithmetic/case.trn
// Namespace: mixed-width-arithmetic
fn test_values_terrane_mixed_width_arithmetic() {
    let left: i8;
    let right: i32;
    let unsigned: u8;
    let total: i32;
    let combined: i16;
    left = 100;
    right = 2;
    unsigned = 120;
    total = __terrane_raised(
        terrane_int_support::fixed_addition(left as i32, right),

            186 /* terrane-site: tests/integration/mixed-width-arithmetic/case.trn:7:17-7:29 */,
    );
    combined = __terrane_raised(
        terrane_int_support::fixed_addition(left as i16, unsigned as i16),

            187 /* terrane-site: tests/integration/mixed-width-arithmetic/case.trn:8:20-8:35 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&combined));
}
// Source: tests/integration/mixed-width-comparisons/case.trn
// Namespace: mixed-width-comparisons
fn test_values_terrane_mixed_width_comparisons() {
    let small: i8;
    let wide: i32;
    small = 5;
    wide = 9;
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 == wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 != wide)));
    println!("{}", terrane_scalar_support::scalar_text(&((small as i32) < wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 <= wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 > wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 >= wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide == small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide != small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide < small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide <= small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide > small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide >= small as i32)));
}
// Source: tests/integration/mutable-receiver-field-snapshot/case.trn
// Namespace: mutable-receiver-field-snapshot
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
    pub fn bump(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value: terrane_int_support::Int = &self.value.clone()
                + &amount;
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
}
fn test_scenario_terrane_mutable_receiver_field_snapshot() {
    let mut value: Counter;
    value = Counter::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.bump(value.value.clone()))
    );
}
// Source: tests/integration/mutable-reference-capture-reuse/case.trn
// Namespace: mutable-reference-capture-reuse
fn test_scenario_terrane_mutable_reference_capture_reuse() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    mutate_and_read(&mut values);
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        188 /* terrane-site: tests/integration/mutable-reference-capture-reuse/case.trn:8:10-8:19 */)),
        188 /* terrane-site: tests/integration/mutable-reference-capture-reuse/case.trn:8:10-8:19 */)),
        String::from(", "), terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        189 /* terrane-site: tests/integration/mutable-reference-capture-reuse/case.trn:8:27-8:36 */)),
        189 /* terrane-site: tests/integration/mutable-reference-capture-reuse/case.trn:8:27-8:36 */)),
        String::from(", "), terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        190 /* terrane-site: tests/integration/mutable-reference-capture-reuse/case.trn:8:44-8:53 */)),
        190 /* terrane-site: tests/integration/mutable-reference-capture-reuse/case.trn:8:44-8:53 */))
    );
}
fn mutate_and_read(
    values: &mut terrane_collection_support::List<terrane_int_support::Int>,
) {
    let mut callback: TerraneBorrowedMutableCallable<'_, (), terrane_int_support::Int>;
    let reader: std::boxed::Box<dyn Fn() -> terrane_int_support::Int + '_>;
    callback = {
        let values = &mut *values;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            {
                let __terrane_collection_argument_0 = terrane_int_support::Int::from(
                    3_i128,
                );
                values.append(__terrane_collection_argument_0);
            };
            return terrane_int_support::Int::from((*values).clone().length());
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&callback.call(())));
    drop(callback);
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(4_i128);
        values.append(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((*
        values).clone().length()))
    );
    reader = {
        let values = &*values;
        std::boxed::Box::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from((*values).clone().length());
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((*
        values).clone().length()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&reader()));
    drop(reader);
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(5_i128);
        values.append(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((*
        values).clone().length()))
    );
}
// Source: tests/integration/narrowed-optional-async-callable/case.trn
// Namespace: narrowed-optional-async-callable
async fn test_scenario_terrane_narrowed_optional_async_callable() {
    let callback: Option<
        std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >,
    >;
    callback = Some({
        std::sync::Arc::new(move || -> std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
        > {
            Box::pin(async move {
                return terrane_int_support::Int::from(9_i128);
            })
        })
    });
    if (&callback).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(match &callback {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            ()). await)
        );
    }
}
// Source: tests/integration/narrowed-optional-callable/case.trn
// Namespace: narrowed-optional-callable
fn first_terrane_narrowed_optional_callable() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
#[derive(Clone)]
pub struct TerraneNsNarrowedOptionalCallableHolderIdentity2F6E6172726F7765642D6F7074696F6E616C2D63616C6C61626C6500686F6C646572 {
    pub f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
}
impl TerraneNsNarrowedOptionalCallableHolderIdentity2F6E6172726F7765642D6F7074696F6E616C2D63616C6C61626C6500686F6C646572 {
    pub fn terrane_construct(
        f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
    ) {
        {
            let __terrane_value: Option<
                std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
            > = f;
            self.f = __terrane_value;
        }
    }
}
fn invoke_terrane_narrowed_optional_callable(
    callback: Option<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
) -> terrane_int_support::Int {
    if (&callback).is_some() {
        return match &callback {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }();
    }
    return terrane_int_support::Int::from(0_i128);
}
fn test_scenario_terrane_narrowed_optional_callable() {
    let local: Option<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    let h: TerraneNsNarrowedOptionalCallableHolderIdentity2F6E6172726F7765642D6F7074696F6E616C2D63616C6C61626C6500686F6C646572;
    let selected: Option<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    local = Some(std::sync::Arc::new(first_terrane_narrowed_optional_callable));
    if (&local).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &local { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } ())
        );
    }
    h = TerraneNsNarrowedOptionalCallableHolderIdentity2F6E6172726F7765642D6F7074696F6E616C2D63616C6C61626C6500686F6C646572::terrane_construct(
        Some(std::sync::Arc::new(first_terrane_narrowed_optional_callable)),
    );
    selected = h.f.clone();
    if (&selected).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &selected { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } ())
        );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&invoke_terrane_narrowed_optional_callable(Some(std::sync::Arc::new(first_terrane_narrowed_optional_callable))))
    );
}
// Source: tests/integration/narrowed-optional-consuming-callable/case.trn
// Namespace: narrowed-optional-consuming-callable
fn test_scenario_terrane_narrowed_optional_consuming_callable() {
    let callback: Option<TerraneConsumingCallable<(), terrane_int_support::Int>>;
    callback = Some({
        TerraneConsumingCallable::new(move |(): ()| -> terrane_int_support::Int {
            return terrane_int_support::Int::from(8_i128);
        })
    });
    if (&callback).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match callback { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .call(()))
        );
    }
}
// Source: tests/integration/narrowed-optional-mutable-callable/case.trn
// Namespace: narrowed-optional-mutable-callable
fn test_scenario_terrane_narrowed_optional_mutable_callable() {
    let n: terrane_int_support::Int;
    let callback: Option<TerraneMutableCallable<(), terrane_int_support::Int>>;
    n = terrane_int_support::Int::from(0_i128);
    callback = Some({
        let mut n = n.clone();
        TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            n = &n + &terrane_int_support::Int::from(1_i128);
            return n.clone();
        })
    });
    if (&callback).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &callback { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .call(()))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &callback { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .call(()))
        );
    }
}
// Source: tests/integration/narrowed-optional-throwing-callable/case.trn
// Namespace: narrowed-optional-throwing-callable
#[derive(Clone)]
pub struct TerraneNsNarrowedOptionalThrowingCallableFailureIdentity2F6E6172726F7765642D6F7074696F6E616C2D7468726F77696E672D63616C6C61626C65006661696C757265 {
    pub message: String,
}
impl TerraneNsNarrowedOptionalThrowingCallableFailureIdentity2F6E6172726F7765642D6F7074696F6E616C2D7468726F77696E672D63616C6C61626C65006661696C757265 {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("optional failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_narrowed_optional_throwing_callable() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsNarrowedOptionalThrowingCallableFailureIdentity2F6E6172726F7765642D6F7074696F6E616C2D7468726F77696E672D63616C6C61626C65006661696C757265::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(3)),
            message,
            rendered,

                191 /* terrane-site: tests/integration/narrowed-optional-throwing-callable/case.trn:8:5-8:28 */,
        )
    });
}
fn test_scenario_terrane_narrowed_optional_throwing_callable() {
    let callback: Option<
        std::sync::Arc<
            dyn Fn() -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    >;
    let mut error: Option<TerraneError> = None;
    callback = Some(
        std::sync::Arc::new(bad_terrane_narrowed_optional_throwing_callable),
    );
    if (&callback).is_some() {
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(match
                    &callback { Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") } (),
                    192 /* terrane-site: tests/integration/narrowed-optional-throwing-callable/case.trn:13:21-13:30 */))
                );
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
                        && __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(3))
                    {
                        __terrane_handled_0 = true;
                        let _ = error.insert(__terrane_error_0.clone());
                        {
                            let __terrane_error_text_0 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            println!("{}", __terrane_error_text_0);
                        };
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
}
// Source: tests/integration/negative-wrapped-shift/case.trn
// Namespace: negative-wrapped-shift
fn test_scenario_terrane_negative_wrapped_shift() {
    let value: i8;
    value = 1;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_int_support::fixed_shift_left_wrap(value,
                &- terrane_int_support::Int::from(1_i128)), 193 /* terrane-site: tests/integration/negative-wrapped-shift/case.trn:6:13-6:38 */))
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
                    && __terrane_error_0.kind == TerraneErrorKind::NegativeShiftCount
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught negative shift"));
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
// Source: tests/integration/nested-call-and-receiver-loans/case.trn
// Namespace: nested-call-and-receiver-loans
fn identity(value: String) -> String {
    return value;
}
fn identity_list(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_collection_support::List<terrane_int_support::Int> {
    return value;
}
fn inspect_string(first: &String, second: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &*first,
            &mut __terrane_text,
        );
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &second,
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn inspect_list(
    first: &terrane_collection_support::List<terrane_int_support::Int>,
    second: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_collection_support::List<terrane_int_support::Int> {
    let _ = &first;
    return second;
}
fn inspect_ref(value: &String) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        terrane_string_support::length(&*value) as i128,
    );
}
fn inspect_string_length(
    first: &String,
    second: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return &terrane_int_support::Int::from(
        terrane_string_support::length(&*first) as i128,
    ) + &second;
}
#[derive(Clone)]
pub struct TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D {
    pub value: terrane_int_support::Int,
}
impl TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
    pub fn take(
        &self,
        other: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D,
    ) -> terrane_int_support::Int {
        return self.value.clone() + other.value.clone();
    }
    pub fn take_owned(
        self,
        other: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D,
    ) -> terrane_int_support::Int {
        return self.value.clone() + other.value.clone();
    }
}
fn receiver_parameter(
    value: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D,
) -> terrane_int_support::Int {
    return value.take(value.clone());
}
fn test_scenario_terrane_nested_call_and_receiver_loans() {
    let text: String;
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let returned_values: terrane_collection_support::List<terrane_int_support::Int>;
    let alias: &String;
    let value: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D;
    let local: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D;
    let consuming_owner: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D;
    let different_owner: TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D;
    text = String::from("abc");
    println!(
        "{}", terrane_scalar_support::scalar_text(&inspect_string_length(&text,
        inspect_ref(&text)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&inspect_string(&text,
        identity(identity(text.clone()))))
    );
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    returned_values = inspect_list(
        &values,
        identity_list(identity_list(values.clone())),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(returned_values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        194 /* terrane-site: tests/integration/nested-call-and-receiver-loans/case.trn:38:12-38:30 */)),
        194 /* terrane-site: tests/integration/nested-call-and-receiver-loans/case.trn:38:12-38:30 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(returned_values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        195 /* terrane-site: tests/integration/nested-call-and-receiver-loans/case.trn:39:12-39:30 */)),
        195 /* terrane-site: tests/integration/nested-call-and-receiver-loans/case.trn:39:12-39:30 */))
    );
    alias = &text;
    println!(
        "{}", terrane_scalar_support::scalar_text(&inspect_string(alias, identity(alias
        .clone())))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&inspect_string(&text,
        identity(String::from("different"))))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&inspect_string_length(&text,
        inspect_ref(&text)))
    );
    value = TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&receiver_parameter(value)));
    local = TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&local.take(local.clone())));
    consuming_owner = TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D::terrane_construct();
    different_owner = TerraneNsNestedCallAndReceiverLoansItemIdentity2F6E65737465642D63616C6C2D616E642D72656365697665722D6C6F616E73006974656D::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&consuming_owner
        .take_owned(different_owner))
    );
}
// Source: tests/integration/none-value-display/case.trn
// Namespace: none-value-display
fn test_scenario_terrane_none_value_display() {
    let value: ();
    println!("{}", terrane_scalar_support::scalar_text(&()));
    value = ();
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
// Source: tests/integration/null-byte-literal/case.trn
// Namespace: null-byte-literal
fn test_scenario_terrane_null_byte_literal() -> Result<(), TerraneError> {
    let raw: Vec<u8>;
    let mut byte: u8;
    raw = Vec::<u8>::from([97, 0, 99]);
    {
        let __terrane_iterable_0 = &raw;
        let mut __terrane_iterator_0 = __terrane_iterable_0.iter().copied();
        loop {
            byte = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&byte));
        }
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(raw.len() as i128)),
        terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&raw,
        __terrane_string_argument_0), 196 /* terrane-site: tests/integration/null-byte-literal/case.trn:6:23-6:39 */) ? })
    );
    Ok(())
}
// Source: tests/integration/numeric-destination-conversion/case.trn
// Namespace: numeric-destination-conversion
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF96S128 {
    Arm0(i8),
    Arm1(i32),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF96S128 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn test_values_terrane_numeric_destination_conversion() -> Result<(), TerraneError> {
    let small: i8;
    let adaptive: i64;
    let wide: i32;
    let mut selected: TerraneUnionF96S128;
    let count: i32;
    let total: f64;
    let exact: i64;
    let exact_float: f64;
    let whole: f64;
    let converted: terrane_int_support::Int;
    let other: i32;
    let myvar: i64;
    let slop: i8;
    small = 12;
    adaptive = small as i64;
    wide = small as i32;
    selected = TerraneUnionF96S128::Arm0(small);
    count = 16777216;
    total = count as f64;
    exact = 18014398509481984;
    exact_float = __terrane_raised_err(
        terrane_int_support::exact_f64(&exact),

            197 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:11:23-11:28 */,
    )?;
    whole = 4.0;
    converted = __terrane_raised_err(
        terrane_int_support::exact_int_f64(whole),

            198 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:13:19-13:24 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&wide));
    println!(
        "{}", terrane_scalar_support::scalar_text(&* match &selected {
        TerraneUnionF96S128::Arm0(value) => value, _ =>
        unreachable!("flow-proven storage refinement") })
    );
    other = 13;
    selected = TerraneUnionF96S128::Arm1(other);
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&&selected,
        TerraneUnionF96S128::Arm1(_)))
    );
    selected = TerraneUnionF96S128::Arm0(small);
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&&selected,
        TerraneUnionF96S128::Arm0(_)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&exact_float));
    println!("{}", terrane_scalar_support::scalar_text(&converted));
    myvar = 12;
    slop = {
        let source_value = __terrane_raised_err(
            terrane_int_support::fixed_addition(myvar, 1),

                199 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:27:10-27:19 */,
        )?;
        __terrane_raised_err(
            i8::try_from(source_value)
                .map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(
                    &source_value,
                    "int64",
                    "int8",
                    "the value is outside the destination range",
                )),

                199 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:27:10-27:19 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&slop));
    Ok(())
}
// Source: tests/integration/numeric-literals/case.trn
// Namespace: numeric-literals
fn test_values_terrane_numeric_literals() {
    let adaptive: i64;
    let single: f32;
    let double: f64;
    let inferred: f64;
    let signed_value: i8;
    let unsigned_value: u8;
    let minimum: i8;
    let negative_hex: i8;
    adaptive = 16;
    single = 1.5_f32;
    double = 2.25;
    inferred = 3.5;
    signed_value = 127;
    unsigned_value = 255;
    minimum = -128;
    negative_hex = -16;
    println!("{}", terrane_scalar_support::scalar_text(&adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&single));
    println!("{}", terrane_scalar_support::scalar_text(&double));
    println!("{}", terrane_scalar_support::scalar_text(&inferred));
    println!("{}", terrane_scalar_support::scalar_text(&signed_value));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned_value));
    println!("{}", terrane_scalar_support::scalar_text(&minimum));
    println!("{}", terrane_scalar_support::scalar_text(&negative_hex));
}
// Source: tests/integration/optional-enum-payload/case.trn
// Namespace: optional-enum-payload
#[derive(Clone)]
pub enum TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D {
    Value(Option<terrane_int_support::Int>),
}
fn test_scenario_terrane_optional_enum_payload() {
    let present: TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D;
    let payload: Option<terrane_int_support::Int>;
    let absent: TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D;
    let payload_2: Option<terrane_int_support::Int>;
    present = {
        let __terrane_enum_payload_0 = Some(terrane_int_support::Int::from(7_i128));
        TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D::Value(
            __terrane_enum_payload_0,
        )
    };
    let __terrane_match_value_148 = present.clone();
    match __terrane_match_value_148 {
        TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D::Value(
            __terrane_pattern_170_payload,
        ) => {
            payload = __terrane_pattern_170_payload;
            println!(
                "{}", { let mut __terrane_text = String::new(); match &payload {
                Some(__terrane_present) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present,
                &mut __terrane_text); }, None => __terrane_text.push_str("none"), }
                __terrane_text }
            );
        }
    }
    absent = {
        let __terrane_enum_payload_0 = None;
        TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D::Value(
            __terrane_enum_payload_0,
        )
    };
    let __terrane_match_value_277 = absent.clone();
    match __terrane_match_value_277 {
        TerraneNsOptionalEnumPayloadItemIdentity2F6F7074696F6E616C2D656E756D2D7061796C6F6164006974656D::Value(
            __terrane_pattern_298_payload,
        ) => {
            payload_2 = __terrane_pattern_298_payload;
            println!("{}", terrane_scalar_support::scalar_text(&payload_2.is_none()));
        }
    }
}
// Source: tests/integration/optional-none-guard-flow/case.trn
// Namespace: optional-none-guard-flow
fn size(
    value: Option<terrane_collection_support::List<terrane_int_support::Int>>,
) -> terrane_int_support::Int {
    if value.is_none() {
        return terrane_int_support::Int::from(0_i128);
    }
    return terrane_int_support::Int::from(
        match &value {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .length(),
    );
}
fn observe_terrane_optional_none_guard_flow(value: Option<terrane_int_support::Int>) {
    if value.is_none() {
        println!("{}", String::from("absent"));
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } +
            &terrane_int_support::Int::from(1_i128)))
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&value.is_none()));
}
fn test_scenario_terrane_optional_none_guard_flow() {
    println!("{}", terrane_scalar_support::scalar_text(&size(None)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&size(Some(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![]))))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&size(Some(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(7_i128),
        terrane_int_support::Int::from(8_i128)]))))
    );
    observe_terrane_optional_none_guard_flow(None);
    observe_terrane_optional_none_guard_flow(
        Some(terrane_int_support::Int::from(3_i128)),
    );
}
// Source: tests/integration/optional-scalar-union/case.trn
// Namespace: optional-scalar-union
fn test_scenario_terrane_optional_scalar_union() {
    let mut value: Option<terrane_int_support::Int>;
    value = Some(terrane_int_support::Int::from(7_i128));
    println!(
        "{}", { let mut __terrane_text = String::new(); match &value {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    value = None;
    println!("{}", terrane_scalar_support::scalar_text(&(&value).is_none()));
}
// Source: tests/integration/owned-consuming-string-capture/case.trn
// Namespace: owned-consuming-string-capture
fn test_scenario_terrane_owned_consuming_string_capture() {
    let value: String;
    let callback: TerraneConsumingCallable<(), String>;
    value = String::from("captured");
    callback = {
        let value = value.clone();
        TerraneConsumingCallable::new(move |(): ()| -> String {
            return value;
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&callback.call(())));
}
// Source: tests/integration/owned-copy-collection-capture/case.trn
// Namespace: owned-copy-collection-capture
pub trait TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564(
    Box<
        dyn TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol
for TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565,
>
for TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564 {
    fn from(
        value: TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_owned_copy_collection_capture<
    TerraneType54: TerraneNsOwnedCopyCollectionCaptureNamedIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D63617074757265006E616D6564Protocol,
>(
    fs: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >,
    item: TerraneType54,
) {
    let mut local: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    let callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    local = fs;
    {
        let __terrane_collection_argument_0 = {
            let item = item;
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return item.name();
            })
        };
        local.append(__terrane_collection_argument_0);
    };
    callback = __terrane_raised(
        local
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        200 /* terrane-site: tests/integration/owned-copy-collection-capture/case.trn:15:16-15:24 */,
                ),
            ),

            200 /* terrane-site: tests/integration/owned-copy-collection-capture/case.trn:15:16-15:24 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
}
fn test_scenario_terrane_owned_copy_collection_capture() {
    let fs: terrane_collection_support::List<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >;
    fs = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
    >::new(vec![]);
    go_terrane_owned_copy_collection_capture(
        fs.clone(),
        TerraneNsOwnedCopyCollectionCaptureValueIdentity2F6F776E65642D636F70792D636F6C6C656374696F6E2D636170747572650076616C7565::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fs
        .length()))
    );
}
// Source: tests/integration/parse-radix-families/case.trn
// Namespace: parse-radix-families
fn from_hex(source: String) -> terrane_int_support::Int {
    return __terrane_raised(
        terrane_int_support::parse_radix(&source, &16),

            201 /* terrane-site: tests/integration/parse-radix-families/case.trn:4:10-4:26 */,
    );
}
fn fail_terrane_parse_radix_families(
    source: String,
) -> Result<terrane_int_support::Int, TerraneError> {
    println!("{}", terrane_scalar_support::scalar_text(&source));
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,

                202 /* terrane-site: tests/integration/parse-radix-families/case.trn:7:3-7:23 */,
        ),
    );
}
fn test_scenario_terrane_parse_radix_families() {
    let text: String;
    let parsed: terrane_int_support::Int;
    let value: i64;
    let bad: String;
    text = String::from("ff");
    parsed = from_hex(text);
    println!("{}", terrane_scalar_support::scalar_text(&parsed));
    value = 255;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::format_radix(&value,
        &16), 203 /* terrane-site: tests/integration/parse-radix-families/case.trn:13:11-13:26 */))
    );
    bad = String::from("x");
    fail_terrane_parse_radix_families(bad).ok();
}
// Source: tests/integration/pipeline-borrowed-callbacks/case.trn
// Namespace: pipeline-borrowed-callbacks
fn collect_other(
    values: &mut terrane_collection_support::List<terrane_int_support::Int>,
    other: terrane_collection_support::List<terrane_int_support::Int>,
) {
    let mut grow: TerraneBorrowedMutableCallable<'_, (terrane_int_support::Int,), bool>;
    let filtered: terrane_collection_support::List<terrane_int_support::Int>;
    let mapped: terrane_collection_support::List<terrane_int_support::Int>;
    let mut map_ref: TerraneBorrowedMutableInputsR<
        '_,
        terrane_int_support::Int,
        terrane_int_support::Int,
    >;
    let borrowed_input: terrane_collection_support::List<terrane_int_support::Int>;
    let inline_filter: terrane_collection_support::List<terrane_int_support::Int>;
    let earlier_count: terrane_collection_support::List<terrane_int_support::Int>;
    let generated: terrane_collection_support::List<terrane_int_support::Int>;
    grow = {
        let values = &mut *values;
        TerraneBorrowedMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> bool {
            {
                let __terrane_collection_argument_0 = value.clone();
                values.append(__terrane_collection_argument_0);
            };
            return true;
        })
    };
    filtered = {
        let __terrane_pipeline_source = &other;
        let __terrane_pipeline_callback_1 = { &mut grow };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1
                    .call(((*__terrane_pipeline_item).clone(),))
                {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    drop(grow);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(filtered
        .length())), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* values)
        .clone().length()))
    );
    mapped = {
        let __terrane_pipeline_source = &other;
        let mut __terrane_pipeline_callback_1 = {
            let values = &mut *values;
            TerraneBorrowedMutableCallable::new(move |
                (value,): (terrane_int_support::Int,),
            | -> terrane_int_support::Int {
                {
                    let __terrane_collection_argument_0 = value.clone();
                    values.append(__terrane_collection_argument_0);
                };
                return &value * &terrane_int_support::Int::from(2_i128);
            })
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source.as_slice().len(),
        );
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            {
                let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                    .call(((*__terrane_pipeline_item).clone(),));
                __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(mapped
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        204 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:20:12-20:21 */)),
        204 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:20:12-20:21 */)),
        String::from(" "), terrane_scalar_support::scalar_text(&__terrane_raised(mapped
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        205 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:20:28-20:37 */)),
        205 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:20:28-20:37 */)),
        String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* values)
        .clone().length()))
    );
    map_ref = {
        let values = &mut *values;
        TerraneBorrowedMutableInputsR::new(move |
            (value,): (&terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            {
                let __terrane_collection_argument_0 = value.clone();
                values.append(__terrane_collection_argument_0);
            };
            return value.clone();
        })
    };
    borrowed_input = {
        let __terrane_pipeline_source = &other;
        let __terrane_pipeline_callback_1 = { &mut map_ref };
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                        .call((__terrane_pipeline_item,));
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    drop(map_ref);
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(borrowed_input
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        206 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:29:12-29:29 */)),
        206 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:29:12-29:29 */)),
        String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* values)
        .clone().length()))
    );
    inline_filter = {
        let __terrane_pipeline_source = &other;
        let mut __terrane_pipeline_callback_1 = {
            let values = &mut *values;
            TerraneBorrowedMutableCallable::new(move |
                (value,): (terrane_int_support::Int,),
            | -> bool {
                {
                    let __terrane_collection_argument_0 = value.clone();
                    values.append(__terrane_collection_argument_0);
                };
                return value > terrane_int_support::Int::from(0_i128);
            })
        };
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                '__terrane_pipeline_candidate: {
                    if !__terrane_pipeline_callback_1
                        .call(((*__terrane_pipeline_item).clone(),))
                    {
                        break '__terrane_pipeline_candidate;
                    }
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(inline_filter
        .length())), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* values)
        .clone().length()))
    );
    earlier_count = {
        let __terrane_pipeline_source = &other;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = terrane_int_support::Int::from(
                (*values).clone().length(),
            );
            if __terrane_pipeline_count_1 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_callback_2 = {
            let values = &mut *values;
            TerraneBorrowedMutableCallable::new(move |
                (value,): (terrane_int_support::Int,),
            | -> bool {
                {
                    let __terrane_collection_argument_0 = value.clone();
                    values.append(__terrane_collection_argument_0);
                };
                return true;
            })
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        if !(__terrane_pipeline_remaining_1 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                '__terrane_pipeline_candidate: {
                    __terrane_pipeline_remaining_1 -= 1;
                    if !__terrane_pipeline_callback_2
                        .call(((*__terrane_pipeline_item).clone(),))
                    {
                        break '__terrane_pipeline_candidate;
                    }
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_1 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(earlier_count
        .length())), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* values)
        .clone().length()))
    );
    generated = {
        let __terrane_pipeline_source = &make_other(&mut *values);
        let mut __terrane_pipeline_callback_1 = {
            let values = &mut *values;
            TerraneBorrowedMutableCallable::new(move |
                (value,): (terrane_int_support::Int,),
            | -> bool {
                {
                    let __terrane_collection_argument_0 = value.clone();
                    values.append(__terrane_collection_argument_0);
                };
                return true;
            })
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1
                    .call(((*__terrane_pipeline_item).clone(),))
                {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(generated
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        207 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:52:12-52:24 */)),
        207 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:52:12-52:24 */)),
        String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((* values)
        .clone().length()))
    );
}
fn make_other(
    values: &mut terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_collection_support::List<terrane_int_support::Int> {
    return terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from((* values).clone().length())]);
}
fn observe_state(values: &terrane_collection_support::List<terrane_int_support::Int>) {
    let seen: terrane_int_support::Int;
    let mut reader: TerraneBorrowedMutableCallable<
        '_,
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >;
    let first: terrane_collection_support::List<terrane_int_support::Int>;
    let second: terrane_collection_support::List<terrane_int_support::Int>;
    let accepts: std::boxed::Box<dyn Fn(terrane_int_support::Int) -> bool + '_>;
    let shared: terrane_collection_support::List<terrane_int_support::Int>;
    seen = terrane_int_support::Int::from(0_i128);
    reader = {
        let mut seen = seen.clone();
        let values = values;
        TerraneBorrowedMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            let _ = &value;
            seen = &seen + &terrane_int_support::Int::from(1_i128);
            return seen.clone() + terrane_int_support::Int::from((*values).length());
        })
    };
    first = {
        let __terrane_pipeline_source = &*values;
        let __terrane_pipeline_callback_1 = { &mut reader };
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                        .call(((*__terrane_pipeline_item).clone(),));
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    second = {
        let __terrane_pipeline_source = &*values;
        let __terrane_pipeline_callback_1 = { &mut reader };
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                        .call(((*__terrane_pipeline_item).clone(),));
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    drop(reader);
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(first
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        208 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:70:12-70:20 */)),
        208 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:70:12-70:20 */)),
        String::from(" "), terrane_scalar_support::scalar_text(&__terrane_raised(second
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        209 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:70:27-70:36 */)),
        209 /* terrane-site: tests/integration/pipeline-borrowed-callbacks/case.trn:70:27-70:36 */))
    );
    accepts = {
        let values = values;
        std::boxed::Box::new(move |value: terrane_int_support::Int| -> bool {
            return value <= terrane_int_support::Int::from((*values).length());
        })
    };
    shared = {
        let __terrane_pipeline_source = &*values;
        let __terrane_pipeline_callback_1 = { &accepts };
        let __terrane_pipeline_callback_2 = { &accepts };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1((*__terrane_pipeline_item).clone()) {
                    break '__terrane_pipeline_candidate;
                }
                if !__terrane_pipeline_callback_2((*__terrane_pipeline_item).clone()) {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    drop(accepts);
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(shared
        .length()))
    );
}
fn test_scenario_terrane_pipeline_borrowed_callbacks() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    collect_other(
        &mut values,
        terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(7_i128),
                terrane_int_support::Int::from(8_i128)
            ],
        ),
    );
    observe_state(&values);
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
// Source: tests/integration/quoted-interpolation/case.trn
// Namespace: quoted-interpolation
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF105S2181 {
    Arm0(Server),
    Arm1(terrane_int_support::Int),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF105S2181 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub struct Server {
    pub name: String,
}
impl Server {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from("primary"),
        }
    }
    pub fn interpolate(&self, key: &str) -> String {
        let callback: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
        let mut copied: String;
        if key == "overview" {
            return {
                let mut __terrane_text = String::with_capacity(16);
                __terrane_text.push_str("object overview:");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &self.name,
                    &mut __terrane_text,
                );
                __terrane_text
            };
        }
        if key == "closure" {
            callback = {
                let key = key.to_owned();
                std::sync::Arc::new(move || -> String {
                    return key.clone();
                })
            };
            return callback();
        }
        if key == "identity" {
            return key.to_owned();
        }
        if key == "copy" {
            copied = key.to_owned();
            copied = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &copied,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("!");
                __terrane_text
            };
            return copied;
        }
        return {
            let mut __terrane_text = String::with_capacity(15);
            __terrane_text.push_str("object default:");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &self.name,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    pub fn to_string(&self) -> String {
        return String::from("ordinary display");
    }
}
impl InterpolatableProtocol for Server {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(Server::interpolate(&*self, key))
    }
}
impl From<Server> for Interpolatable {
    fn from(value: Server) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DefaultKeyServer {}
impl DefaultKeyServer {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn interpolate(&self, key: &str) -> String {
        return key.to_owned();
    }
}
impl InterpolatableProtocol for DefaultKeyServer {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(DefaultKeyServer::interpolate(&*self, key))
    }
}
impl From<DefaultKeyServer> for Interpolatable {
    fn from(value: DefaultKeyServer) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct KeyMutator {}
impl KeyMutator {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn interpolate(&self, mut key: String) -> String {
        key = {
            let mut __terrane_text = String::with_capacity(1);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &key,
                &mut __terrane_text,
            );
            __terrane_text.push_str("!");
            __terrane_text
        };
        return key;
    }
}
impl InterpolatableProtocol for KeyMutator {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(KeyMutator::interpolate(&*self, key.to_owned()))
    }
}
impl From<KeyMutator> for Interpolatable {
    fn from(value: KeyMutator) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct RenderFailure {
    pub message: String,
}
impl RenderFailure {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("render failed"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
    pub fn interpolate(&self, key: &str) -> String {
        return {
            let mut __terrane_text = String::with_capacity(7);
            __terrane_text.push_str("error:");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &key,
                &mut __terrane_text,
            );
            __terrane_text.push_str(":");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &self.message,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
}
impl InterpolatableProtocol for RenderFailure {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        Ok(RenderFailure::interpolate(&*self, key))
    }
}
impl From<RenderFailure> for Interpolatable {
    fn from(value: RenderFailure) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct FailingServer {}
impl FailingServer {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        let _ = &key;
        return Err({
            let value = RenderFailure::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(4)),
                message,
                rendered,

                    210 /* terrane-site: tests/integration/quoted-interpolation/case.trn:45:9-45:39 */,
            )
        });
    }
}
impl InterpolatableProtocol for FailingServer {
    fn clone_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterpolatableProtocol> {
        Box::new(self.clone())
    }
    fn interpolate(&self, key: &str) -> Result<String, TerraneError> {
        FailingServer::interpolate(&*self, key)
    }
}
impl From<FailingServer> for Interpolatable {
    fn from(value: FailingServer) -> Self {
        Self(Box::new(value))
    }
}
fn decorate_terrane_quoted_interpolation(value: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(2);
        __terrane_text.push_str("[");
        terrane_scalar_support::ScalarDisplay::write_scalar(&value, &mut __terrane_text);
        __terrane_text.push_str("]");
        __terrane_text
    };
}
fn overview(value: Server) -> String {
    return {
        let mut __terrane_text = String::with_capacity(13);
        __terrane_text.push_str("app overview:");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &value.name,
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn detail(value: Server) -> String {
    return {
        let mut __terrane_text = String::with_capacity(11);
        __terrane_text.push_str("app detail:");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &value.name,
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn first_terrane_quoted_interpolation(value: terrane_int_support::Int) -> String {
    return {
        let mut __terrane_text = String::with_capacity(6);
        __terrane_text.push_str("first:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&value, &mut __terrane_text);
        __terrane_text
    };
}
fn second_terrane_quoted_interpolation(value: terrane_int_support::Int) -> String {
    return {
        let mut __terrane_text = String::with_capacity(7);
        __terrane_text.push_str("second:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&value, &mut __terrane_text);
        __terrane_text
    };
}
fn switch_number(
    renderer: &mut std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
    >,
) -> terrane_int_support::Int {
    *renderer = std::sync::Arc::new(second_terrane_quoted_interpolation);
    return terrane_int_support::Int::from(9_i128);
}
fn generic_text<TerraneType54: InterpolatableProtocol>(
    value: TerraneType54,
) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_interpolated_0 = String::with_capacity(8);
        __terrane_interpolated_0.push_str("generic:");
        __terrane_traced_err(
            <TerraneType54 as InterpolatableProtocol>::__terrane_write_interpolation(
                &value,
                "",
                &mut __terrane_interpolated_0,
            ),

                211 /* terrane-site: tests/integration/quoted-interpolation/case.trn:67:22-67:27 */,
        )?;
        __terrane_interpolated_0
    });
}
fn generic_key<TerraneType54: InterpolatableProtocol>(
    value: TerraneType54,
) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_interpolated_1 = String::with_capacity(6);
        __terrane_interpolated_1.push_str("keyed:");
        __terrane_traced_err(
            <TerraneType54 as InterpolatableProtocol>::__terrane_write_interpolation(
                &value,
                "overview",
                &mut __terrane_interpolated_1,
            ),

                212 /* terrane-site: tests/integration/quoted-interpolation/case.trn:70:20-70:25 */,
        )?;
        __terrane_interpolated_1
    });
}
fn union_text(selected: bool) -> String {
    let value: TerraneUnionF105S2181;
    if selected {
        value = TerraneUnionF105S2181::Arm0(Server::terrane_construct());
    } else {
        value = TerraneUnionF105S2181::Arm1(terrane_int_support::Int::from(1_i128));
    }
    return {
        let mut __terrane_interpolated_2 = String::with_capacity(6);
        __terrane_interpolated_2.push_str("union:");
        match &value {
            TerraneUnionF105S2181::Arm0(__terrane_arm) => {
                let __terrane_rendered = __terrane_arm.interpolate("");
                if __terrane_interpolated_2.capacity() == 0 {
                    __terrane_interpolated_2 = __terrane_rendered;
                } else {
                    __terrane_interpolated_2.push_str(&__terrane_rendered);
                }
            }
            TerraneUnionF105S2181::Arm1(__terrane_arm) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &__terrane_arm,
                    &mut __terrane_interpolated_2,
                );
            }
        }
        __terrane_interpolated_2
    };
}
fn optional_text(value: Option<Server>) -> String {
    return {
        let mut __terrane_interpolated_3 = String::with_capacity(9);
        __terrane_interpolated_3.push_str("optional:");
        match &value {
            Some(__terrane_present) => {
                let __terrane_rendered = __terrane_present.interpolate("");
                if __terrane_interpolated_3.capacity() == 0 {
                    __terrane_interpolated_3 = __terrane_rendered;
                } else {
                    __terrane_interpolated_3.push_str(&__terrane_rendered);
                }
            }
            None => __terrane_interpolated_3.push_str("none"),
        }
        __terrane_interpolated_3
    };
}
fn borrowed_text<TerraneType54: InterpolatableProtocol>(
    value: &TerraneType54,
) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_interpolated_4 = String::with_capacity(9);
        __terrane_interpolated_4.push_str("borrowed:");
        __terrane_traced_err(
            <TerraneType54 as InterpolatableProtocol>::__terrane_write_interpolation(
                &*value,
                "",
                &mut __terrane_interpolated_4,
            ),

                213 /* terrane-site: tests/integration/quoted-interpolation/case.trn:83:23-83:28 */,
        )?;
        __terrane_interpolated_4
    });
}
fn erased_text(value: Interpolatable) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_interpolated_5 = String::with_capacity(7);
        __terrane_interpolated_5.push_str("erased:");
        __terrane_traced_err(
            <Interpolatable as InterpolatableProtocol>::__terrane_write_interpolation(
                &value,
                "",
                &mut __terrane_interpolated_5,
            ),

                214 /* terrane-site: tests/integration/quoted-interpolation/case.trn:86:21-86:26 */,
        )?;
        __terrane_interpolated_5
    });
}
fn test_scenario_terrane_quoted_interpolation() -> Result<(), TerraneError> {
    let name: String;
    let item: Server;
    let key: String;
    let bound: std::sync::Arc<dyn Fn(String) -> String + Send + Sync>;
    let defaulted: DefaultKeyServer;
    let mut maybe: Option<Server>;
    let erased: Interpolatable;
    let mutator: KeyMutator;
    let original: String;
    name = String::from("Ada");
    __terrane_traced_err(
        assert_equal_string(
            String::from("literal {name}\\n"),
            {
                let mut __terrane_text = String::with_capacity(16);
                __terrane_text.push_str("literal {name}");
                __terrane_text.push_str("\\n");
                __terrane_text
            },
        ),

            215 /* terrane-site: tests/integration/quoted-interpolation/case.trn:90:5-90:70 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(String::from("It's ready"), String::from("It's ready")),

            216 /* terrane-site: tests/integration/quoted-interpolation/case.trn:91:5-91:53 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_6 = String::with_capacity(8);
                __terrane_interpolated_6.push_str("Hello, ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &name,
                    &mut __terrane_interpolated_6,
                );
                __terrane_interpolated_6.push_str("!");
                __terrane_interpolated_6
            },
            String::from("Hello, Ada!"),
        ),

            217 /* terrane-site: tests/integration/quoted-interpolation/case.trn:92:5-92:57 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(String::from("{name}"), String::from("{name}")),

            218 /* terrane-site: tests/integration/quoted-interpolation/case.trn:93:5-93:46 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_7 = String::with_capacity(2);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &(terrane_int_support::Int::from(2_i128)
                        + terrane_int_support::Int::from(3_i128)),
                    &mut __terrane_interpolated_7,
                );
                __terrane_interpolated_7.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &true,
                    &mut __terrane_interpolated_7,
                );
                __terrane_interpolated_7.push_str(":");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &(),
                    &mut __terrane_interpolated_7,
                );
                __terrane_interpolated_7
            },
            String::from("5:true:none"),
        ),

            219 /* terrane-site: tests/integration/quoted-interpolation/case.trn:94:5-94:64 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_8 = String::with_capacity(7);
                __terrane_interpolated_8.push_str("Outer: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &decorate_terrane_quoted_interpolation({
                        let mut __terrane_interpolated_9 = String::with_capacity(6);
                        __terrane_interpolated_9.push_str("Hello ");
                        terrane_scalar_support::ScalarDisplay::write_scalar(
                            &name,
                            &mut __terrane_interpolated_9,
                        );
                        __terrane_interpolated_9
                    }),
                    &mut __terrane_interpolated_8,
                );
                __terrane_interpolated_8
            },
            String::from("Outer: [Hello Ada]"),
        ),

            220 /* terrane-site: tests/integration/quoted-interpolation/case.trn:95:5-95:83 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_10 = String::with_capacity(7);
                __terrane_interpolated_10.push_str("Outer: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &decorate_terrane_quoted_interpolation(String::from("Hello {name}")),
                    &mut __terrane_interpolated_10,
                );
                __terrane_interpolated_10
            },
            String::from("Outer: [Hello {name}]"),
        ),

            221 /* terrane-site: tests/integration/quoted-interpolation/case.trn:96:5-96:88 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_11 = String::with_capacity(7);
                __terrane_interpolated_11.push_str("Outer: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &decorate_terrane_quoted_interpolation(String::from("Hello {name}")),
                    &mut __terrane_interpolated_11,
                );
                __terrane_interpolated_11
            },
            String::from("Outer: [Hello {name}]"),
        ),

            222 /* terrane-site: tests/integration/quoted-interpolation/case.trn:97:5-97:86 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            decorate_terrane_quoted_interpolation(String::from("}")),
            String::from("[}]"),
        ),

            223 /* terrane-site: tests/integration/quoted-interpolation/case.trn:98:5-98:51 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_12 = String::with_capacity(7);
                __terrane_interpolated_12.push_str("Outer: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &decorate_terrane_quoted_interpolation({
                        let mut __terrane_interpolated_13 = String::with_capacity(6);
                        __terrane_interpolated_13.push_str("Hello ");
                        terrane_scalar_support::ScalarDisplay::write_scalar(
                            &decorate_terrane_quoted_interpolation(String::from("}")),
                            &mut __terrane_interpolated_13,
                        );
                        __terrane_interpolated_13
                    }),
                    &mut __terrane_interpolated_12,
                );
                __terrane_interpolated_12
            },
            String::from("Outer: [Hello [}]]"),
        ),

            224 /* terrane-site: tests/integration/quoted-interpolation/case.trn:99:5-99:93 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            decorate_terrane_quoted_interpolation(String::from("comment")),
            String::from("[comment]"),
        ),

            225 /* terrane-site: tests/integration/quoted-interpolation/case.trn:100:5-100:72 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(String::from("😀"), String::from("😀")),

            226 /* terrane-site: tests/integration/quoted-interpolation/case.trn:101:5-101:42 */,
    )?;
    __terrane_traced_err(
        assert(
            terrane_int_support::Int::from({
                let __terrane_receiver_value_14 = &String::from("\0");
                __terrane_receiver_value_14.len() as i128
            }) == terrane_int_support::Int::from(1_i128),
        ),

            227 /* terrane-site: tests/integration/quoted-interpolation/case.trn:102:5-102:35 */,
    )?;
    __terrane_traced_err(
        assert(
            terrane_int_support::Int::from({
                let __terrane_receiver_value_15 = &String::from("\r\n\t");
                __terrane_receiver_value_15.len() as i128
            }) == terrane_int_support::Int::from(3_i128),
        ),

            228 /* terrane-site: tests/integration/quoted-interpolation/case.trn:103:5-103:39 */,
    )?;
    item = Server::terrane_construct();
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_16 = String::with_capacity(8);
                __terrane_interpolated_16.push_str("Server: ");
                {
                    let __terrane_rendered = item.interpolate("");
                    if __terrane_interpolated_16.capacity() == 0 {
                        __terrane_interpolated_16 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_16.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_16
            },
            String::from("Server: object default:primary"),
        ),

            229 /* terrane-site: tests/integration/quoted-interpolation/case.trn:105:5-105:76 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_17 = String::with_capacity(8);
                __terrane_interpolated_17.push_str("Server: ");
                {
                    let __terrane_rendered = item.interpolate("overview");
                    if __terrane_interpolated_17.capacity() == 0 {
                        __terrane_interpolated_17 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_17.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_17
            },
            String::from("Server: object overview:primary"),
        ),

            230 /* terrane-site: tests/integration/quoted-interpolation/case.trn:106:5-106:89 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            item.interpolate("overview"),
            String::from("object overview:primary"),
        ),

            231 /* terrane-site: tests/integration/quoted-interpolation/case.trn:107:5-107:83 */,
    )?;
    key = String::from("overview");
    __terrane_traced_err(
        assert_equal_string(
            item.interpolate(AsRef::<str>::as_ref(&key)),
            String::from("object overview:primary"),
        ),

            232 /* terrane-site: tests/integration/quoted-interpolation/case.trn:109:5-109:76 */,
    )?;
    bound = {
        let receiver = item.clone();
        std::sync::Arc::new(move |argument_0: String| receiver.interpolate(&argument_0))
    };
    __terrane_traced_err(
        assert_equal_string(bound(key), String::from("object overview:primary")),

            233 /* terrane-site: tests/integration/quoted-interpolation/case.trn:111:5-111:65 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_18 = String::with_capacity(0);
                {
                    let __terrane_rendered = item.interpolate("closure");
                    if __terrane_interpolated_18.capacity() == 0 {
                        __terrane_interpolated_18 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_18.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_18
            },
            String::from("closure"),
        ),

            234 /* terrane-site: tests/integration/quoted-interpolation/case.trn:112:5-112:56 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_19 = String::with_capacity(0);
                {
                    let __terrane_rendered = item.interpolate("identity");
                    if __terrane_interpolated_19.capacity() == 0 {
                        __terrane_interpolated_19 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_19.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_19
            },
            String::from("identity"),
        ),

            235 /* terrane-site: tests/integration/quoted-interpolation/case.trn:113:5-113:58 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_20 = String::with_capacity(0);
                {
                    let __terrane_rendered = item.interpolate("copy");
                    if __terrane_interpolated_20.capacity() == 0 {
                        __terrane_interpolated_20 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_20.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_20
            },
            String::from("copy!"),
        ),

            236 /* terrane-site: tests/integration/quoted-interpolation/case.trn:114:5-114:51 */,
    )?;
    defaulted = DefaultKeyServer::terrane_construct();
    __terrane_traced_err(
        assert_equal_string(defaulted.interpolate("default"), String::from("default")),

            237 /* terrane-site: tests/integration/quoted-interpolation/case.trn:116:5-116:61 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_21 = String::with_capacity(0);
                {
                    let __terrane_rendered = defaulted.interpolate("");
                    if __terrane_interpolated_21.capacity() == 0 {
                        __terrane_interpolated_21 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_21.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_21
            },
            String::from(""),
        ),

            238 /* terrane-site: tests/integration/quoted-interpolation/case.trn:117:5-117:43 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_22 = String::with_capacity(0);
                {
                    let __terrane_rendered = defaulted.interpolate("other");
                    if __terrane_interpolated_22.capacity() == 0 {
                        __terrane_interpolated_22 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_22.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_22
            },
            String::from("other"),
        ),

            239 /* terrane-site: tests/integration/quoted-interpolation/case.trn:118:5-118:57 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_text(terrane_int_support::Int::from(7_i128)),

                    240 /* terrane-site: tests/integration/quoted-interpolation/case.trn:119:27-119:42 */,
            )?,
            String::from("generic:7"),
        ),

            241 /* terrane-site: tests/integration/quoted-interpolation/case.trn:119:5-119:56 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_text(item.clone()),

                    242 /* terrane-site: tests/integration/quoted-interpolation/case.trn:120:27-120:45 */,
            )?,
            String::from("generic:object default:primary"),
        ),

            243 /* terrane-site: tests/integration/quoted-interpolation/case.trn:120:5-120:80 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_key(terrane_int_support::Int::from(7_i128)),

                    244 /* terrane-site: tests/integration/quoted-interpolation/case.trn:121:27-121:41 */,
            )?,
            String::from("keyed:7"),
        ),

            245 /* terrane-site: tests/integration/quoted-interpolation/case.trn:121:5-121:53 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_key(item.clone()),

                    246 /* terrane-site: tests/integration/quoted-interpolation/case.trn:122:27-122:44 */,
            )?,
            String::from("keyed:object overview:primary"),
        ),

            247 /* terrane-site: tests/integration/quoted-interpolation/case.trn:122:5-122:78 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                borrowed_text(&item),

                    248 /* terrane-site: tests/integration/quoted-interpolation/case.trn:123:27-123:46 */,
            )?,
            String::from("borrowed:object default:primary"),
        ),

            249 /* terrane-site: tests/integration/quoted-interpolation/case.trn:123:5-123:82 */,
    )?;
    maybe = Some(item.clone());
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_text(maybe),

                    250 /* terrane-site: tests/integration/quoted-interpolation/case.trn:125:27-125:46 */,
            )?,
            String::from("generic:object default:primary"),
        ),

            251 /* terrane-site: tests/integration/quoted-interpolation/case.trn:125:5-125:81 */,
    )?;
    maybe = None;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_text(maybe.clone()),

                    252 /* terrane-site: tests/integration/quoted-interpolation/case.trn:127:27-127:46 */,
            )?,
            String::from("generic:none"),
        ),

            253 /* terrane-site: tests/integration/quoted-interpolation/case.trn:127:5-127:63 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                erased_text(
                    Interpolatable(Box::new(terrane_int_support::Int::from(7_i128))),
                ),

                    254 /* terrane-site: tests/integration/quoted-interpolation/case.trn:128:27-128:41 */,
            )?,
            String::from("erased:7"),
        ),

            255 /* terrane-site: tests/integration/quoted-interpolation/case.trn:128:5-128:54 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                erased_text(<Interpolatable>::from(item.clone())),

                    256 /* terrane-site: tests/integration/quoted-interpolation/case.trn:129:27-129:44 */,
            )?,
            String::from("erased:object default:primary"),
        ),

            257 /* terrane-site: tests/integration/quoted-interpolation/case.trn:129:5-129:78 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                erased_text(Interpolatable(Box::new(maybe))),

                    258 /* terrane-site: tests/integration/quoted-interpolation/case.trn:130:27-130:45 */,
            )?,
            String::from("erased:none"),
        ),

            259 /* terrane-site: tests/integration/quoted-interpolation/case.trn:130:5-130:61 */,
    )?;
    erased = Interpolatable(Box::new(terrane_int_support::Int::from(8_i128)));
    __terrane_traced_err(
        assert_equal_string(
            __terrane_traced_err(
                generic_text(erased),

                    260 /* terrane-site: tests/integration/quoted-interpolation/case.trn:132:27-132:47 */,
            )?,
            String::from("generic:8"),
        ),

            261 /* terrane-site: tests/integration/quoted-interpolation/case.trn:132:5-132:61 */,
    )?;
    mutator = KeyMutator::terrane_construct();
    original = String::from("before");
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_23 = String::with_capacity(0);
                {
                    let __terrane_rendered = mutator.interpolate(String::from("custom"));
                    if __terrane_interpolated_23.capacity() == 0 {
                        __terrane_interpolated_23 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_23.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_23
            },
            String::from("custom!"),
        ),

            262 /* terrane-site: tests/integration/quoted-interpolation/case.trn:135:5-135:58 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            mutator.interpolate(original.clone()),
            String::from("before!"),
        ),

            263 /* terrane-site: tests/integration/quoted-interpolation/case.trn:136:5-136:68 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(original, String::from("before")),

            264 /* terrane-site: tests/integration/quoted-interpolation/case.trn:137:5-137:44 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            union_text(true),
            String::from("union:object default:primary"),
        ),

            265 /* terrane-site: tests/integration/quoted-interpolation/case.trn:138:5-138:76 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(union_text(false), String::from("union:1")),

            266 /* terrane-site: tests/integration/quoted-interpolation/case.trn:139:5-139:56 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            optional_text(Some(item)),
            String::from("optional:object default:primary"),
        ),

            267 /* terrane-site: tests/integration/quoted-interpolation/case.trn:140:5-140:82 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(optional_text(None), String::from("optional:none")),

            268 /* terrane-site: tests/integration/quoted-interpolation/case.trn:141:5-141:64 */,
    )?;
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_24 = String::with_capacity(0);
                {
                    let __terrane_rendered = RenderFailure::terrane_construct()
                        .interpolate("overview");
                    if __terrane_interpolated_24.capacity() == 0 {
                        __terrane_interpolated_24 = __terrane_rendered;
                    } else {
                        __terrane_interpolated_24.push_str(&__terrane_rendered);
                    }
                }
                __terrane_interpolated_24
            },
            String::from("error:overview:render failed"),
        ),

            269 /* terrane-site: tests/integration/quoted-interpolation/case.trn:142:5-142:98 */,
    )?;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let _: String = {
                    let mut __terrane_interpolated_25 = String::with_capacity(0);
                    {
                        let __terrane_rendered = __terrane_traced_completion!(
                            FailingServer::terrane_construct().interpolate(""),
                            270 /* terrane-site: tests/integration/quoted-interpolation/case.trn:144:15-144:39 */
                        );
                        if __terrane_interpolated_25.capacity() == 0 {
                            __terrane_interpolated_25 = __terrane_rendered;
                        } else {
                            __terrane_interpolated_25.push_str(&__terrane_rendered);
                        }
                    }
                    __terrane_interpolated_25
                };
            }
            __terrane_traced_completion!(
                assert(false), 271 /* terrane-site: tests/integration/quoted-interpolation/case.trn:145:9-145:22 */
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
                        == TerraneErrorKind::Custom(DescriptorId(4))
                {
                    __terrane_handled_0 = true;
                    __terrane_traced_completion!(
                        assert(true), 272 /* terrane-site: tests/integration/quoted-interpolation/case.trn:147:9-147:21 */
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
fn unrelated_catch() -> String {
    let mut renderer: Option<TerraneError> = None;
    let __terrane_completion_1: TerraneCompletion<String> = (|| {
        let __terrane_try_1: TerraneCompletion<String> = (|| {
            return TerraneCompletion::Error({
                let value = RenderFailure::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(4)),
                    message,
                    rendered,

                        273 /* terrane-site: tests/integration/quoted-interpolation/case.trn:151:9-151:39 */,
                )
            });
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
                        == TerraneErrorKind::Custom(DescriptorId(4))
                {
                    __terrane_handled_1 = true;
                    let _ = renderer.insert(__terrane_error_1.clone());
                    return TerraneCompletion::Return(
                        renderer
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message()
                            .to_owned(),
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
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn test_renderer_bindings() -> Result<(), TerraneError> {
    let item: Server;
    let mut renderer: std::sync::Arc<dyn Fn(Server) -> String + Send + Sync>;
    let mut number_renderer: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
    >;
    let mut error: Option<TerraneError> = None;
    item = Server::terrane_construct();
    renderer = std::sync::Arc::new(overview);
    __terrane_traced_err(
        assert_equal_string(
            {
                let __terrane_interpolation_argument_26 = item.clone();
                renderer(__terrane_interpolation_argument_26)
            },
            String::from("app overview:primary"),
        ),

            274 /* terrane-site: tests/integration/quoted-interpolation/case.trn:158:5-158:70 */,
    )?;
    if item.name.as_str() == "primary" {
        renderer = std::sync::Arc::new(detail);
    }
    __terrane_traced_err(
        assert_equal_string(
            {
                let __terrane_interpolation_argument_27 = item;
                renderer(__terrane_interpolation_argument_27)
            },
            String::from("app detail:primary"),
        ),

            275 /* terrane-site: tests/integration/quoted-interpolation/case.trn:161:5-161:68 */,
    )?;
    number_renderer = std::sync::Arc::new(first_terrane_quoted_interpolation);
    __terrane_traced_err(
        assert_equal_string(
            {
                let __terrane_interpolation_argument_28 = terrane_int_support::Int::from(
                    7_i128,
                );
                number_renderer(__terrane_interpolation_argument_28)
            },
            String::from("first:7"),
        ),

            276 /* terrane-site: tests/integration/quoted-interpolation/case.trn:163:5-163:52 */,
    )?;
    number_renderer = std::sync::Arc::new(second_terrane_quoted_interpolation);
    __terrane_traced_err(
        assert_equal_string(
            {
                let __terrane_interpolation_argument_29 = terrane_int_support::Int::from(
                    8_i128,
                );
                number_renderer(__terrane_interpolation_argument_29)
            },
            String::from("second:8"),
        ),

            277 /* terrane-site: tests/integration/quoted-interpolation/case.trn:165:5-165:53 */,
    )?;
    number_renderer = std::sync::Arc::new(first_terrane_quoted_interpolation);
    __terrane_traced_err(
        assert_equal_string(
            {
                let mut __terrane_interpolated_30 = String::with_capacity(2);
                {
                    let __terrane_rendered = {
                        let __terrane_interpolation_argument_31 = terrane_int_support::Int::from(
                            1_i128,
                        );
                        number_renderer(__terrane_interpolation_argument_31)
                    };
                    __terrane_interpolated_30.push_str(&__terrane_rendered);
                }
                __terrane_interpolated_30.push_str("|");
                {
                    let __terrane_rendered = {
                        let __terrane_interpolation_argument_32 = switch_number(
                            &mut number_renderer,
                        );
                        number_renderer(__terrane_interpolation_argument_32)
                    };
                    __terrane_interpolated_30.push_str(&__terrane_rendered);
                }
                __terrane_interpolated_30.push_str("|");
                {
                    let __terrane_rendered = {
                        let __terrane_interpolation_argument_33 = terrane_int_support::Int::from(
                            3_i128,
                        );
                        number_renderer(__terrane_interpolation_argument_33)
                    };
                    __terrane_interpolated_30.push_str(&__terrane_rendered);
                }
                __terrane_interpolated_30
            },
            String::from("first:1|second:9|second:3"),
        ),

            278 /* terrane-site: tests/integration/quoted-interpolation/case.trn:167:5-167:127 */,
    )?;
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = RenderFailure::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(4)),
                    message,
                    rendered,

                        279 /* terrane-site: tests/integration/quoted-interpolation/case.trn:169:9-169:39 */,
                )
            });
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
                        == TerraneErrorKind::Custom(DescriptorId(4))
                {
                    __terrane_handled_2 = true;
                    let _ = error.insert(__terrane_error_2.clone());
                    __terrane_traced_completion!(
                        assert_equal_string(error.as_ref()
                        .expect("flow-proven available binding").message().to_owned(),
                        String::from("render failed")), 280 /* terrane-site: tests/integration/quoted-interpolation/case.trn:171:9-171:60 */
                    );
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    __terrane_traced_err(
        assert_equal_string(unrelated_catch(), String::from("render failed")),

            281 /* terrane-site: tests/integration/quoted-interpolation/case.trn:172:5-172:61 */,
    )?;
    Ok(())
}
// Source: tests/integration/required-async-callable-field/case.trn
// Namespace: required-async-callable-field
#[derive(Clone)]
pub struct TerraneNsRequiredAsyncCallableFieldHolderIdentity2F72657175697265642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572 {
    pub f: Option<
        std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >,
    >,
}
impl TerraneNsRequiredAsyncCallableFieldHolderIdentity2F72657175697265642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572 {
    pub fn terrane_construct(
        f: std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn(
                ) -> std::pin::Pin<
                        Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                    > + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
}
async fn test_scenario_terrane_required_async_callable_field() {
    let h: TerraneNsRequiredAsyncCallableFieldHolderIdentity2F72657175697265642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572;
    let alias: std::sync::Arc<
        dyn Fn(
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >;
    h = TerraneNsRequiredAsyncCallableFieldHolderIdentity2F72657175697265642D6173796E632D63616C6C61626C652D6669656C6400686F6C646572::terrane_construct({
        std::sync::Arc::new(move || -> std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
        > {
            Box::pin(async move {
                return terrane_int_support::Int::from(9_i128);
            })
        })
    });
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_await(h.f.as_ref()
        .expect("required field initialized") ()). await)
    );
    alias = h.f.as_ref().expect("required field initialized").clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_await(alias()). await)
    );
}
// Source: tests/integration/required-mutable-callable-field/case.trn
// Namespace: required-mutable-callable-field
#[derive(Clone)]
pub struct TerraneNsRequiredMutableCallableFieldHolderIdentity2F72657175697265642D6D757461626C652D63616C6C61626C652D6669656C6400686F6C646572 {
    pub f: Option<TerraneMutableCallable<(), terrane_int_support::Int>>,
}
impl TerraneNsRequiredMutableCallableFieldHolderIdentity2F72657175697265642D6D757461626C652D63616C6C61626C652D6669656C6400686F6C646572 {
    pub fn terrane_construct(
        f: TerraneMutableCallable<(), terrane_int_support::Int>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: TerraneMutableCallable<(), terrane_int_support::Int>,
    ) {
        {
            let __terrane_value: TerraneMutableCallable<(), terrane_int_support::Int> = f
                .clone();
            self.f = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_required_mutable_callable_field() {
    let n: terrane_int_support::Int;
    let h: TerraneNsRequiredMutableCallableFieldHolderIdentity2F72657175697265642D6D757461626C652D63616C6C61626C652D6669656C6400686F6C646572;
    let alias: TerraneMutableCallable<(), terrane_int_support::Int>;
    n = terrane_int_support::Int::from(0_i128);
    h = TerraneNsRequiredMutableCallableFieldHolderIdentity2F72657175697265642D6D757461626C652D63616C6C61626C652D6669656C6400686F6C646572::terrane_construct({
        let mut n = n.clone();
        TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            n = &n + &terrane_int_support::Int::from(1_i128);
            return n.clone();
        })
    });
    println!(
        "{}", terrane_scalar_support::scalar_text(&h.f.as_ref()
        .expect("required field initialized").call(()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&h.f.as_ref()
        .expect("required field initialized").call(()))
    );
    alias = h.f.as_ref().expect("required field initialized").clone();
    println!("{}", terrane_scalar_support::scalar_text(&alias.call(())));
    println!(
        "{}", terrane_scalar_support::scalar_text(&h.f.as_ref()
        .expect("required field initialized").call(()))
    );
}
// Source: tests/integration/required-throwing-callable-field/case.trn
// Namespace: required-throwing-callable-field
#[derive(Clone)]
pub struct TerraneNsRequiredThrowingCallableFieldFailureIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C64006661696C757265 {
    pub message: String,
}
impl TerraneNsRequiredThrowingCallableFieldFailureIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C64006661696C757265 {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("field failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct TerraneNsRequiredThrowingCallableFieldHolderIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C6400686F6C646572 {
    pub f: Option<
        std::sync::Arc<
            dyn Fn() -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    >,
}
impl TerraneNsRequiredThrowingCallableFieldHolderIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C6400686F6C646572 {
    pub fn terrane_construct(
        f: std::sync::Arc<
            dyn Fn() -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct(f);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        f: std::sync::Arc<
            dyn Fn() -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) {
        {
            let __terrane_value: std::sync::Arc<
                dyn Fn() -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
            > = f.clone();
            self.f = Some(__terrane_value);
        }
    }
}
fn bad_terrane_required_throwing_callable_field() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsRequiredThrowingCallableFieldFailureIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C64006661696C757265::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(5)),
            message,
            rendered,

                282 /* terrane-site: tests/integration/required-throwing-callable-field/case.trn:12:5-12:28 */,
        )
    });
}
fn test_scenario_terrane_required_throwing_callable_field() -> Result<(), TerraneError> {
    let h: TerraneNsRequiredThrowingCallableFieldHolderIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C6400686F6C646572;
    let mut error: Option<TerraneError> = None;
    let alias: std::sync::Arc<
        dyn Fn() -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
    >;
    let mut error_2: Option<TerraneError> = None;
    h = TerraneNsRequiredThrowingCallableFieldHolderIdentity2F72657175697265642D7468726F77696E672D63616C6C61626C652D6669656C6400686F6C646572::terrane_construct(
        std::sync::Arc::new(bad_terrane_required_throwing_callable_field),
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_traced_completion!(h
                .f.as_ref().expect("required field initialized") (),
                283 /* terrane-site: tests/integration/required-throwing-callable-field/case.trn:16:17-16:21 */))
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
                        == TerraneErrorKind::Custom(DescriptorId(5))
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_0 = error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_0);
                    };
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    alias = h.f.as_ref().expect("required field initialized").clone();
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(alias(),
                284 /* terrane-site: tests/integration/required-throwing-callable-field/case.trn:21:17-21:23 */))
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
                        == TerraneErrorKind::Custom(DescriptorId(5))
                {
                    __terrane_handled_1 = true;
                    let _ = error_2.insert(__terrane_error_1.clone());
                    {
                        let __terrane_error_text_1 = error_2
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_1);
                    };
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/scalar-reference-transparency/case.trn
// Namespace: scalar-reference-transparency
fn test_scenario_terrane_scalar_reference_transparency() -> Result<(), TerraneError> {
    let text: String;
    let seen: &String;
    let encoded: Vec<u8>;
    let decoded: &Vec<u8>;
    let number: std::sync::Arc<TerraneSharedCell<i8>>;
    let observed: std::sync::Weak<TerraneSharedCell<i8>>;
    let owner: std::sync::Arc<TerraneSharedCell<i8>>;
    text = String::from("abc");
    seen = &text;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&*
        seen) as i128))
    );
    println!("{}", terrane_scalar_support::scalar_text(&* seen));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(1); terrane_scalar_support::ScalarDisplay::write_scalar(&*
        seen, &mut __terrane_text); __terrane_text.push_str("!"); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &seen
        .clone(); let mut __terrane_text = String::with_capacity(2); __terrane_text
        .push_str("x"); __terrane_text.push_str(__terrane_separator); __terrane_text
        .push_str("y"); __terrane_text })
    );
    encoded = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(&text, __terrane_string_argument_0)
    };
    decoded = &encoded;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&decoded,
        __terrane_string_argument_0), 285 /* terrane-site: tests/integration/scalar-reference-transparency/case.trn:12:13-12:33 */) ? })
    );
    number = std::sync::Arc::new(TerraneSharedCell::new(7));
    observed = std::sync::Arc::downgrade(&number.clone());
    println!(
        "{}", { let __terrane_owner = observed.upgrade().expect("reference expired");
        terrane_scalar_support::scalar_text(&* __terrane_owner.lock()
        .expect("reference lock poisoned")) }
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value } as i128))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::fixed_addition({
        let __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value }, 2), 286 /* terrane-site: tests/integration/scalar-reference-transparency/case.trn:17:13-17:28 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::fixed_addition({
        let __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value }, 3), 287 /* terrane-site: tests/integration/scalar-reference-transparency/case.trn:18:12-18:24 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&- { let __terrane_owner = observed
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })
    );
    owner = number.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&* owner.lock()
        .expect("shared reference lock poisoned"))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_int_support::fixed_multiplication({
        let __terrane_value = owner.lock().expect("shared reference lock poisoned")
        .clone(); __terrane_value }, 2), 288 /* terrane-site: tests/integration/scalar-reference-transparency/case.trn:22:12-22:21 */) ?)
    );
    Ok(())
}
// Source: tests/integration/sequential-pipeline/case.trn
// Namespace: sequential-pipeline
fn source_list() -> terrane_collection_support::List<terrane_int_support::Int> {
    return terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(5_i128),
            terrane_int_support::Int::from(6_i128),
            terrane_int_support::Int::from(7_i128),
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(9_i128),
            terrane_int_support::Int::from(10_i128)
        ],
    );
}
fn borrow_list<'a>(
    values: &'a terrane_collection_support::List<terrane_int_support::Int>,
) -> &'a terrane_collection_support::List<terrane_int_support::Int> {
    return values;
}
fn large_source() -> terrane_collection_support::List<terrane_int_support::Int> {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    let __terrane_iterable_0 = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(1000_i128),
            terrane_int_support::Int::from(1_i64),
        ),

            289 /* terrane-site: tests/integration/sequential-pipeline/case.trn:16:17-16:31 */,
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    {
        let __terrane_list_append_0 = values.make_unique();
        loop {
            value = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            {
                let __terrane_collection_argument_0 = value.clone();
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
        }
    }
    return values;
}
fn failing_map(
    value: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, TerraneError> {
    if value == terrane_int_support::Int::from(2_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::ArithmeticOverflow,

                    290 /* terrane-site: tests/integration/sequential-pipeline/case.trn:22:5-22:30 */,
            ),
        );
    }
    return Ok(value.clone());
}
fn identity_map(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn string_size(value: &String) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        terrane_string_support::length(&*value) as i128,
    );
}
fn make_mapper() -> std::sync::Arc<
    dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
> {
    println!("{}", String::from("mapper evaluated"));
    return std::sync::Arc::new(identity_map);
}
fn make_count() -> terrane_int_support::Int {
    println!("{}", String::from("count evaluated"));
    return terrane_int_support::Int::from(0_i128);
}
fn observed_source<'a>(
    values: &'a terrane_collection_support::List<terrane_int_support::Int>,
) -> &'a terrane_collection_support::List<terrane_int_support::Int> {
    println!("{}", String::from("source evaluated"));
    return values;
}
fn test_scenario_terrane_sequential_pipeline() -> Result<(), TerraneError> {
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let counts: std::sync::Arc<TerraneSharedCell<CounterBox>>;
    let borrowed: &terrane_collection_support::List<terrane_int_support::Int>;
    let predicate: TerraneMutableCallable<
        (terrane_int_support::Int,),
        Result<bool, TerraneError>,
    >;
    let mapper: TerraneMutableCallable<(terrane_int_support::Int,), String>;
    let pipeline: terrane_collection_support::List<String>;
    let zero_counts: std::sync::Arc<TerraneSharedCell<CounterBox>>;
    let zero_predicate: TerraneMutableCallable<(terrane_int_support::Int,), bool>;
    let zero_mapper: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >;
    let zero: terrane_collection_support::List<terrane_int_support::Int>;
    let before_counts: std::sync::Arc<TerraneSharedCell<CounterBox>>;
    let before_predicate: TerraneMutableCallable<
        (terrane_int_support::Int,),
        Result<bool, TerraneError>,
    >;
    let before: terrane_collection_support::List<terrane_int_support::Int>;
    let type_mapper: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
    >;
    let changed_type: terrane_collection_support::List<String>;
    let empty: terrane_collection_support::List<terrane_int_support::Int>;
    let empty_counts: std::sync::Arc<TerraneSharedCell<CounterBox>>;
    let empty_predicate: TerraneMutableCallable<(terrane_int_support::Int,), bool>;
    let empty_result: terrane_collection_support::List<terrane_int_support::Int>;
    let multiple_takes: terrane_collection_support::List<terrane_int_support::Int>;
    let large: terrane_collection_support::List<terrane_int_support::Int>;
    let large_counts: std::sync::Arc<TerraneSharedCell<CounterBox>>;
    let large_predicate: TerraneMutableCallable<
        (terrane_int_support::Int,),
        Result<bool, TerraneError>,
    >;
    let large_mapper: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >;
    let bounded: terrane_collection_support::List<terrane_int_support::Int>;
    let upfront: terrane_collection_support::List<terrane_int_support::Int>;
    let safe_prefix: terrane_collection_support::List<terrane_int_support::Int>;
    let mut failed: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    let negative: terrane_collection_support::List<terrane_int_support::Int>;
    let huge: terrane_collection_support::List<terrane_int_support::Int>;
    let twice: terrane_collection_support::List<terrane_int_support::Int>;
    let limit: u8;
    let typed: terrane_collection_support::List<terrane_int_support::Int>;
    values = source_list();
    counts = std::sync::Arc::new(
        TerraneSharedCell::new(CounterBox::terrane_construct()),
    );
    borrowed = borrow_list(&values);
    predicate = {
        let counts = counts.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> Result<bool, TerraneError> {
            {
                let __terrane_owner_0 = counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .predicates
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_0
                    .lock()
                    .expect("shared reference lock poisoned")
                    .predicates = __terrane_value;
            }
            return Ok(
                __terrane_raised_err(
                    value.modulo(&terrane_int_support::Int::from(2_i128)),

                        291 /* terrane-site: tests/integration/sequential-pipeline/case.trn:49:12-49:21 */,
                )? == terrane_int_support::Int::from(0_i128),
            );
        })
    };
    mapper = {
        let counts = counts.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> String {
            {
                let __terrane_owner_1 = counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .mapped
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_1
                    .lock()
                    .expect("shared reference lock poisoned")
                    .mapped = __terrane_value;
            }
            return {
                let mut __terrane_text = String::with_capacity(5);
                __terrane_text.push_str("item:");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &value,
                    &mut __terrane_text,
                );
                __terrane_text
            };
        })
    };
    pipeline = {
        let __terrane_pipeline_source = &*borrowed;
        let __terrane_pipeline_callback_1 = predicate;
        let __terrane_pipeline_callback_2 = mapper;
        let mut __terrane_pipeline_remaining_3 = {
            let __terrane_pipeline_count_3 = terrane_int_support::Int::from(3_i128);
            if __terrane_pipeline_count_3 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_3.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<String> = Vec::new();
        if !(__terrane_pipeline_remaining_3 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                '__terrane_pipeline_candidate: {
                    if !__terrane_traced_err(
                        __terrane_pipeline_callback_1
                            .call(((*__terrane_pipeline_item).clone(),)),

                            292 /* terrane-site: tests/integration/sequential-pipeline/case.trn:53:39-55:1 */,
                    )? {
                        break '__terrane_pipeline_candidate;
                    }
                    let __terrane_pipeline_mapped_2 = __terrane_pipeline_callback_2
                        .call(((*__terrane_pipeline_item).clone(),));
                    __terrane_pipeline_remaining_3 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_2);
                }
                if __terrane_pipeline_remaining_3 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(pipeline
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(pipeline
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        293 /* terrane-site: tests/integration/sequential-pipeline/case.trn:59:10-59:21 */) ?),
        293 /* terrane-site: tests/integration/sequential-pipeline/case.trn:59:10-59:21 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(pipeline
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        294 /* terrane-site: tests/integration/sequential-pipeline/case.trn:60:10-60:21 */) ?),
        294 /* terrane-site: tests/integration/sequential-pipeline/case.trn:60:10-60:21 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(pipeline
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        295 /* terrane-site: tests/integration/sequential-pipeline/case.trn:61:10-61:21 */) ?),
        295 /* terrane-site: tests/integration/sequential-pipeline/case.trn:61:10-61:21 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = counts
        .lock().expect("shared reference lock poisoned").predicates.clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = counts
        .lock().expect("shared reference lock poisoned").mapped.clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    zero_counts = std::sync::Arc::new(
        TerraneSharedCell::new(CounterBox::terrane_construct()),
    );
    zero_predicate = {
        let zero_counts = zero_counts.clone();
        TerraneMutableCallable::new(move |(value,): (terrane_int_support::Int,)| -> bool {
            let _ = &value;
            {
                let __terrane_owner_2 = zero_counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = zero_counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .predicates
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_2
                    .lock()
                    .expect("shared reference lock poisoned")
                    .predicates = __terrane_value;
            }
            return true;
        })
    };
    zero_mapper = {
        let zero_counts = zero_counts.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            {
                let __terrane_owner_3 = zero_counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = zero_counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .mapped
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_3
                    .lock()
                    .expect("shared reference lock poisoned")
                    .mapped = __terrane_value;
            }
            return value.clone();
        })
    };
    zero = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = zero_predicate;
        let __terrane_pipeline_callback_2 = zero_mapper;
        let mut __terrane_pipeline_remaining_3 = {
            let __terrane_pipeline_count_3 = terrane_int_support::Int::from(0_i128);
            if __terrane_pipeline_count_3 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_3.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        if !(__terrane_pipeline_remaining_3 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                '__terrane_pipeline_candidate: {
                    if !__terrane_pipeline_callback_1
                        .call(((*__terrane_pipeline_item).clone(),))
                    {
                        break '__terrane_pipeline_candidate;
                    }
                    let __terrane_pipeline_mapped_2 = __terrane_pipeline_callback_2
                        .call(((*__terrane_pipeline_item).clone(),));
                    __terrane_pipeline_remaining_3 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_2);
                }
                if __terrane_pipeline_remaining_3 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(zero
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = zero_counts
        .lock().expect("shared reference lock poisoned").predicates.clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = zero_counts
        .lock().expect("shared reference lock poisoned").mapped.clone();
        __terrane_snapshot })
    );
    before_counts = std::sync::Arc::new(
        TerraneSharedCell::new(CounterBox::terrane_construct()),
    );
    before_predicate = {
        let before_counts = before_counts.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> Result<bool, TerraneError> {
            {
                let __terrane_owner_4 = before_counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = before_counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .predicates
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_4
                    .lock()
                    .expect("shared reference lock poisoned")
                    .predicates = __terrane_value;
            }
            return Ok(
                __terrane_raised_err(
                    value.modulo(&terrane_int_support::Int::from(2_i128)),

                        296 /* terrane-site: tests/integration/sequential-pipeline/case.trn:85:12-85:21 */,
                )? == terrane_int_support::Int::from(0_i128),
            );
        })
    };
    before = {
        let __terrane_pipeline_source = &values;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = terrane_int_support::Int::from(3_i128);
            if __terrane_pipeline_count_1 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let __terrane_pipeline_callback_2 = before_predicate;
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        if !(__terrane_pipeline_remaining_1 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                '__terrane_pipeline_candidate: {
                    __terrane_pipeline_remaining_1 -= 1;
                    if !__terrane_traced_err(
                        __terrane_pipeline_callback_2
                            .call(((*__terrane_pipeline_item).clone(),)),

                            297 /* terrane-site: tests/integration/sequential-pipeline/case.trn:86:37-89:1 */,
                    )? {
                        break '__terrane_pipeline_candidate;
                    }
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_1 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(before
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(before
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        298 /* terrane-site: tests/integration/sequential-pipeline/case.trn:91:10-91:19 */) ?),
        298 /* terrane-site: tests/integration/sequential-pipeline/case.trn:91:10-91:19 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        before_counts.lock().expect("shared reference lock poisoned").predicates.clone();
        __terrane_snapshot })
    );
    type_mapper = {
        std::sync::Arc::new(move |value: terrane_int_support::Int| -> String {
            return {
                let mut __terrane_text = String::with_capacity(2);
                __terrane_text.push_str("n=");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &value,
                    &mut __terrane_text,
                );
                __terrane_text
            };
        })
    };
    changed_type = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = { &type_mapper };
        let mut __terrane_pipeline_output: Vec<String> = Vec::with_capacity(
            __terrane_pipeline_source.as_slice().len(),
        );
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            {
                let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1(
                    (*__terrane_pipeline_item).clone(),
                );
                __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(changed_type
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(changed_type
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        299 /* terrane-site: tests/integration/sequential-pipeline/case.trn:100:10-100:25 */) ?),
        299 /* terrane-site: tests/integration/sequential-pipeline/case.trn:100:10-100:25 */) ?)
    );
    empty = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    empty_counts = std::sync::Arc::new(
        TerraneSharedCell::new(CounterBox::terrane_construct()),
    );
    empty_predicate = {
        let empty_counts = empty_counts.clone();
        TerraneMutableCallable::new(move |(value,): (terrane_int_support::Int,)| -> bool {
            let _ = &value;
            {
                let __terrane_owner_5 = empty_counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = empty_counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .predicates
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_5
                    .lock()
                    .expect("shared reference lock poisoned")
                    .predicates = __terrane_value;
            }
            return true;
        })
    };
    empty_result = {
        let __terrane_pipeline_source = &empty;
        let __terrane_pipeline_callback_1 = empty_predicate;
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1
                    .call(((*__terrane_pipeline_item).clone(),))
                {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty_result
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        empty_counts.lock().expect("shared reference lock poisoned").predicates.clone();
        __terrane_snapshot })
    );
    multiple_takes = {
        let __terrane_pipeline_source = &values;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = terrane_int_support::Int::from(7_i128);
            if __terrane_pipeline_count_1 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(2_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_1)
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_1 == 0 || __terrane_pipeline_remaining_2 == 0)
        {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    __terrane_pipeline_remaining_1 -= 1;
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_1 == 0
                    || __terrane_pipeline_remaining_2 == 0
                {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(multiple_takes
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(multiple_takes
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        300 /* terrane-site: tests/integration/sequential-pipeline/case.trn:118:10-118:27 */) ?),
        300 /* terrane-site: tests/integration/sequential-pipeline/case.trn:118:10-118:27 */) ?)
    );
    large = large_source();
    large_counts = std::sync::Arc::new(
        TerraneSharedCell::new(CounterBox::terrane_construct()),
    );
    large_predicate = {
        let large_counts = large_counts.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> Result<bool, TerraneError> {
            {
                let __terrane_owner_6 = large_counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = large_counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .predicates
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_6
                    .lock()
                    .expect("shared reference lock poisoned")
                    .predicates = __terrane_value;
            }
            return Ok(
                __terrane_raised_err(
                    value.modulo(&terrane_int_support::Int::from(2_i128)),

                        301 /* terrane-site: tests/integration/sequential-pipeline/case.trn:124:12-124:21 */,
                )? == terrane_int_support::Int::from(0_i128),
            );
        })
    };
    large_mapper = {
        let large_counts = large_counts.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            {
                let __terrane_owner_7 = large_counts.clone();
                let __terrane_value: terrane_int_support::Int = {
                    let __terrane_snapshot = large_counts
                        .lock()
                        .expect("shared reference lock poisoned")
                        .mapped
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_7
                    .lock()
                    .expect("shared reference lock poisoned")
                    .mapped = __terrane_value;
            }
            return value.clone();
        })
    };
    bounded = {
        let __terrane_pipeline_source = &large;
        let __terrane_pipeline_callback_1 = large_predicate;
        let __terrane_pipeline_callback_2 = large_mapper;
        let mut __terrane_pipeline_remaining_3 = {
            let __terrane_pipeline_count_3 = terrane_int_support::Int::from(5_i128);
            if __terrane_pipeline_count_3 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_3.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        if !(__terrane_pipeline_remaining_3 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                '__terrane_pipeline_candidate: {
                    if !__terrane_traced_err(
                        __terrane_pipeline_callback_1
                            .call(((*__terrane_pipeline_item).clone(),)),

                            302 /* terrane-site: tests/integration/sequential-pipeline/case.trn:128:38-130:1 */,
                    )? {
                        break '__terrane_pipeline_candidate;
                    }
                    let __terrane_pipeline_mapped_2 = __terrane_pipeline_callback_2
                        .call(((*__terrane_pipeline_item).clone(),));
                    __terrane_pipeline_remaining_3 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_2);
                }
                if __terrane_pipeline_remaining_3 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(bounded
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        large_counts.lock().expect("shared reference lock poisoned").predicates.clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        large_counts.lock().expect("shared reference lock poisoned").mapped.clone();
        __terrane_snapshot })
    );
    upfront = {
        let __terrane_pipeline_source = &*observed_source(&values);
        let __terrane_pipeline_callback_1 = make_mapper();
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = make_count();
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1(
                        (*__terrane_pipeline_item).clone(),
                    );
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(upfront
        .length()))
    );
    safe_prefix = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = failing_map;
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_traced_err(
                        __terrane_pipeline_callback_1(
                            (*__terrane_pipeline_item).clone(),
                        ),

                            303 /* terrane-site: tests/integration/sequential-pipeline/case.trn:143:33-145:1 */,
                    )?;
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(safe_prefix
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        304 /* terrane-site: tests/integration/sequential-pipeline/case.trn:147:10-147:24 */) ?),
        304 /* terrane-site: tests/integration/sequential-pipeline/case.trn:147:10-147:24 */) ?)
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = failed
                .insert({
                    let __terrane_pipeline_source = &values;
                    let __terrane_pipeline_callback_1 = failing_map;
                    let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
                        __terrane_pipeline_source.as_slice().len(),
                    );
                    for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                        {
                            let __terrane_pipeline_mapped_1 = __terrane_traced_completion!(
                                __terrane_pipeline_callback_1((* __terrane_pipeline_item)
                                .clone()), 305 /* terrane-site: tests/integration/sequential-pipeline/case.trn:149:30-151:1 */
                            );
                            __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                        }
                    }
                    terrane_collection_support::List::new(__terrane_pipeline_output)
                });
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(failed
                .as_ref().expect("flow-proven available binding").clone().length()))
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
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught pipeline error"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    negative = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = failing_map;
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(-1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_traced_err(
                        __terrane_pipeline_callback_1(
                            (*__terrane_pipeline_item).clone(),
                        ),

                            306 /* terrane-site: tests/integration/sequential-pipeline/case.trn:156:30-158:1 */,
                    )?;
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(negative
        .length()))
    );
    huge = {
        let __terrane_pipeline_source = &values;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = terrane_int_support::Int::from(
                184467440737095516160000_i128,
            );
            if __terrane_pipeline_count_1 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_1),
        );
        if !(__terrane_pipeline_remaining_1 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    __terrane_pipeline_remaining_1 -= 1;
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_1 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(huge
        .length()))
    );
    twice = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = type_mapper;
        let __terrane_pipeline_callback_2 = string_size;
        let mut __terrane_pipeline_remaining_3 = {
            let __terrane_pipeline_count_3 = terrane_int_support::Int::from(2_i128);
            if __terrane_pipeline_count_3 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_3.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_3),
        );
        if !(__terrane_pipeline_remaining_3 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1(
                        (*__terrane_pipeline_item).clone(),
                    );
                    let __terrane_pipeline_mapped_2 = __terrane_pipeline_callback_2(
                        &__terrane_pipeline_mapped_1,
                    );
                    __terrane_pipeline_remaining_3 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_2);
                }
                if __terrane_pipeline_remaining_3 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(twice
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        307 /* terrane-site: tests/integration/sequential-pipeline/case.trn:170:10-170:18 */) ?),
        307 /* terrane-site: tests/integration/sequential-pipeline/case.trn:170:10-170:18 */) ?),
        String::from(", "),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(twice
        .get_or_error(__terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        308 /* terrane-site: tests/integration/sequential-pipeline/case.trn:170:26-170:34 */) ?),
        308 /* terrane-site: tests/integration/sequential-pipeline/case.trn:170:26-170:34 */) ?)
    );
    limit = 2;
    typed = {
        let __terrane_pipeline_source = &values;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = terrane_int_support::Int::from(
                limit as i128,
            );
            if __terrane_pipeline_count_1 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_1),
        );
        if !(__terrane_pipeline_remaining_1 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    __terrane_pipeline_remaining_1 -= 1;
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_1 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(typed
        .length()))
    );
    inspect_copy_contracts(&values);
    Ok(())
}
fn inspect_copy_contracts(
    values: &terrane_collection_support::List<terrane_int_support::Int>,
) {
    let state: terrane_int_support::Int;
    let counter: TerraneOwnedMutableInputsR<
        terrane_int_support::Int,
        terrane_int_support::Int,
    >;
    let copied: terrane_collection_support::List<terrane_int_support::Int>;
    let loop_state: terrane_int_support::Int;
    let loop_counter: TerraneOwnedMutableInputsR<
        terrane_int_support::Int,
        terrane_int_support::Int,
    >;
    let mut iteration: terrane_int_support::Int;
    let mut repeated: terrane_collection_support::List<terrane_int_support::Int>;
    let reusable: std::boxed::Box<dyn Fn() -> terrane_int_support::Int + '_>;
    let increment: std::sync::Arc<
        dyn Fn(&terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let shared: terrane_collection_support::List<terrane_int_support::Int>;
    let big_limit: terrane_int_support::Int;
    let big_prefix: terrane_collection_support::List<terrane_int_support::Int>;
    let small: terrane_collection_support::List<terrane_int_support::Int>;
    let keep: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> bool + Send + Sync>;
    let shared_twice: terrane_collection_support::List<terrane_int_support::Int>;
    let seen: terrane_int_support::Int;
    let limited: TerraneMutableCallable<(terrane_int_support::Int,), bool>;
    let forked: terrane_collection_support::List<terrane_int_support::Int>;
    state = terrane_int_support::Int::from(0_i128);
    counter = {
        let mut state = state.clone();
        TerraneOwnedMutableInputsR::new(move |
            (value,): (&terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            let _ = &value;
            state = &state + &terrane_int_support::Int::from(1_i128);
            return state.clone();
        })
    };
    copied = {
        let __terrane_pipeline_source = &*values;
        let __terrane_pipeline_callback_1 = counter.clone();
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(2_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                        .call((__terrane_pipeline_item,));
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(copied
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        309 /* terrane-site: tests/integration/sequential-pipeline/case.trn:187:10-187:19 */)),
        309 /* terrane-site: tests/integration/sequential-pipeline/case.trn:187:10-187:19 */)),
        String::from(", "), terrane_scalar_support::scalar_text(&__terrane_raised(copied
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        310 /* terrane-site: tests/integration/sequential-pipeline/case.trn:187:27-187:36 */)),
        310 /* terrane-site: tests/integration/sequential-pipeline/case.trn:187:27-187:36 */)),
        String::from("; original "), terrane_scalar_support::scalar_text(&counter.call(({
        let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        311 /* terrane-site: tests/integration/sequential-pipeline/case.trn:187:63-187:72 */);
        __terrane_raised(values.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        311 /* terrane-site: tests/integration/sequential-pipeline/case.trn:187:63-187:72 */) },)))
    );
    loop_state = terrane_int_support::Int::from(0_i128);
    loop_counter = {
        let mut loop_state = loop_state.clone();
        TerraneOwnedMutableInputsR::new(move |
            (value,): (&terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            let _ = &value;
            loop_state = &loop_state + &terrane_int_support::Int::from(1_i128);
            return loop_state.clone();
        })
    };
    let __terrane_iterable_1 = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(1_i64),
        ),

            312 /* terrane-site: tests/integration/sequential-pipeline/case.trn:193:21-193:32 */,
    );
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        iteration = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &iteration;
        repeated = {
            let __terrane_pipeline_source = &*values;
            let __terrane_pipeline_callback_1 = loop_counter.clone();
            let mut __terrane_pipeline_remaining_2 = {
                let __terrane_pipeline_count_2 = terrane_int_support::Int::from(2_i128);
                if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                    0
                } else {
                    __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
                }
            };
            let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
                __terrane_pipeline_source
                    .as_slice()
                    .len()
                    .min(__terrane_pipeline_remaining_2),
            );
            if !(__terrane_pipeline_remaining_2 == 0) {
                for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                    {
                        let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                            .call((__terrane_pipeline_item,));
                        __terrane_pipeline_remaining_2 -= 1;
                        __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                    }
                    if __terrane_pipeline_remaining_2 == 0 {
                        break;
                    }
                }
            }
            terrane_collection_support::List::new(__terrane_pipeline_output)
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(repeated
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            313 /* terrane-site: tests/integration/sequential-pipeline/case.trn:198:12-198:23 */)),
            313 /* terrane-site: tests/integration/sequential-pipeline/case.trn:198:12-198:23 */)),
            String::from(", "),
            terrane_scalar_support::scalar_text(&__terrane_raised(repeated
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            314 /* terrane-site: tests/integration/sequential-pipeline/case.trn:198:31-198:42 */)),
            314 /* terrane-site: tests/integration/sequential-pipeline/case.trn:198:31-198:42 */))
        );
    }
    reusable = {
        let counter = counter.clone();
        let values = values;
        std::boxed::Box::new(move || -> terrane_int_support::Int {
            let captured: terrane_collection_support::List<terrane_int_support::Int>;
            captured = {
                let __terrane_pipeline_source = &*values;
                let __terrane_pipeline_callback_1 = counter.clone();
                let mut __terrane_pipeline_remaining_2 = {
                    let __terrane_pipeline_count_2 = terrane_int_support::Int::from(
                        1_i128,
                    );
                    if __terrane_pipeline_count_2
                        <= terrane_int_support::Int::from(0_i64)
                    {
                        0
                    } else {
                        __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
                    }
                };
                let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
                    __terrane_pipeline_source
                        .as_slice()
                        .len()
                        .min(__terrane_pipeline_remaining_2),
                );
                if !(__terrane_pipeline_remaining_2 == 0) {
                    for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                        {
                            let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1
                                .call((__terrane_pipeline_item,));
                            __terrane_pipeline_remaining_2 -= 1;
                            __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                        }
                        if __terrane_pipeline_remaining_2 == 0 {
                            break;
                        }
                    }
                }
                terrane_collection_support::List::new(__terrane_pipeline_output)
            };
            return __terrane_raised(
                captured
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &terrane_int_support::Int::from(0_i128),
                            ),

                                315 /* terrane-site: tests/integration/sequential-pipeline/case.trn:205:12-205:23 */,
                        ),
                    ),

                    315 /* terrane-site: tests/integration/sequential-pipeline/case.trn:205:12-205:23 */,
            );
        })
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&reusable()), String::from(", "),
        terrane_scalar_support::scalar_text(&reusable())
    );
    drop(reusable);
    increment = {
        std::sync::Arc::new(move |
            value: &terrane_int_support::Int,
        | -> terrane_int_support::Int {
            return value.clone() + terrane_int_support::Int::from(1_i128);
        })
    };
    shared = {
        let __terrane_pipeline_source = &*values;
        let __terrane_pipeline_callback_1 = { &increment };
        let mut __terrane_pipeline_remaining_2 = {
            let __terrane_pipeline_count_2 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_2 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_2.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_2),
        );
        if !(__terrane_pipeline_remaining_2 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    let __terrane_pipeline_mapped_1 = __terrane_pipeline_callback_1(
                        __terrane_pipeline_item,
                    );
                    __terrane_pipeline_remaining_2 -= 1;
                    __terrane_pipeline_output.push(__terrane_pipeline_mapped_1);
                }
                if __terrane_pipeline_remaining_2 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(shared
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        316 /* terrane-site: tests/integration/sequential-pipeline/case.trn:214:10-214:19 */)),
        316 /* terrane-site: tests/integration/sequential-pipeline/case.trn:214:10-214:19 */)),
        String::from("; reused "), terrane_scalar_support::scalar_text(&increment({ let
        __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        317 /* terrane-site: tests/integration/sequential-pipeline/case.trn:214:46-214:55 */);
        __terrane_raised(values.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        317 /* terrane-site: tests/integration/sequential-pipeline/case.trn:214:46-214:55 */) }))
    );
    big_limit = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211456",
    );
    big_prefix = {
        let __terrane_pipeline_source = &*values;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = &big_limit;
            if __terrane_pipeline_count_1 <= &terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_1),
        );
        if !(__terrane_pipeline_remaining_1 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    __terrane_pipeline_remaining_1 -= 1;
                    __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
                }
                if __terrane_pipeline_remaining_1 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(big_prefix
        .length())), String::from("; limit "),
        terrane_scalar_support::scalar_text(&big_limit)
    );
    small = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(4_i128)
        ],
    );
    keep = {
        std::sync::Arc::new(move |value: terrane_int_support::Int| -> bool {
            return value > terrane_int_support::Int::from(1_i128);
        })
    };
    shared_twice = {
        let __terrane_pipeline_source = &small;
        let __terrane_pipeline_callback_1 = { &keep };
        let __terrane_pipeline_callback_2 = { &keep };
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1((*__terrane_pipeline_item).clone()) {
                    break '__terrane_pipeline_candidate;
                }
                if !__terrane_pipeline_callback_2((*__terrane_pipeline_item).clone()) {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(shared_twice
        .length()))
    );
    seen = terrane_int_support::Int::from(0_i128);
    limited = {
        let mut seen = seen.clone();
        TerraneMutableCallable::new(move |(value,): (terrane_int_support::Int,)| -> bool {
            let _ = &value;
            seen = &seen + &terrane_int_support::Int::from(1_i128);
            return seen <= terrane_int_support::Int::from(2_i128);
        })
    };
    forked = {
        let __terrane_pipeline_source = &small;
        let __terrane_pipeline_callback_1 = limited.clone();
        let __terrane_pipeline_callback_2 = limited;
        let mut __terrane_pipeline_output: Vec<terrane_int_support::Int> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1
                    .call(((*__terrane_pipeline_item).clone(),))
                {
                    break '__terrane_pipeline_candidate;
                }
                if !__terrane_pipeline_callback_2
                    .call(((*__terrane_pipeline_item).clone(),))
                {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push((*__terrane_pipeline_item).clone());
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(forked
        .length())), String::from(": "),
        terrane_scalar_support::scalar_text(&__terrane_raised(forked
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        318 /* terrane-site: tests/integration/sequential-pipeline/case.trn:239:31-239:40 */)),
        318 /* terrane-site: tests/integration/sequential-pipeline/case.trn:239:31-239:40 */)),
        String::from(", "), terrane_scalar_support::scalar_text(&__terrane_raised(forked
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        319 /* terrane-site: tests/integration/sequential-pipeline/case.trn:239:48-239:57 */)),
        319 /* terrane-site: tests/integration/sequential-pipeline/case.trn:239:48-239:57 */))
    );
}
#[derive(Clone)]
pub struct CounterBox {
    pub predicates: terrane_int_support::Int,
    pub mapped: terrane_int_support::Int,
}
impl CounterBox {
    pub fn terrane_construct() -> Self {
        Self {
            predicates: terrane_int_support::Int::from(0_i128),
            mapped: terrane_int_support::Int::from(0_i128),
        }
    }
}
// Source: tests/integration/shared-reference-capture-reassign/case.trn
// Namespace: shared-reference-capture-reassign
#[derive(Clone)]
pub struct HandleCounter {
    pub value: terrane_int_support::Int,
}
impl HandleCounter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
fn test_scenario_terrane_shared_reference_capture_reassign() {
    let counts: std::sync::Arc<TerraneSharedCell<HandleCounter>>;
    let original: std::sync::Arc<TerraneSharedCell<HandleCounter>>;
    let replacement: std::sync::Arc<TerraneSharedCell<HandleCounter>>;
    let change: TerraneMutableCallable<(), terrane_int_support::Int>;
    counts = std::sync::Arc::new(
        TerraneSharedCell::new(HandleCounter::terrane_construct()),
    );
    original = counts.clone();
    replacement = std::sync::Arc::new(
        TerraneSharedCell::new(HandleCounter::terrane_construct()),
    );
    {
        let __terrane_owner_0 = replacement.clone();
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            10_i128,
        );
        __terrane_owner_0.lock().expect("shared reference lock poisoned").value = __terrane_value;
    }
    change = {
        let mut counts = counts.clone();
        let replacement = replacement.clone();
        TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            {
                let __terrane_owner_1 = counts.clone();
                let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
                    1_i128,
                );
                __terrane_owner_1
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value = __terrane_value;
            }
            counts = replacement.clone();
            return {
                let __terrane_snapshot = counts
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value
                    .clone();
                __terrane_snapshot
            };
        })
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&change.call(())),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot
        = replacement.lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot }), String::from(" "), terrane_scalar_support::scalar_text(&{
        let __terrane_snapshot = original.lock().expect("shared reference lock poisoned")
        .value.clone(); __terrane_snapshot })
    );
}
// Source: tests/integration/stable-list-sorting/case.trn
// Namespace: stable-list-sorting
fn test_scenario_terrane_stable_list_sorting() {
    let mut adaptive: terrane_collection_support::List<terrane_int_support::Int>;
    let mut int8s: terrane_collection_support::List<i8>;
    let mut int16s: terrane_collection_support::List<i16>;
    let mut int32s: terrane_collection_support::List<i32>;
    let mut int64s: terrane_collection_support::List<i64>;
    let mut int128s: terrane_collection_support::List<i128>;
    let mut uint8s: terrane_collection_support::List<u8>;
    let mut uint16s: terrane_collection_support::List<u16>;
    let mut uint32s: terrane_collection_support::List<u32>;
    let mut uint64s: terrane_collection_support::List<u64>;
    let mut uint128s: terrane_collection_support::List<u128>;
    let mut words: terrane_collection_support::List<String>;
    let zero: f64;
    let negative_zero: f64;
    let one: f64;
    let negative_one: f64;
    let infinity: f64;
    let negative_infinity: f64;
    let first_nan: f64;
    let second_nan: f64;
    let mut floats: terrane_collection_support::List<f64>;
    let mut narrow: terrane_collection_support::List<f32>;
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let preserved: terrane_collection_support::List<terrane_int_support::Int>;
    let mut returned: terrane_collection_support::List<terrane_int_support::Int>;
    let indexed: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let mut keys: terrane_collection_support::List<String>;
    let ascending: terrane_collection_support::List<String>;
    let mut key: String;
    let descending: terrane_collection_support::List<String>;
    let mut empty: terrane_collection_support::List<terrane_int_support::Int>;
    let mut single: terrane_collection_support::List<terrane_int_support::Int>;
    adaptive = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from_decimal("900000000000000000000000000000000000000"),
            terrane_int_support::Int::from(- 2_i128),
            terrane_int_support::Int::from(7_i128),
            terrane_int_support::Int::from(7_i128)
        ],
    );
    {
        adaptive.sort_by(|left, right| left.cmp(right));
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        320 /* terrane-site: tests/integration/stable-list-sorting/case.trn:7:10-7:21 */)),
        320 /* terrane-site: tests/integration/stable-list-sorting/case.trn:7:10-7:21 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        321 /* terrane-site: tests/integration/stable-list-sorting/case.trn:7:23-7:34 */)),
        321 /* terrane-site: tests/integration/stable-list-sorting/case.trn:7:23-7:34 */))
    );
    {
        adaptive.sort_by(|left, right| right.cmp(left));
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        322 /* terrane-site: tests/integration/stable-list-sorting/case.trn:9:10-9:21 */)),
        322 /* terrane-site: tests/integration/stable-list-sorting/case.trn:9:10-9:21 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        323 /* terrane-site: tests/integration/stable-list-sorting/case.trn:9:23-9:34 */)),
        323 /* terrane-site: tests/integration/stable-list-sorting/case.trn:9:23-9:34 */))
    );
    int8s = terrane_collection_support::List::<i8>::new(vec![2, - 1]);
    int16s = terrane_collection_support::List::<i16>::new(vec![2, - 1]);
    int32s = terrane_collection_support::List::<i32>::new(vec![2, - 1]);
    int64s = terrane_collection_support::List::<i64>::new(vec![2, - 1]);
    int128s = terrane_collection_support::List::<i128>::new(vec![2, - 1]);
    uint8s = terrane_collection_support::List::<u8>::new(vec![2, 1]);
    uint16s = terrane_collection_support::List::<u16>::new(vec![2, 1]);
    uint32s = terrane_collection_support::List::<u32>::new(vec![2, 1]);
    uint64s = terrane_collection_support::List::<u64>::new(vec![2, 1]);
    uint128s = terrane_collection_support::List::<u128>::new(vec![2, 1]);
    {
        int8s.sort_by(|left, right| left.cmp(right));
    };
    {
        int16s.sort_by(|left, right| left.cmp(right));
    };
    {
        int32s.sort_by(|left, right| left.cmp(right));
    };
    {
        int64s.sort_by(|left, right| left.cmp(right));
    };
    {
        int128s.sort_by(|left, right| left.cmp(right));
    };
    {
        uint8s.sort_by(|left, right| left.cmp(right));
    };
    {
        uint16s.sort_by(|left, right| left.cmp(right));
    };
    {
        uint32s.sort_by(|left, right| left.cmp(right));
    };
    {
        uint64s.sort_by(|left, right| left.cmp(right));
    };
    {
        uint128s.sort_by(|left, right| left.cmp(right));
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(int8s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        324 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:10-31:18 */)),
        324 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:10-31:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int16s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        325 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:20-31:29 */)),
        325 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:20-31:29 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int32s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        326 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:31-31:40 */)),
        326 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:31-31:40 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int64s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        327 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:42-31:51 */)),
        327 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:42-31:51 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int128s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        328 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:53-31:63 */)),
        328 /* terrane-site: tests/integration/stable-list-sorting/case.trn:31:53-31:63 */))
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(uint8s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        329 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:10-32:19 */)),
        329 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:10-32:19 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint16s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        330 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:21-32:31 */)),
        330 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:21-32:31 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint32s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        331 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:33-32:43 */)),
        331 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:33-32:43 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint64s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        332 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:45-32:55 */)),
        332 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:45-32:55 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint128s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        333 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:57-32:68 */)),
        333 /* terrane-site: tests/integration/stable-list-sorting/case.trn:32:57-32:68 */))
    );
    words = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("é"), String::from("e"), String::from("z")]);
    {
        words.sort_by(|left, right| left.cmp(right));
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        334 /* terrane-site: tests/integration/stable-list-sorting/case.trn:36:10-36:18 */)),
        334 /* terrane-site: tests/integration/stable-list-sorting/case.trn:36:10-36:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        335 /* terrane-site: tests/integration/stable-list-sorting/case.trn:36:20-36:28 */)),
        335 /* terrane-site: tests/integration/stable-list-sorting/case.trn:36:20-36:28 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        336 /* terrane-site: tests/integration/stable-list-sorting/case.trn:36:30-36:38 */)),
        336 /* terrane-site: tests/integration/stable-list-sorting/case.trn:36:30-36:38 */))
    );
    {
        words.sort_by(|left, right| right.cmp(left));
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        337 /* terrane-site: tests/integration/stable-list-sorting/case.trn:38:10-38:18 */)),
        337 /* terrane-site: tests/integration/stable-list-sorting/case.trn:38:10-38:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        338 /* terrane-site: tests/integration/stable-list-sorting/case.trn:38:20-38:28 */)),
        338 /* terrane-site: tests/integration/stable-list-sorting/case.trn:38:20-38:28 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        339 /* terrane-site: tests/integration/stable-list-sorting/case.trn:38:30-38:38 */)),
        339 /* terrane-site: tests/integration/stable-list-sorting/case.trn:38:30-38:38 */))
    );
    zero = 0.0;
    negative_zero = -0.0_f64;
    one = 1.0;
    negative_one = -1.0_f64;
    infinity = one / zero;
    negative_infinity = negative_one / zero;
    first_nan = zero / zero;
    second_nan = negative_zero / zero;
    floats = terrane_collection_support::List::<
        f64,
    >::new(
        vec![first_nan, negative_zero, infinity, zero, negative_infinity, second_nan],
    );
    {
        floats.sort_by(terrane_collection_support::compare_float64_ascending);
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        340 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:10-50:19 */)),
        340 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:10-50:19 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        341 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:30-50:39 */)),
        341 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:30-50:39 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        342 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:55-50:64 */)),
        342 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:55-50:64 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        343 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:80-50:89 */)),
        343 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:80-50:89 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        344 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:105-50:114 */)),
        344 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:105-50:114 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        345 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:125-50:134 */)),
        345 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:125-50:134 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        346 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:149-50:158 */)),
        346 /* terrane-site: tests/integration/stable-list-sorting/case.trn:50:149-50:158 */).is_nan())
    );
    {
        floats.sort_by(terrane_collection_support::compare_float64_descending);
    };
    println!(
        "{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        347 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:10-52:19 */)),
        347 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:10-52:19 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        348 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:30-52:39 */)),
        348 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:30-52:39 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        349 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:55-52:64 */)),
        349 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:55-52:64 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        350 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:80-52:89 */)),
        350 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:80-52:89 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        351 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:105-52:114 */)),
        351 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:105-52:114 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        352 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:125-52:134 */)),
        352 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:125-52:134 */)
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        353 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:150-52:159 */)),
        353 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:150-52:159 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        354 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:174-52:183 */)),
        354 /* terrane-site: tests/integration/stable-list-sorting/case.trn:52:174-52:183 */).is_nan())
    );
    narrow = terrane_collection_support::List::<f32>::new(vec![2.0_f32, - 1.0_f32]);
    {
        narrow.sort_by(terrane_collection_support::compare_float32_ascending);
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        355 /* terrane-site: tests/integration/stable-list-sorting/case.trn:56:10-56:19 */)),
        355 /* terrane-site: tests/integration/stable-list-sorting/case.trn:56:10-56:19 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        356 /* terrane-site: tests/integration/stable-list-sorting/case.trn:56:21-56:30 */)),
        356 /* terrane-site: tests/integration/stable-list-sorting/case.trn:56:21-56:30 */))
    );
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    preserved = values.clone();
    returned = {
        let __terrane_result = {
            let collection = &mut values;
            collection.sort_by(|left, right| left.cmp(right));
            collection.clone()
        };
        __terrane_result
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(preserved
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        357 /* terrane-site: tests/integration/stable-list-sorting/case.trn:61:10-61:22 */)),
        357 /* terrane-site: tests/integration/stable-list-sorting/case.trn:61:10-61:22 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        358 /* terrane-site: tests/integration/stable-list-sorting/case.trn:61:24-61:33 */)),
        358 /* terrane-site: tests/integration/stable-list-sorting/case.trn:61:24-61:33 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        359 /* terrane-site: tests/integration/stable-list-sorting/case.trn:61:35-61:46 */)),
        359 /* terrane-site: tests/integration/stable-list-sorting/case.trn:61:35-61:46 */))
    );
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(4_i128);
        returned.append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length()))
    );
    indexed = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("c"),
            terrane_int_support::Int::from(3_i128)),
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("b"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    keys = {
        let __terrane_result = indexed.keys();
        __terrane_result
    };
    ascending = {
        let __terrane_result = {
            let collection = &mut keys;
            collection.sort_by(|left, right| left.cmp(right));
            collection.clone()
        };
        __terrane_result
    };
    let __terrane_iterable_0 = ascending;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        key = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&__terrane_raised(indexed
            .get_or_error(&key), 360 /* terrane-site: tests/integration/stable-list-sorting/case.trn:69:17-69:29 */))
        );
    }
    descending = {
        let __terrane_result = {
            let collection = &mut keys;
            collection.sort_by(|left, right| right.cmp(left));
            collection.clone()
        };
        __terrane_result
    };
    let __terrane_iterable_1 = descending;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        key = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&__terrane_raised(indexed
            .get_or_error(&key.clone()), 361 /* terrane-site: tests/integration/stable-list-sorting/case.trn:72:17-72:29 */))
        );
    }
    empty = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    {
        empty.sort_by(|left, right| left.cmp(right));
    };
    single = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    {
        single.sort_by(|left, right| right.cmp(left));
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(single
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        362 /* terrane-site: tests/integration/stable-list-sorting/case.trn:78:24-78:33 */)),
        362 /* terrane-site: tests/integration/stable-list-sorting/case.trn:78:24-78:33 */))
    );
}
// Source: tests/integration/string-display-composition/case.trn
// Namespace: string-display-composition
#[derive(Clone)]
pub struct RenderError {
    pub message: String,
}
impl RenderError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("cannot render"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C {
    pub text: String,
}
impl TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value: String = text;
            self.text = __terrane_value;
        }
    }
    pub fn to_string(&self) -> String {
        println!(
            "{}{}", String::from("render:"), terrane_scalar_support::scalar_text(&self
            .text)
        );
        return self.text.clone();
    }
}
pub trait TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    >;
    fn to_string(&self) -> Result<String, TerraneError>;
}
impl Clone
for Box<
    dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65(
    Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    >,
);
impl Clone
for TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    pub fn to_string(&self) -> Result<String, TerraneError> {
        self.0.to_string()
    }
}
#[derive(Clone)]
pub struct Broken {}
impl Broken {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> Result<String, TerraneError> {
        return Err({
            let value = RenderError::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(6)),
                message,
                rendered,

                    363 /* terrane-site: tests/integration/string-display-composition/case.trn:23:9-23:37 */,
            )
        });
    }
}
impl TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol
for Broken {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn to_string(&self) -> Result<String, TerraneError> {
        Broken::to_string(&*self)
    }
}
impl From<Broken>
for TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    fn from(value: Broken) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Calm {}
impl Calm {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> String {
        return String::from("calm");
    }
}
impl TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol
for Calm {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn to_string(&self) -> Result<String, TerraneError> {
        Ok(Calm::to_string(&*self))
    }
}
impl From<Calm>
for TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    fn from(value: Calm) -> Self {
        Self(Box::new(value))
    }
}
fn number_terrane_string_display_composition(
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    println!(
        "{}{}", String::from("number:"), terrane_scalar_support::scalar_text(&value)
    );
    return value.clone();
}
fn render_generic<
    TerraneType54: TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
>(value: TerraneType54) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_text = String::with_capacity(8);
        __terrane_text.push_str("generic:");
        __terrane_text
            .push_str(
                &__terrane_traced_err(
                    value.to_string(),

                        364 /* terrane-site: tests/integration/string-display-composition/case.trn:34:25-34:30 */,
                )?,
            );
        __terrane_text
    });
}
fn test_composition() -> Result<(), TerraneError> {
    let item: TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C;
    let erased: TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6);
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(2_i128)
        + terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        .push_str(" items"); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("items: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&2, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("items: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(2_i128)
        + terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(1); __terrane_text.push_str("2");
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text }), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text.push_str("2"); __terrane_text })
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&true)),
        String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&())),
        String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&42))
    );
    item = TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C::terrane_construct(
        String::from("label"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&item.to_string()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(13); __terrane_text.push_str("before "); __terrane_text
        .push_str(&item.to_string()); __terrane_text.push_str(" after"); __terrane_text
        })
    );
    println!("{}", item.to_string());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("prefix "); __terrane_text
        .push_str(&item.to_string());
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &":"; let
        mut __terrane_text = String::with_capacity(1); __terrane_text.push_str(&item
        .to_string()); __terrane_text.push_str(__terrane_separator);
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(8); __terrane_text.push_str("ordered ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&number_terrane_string_display_composition(terrane_int_support::Int::from(2_i128)),
        &mut __terrane_text); __terrane_text.push_str(&item.to_string());
        terrane_scalar_support::ScalarDisplay::write_scalar(&number_terrane_string_display_composition(terrane_int_support::Int::from(3_i128)),
        &mut __terrane_text); __terrane_text })
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
                String::with_capacity(7); __terrane_text.push_str("broken ");
                __terrane_text
                .push_str(&__terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 365 /* terrane-site: tests/integration/string-display-composition/case.trn:50:29-50:45 */));
                __terrane_text })
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
                        == TerraneErrorKind::Custom(DescriptorId(6))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught addition"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}", __terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 366 /* terrane-site: tests/integration/string-display-composition/case.trn:54:17-54:33 */)
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
                        == TerraneErrorKind::Custom(DescriptorId(6))
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("caught print"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            {
                let _: String = {
                    let mut __terrane_text = String::with_capacity(0);
                    __terrane_text.push_str("");
                    __terrane_text
                        .push_str(
                            &__terrane_traced_completion!(
                                Broken::terrane_construct().to_string(),
                                367 /* terrane-site: tests/integration/string-display-composition/case.trn:58:25-58:41 */
                            ),
                        );
                    __terrane_text
                };
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(6))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("caught concat"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            {
                let _: String = {
                    let __terrane_separator = &":";
                    let mut __terrane_text = String::with_capacity(0);
                    __terrane_text
                        .push_str(
                            &__terrane_traced_completion!(
                                Broken::terrane_construct().to_string(),
                                368 /* terrane-site: tests/integration/string-display-composition/case.trn:62:24-62:40 */
                            ),
                        );
                    __terrane_text
                };
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && __terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(6))
                {
                    __terrane_handled_3 = true;
                    println!("{}", String::from("caught join"));
                }
                if !__terrane_handled_3 {
                    return TerraneCompletion::Error(__terrane_error_3);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_3 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(render_generic(Broken::terrane_construct()),
                369 /* terrane-site: tests/integration/string-display-composition/case.trn:66:17-66:51 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_4 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_4) => {
                let mut __terrane_handled_4 = false;
                if !__terrane_handled_4
                    && __terrane_error_4.kind
                        == TerraneErrorKind::Custom(DescriptorId(6))
                {
                    __terrane_handled_4 = true;
                    println!("{}", String::from("caught generic"));
                }
                if !__terrane_handled_4 {
                    return TerraneCompletion::Error(__terrane_error_4);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_4 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(render_generic(Calm::terrane_construct()),
        370 /* terrane-site: tests/integration/string-display-composition/case.trn:69:13-69:45 */) ?)
    );
    erased = <TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65>::from(
        Calm::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("interface:"); __terrane_text
        .push_str(&__terrane_traced_err(erased.to_string(), 371 /* terrane-site: tests/integration/string-display-composition/case.trn:71:27-71:33 */) ?);
        __terrane_text })
    );
    Ok(())
}
// Source: tests/integration/string-display-generic/case.trn
// Namespace: string-display-generic
pub trait TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    >;
    fn to_string(&self) -> String;
}
impl Clone
for Box<
    dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65(
    Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    >,
);
impl Clone
for TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65 {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone)]
pub struct TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C {
    pub text: String,
}
impl TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value: String = text;
            self.text = __terrane_value;
        }
    }
    pub fn to_string(&self) -> String {
        return self.text.clone();
    }
}
impl TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol
for TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn to_string(&self) -> String {
        TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C::to_string(
            &*self,
        )
    }
}
impl From<
    TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C,
>
for TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65 {
    fn from(
        value: TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn decorate_terrane_string_display_generic<
    TerraneType54: TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
>(value: TerraneType54) -> String {
    println!("{}", value.to_string());
    return {
        let mut __terrane_text = String::with_capacity(8);
        __terrane_text.push_str("generic:");
        __terrane_text.push_str(&value.to_string());
        __terrane_text
    };
}
fn compose<
    TerraneType54: TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
>(value: TerraneType54) -> String {
    return {
        let mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &{
                let mut __terrane_text = String::with_capacity(7);
                __terrane_text.push_str("prefix ");
                __terrane_text.push_str(&value.to_string());
                __terrane_text
            },
            &mut __terrane_text,
        );
        __terrane_text.push_str("|");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &{
                let __terrane_separator = &":";
                let mut __terrane_text = String::with_capacity(1);
                __terrane_text.push_str(&value.to_string());
                __terrane_text.push_str(__terrane_separator);
                __terrane_text.push_str(&value.to_string());
                __terrane_text
            },
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn test_generic_terrane_string_display_generic() {
    let value: TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C;
    let erased: TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65;
    value = TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C::terrane_construct(
        String::from("label"),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&decorate_terrane_string_display_generic(value
        .clone()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&compose(value.clone())));
    erased = <TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65>::from(
        value,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("interface:"); __terrane_text
        .push_str(&erased.to_string()); __terrane_text })
    );
}
// Source: tests/integration/string-ref-then-owned/case.trn
// Namespace: string-ref-then-owned
fn look_terrane_string_ref_then_owned(
    a: &String,
    b: String,
) -> terrane_int_support::Int {
    let _ = (&a, &b);
    return terrane_int_support::Int::from(1_i128);
}
fn f_terrane_string_ref_then_owned(o: String) -> terrane_int_support::Int {
    return look_terrane_string_ref_then_owned(&o, o.clone());
}
fn test_scenario_terrane_string_ref_then_owned() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_string_ref_then_owned(String::from("a")))
    );
}
// Source: tests/integration/string-result-list-iteration/case.trn
// Namespace: string-result-list-iteration
fn test_scenario_terrane_string_result_list_iteration() {
    let parts: Vec<String>;
    let mut part: String;
    let matches: Vec<terrane_string_support::TextRange>;
    let mut found: terrane_string_support::TextRange;
    parts = {
        let __terrane_receiver_value_0 = &String::from("red,green,blue");
        let __terrane_string_argument_0 = ",";
        terrane_string_support::split(
            &__terrane_receiver_value_0,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    let __terrane_iterable_0 = parts.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_0,
    );
    loop {
        part = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&part));
    }
    println!("{}", terrane_scalar_support::scalar_text(&(parts.len() as i128)));
    let __terrane_iterable_1 = parts;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_1,
    );
    loop {
        part = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&part));
    }
    matches = {
        let __terrane_receiver_value_1 = &String::from("banana");
        let __terrane_string_argument_0 = "an";
        terrane_string_support::find_all(
            &__terrane_receiver_value_1,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    let __terrane_iterable_2 = matches.clone();
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_2,
    );
    loop {
        found = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&found.text().to_owned()));
    }
    println!("{}", terrane_scalar_support::scalar_text(&(matches.len() as i128)));
}
// Source: tests/integration/string-value-parameter-reuse/case.trn
// Namespace: string-value-parameter-reuse
fn consume_terrane_string_value_parameter_reuse(value: String) {
    println!(
        "{}{}", String::from("consume:"), terrane_scalar_support::scalar_text(&value)
    );
}
#[derive(Clone)]
pub struct MessageBox {
    pub message: String,
}
impl MessageBox {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String) {
        {
            let __terrane_value: String = message.clone();
            self.message = __terrane_value;
        }
        println!(
            "{}{}", String::from("parameter:"),
            terrane_scalar_support::scalar_text(&message)
        );
    }
}
fn test_scenario_terrane_string_value_parameter_reuse() {
    let entries: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let mut key: String;
    let mut value: terrane_int_support::Int;
    let holder: MessageBox;
    entries = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    let __terrane_iterable_0 = entries;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_0.key;
        value = __terrane_item_0.value;
        consume_terrane_string_value_parameter_reuse(key.clone());
        println!(
            "{}{}{}{}", String::from("key:"), terrane_scalar_support::scalar_text(&key),
            String::from("="), terrane_scalar_support::scalar_text(&value)
        );
    }
    holder = MessageBox::terrane_construct(String::from("saved"));
    println!(
        "{}{}", String::from("field:"), terrane_scalar_support::scalar_text(&holder
        .message)
    );
}
// Source: tests/integration/text-families/case.trn
// Namespace: text-families
fn test_scenario_terrane_text_families() -> Result<(), TerraneError> {
    let text: String;
    let decomposed: String;
    let rtl: String;
    let new_combining_mark: String;
    let mut grapheme: String;
    let searches: terrane_collection_support::List<String>;
    let html_replacements: terrane_collection_support::List<String>;
    let reversed_searches: terrane_collection_support::List<String>;
    let reversed_replacements: terrane_collection_support::List<String>;
    let broadcast_searches: terrane_collection_support::List<String>;
    let priority_searches: terrane_collection_support::List<String>;
    let priority_replacements: terrane_collection_support::List<String>;
    let no_searches: terrane_collection_support::List<String>;
    let no_replacements: terrane_collection_support::List<String>;
    let mismatch_searches: terrane_collection_support::List<String>;
    text = String::from("  Straße  ");
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_string_support::trim(&text))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::trim_start(&text,
        None)),
        terrane_scalar_support::scalar_text(&terrane_string_support::trim_end(&text,
        None))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::case_fold(&text))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
        &String::from("straße");
        terrane_string_support::upper_first(&__terrane_receiver_value_0) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_1 =
        &String::from("hello world");
        terrane_string_support::upper_words(&__terrane_receiver_value_1) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 =
        &String::from("Hello");
        terrane_string_support::lower_first(&__terrane_receiver_value_2) })
    );
    decomposed = String::from("e\u{301}");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::normalise(&decomposed,
        "nfc"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_3 =
        &String::from("banana"); let __terrane_string_argument_0 = "ana"; let
        __terrane_string_argument_1 = "X";
        terrane_string_support::replace(&__terrane_receiver_value_3,
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0),
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_1)) })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_4 =
        &String::from("banana"); let __terrane_string_argument_0 = "ana";
        terrane_int_support::Int::from(terrane_string_support::find_all(&__terrane_receiver_value_4,
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0)).len() as
        i128) }), terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_5 =
        &String::from("banana"); let __terrane_string_argument_0 = "";
        terrane_int_support::Int::from(terrane_string_support::find_all(&__terrane_receiver_value_5,
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0)).len() as
        i128) })
    );
    rtl = String::from("שלום");
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        "ש"; rtl.starts_with(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 = "ם";
        rtl.ends_with(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_string_argument_0 =
        ""; terrane_string_support::find_all(&decomposed, &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) } .len() as i128))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_string_argument_0 =
        ""; terrane_string_support::split(&decomposed, &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) } .len() as i128))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        ""; let __terrane_string_argument_1 = "X";
        terrane_string_support::replace(&decomposed, &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0), &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_1)) })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(decomposed.len() as i128)),
        terrane_scalar_support::scalar_text(&(decomposed.chars().count() as i128)),
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&decomposed)
        as i128))
    );
    new_combining_mark = String::from("A\u{1ae0}");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&new_combining_mark)
        as i128))
    );
    {
        let __terrane_iterable_0 = &new_combining_mark;
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            grapheme = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&grapheme));
        }
    }
    searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&"), String::from("<")]);
    html_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&amp;"), String::from("&lt;")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_6 =
        &String::from("&<"); let __terrane_string_argument_0 = &searches; let
        __terrane_string_argument_1 = &html_replacements;
        __terrane_raised_err(terrane_string_support::replace_many(&__terrane_receiver_value_6,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        372 /* terrane-site: tests/integration/text-families/case.trn:29:11-29:52 */)
        ? })
    );
    reversed_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("<"), String::from("&")]);
    reversed_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&lt;"), String::from("&amp;")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_7 =
        &String::from("&<"); let __terrane_string_argument_0 = &reversed_searches; let
        __terrane_string_argument_1 = &reversed_replacements;
        __terrane_raised_err(terrane_string_support::replace_many(&__terrane_receiver_value_7,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        373 /* terrane-site: tests/integration/text-families/case.trn:32:11-32:65 */)
        ? })
    );
    broadcast_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("one"), String::from("two")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_8 =
        &String::from("one and two"); let __terrane_string_argument_0 =
        &broadcast_searches; let __terrane_string_argument_1 = "x";
        terrane_string_support::replace_broadcast(&__terrane_receiver_value_8,
        __terrane_string_argument_0.as_slice(), &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_1)) })
    );
    priority_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("a"), String::from("ab")]);
    priority_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("first"), String::from("second")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_9 =
        &String::from("ab"); let __terrane_string_argument_0 = &priority_searches; let
        __terrane_string_argument_1 = &priority_replacements;
        __terrane_raised_err(terrane_string_support::replace_many(&__terrane_receiver_value_9,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        374 /* terrane-site: tests/integration/text-families/case.trn:37:11-37:65 */)
        ? })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_10 =
        &String::from("abc"); let __terrane_string_argument_0 =
        &terrane_collection_support::List:: < String > ::new(vec![String::from("b")]);
        let __terrane_string_argument_1 = &terrane_collection_support::List:: < String >
        ::new(vec![String::from("abc")]);
        __terrane_raised_err(terrane_string_support::replace_many(&__terrane_receiver_value_10,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        375 /* terrane-site: tests/integration/text-families/case.trn:38:11-38:52 */)
        ? })
    );
    no_searches = terrane_collection_support::List::<String>::new(Vec::new());
    no_replacements = terrane_collection_support::List::<String>::new(Vec::new());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_11 =
        &String::from("text"); let __terrane_string_argument_0 = &no_searches; let
        __terrane_string_argument_1 = &no_replacements;
        __terrane_raised_err(terrane_string_support::replace_many(&__terrane_receiver_value_11,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        376 /* terrane-site: tests/integration/text-families/case.trn:41:11-41:55 */)
        ? })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        &terrane_collection_support::List:: < String > ::new(vec![String::from("")]); let
        __terrane_string_argument_1 = &terrane_collection_support::List:: < String >
        ::new(vec![String::from("-")]);
        __terrane_raised_err(terrane_string_support::replace_many(&decomposed,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        377 /* terrane-site: tests/integration/text-families/case.trn:42:11-42:54 */)
        ? })
    );
    mismatch_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("x")]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_receiver_value_12 = &String::from("x"); let
                __terrane_string_argument_0 = &mismatch_searches; let
                __terrane_string_argument_1 = &no_replacements;
                __terrane_raised_completion!(terrane_string_support::replace_many(&__terrane_receiver_value_12,
                __terrane_string_argument_0.as_slice(), __terrane_string_argument_1
                .as_slice()), 378 /* terrane-site: tests/integration/text-families/case.trn:45:13-45:60 */) })
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
                    && __terrane_error_0.kind == TerraneErrorKind::ReplaceError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("replacement-list-mismatch"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/truth-protocol/case.trn
// Namespace: truth-protocol
#[derive(Clone)]
pub struct Gate {
    pub open: bool,
}
impl Gate {
    pub fn terrane_construct(open: bool) -> Self {
        let mut __terrane_constructed_value = Self { open: false };
        __terrane_constructed_value.construct(open);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, open: bool) {
        {
            let __terrane_value: bool = open;
            self.open = __terrane_value;
        }
    }
    pub fn truth(&self) -> bool {
        return self.open;
    }
}
fn test_scenario_terrane_truth_protocol() {
    let enabled: Gate;
    let disabled: Gate;
    enabled = Gate::terrane_construct(true);
    disabled = Gate::terrane_construct(false);
    if enabled.truth() {
        println!("{}", String::from("enabled"));
    }
    if disabled.truth() {
        println!("{}", String::from("unexpected"));
    } else {
        println!("{}", String::from("disabled"));
    }
}
// Source: tests/integration/tuple-type-boundaries/case.trn
// Namespace: tuple-type-boundaries
fn echo(
    values: terrane_collection_support::Tuple<terrane_int_support::Int>,
) -> terrane_collection_support::Tuple<terrane_int_support::Int> {
    return values;
}
fn nested(
    values: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >,
) -> terrane_collection_support::List<
    terrane_collection_support::Tuple<terrane_int_support::Int>,
> {
    return values;
}
fn test_scenario_terrane_tuple_type_boundaries() {
    let pair: terrane_collection_support::Tuple<terrane_int_support::Int>;
    let returned: terrane_collection_support::Tuple<terrane_int_support::Int>;
    let empty: terrane_collection_support::Tuple<terrane_int_support::Int>;
    let groups: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let echoed: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    pair = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    returned = echo(pair);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        379 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:13:27-13:38 */)),
        379 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:13:27-13:38 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        380 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:13:40-13:51 */)),
        380 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:13:40-13:51 */))
    );
    empty = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(Vec::new());
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty
        .length()))
    );
    groups = terrane_collection_support::List::<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Tuple::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(4_i128)]),
            terrane_collection_support::Tuple::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(5_i128)])
        ],
    );
    echoed = nested(groups);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(echoed
        .length())), terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        381 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:25-18:34 */); let
        __terrane_index_place_1 = __terrane_raised(echoed.get(__terrane_index_0)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        381 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:25-18:34 */);
        __terrane_raised(__terrane_index_place_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        382 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:25-18:37 */)),
        382 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:25-18:37 */) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_2 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        383 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:39-18:48 */); let
        __terrane_index_place_3 = __terrane_raised(echoed.get(__terrane_index_2)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        383 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:39-18:48 */);
        __terrane_raised(__terrane_index_place_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        384 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:39-18:51 */)),
        384 /* terrane-site: tests/integration/tuple-type-boundaries/case.trn:18:39-18:51 */) })
    );
}
// Source: tests/integration/typed-catch-bindings/case.trn
// Namespace: typed-catch-bindings
fn test_scenario_terrane_typed_catch_bindings() -> Result<(), TerraneError> {
    let mut caught: Option<TerraneError> = None;
    let mut replacement: Option<TerraneError> = None;
    let mut caught_2: Option<TerraneError> = None;
    let mut reraised: Option<TerraneError> = None;
    let mut failure: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let __terrane_completion_1: TerraneCompletion<()> = (|| {
                let __terrane_try_1: TerraneCompletion<()> = (|| {
                    return TerraneCompletion::Error(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,

                                385 /* terrane-site: tests/integration/typed-catch-bindings/case.trn:7:7-7:27 */,
                        ),
                    );
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
                            let _ = caught.insert(__terrane_error_1.clone());
                            {
                                let __terrane_error_text_0 = caught
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                println!("{}", __terrane_error_text_0);
                            };
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&{ let
                                __terrane_receiver_value_1 = &caught.as_ref()
                                .expect("flow-proven available binding").render(); let
                                __terrane_string_argument_0 =
                                "coercion-error: coercion has no compatible result";
                                __terrane_receiver_value_1.contains(std::convert::AsRef:: <
                                str > ::as_ref(&__terrane_string_argument_0)) })
                            );
                            return TerraneCompletion::Error(
                                TerraneError::raised(
                                        TerraneErrorKind::ArithmeticOverflow,

                                            386 /* terrane-site: tests/integration/typed-catch-bindings/case.trn:11:7-11:32 */,
                                    )
                                    .with_cause(__terrane_error_1.clone()),
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
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    let _ = replacement.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&replacement.as_ref()
                        .expect("flow-proven available binding").detail.as_ref()
                        .and_then(| detail | detail.cause.as_deref()).cloned().is_none())
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            let __terrane_completion_3: TerraneCompletion<()> = (|| {
                let __terrane_try_3: TerraneCompletion<()> = (|| {
                    return TerraneCompletion::Error(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,

                                387 /* terrane-site: tests/integration/typed-catch-bindings/case.trn:17:7-17:27 */,
                        ),
                    );
                })();
                match __terrane_try_3 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_3) => {
                        let mut __terrane_handled_3 = false;
                        if !__terrane_handled_3
                            && __terrane_error_3.kind == TerraneErrorKind::CoercionError
                        {
                            __terrane_handled_3 = true;
                            let _ = caught_2.insert(__terrane_error_3.clone());
                            return TerraneCompletion::Error(
                                caught_2
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .clone()
                                    .at(

                                            388 /* terrane-site: tests/integration/typed-catch-bindings/case.trn:19:7-19:19 */,
                                    ),
                            );
                        }
                        if !__terrane_handled_3 {
                            return TerraneCompletion::Error(__terrane_error_3);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            match __terrane_completion_3 {
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_2 = true;
                    let _ = reraised.insert(__terrane_error_2.clone());
                    {
                        let __terrane_error_text_2 = reraised
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_2);
                    };
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,

                        389 /* terrane-site: tests/integration/typed-catch-bindings/case.trn:24:5-24:30 */,
                ),
            );
        })();
        match __terrane_try_4 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_4) => {
                let mut __terrane_handled_4 = false;
                if !__terrane_handled_4 {
                    __terrane_handled_4 = true;
                    let _ = failure.insert(__terrane_error_4.clone());
                    {
                        let __terrane_error_text_3 = failure
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_3);
                    };
                }
                if !__terrane_handled_4 {
                    return TerraneCompletion::Error(__terrane_error_4);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_4 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/typed-destinations/case.trn
// Namespace: typed-destinations
fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(41_i128);
}
fn test_scenario_terrane_typed_destinations() {
    let text: String;
    let mut total: terrane_int_support::Int;
    text = String::from("Terrane");
    total = terrane_int_support::Int::from(
        terrane_string_support::length(&text) as i128,
    );
    total = &total + &terrane_int_support::Int::from(1_i128);
    println!("{}", terrane_scalar_support::scalar_text(&answer()));
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
// Source: tests/integration/typed-document-decoding/case.trn
// Namespace: typed-document-decoding
#[derive(Clone)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
}
impl Endpoint {
    pub fn terrane_construct() -> Self {
        Self {
            host: String::from(""),
            port: 443,
        }
    }
}
impl TerraneDocumentDecode for Endpoint {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "endpoint",
                source,
                "tests/integration/typed-document-decoding/case.trn:8:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["host", "port"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "endpoint",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-document-decoding/case.trn:8:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("host");
            let field_path = __terrane_document_child_path(path, "host");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:9:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.host = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("port");
            let field_path = __terrane_document_child_path(path, "port");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <u16 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:10:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.port = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Endpoint {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Endpoint> for DocumentDecodable {
    fn from(value: Endpoint) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct ServiceConfig {
    pub service_name: String,
    pub endpoint: Endpoint,
    pub note: Option<String>,
    pub ports: terrane_collection_support::List<u16>,
    pub labels: terrane_collection_support::Map<String, String>,
    pub coordinates: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub ratio: f64,
}
impl ServiceConfig {
    pub fn terrane_construct() -> Self {
        Self {
            service_name: String::from(""),
            endpoint: Endpoint::terrane_construct(),
            note: None,
            ports: terrane_collection_support::List::<u16>::new(vec![0]),
            labels: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            coordinates: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            ratio: 0.1,
        }
    }
    pub fn validate_document(&self) -> Option<String> {
        if self.service_name.as_str() == "reserved" {
            return Some(String::from("service name is reserved"));
        }
        return None;
    }
}
impl TerraneDocumentDecode for ServiceConfig {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "service-config",
                source,
                "tests/integration/typed-document-decoding/case.trn:12:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "serviceName",
            "endpoint",
            "note",
            "ports",
            "labels",
            "coordinates",
            "ratio",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "service-config",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-document-decoding/case.trn:12:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("serviceName");
            let field_path = __terrane_document_child_path(path, "serviceName");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:13:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.service_name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("endpoint");
            let field_path = __terrane_document_child_path(path, "endpoint");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Endpoint as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:14:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.endpoint = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("note");
            let field_path = __terrane_document_child_path(path, "note");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:15:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.note = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("ports");
            let field_path = __terrane_document_child_path(path, "ports");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    u16,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:16:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.ports = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("labels");
            let field_path = __terrane_document_child_path(path, "labels");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:17:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.labels = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("coordinates");
            let field_path = __terrane_document_child_path(path, "coordinates");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:18:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.coordinates = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("ratio");
            let field_path = __terrane_document_child_path(path, "ratio");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:19:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.ratio = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() {
            if let Some(message) = value.validate_document() {
                diagnostics
                    .push(
                        __terrane_document_diagnostic(
                            path,
                            "service-config",
                            "map",
                            "validation",
                            message,
                            source,
                            "tests/integration/typed-document-decoding/case.trn:12:1",
                        ),
                    );
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for ServiceConfig {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<ServiceConfig> for DocumentDecodable {
    fn from(value: ServiceConfig) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentValidatableProtocol for ServiceConfig {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn validate_document(&self) -> Option<String> {
        ServiceConfig::validate_document(&*self)
    }
}
impl From<ServiceConfig> for DocumentValidatable {
    fn from(value: ServiceConfig) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
}
impl Waypoint {
    pub fn terrane_construct() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl TerraneDocumentDecode for Waypoint {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "waypoint",
                source,
                "tests/integration/typed-document-decoding/case.trn:26:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["x", "y"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "waypoint",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-document-decoding/case.trn:26:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("x");
            let field_path = __terrane_document_child_path(path, "x");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:27:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.x = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("y");
            let field_path = __terrane_document_child_path(path, "y");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:28:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.y = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Waypoint {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Waypoint> for DocumentDecodable {
    fn from(value: Waypoint) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Route {
    pub name: String,
    pub points: terrane_collection_support::List<Waypoint>,
    pub active: bool,
}
impl Route {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from(""),
            points: terrane_collection_support::List::<
                Waypoint,
            >::new(vec![Waypoint::terrane_construct()]),
            active: false,
        }
    }
}
impl TerraneDocumentDecode for Route {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "route",
                source,
                "tests/integration/typed-document-decoding/case.trn:30:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["name", "points", "active"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "route",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-document-decoding/case.trn:30:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("name");
            let field_path = __terrane_document_child_path(path, "name");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:31:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("points");
            let field_path = __terrane_document_child_path(path, "points");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    Waypoint,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:32:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.points = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("active");
            let field_path = __terrane_document_child_path(path, "active");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <bool as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-decoding/case.trn:33:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.active = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Route {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Route> for DocumentDecodable {
    fn from(value: Route) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_typed_document_decoding() {
    let decoded: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let malformed: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let invalid: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let permissive: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let yaml: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let journey: TerraneDocumentDecodeOutcome<Route>;
    decoded = {
        let __terrane_document_source = &String::from(
            "{\"serviceName\":\"api\",\"endpoint\":{\"host\":\"localhost\",\"port\":8443},\"ports\":[80,443],\"labels\":{\"tier\":\"edge\"},\"coordinates\":[4,9],\"ratio\":0.1}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            ServiceConfig,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || ServiceConfig::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-document-decoding/case.trn:36:15",
            "service-config",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(decoded.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&decoded.value.clone().service_name),
        terrane_scalar_support::scalar_text(&decoded.value.clone().endpoint.host),
        terrane_scalar_support::scalar_text(&decoded.value.clone().endpoint.port)
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0
        = &decoded.value.clone(); __terrane_raised(__terrane_receiver_value_0.ports
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        390 /* terrane-site: tests/integration/typed-document-decoding/case.trn:38:12-38:34 */)),
        390 /* terrane-site: tests/integration/typed-document-decoding/case.trn:38:12-38:34 */) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_1 = &decoded
        .value.clone(); __terrane_raised(__terrane_receiver_value_1.labels
        .get_or_error(&String::from("tier")), 391 /* terrane-site: tests/integration/typed-document-decoding/case.trn:38:36-38:64 */) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 = &decoded
        .value.clone(); __terrane_raised(__terrane_receiver_value_2.coordinates
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        392 /* terrane-site: tests/integration/typed-document-decoding/case.trn:38:66-38:94 */)),
        392 /* terrane-site: tests/integration/typed-document-decoding/case.trn:38:66-38:94 */) }),
        terrane_scalar_support::scalar_text(&decoded.value.clone().ratio)
    );
    malformed = {
        let __terrane_document_source = &String::from(
            "{\"serviceName\":3,\"endpoint\":{\"host\":false},\"ports\":[1,70000],\"ratio\":0.1,\"extra\":true}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            ServiceConfig,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || ServiceConfig::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-document-decoding/case.trn:40:17",
            "service-config",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(malformed.diagnostics.length() !=
        0)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(malformed
        .diagnostics.clone().length()))
    );
    let __terrane_iterable_0 = malformed.diagnostics.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from(":"), terrane_scalar_support::scalar_text(&diagnostic.reason
            .to_owned())
        );
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_3 =
        &{ let __terrane_receiver_value_4 = &malformed.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_4
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        393 /* terrane-site: tests/integration/typed-document-decoding/case.trn:44:13-44:37 */)),
        393 /* terrane-site: tests/integration/typed-document-decoding/case.trn:44:13-44:37 */) } .source
        .clone(); let __terrane_string_argument_0 = "case.trn";
        __terrane_receiver_value_3.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_5 = &{ let
        __terrane_receiver_value_6 = &malformed.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_6
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        394 /* terrane-site: tests/integration/typed-document-decoding/case.trn:44:69-44:93 */)),
        394 /* terrane-site: tests/integration/typed-document-decoding/case.trn:44:69-44:93 */) }
        .field_source.clone(); let __terrane_string_argument_0 = "case.trn";
        __terrane_receiver_value_5.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    invalid = {
        let __terrane_document_source = &String::from(
            "{\"serviceName\":\"reserved\",\"endpoint\":{\"host\":\"ok\"}}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            ServiceConfig,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || ServiceConfig::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-document-decoding/case.trn:46:15",
            "service-config",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_7 =
        &invalid.diagnostics.clone(); __terrane_raised(__terrane_receiver_value_7
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        395 /* terrane-site: tests/integration/typed-document-decoding/case.trn:47:28-47:50 */)),
        395 /* terrane-site: tests/integration/typed-document-decoding/case.trn:47:28-47:50 */) } .reason
        .to_owned()), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_8 = &invalid.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_8
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        396 /* terrane-site: tests/integration/typed-document-decoding/case.trn:47:59-47:81 */)),
        396 /* terrane-site: tests/integration/typed-document-decoding/case.trn:47:59-47:81 */) } .message
        .clone())
    );
    permissive = {
        let __terrane_document_source = &String::from(
            "{\"serviceName\":\"extra\",\"endpoint\":{\"host\":\"ok\"},\"extra\":true}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = true;
        __terrane_decode_document::<
            ServiceConfig,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || ServiceConfig::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-document-decoding/case.trn:49:18",
            "service-config",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(permissive.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&permissive.value.clone().endpoint.port)
    );
    yaml = {
        let __terrane_document_source = &String::from(
            "serviceName: yaml\nendpoint:\n  host: local",
        );
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            ServiceConfig,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || ServiceConfig::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-document-decoding/case.trn:52:12",
            "service-config",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        terrane_scalar_support::scalar_text(&yaml.value.clone().service_name),
        terrane_scalar_support::scalar_text(&yaml.value.clone().endpoint.host)
    );
    journey = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"coast\",\"points\":[{\"x\":1.25,\"y\":2.75},{\"x\":3.5,\"y\":4.5}],\"active\":true}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Route,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Route::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-document-decoding/case.trn:55:15",
            "route",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(journey.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&journey.value.clone().name),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_9 = &journey
        .value.clone(); __terrane_raised(__terrane_receiver_value_9.points
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        397 /* terrane-site: tests/integration/typed-document-decoding/case.trn:56:48-56:71 */)),
        397 /* terrane-site: tests/integration/typed-document-decoding/case.trn:56:48-56:71 */) } .y),
        terrane_scalar_support::scalar_text(&journey.value.clone().active)
    );
}
// Source: tests/integration/typed-document-encoding/case.trn
// Namespace: typed-document-encoding
#[derive(Clone)]
pub struct TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 {
    pub display: String,
    pub count: terrane_int_support::Int,
}
impl TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {
            display: String::from(""),
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "child",
                source,
                "tests/integration/typed-document-encoding/case.trn:8:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["displayName", "count"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "child",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-document-encoding/case.trn:8:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("displayName");
            let field_path = __terrane_document_child_path(path, "displayName");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:9:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.display = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("count");
            let field_path = __terrane_document_child_path(path, "count");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:10:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.count = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("displayName")?;
        TerraneDocumentEncode::terrane_encode_document(&self.display, writer)?;
        writer.key("count")?;
        TerraneDocumentEncode::terrane_encode_document(&self.count, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Payload {
    pub title: String,
    pub child: TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64,
    pub enabled: bool,
    pub scores: terrane_collection_support::List<terrane_int_support::Int>,
    pub tags: terrane_collection_support::Map<String, String>,
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub note: Option<String>,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            title: String::from(""),
            child: TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64::terrane_construct(),
            enabled: false,
            scores: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            tags: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            note: None,
        }
    }
}
impl TerraneDocumentDecode for Payload {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "payload",
                source,
                "tests/integration/typed-document-encoding/case.trn:12:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "title",
            "child",
            "enabled",
            "scores",
            "tags",
            "pair",
            "note",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "payload",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-document-encoding/case.trn:12:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("title");
            let field_path = __terrane_document_child_path(path, "title");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:13:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.title = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("child");
            let field_path = __terrane_document_child_path(path, "child");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:14:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.child = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("enabled");
            let field_path = __terrane_document_child_path(path, "enabled");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <bool as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:15:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.enabled = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("scores");
            let field_path = __terrane_document_child_path(path, "scores");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:16:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.scores = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("tags");
            let field_path = __terrane_document_child_path(path, "tags");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:17:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.tags = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("pair");
            let field_path = __terrane_document_child_path(path, "pair");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:18:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("note");
            let field_path = __terrane_document_child_path(path, "note");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-document-encoding/case.trn:19:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.note = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode for Payload {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("title")?;
        TerraneDocumentEncode::terrane_encode_document(&self.title, writer)?;
        writer.key("child")?;
        TerraneDocumentEncode::terrane_encode_document(&self.child, writer)?;
        writer.key("enabled")?;
        TerraneDocumentEncode::terrane_encode_document(&self.enabled, writer)?;
        writer.key("scores")?;
        TerraneDocumentEncode::terrane_encode_document(&self.scores, writer)?;
        writer.key("tags")?;
        TerraneDocumentEncode::terrane_encode_document(&self.tags, writer)?;
        writer.key("pair")?;
        TerraneDocumentEncode::terrane_encode_document(&self.pair, writer)?;
        writer.key("note")?;
        TerraneDocumentEncode::terrane_encode_document(&self.note, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Payload {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Payload> for DocumentEncodable {
    fn from(value: Payload) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol for Payload {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Payload> for DocumentDecodable {
    fn from(value: Payload) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_typed_document_encoding() {
    let mut value: Payload;
    let mut nested: TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64;
    let json: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    value = Payload::terrane_construct();
    {
        let __terrane_value: String = String::from("sample");
        value.title = __terrane_value;
    }
    nested = TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64::terrane_construct();
    {
        let __terrane_value: String = String::from("nested");
        nested.display = __terrane_value;
    }
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            4_i128,
        );
        nested.count = __terrane_value;
    }
    {
        let __terrane_value: TerraneNsTypedDocumentEncodingChildIdentity2F74797065642D646F63756D656E742D656E636F64696E67006368696C64 = nested;
        value.child = __terrane_value;
    }
    {
        let __terrane_value: bool = true;
        value.enabled = __terrane_value;
    }
    {
        let __terrane_value: terrane_collection_support::List<
            terrane_int_support::Int,
        > = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(7_i128),
                terrane_int_support::Int::from(9_i128)
            ],
        );
        value.scores = __terrane_value;
    }
    {
        let __terrane_value: terrane_collection_support::Map<String, String> = terrane_collection_support::Map::<
            String,
            String,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, String
                >::new(String::from("tier"), String::from("edge"))
            ],
        );
        value.tags = __terrane_value;
    }
    {
        let __terrane_value: terrane_collection_support::Tuple<
            terrane_int_support::Int,
        > = terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(3_i128)
            ],
        );
        value.pair = __terrane_value;
    }
    json = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-document-encoding/case.trn:32:12",
            "payload",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(json.diagnostics.length() != 0)),
        String::from(" "), terrane_scalar_support::scalar_text(&json.value.clone())
    );
    yaml = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-document-encoding/case.trn:34:12",
            "payload",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from(" "), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
}
// Source: tests/integration/typed-float-output/case.trn
// Namespace: typed-float-output
fn test_scenario_terrane_typed_float_output() {
    let large: f64;
    let small: f64;
    let options: JsonOptions;
    let encoded_large: TerraneDocumentDecodeOutcome<String>;
    let decoded_large: TerraneDocumentDecodeOutcome<f64>;
    let encoded_small: TerraneDocumentDecodeOutcome<String>;
    let decoded_small: TerraneDocumentDecodeOutcome<f64>;
    large = 1.0e300_f64;
    small = f64::from_bits(1);
    options = default_json_options();
    encoded_large = {
        let __terrane_document_source = &large;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-float-output/case.trn:9:21",
            "float64",
        )
    };
    decoded_large = {
        let __terrane_receiver_value_0 = &encoded_large.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_0;
        let __terrane_document_fallback = &large;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f64,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-float-output/case.trn:10:21",
            "float64",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(encoded_large.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_string_support::length(&encoded_large
        .value.clone()) as i128) < terrane_int_support::Int::from(32_i128))),
        String::from("|"), terrane_scalar_support::scalar_text(&(decoded_large
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(decoded_large.value.clone() == large))
    );
    encoded_small = {
        let __terrane_document_source = &small;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-float-output/case.trn:12:21",
            "float64",
        )
    };
    decoded_small = {
        let __terrane_receiver_value_1 = &encoded_small.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_1;
        let __terrane_document_fallback = &small;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f64,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-float-output/case.trn:13:21",
            "float64",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(encoded_small.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_string_support::length(&encoded_small
        .value.clone()) as i128) < terrane_int_support::Int::from(32_i128))),
        String::from("|"), terrane_scalar_support::scalar_text(&(decoded_small
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(decoded_small.value.clone() == small))
    );
}
// Source: tests/integration/typed-json-boundaries/case.trn
// Namespace: typed-json-boundaries
#[derive(Clone)]
pub struct PolicyError {
    pub message: String,
}
impl PolicyError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("policy-error"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn unknown_policy() -> Result<bool, TerraneError> {
    return Err({
        let value = PolicyError::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(7)),
            message,
            rendered,

                398 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:13:5-13:33 */,
        )
    });
}
#[derive(Clone)]
pub struct Numbers {
    pub signed: i128,
    pub unsigned: u128,
    pub adaptive: terrane_int_support::Int,
    pub integral: i64,
    pub real: f64,
    pub narrow: f32,
    pub optional: Option<i64>,
}
impl Numbers {
    pub fn terrane_construct() -> Self {
        Self {
            signed: 0,
            unsigned: 0,
            adaptive: terrane_int_support::Int::from(0_i128),
            integral: 0,
            real: 0.0_f64,
            narrow: 0.0_f32,
            optional: None,
        }
    }
}
impl TerraneDocumentDecode for Numbers {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "numbers",
                source,
                "tests/integration/typed-json-boundaries/case.trn:15:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "signed",
            "unsigned",
            "adaptive",
            "integral",
            "real",
            "narrow",
            "optional",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "numbers",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-json-boundaries/case.trn:15:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("signed");
            let field_path = __terrane_document_child_path(path, "signed");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <i128 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:16:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.signed = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("unsigned");
            let field_path = __terrane_document_child_path(path, "unsigned");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <u128 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:17:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.unsigned = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("adaptive");
            let field_path = __terrane_document_child_path(path, "adaptive");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:18:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.adaptive = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("integral");
            let field_path = __terrane_document_child_path(path, "integral");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <i64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:19:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.integral = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("real");
            let field_path = __terrane_document_child_path(path, "real");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:20:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.real = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("narrow");
            let field_path = __terrane_document_child_path(path, "narrow");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f32 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:21:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.narrow = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("optional");
            let field_path = __terrane_document_child_path(path, "optional");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Option<
                    i64,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:22:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.optional = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Numbers {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Numbers> for DocumentDecodable {
    fn from(value: Numbers) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D {
    pub amount: i64,
}
impl TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D {
    pub fn terrane_construct() -> Self {
        Self { amount: 0 }
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "item",
                source,
                "tests/integration/typed-json-boundaries/case.trn:24:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["amount"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "item",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-json-boundaries/case.trn:24:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("amount");
            let field_path = __terrane_document_child_path(path, "amount");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <i64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:25:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.amount = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065 {
    pub children: terrane_collection_support::List<
        TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
    >,
    pub keyed: terrane_collection_support::Map<
        String,
        TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
    >,
    pub pair: terrane_collection_support::Tuple<i64>,
}
impl TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065 {
    pub fn terrane_construct() -> Self {
        Self {
            children: terrane_collection_support::List::<
                TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
            >::new(Vec::new()),
            keyed: terrane_collection_support::Map::<
                String,
                TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
            >::new(Vec::new()),
            pair: terrane_collection_support::Tuple::<i64>::new(vec![0, 0]),
        }
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065 {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "envelope",
                source,
                "tests/integration/typed-json-boundaries/case.trn:27:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["odd.key", "keyed", "pair"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "envelope",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-json-boundaries/case.trn:27:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("odd.key");
            let field_path = __terrane_document_child_path(path, "odd.key");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:28:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.children = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("keyed");
            let field_path = __terrane_document_child_path(path, "keyed");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    TerraneNsTypedJsonBoundariesItemIdentity2F74797065642D6A736F6E2D626F756E646172696573006974656D,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:29:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.keyed = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("pair");
            let field_path = __terrane_document_child_path(path, "pair");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    i64,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-json-boundaries/case.trn:30:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065 {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_typed_json_boundaries() {
    let options: JsonOptions;
    let good: TerraneDocumentDecodeOutcome<Numbers>;
    let bad: TerraneDocumentDecodeOutcome<Numbers>;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let nested: TerraneDocumentDecodeOutcome<
        TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065,
    >;
    let duplicate: TerraneDocumentDecodeOutcome<Numbers>;
    let trailing: TerraneDocumentDecodeOutcome<Numbers>;
    let too_deep: TerraneDocumentDecodeOutcome<
        TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065,
    >;
    let too_large: TerraneDocumentDecodeOutcome<Numbers>;
    let source: bool;
    let text: String;
    let rejected: TerraneDocumentDecodeOutcome<Numbers>;
    let unknown: TerraneDocumentDecodeOutcome<Numbers>;
    let mut decoded: Option<TerraneDocumentDecodeOutcome<Numbers>> = None;
    let mut error: Option<TerraneError> = None;
    options = default_json_options();
    good = {
        let __terrane_document_source = &String::from(
            "{\"signed\":-170141183460469231731687303715884105728,\"unsigned\":340282366920938463463374607431768211455,\"adaptive\":1e40,\"integral\":1.2e2,\"real\":1e308,\"narrow\":3.4e38,\"optional\":null}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:34:12",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().signed));
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().unsigned));
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().integral));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(good.value.clone().real > 1.0 &&good
        .value.clone().narrow > 0.0_f32))
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &good.value.clone()
        .optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    bad = {
        let __terrane_document_source = &String::from(
            "{\"signed\":-170141183460469231731687303715884105729,\"unsigned\":340282366920938463463374607431768211456,\"adaptive\":1.5,\"real\":1e309,\"narrow\":3.5e38,\"optional\":false}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:41:11",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(bad
        .diagnostics.clone().length()))
    );
    let __terrane_iterable_0 = bad.diagnostics.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic
            .actual_kind.clone()), String::from("|"),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    nested = {
        let __terrane_document_source = &String::from(
            "{\"odd.key\":[{\"amount\":\"no\"}],\"keyed\":{\"a.b\":{\"amount\":false}},\"pair\":[\"bad\"],\"spare\":true}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:45:14",
            "envelope",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    let __terrane_iterable_1 = nested.diagnostics.clone();
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        diagnostic = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic
            .actual_kind.clone()), String::from("|"),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    duplicate = {
        let __terrane_document_source = &String::from("{\"integral\":1,\"integral\":2}");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:48:17",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    trailing = {
        let __terrane_document_source = &String::from("{\"integral\":1} true");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:49:16",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    too_deep = {
        let __terrane_document_source = &String::from("{\"odd.key\":[{\"amount\":1}]}");
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(128_i128),
        );
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedJsonBoundariesEnvelopeIdentity2F74797065642D6A736F6E2D626F756E64617269657300656E76656C6F7065::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:50:16",
            "envelope",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    too_large = {
        let __terrane_document_source = &String::from("{\"integral\":12345}");
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(8_i128),
        );
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:51:17",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(duplicate.diagnostics.length() != 0
        &&{ let __terrane_receiver_value_0 = &duplicate.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        399 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:33-52:57 */)),
        399 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:33-52:57 */) } .reason
        .to_owned().as_str() == "parse" &&trailing.diagnostics.length() != 0 &&{ let
        __terrane_receiver_value_1 = &trailing.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        400 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:100-52:123 */)),
        400 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:100-52:123 */) } .reason
        .to_owned().as_str() == "parse" &&too_deep.diagnostics.length() != 0 &&{ let
        __terrane_receiver_value_2 = &too_deep.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_2
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        401 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:166-52:189 */)),
        401 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:166-52:189 */) } .reason
        .to_owned().as_str() == "parse" &&too_large.diagnostics.length() != 0 &&{ let
        __terrane_receiver_value_3 = &too_large.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        402 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:233-52:257 */)),
        402 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:52:233-52:257 */) } .reason
        .to_owned().as_str() == "parse"))
    );
    source = true;
    text = String::from("{\"integral\":7,\"future\":{\"anything\":[1,2,3]}}");
    rejected = {
        let __terrane_document_source = &text;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:55:16",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    unknown = {
        let __terrane_document_source = &text;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = source;
        __terrane_decode_document::<
            Numbers,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Numbers::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-json-boundaries/case.trn:56:15",
            "numbers",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(rejected.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&unknown.value.clone().integral)
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = decoded
                .insert({
                    let __terrane_document_source = &String::from("not json");
                    let __terrane_document_options = &options;
                    let __terrane_document_allow_unknown = __terrane_traced_completion!(
                        unknown_policy(), 403 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:59:69-59:84 */
                    );
                    __terrane_decode_document::<
                        Numbers,
                    >(
                        terrane_document_support::parse_json_document(
                            __terrane_document_source,
                            terrane_limit(&__terrane_document_options.max_depth),
                            terrane_limit(&__terrane_document_options.max_bytes),
                        ),
                        || Numbers::terrane_construct(),
                        __terrane_document_allow_unknown,
                        "tests/integration/typed-json-boundaries/case.trn:59:19",
                        "numbers",
                        &TerraneDocumentShape {
                            tuple_length: None,
                            inner: None,
                        },
                    )
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_receiver_value_4 = &decoded.as_ref()
                .expect("flow-proven available binding").clone().diagnostics.clone();
                __terrane_raised_completion!(__terrane_receiver_value_4
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                404 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:60:16-60:38 */)),
                404 /* terrane-site: tests/integration/typed-json-boundaries/case.trn:60:16-60:38 */) }
                .reason.to_owned())
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
                        == TerraneErrorKind::Custom(DescriptorId(7))
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_5 = error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_5);
                    };
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
// Source: tests/integration/typed-serialization-boundaries/case.trn
// Namespace: typed-serialization-boundaries
#[derive(Clone)]
pub struct TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    pub value: String,
}
impl TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    pub fn terrane_construct() -> Self {
        Self { value: String::from("") }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("value")?;
        TerraneDocumentEncode::terrane_encode_document(&self.value, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    pub children: terrane_collection_support::List<
        TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
    >,
    pub name: String,
}
impl TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    pub fn terrane_construct() -> Self {
        Self {
            children: terrane_collection_support::List::<
                TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
            >::new(
                vec![
                    TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64::terrane_construct()
                ],
            ),
            name: String::from(""),
        }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("children")?;
        TerraneDocumentEncode::terrane_encode_document(&self.children, writer)?;
        writer.key("marker-like")?;
        TerraneDocumentEncode::terrane_encode_document(&self.name, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_boundaries() {
    let mut nested: TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64;
    let mut value: TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264;
    let normal: TerraneDocumentDecodeOutcome<String>;
    let shallow: TerraneDocumentDecodeOutcome<String>;
    let tiny: TerraneDocumentDecodeOutcome<String>;
    let marker: DocumentResult;
    let oversized: DocumentResult;
    let rendered: DocumentResult;
    let scientific: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    let expanded: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    let rounded: TerraneDocumentDecodeOutcome<f32>;
    let underflow: TerraneDocumentDecodeOutcome<f32>;
    let nonfinite: f64;
    let rejected: TerraneDocumentDecodeOutcome<String>;
    nested = TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64::terrane_construct();
    {
        let __terrane_value: String = String::from("x");
        nested.value = __terrane_value;
    }
    value = TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264::terrane_construct();
    {
        let __terrane_value: terrane_collection_support::List<
            TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
        > = terrane_collection_support::List::<
            TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
        >::new(vec![nested]);
        value.children = __terrane_value;
    }
    {
        let __terrane_value: String = String::from("user-data");
        value.name = __terrane_value;
    }
    normal = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:20:14",
            "record",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(normal.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&normal.value
        .clone())
    );
    shallow = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(1024_i128),
        );
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:22:15",
            "record",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(shallow.diagnostics.length() != 0))
    );
    tiny = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(1_i128),
        );
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:24:12",
            "record",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(tiny.diagnostics.length() != 0))
    );
    marker = parse_json(
        String::from(
            "{\"$serde_json::private::Number\":\"ordinary\",\"value\":123456789012345678901234567890}",
        ),
        default_json_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&marker.failed), String::from("|"),
        terrane_scalar_support::scalar_text(&marker.value.encoded)
    );
    oversized = parse_json(String::from("1e100001"), default_json_options());
    println!("{}", terrane_scalar_support::scalar_text(&oversized.failed));
    rendered = stringify_json(
        marker.value.clone(),
        JsonOptions::terrane_construct(
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(8_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&rendered.failed));
    scientific = {
        let __terrane_document_source = &String::from("1e+3");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_int_support::Int,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || terrane_int_support::Int::from(0_i128),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:32:18",
            "int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(scientific.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&scientific.value
        .clone())
    );
    expanded = {
        let __terrane_document_source = &String::from("1e100000");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_int_support::Int,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || terrane_int_support::Int::from(0_i128),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:34:16",
            "int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(expanded.diagnostics.length() != 0))
    );
    rounded = {
        let __terrane_document_source = &String::from(
            "1.000000059604644775390625000000000001",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f32,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <f32>::default(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:36:15",
            "float32",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(rounded.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&(rounded.value
        .clone() > 1.0_f32))
    );
    underflow = {
        let __terrane_document_source = &String::from("1e-100000");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f32,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <f32>::default(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:38:17",
            "float32",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(underflow.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&(underflow.value
        .clone() == 0.0_f32))
    );
    nonfinite = f64::INFINITY;
    rejected = {
        let __terrane_document_source = &nonfinite;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:42:16",
            "float64",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(rejected.diagnostics.length() != 0))
    );
}
// Source: tests/integration/typed-serialization-custom/case.trn
// Namespace: typed-serialization-custom
#[derive(Clone)]
pub struct Code {
    pub hidden: String,
    pub text: String,
}
impl Code {
    pub fn terrane_construct() -> Self {
        Self {
            hidden: String::from("must-not-leak"),
            text: String::from("public-code"),
        }
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(self.text.clone());
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return parse_json(value.scalar.clone(), default_json_options());
    }
}
impl TerraneDocumentEncode for Code {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        terrane_document_support::write_document_value(writer, &self.to_document().raw)
    }
}
impl SerializableProtocol for Code {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Code::to_document(&*self)
    }
}
impl From<Code> for Serializable {
    fn from(value: Code) -> Self {
        Self(Box::new(value))
    }
}
impl DeserializableProtocol for Code {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        Code::from_document(&*self, value)
    }
}
impl From<Code> for Deserializable {
    fn from(value: Code) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065 {
    pub code: Code,
}
impl TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065 {
    pub fn terrane_construct() -> Self {
        Self {
            code: Code::terrane_construct(),
        }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("code")?;
        TerraneDocumentEncode::terrane_encode_document(&self.code, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_custom() {
    let custom: Code;
    let direct: TerraneDocumentDecodeOutcome<String>;
    let wrapped: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    let decoded: DocumentResult;
    let from_yaml: DocumentResult;
    custom = Code::terrane_construct();
    direct = {
        let __terrane_document_source = &custom;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-custom/case.trn:19:14",
            "code",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(direct.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&direct.value
        .clone())
    );
    wrapped = {
        let __terrane_document_source = &TerraneNsTypedSerializationCustomEnvelopeIdentity2F74797065642D73657269616C697A6174696F6E2D637573746F6D00656E76656C6F7065::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-custom/case.trn:21:15",
            "envelope",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(wrapped.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&wrapped.value
        .clone())
    );
    yaml = {
        let __terrane_document_source = &custom;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-custom/case.trn:23:12",
            "code",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
    decoded = decode_json(
        String::from("\"7\""),
        <Deserializable>::from(custom.clone()),
        default_json_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&decoded.failed),
        String::from("|"), terrane_scalar_support::scalar_text(&decoded.value.scalar)
    );
    from_yaml = decode_yaml(
        String::from("\"9\""),
        <Deserializable>::from(custom),
        default_yaml_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&from_yaml.failed),
        String::from("|"), terrane_scalar_support::scalar_text(&from_yaml.value.scalar)
    );
}
// Source: tests/integration/typed-serialization-destinations/case.trn
// Namespace: typed-serialization-destinations
fn test_destinations() {
    let options: JsonOptions;
    let scalar: TerraneDocumentDecodeOutcome<String>;
    let numbers: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let keys: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Map<String, terrane_int_support::Int>,
    >;
    let pair: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let wrong: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let prototype: Option<String>;
    let nulled: TerraneDocumentDecodeOutcome<Option<String>>;
    let present: TerraneDocumentDecodeOutcome<Option<String>>;
    let invalid: TerraneDocumentDecodeOutcome<Option<String>>;
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let encoded_yaml: TerraneDocumentDecodeOutcome<String>;
    let encoded_none: TerraneDocumentDecodeOutcome<String>;
    options = default_json_options();
    scalar = {
        let __terrane_document_source = &String::from("\"hello\"");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            String,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <String>::default(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:8:14",
            "string",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(scalar.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&scalar.value
        .clone())
    );
    numbers = {
        let __terrane_document_source = &String::from("[3,5]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![terrane_int_support::Int::from(0_i128)]);
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:10:15",
            "list of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(numbers.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_0 = &numbers.value.clone();
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        405 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:33-11:49 */)),
        405 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:33-11:49 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &numbers.value.clone();
        __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        406 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:56-11:72 */)),
        406 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:56-11:72 */) })
    );
    keys = {
        let __terrane_document_source = &String::from("left: 9");
        let __terrane_document_fallback = &terrane_collection_support::Map::<
            String,
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, terrane_int_support::Int
                >::new(String::from(""), terrane_int_support::Int::from(0_i128))
            ],
        );
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Map<String, terrane_int_support::Int>,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:12:12",
            "map of string, int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(keys.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_2 = &keys.value.clone();
        __terrane_raised(__terrane_receiver_value_2.get_or_error(&String::from("left")),
        407 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:13:30-13:48 */) })
    );
    pair = {
        let __terrane_document_source = &String::from("[4,6]");
        let __terrane_document_fallback = &terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:14:12",
            "tuple of int",
            &TerraneDocumentShape {
                tuple_length: Some(2),
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(pair.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_3 = &pair.value.clone();
        __terrane_raised(__terrane_receiver_value_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        408 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:30-15:43 */)),
        408 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:30-15:43 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_4 = &pair.value.clone();
        __terrane_raised(__terrane_receiver_value_4
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        409 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:50-15:63 */)),
        409 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:50-15:63 */) })
    );
    wrong = {
        let __terrane_document_source = &String::from("[4]");
        let __terrane_document_fallback = &terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:16:13",
            "tuple of int",
            &TerraneDocumentShape {
                tuple_length: Some(2),
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(wrong.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_5 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_5
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        410 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:31-17:45 */)),
        410 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:31-17:45 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_6 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_6
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        411 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:52-17:66 */)),
        411 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:52-17:66 */) })
    );
    prototype = Some(String::from("fallback"));
    nulled = {
        let __terrane_document_source = &String::from("null");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:19:14",
            "string|none",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nulled.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &nulled
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    present = {
        let __terrane_document_source = &String::from("\"present\"");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:21:15",
            "string|none",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(present.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &present
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    invalid = {
        let __terrane_document_source = &String::from("false");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:23:15",
            "string|none",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &invalid
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    encoded = {
        let __terrane_document_source = &terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(5_i128)
            ],
        );
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-destinations/case.trn:25:15",
            "list of int",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded.value
        .clone())
    );
    encoded_yaml = {
        let __terrane_document_source = &terrane_collection_support::Map::<
            String,
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, terrane_int_support::Int
                >::new(String::from("left"), terrane_int_support::Int::from(9_i128))
            ],
        );
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-destinations/case.trn:27:20",
            "map of string, int",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded_yaml.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded_yaml
        .value.clone())
    );
    encoded_none = {
        let __terrane_receiver_value_7 = &nulled.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_7;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-destinations/case.trn:29:20",
            "string|none",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded_none.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded_none
        .value.clone())
    );
}
// Source: tests/integration/typed-serialization-generic/case.trn
// Namespace: typed-serialization-generic
#[derive(Clone)]
pub struct TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub count: terrane_int_support::Int,
}
impl<
    TerraneType54,
> TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl<TerraneType54> TerraneDocumentDecode
for TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
    TerraneType54,
> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "holder",
                source,
                "tests/integration/typed-serialization-generic/case.trn:4:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["count"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "holder",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-serialization-generic/case.trn:4:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("count");
            let field_path = __terrane_document_child_path(path, "count");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-generic/case.trn:5:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.count = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl<TerraneType54> TerraneDocumentEncode
for TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
    TerraneType54,
> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("count")?;
        TerraneDocumentEncode::terrane_encode_document(&self.count, writer)?;
        writer.end_object()
    }
}
impl<TerraneType54> DocumentEncodableProtocol
for TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
    TerraneType54,
>
where
    TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
        TerraneType54,
    >: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl<
    TerraneType54,
> From<
    TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
        TerraneType54,
    >,
> for DocumentEncodable
where
    TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
        TerraneType54,
    >: Clone + Send + Sync + 'static,
{
    fn from(
        value: TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
            TerraneType54,
        >,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl<TerraneType54> DocumentDecodableProtocol
for TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
    TerraneType54,
>
where
    TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
        TerraneType54,
    >: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl<
    TerraneType54,
> From<
    TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
        TerraneType54,
    >,
> for DocumentDecodable
where
    TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
        TerraneType54,
    >: Clone + Send + Sync + 'static,
{
    fn from(
        value: TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
            TerraneType54,
        >,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_generic_terrane_typed_serialization_generic() {
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
            String,
        >,
    >;
    encoded = {
        let __terrane_document_source = &TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572::<
            String,
        >::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-generic/case.trn:7:15",
            "holder of string",
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&encoded.value.clone()));
    decoded = {
        let __terrane_document_source = &String::from("{\"count\":9}");
        let __terrane_document_fallback = &TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572::<
            String,
        >::terrane_construct();
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationGenericHolderIdentity2F74797065642D73657269616C697A6174696F6E2D67656E6572696300686F6C646572<
                String,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-generic/case.trn:9:15",
            "holder of string",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&decoded.value.clone().count));
}
// Source: tests/integration/typed-serialization-matrix/case.trn
// Namespace: typed-serialization-matrix
#[derive(Clone)]
pub struct TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D {
    pub display: String,
    pub amount: terrane_int_support::Int,
}
impl TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D {
    pub fn terrane_construct() -> Self {
        Self {
            display: String::from(""),
            amount: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "item",
                source,
                "tests/integration/typed-serialization-matrix/case.trn:8:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["displayName", "amount"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "item",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-serialization-matrix/case.trn:8:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("displayName");
            let field_path = __terrane_document_child_path(path, "displayName");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:9:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.display = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("amount");
            let field_path = __terrane_document_child_path(path, "amount");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:10:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.amount = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("displayName")?;
        TerraneDocumentEncode::terrane_encode_document(&self.display, writer)?;
        writer.key("amount")?;
        TerraneDocumentEncode::terrane_encode_document(&self.amount, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    pub name: String,
    pub item: TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D,
    pub numbers: terrane_collection_support::List<terrane_int_support::Int>,
    pub names: terrane_collection_support::Map<String, String>,
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub optional: Option<String>,
}
impl TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from("default"),
            item: TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D::terrane_construct(),
            numbers: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            names: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            optional: Some(String::from("kept")),
        }
    }
    pub fn validate_document(&self) -> Option<String> {
        if self.name.as_str() == "bad" {
            return Some(String::from("reserved"));
        }
        return None;
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "record",
                source,
                "tests/integration/typed-serialization-matrix/case.trn:12:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "name",
            "item",
            "numbers",
            "names",
            "pair",
            "optional",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "record",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-serialization-matrix/case.trn:12:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("name");
            let field_path = __terrane_document_child_path(path, "name");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:13:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("item");
            let field_path = __terrane_document_child_path(path, "item");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:14:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.item = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("numbers");
            let field_path = __terrane_document_child_path(path, "numbers");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:15:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.numbers = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("names");
            let field_path = __terrane_document_child_path(path, "names");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:16:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.names = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("pair");
            let field_path = __terrane_document_child_path(path, "pair");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:17:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("optional");
            let field_path = __terrane_document_child_path(path, "optional");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:18:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.optional = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() {
            if let Some(message) = value.validate_document() {
                diagnostics
                    .push(
                        __terrane_document_diagnostic(
                            path,
                            "record",
                            "map",
                            "validation",
                            message,
                            source,
                            "tests/integration/typed-serialization-matrix/case.trn:12:1",
                        ),
                    );
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("name")?;
        TerraneDocumentEncode::terrane_encode_document(&self.name, writer)?;
        writer.key("item")?;
        TerraneDocumentEncode::terrane_encode_document(&self.item, writer)?;
        writer.key("numbers")?;
        TerraneDocumentEncode::terrane_encode_document(&self.numbers, writer)?;
        writer.key("names")?;
        TerraneDocumentEncode::terrane_encode_document(&self.names, writer)?;
        writer.key("pair")?;
        TerraneDocumentEncode::terrane_encode_document(&self.pair, writer)?;
        writer.key("optional")?;
        TerraneDocumentEncode::terrane_encode_document(&self.optional, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentValidatableProtocol
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn validate_document(&self) -> Option<String> {
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::validate_document(
            &*self,
        )
    }
}
impl From<
    TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
> for DocumentValidatable {
    fn from(
        value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_matrix() {
    let mut value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264;
    let mut child: TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D;
    let json: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let from_yaml: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let defaults: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let invalid: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let validated: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let nulled: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    value = TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct();
    {
        let __terrane_value: String = String::from("ok");
        value.name = __terrane_value;
    }
    child = TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D::terrane_construct();
    {
        let __terrane_value: String = String::from("nested");
        child.display = __terrane_value;
    }
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            7_i128,
        );
        child.amount = __terrane_value;
    }
    {
        let __terrane_value: TerraneNsTypedSerializationMatrixItemIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978006974656D = child;
        value.item = __terrane_value;
    }
    {
        let __terrane_value: terrane_collection_support::List<
            terrane_int_support::Int,
        > = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(4_i128)
            ],
        );
        value.numbers = __terrane_value;
    }
    {
        let __terrane_value: terrane_collection_support::Map<String, String> = terrane_collection_support::Map::<
            String,
            String,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, String
                >::new(String::from("zone"), String::from("west"))
            ],
        );
        value.names = __terrane_value;
    }
    {
        let __terrane_value: terrane_collection_support::Tuple<
            terrane_int_support::Int,
        > = terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(9_i128)
            ],
        );
        value.pair = __terrane_value;
    }
    json = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-matrix/case.trn:34:12",
            "record",
        )
    };
    yaml = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-matrix/case.trn:35:12",
            "record",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(json.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&json.value.clone())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
    decoded = {
        let __terrane_receiver_value_0 = &json.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_0;
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:38:15",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(decoded
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&decoded.value.clone().name),
        String::from("|"), terrane_scalar_support::scalar_text(&decoded.value.clone()
        .item.display), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_1.numbers
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        412 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:91-39:115 */)),
        412 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:91-39:115 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_2 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_2.names
        .get_or_error(&String::from("zone")), 413 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:122-39:149 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_3 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_3.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        414 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:156-39:177 */)),
        414 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:156-39:177 */) }),
        String::from("|"), { let mut __terrane_text = String::new(); match &decoded.value
        .clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    from_yaml = {
        let __terrane_receiver_value_4 = &yaml.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_4;
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:40:17",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(from_yaml.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&from_yaml.value.clone().name),
        String::from("|"), terrane_scalar_support::scalar_text(&from_yaml.value.clone()
        .item.amount)
    );
    defaults = {
        let __terrane_document_source = &String::from(
            "{\"item\":{\"displayName\":\"x\",\"amount\":1},\"unknown\":2}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = true;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:42:16",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(defaults.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&defaults.value.clone().name),
        String::from("|"), { let mut __terrane_text = String::new(); match &defaults
        .value.clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text },
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_5 = &defaults.value.clone();
        __terrane_raised(__terrane_receiver_value_5.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        415 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:43:90-43:112 */)),
        415 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:43:90-43:112 */) })
    );
    invalid = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"bad\",\"item\":{\"displayName\":false,\"amount\":\"x\"},\"numbers\":[1,\"x\"],\"pair\":[1]}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:44:15",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(invalid
        .diagnostics.clone().length()))
    );
    let __terrane_iterable_0 = invalid.diagnostics.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic.reason
            .to_owned())
        );
    }
    validated = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"bad\",\"optional\":null}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:48:17",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(validated.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(validated
        .diagnostics.clone().length()))
    );
    nulled = {
        let __terrane_document_source = &String::from("{\"optional\":null}");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:50:14",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nulled.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &nulled
        .value.clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
}
// Source: tests/integration/typed-tuple-decode-shapes/case.trn
// Namespace: typed-tuple-decode-shapes
#[derive(Clone)]
pub struct TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264 {
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub rows: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >,
}
impl TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264 {
    pub fn terrane_construct() -> Self {
        Self {
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(0_i128)
                ],
            ),
            rows: terrane_collection_support::List::<
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >::new(
                vec![
                    terrane_collection_support::Tuple::< terrane_int_support::Int
                    >::new(vec![terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(0_i128)])
                ],
            ),
        }
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264 {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "record",
                source,
                "tests/integration/typed-tuple-decode-shapes/case.trn:5:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["pair", "rows"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "record",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-tuple-decode-shapes/case.trn:5:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("pair");
            let field_path = __terrane_document_child_path(path, "pair");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-tuple-decode-shapes/case.trn:6:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("rows");
            let field_path = __terrane_document_child_path(path, "rows");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    terrane_collection_support::Tuple<terrane_int_support::Int>,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-tuple-decode-shapes/case.trn:7:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: Some(
                                    &TerraneDocumentShape {
                                        tuple_length: None,
                                        inner: None,
                                    },
                                ),
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.rows = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_typed_tuple_decode_shapes() {
    let options: JsonOptions;
    let correct: TerraneDocumentDecodeOutcome<
        TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264,
    >;
    let wrong: TerraneDocumentDecodeOutcome<
        TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264,
    >;
    let nested: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >,
    >;
    let keyed: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Map<
            String,
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >,
    >;
    let accepted: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >,
    >;
    options = default_json_options();
    correct = {
        let __terrane_document_source = &String::from(
            "{\"pair\":[3,4],\"rows\":[[5,6]]}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-tuple-decode-shapes/case.trn:10:15",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(correct.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_0 = &correct.value.clone();
        __terrane_raised(__terrane_receiver_value_0.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        416 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:11:33-11:54 */)),
        416 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:11:33-11:54 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &correct.value.clone(); let __terrane_index_2 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        417 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:11:61-11:82 */); let
        __terrane_index_place_3 = __terrane_raised(__terrane_receiver_value_1.rows
        .get(__terrane_index_2).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        417 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:11:61-11:82 */);
        __terrane_raised(__terrane_index_place_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        418 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:11:61-11:85 */)),
        418 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:11:61-11:85 */) })
    );
    wrong = {
        let __terrane_document_source = &String::from(
            "{\"pair\":[1],\"rows\":[[1,2,3]]}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedTupleDecodeShapesRecordIdentity2F74797065642D7475706C652D6465636F64652D736861706573007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-tuple-decode-shapes/case.trn:12:13",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(wrong.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(wrong.value
        .clone().pair.length())), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_receiver_value_4 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_4.rows
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        419 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:14:61-14:80 */)),
        419 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:14:61-14:80 */) }
        .length()))
    );
    nested = {
        let __terrane_document_source = &String::from("[[1,2,3]]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >::new(
            vec![
                terrane_collection_support::Tuple::< terrane_int_support::Int
                >::new(vec![terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)])
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-tuple-decode-shapes/case.trn:15:14",
            "list of tuple of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nested.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_receiver_value_5 = &nested.value.clone();
        __terrane_raised(__terrane_receiver_value_5
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        420 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:17:32-17:47 */)),
        420 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:17:32-17:47 */) }
        .length()))
    );
    keyed = {
        let __terrane_document_source = &String::from("{\"a\":[1]}");
        let __terrane_document_fallback = &terrane_collection_support::Map::<
            String,
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >::new(
            vec![
                terrane_collection_support::Entry::< String,
                terrane_collection_support::Tuple < terrane_int_support::Int
                >>::new(String::from(""), terrane_collection_support::Tuple::<
                terrane_int_support::Int
                >::new(vec![terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)]))
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Map<
                String,
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-tuple-decode-shapes/case.trn:18:13",
            "map of string, tuple of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: Some(2),
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(keyed.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_6 = &keyed.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_6
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        421 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:19:31-19:51 */)),
        421 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:19:31-19:51 */) } .path
        .clone()), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_7 = &keyed.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_7
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        422 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:19:63-19:83 */)),
        422 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:19:63-19:83 */) } .reason
        .to_owned())
    );
    accepted = {
        let __terrane_document_source = &String::from("[[4,6]]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >::new(
            vec![
                terrane_collection_support::Tuple::< terrane_int_support::Int
                >::new(vec![terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)])
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-tuple-decode-shapes/case.trn:20:16",
            "list of tuple of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(accepted.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_8 = &accepted.value.clone(); let __terrane_index_9 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        423 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:21:34-21:51 */); let
        __terrane_index_place_10 = __terrane_raised(__terrane_receiver_value_8
        .get(__terrane_index_9).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_9)),
        423 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:21:34-21:51 */);
        __terrane_raised(__terrane_index_place_10
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        424 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:21:34-21:54 */)),
        424 /* terrane-site: tests/integration/typed-tuple-decode-shapes/case.trn:21:34-21:54 */) })
    );
}
// Source: tests/integration/union-carrier-assignment/case.trn
// Namespace: union-carrier-assignment
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF133S73 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF133S73 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF133S146 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF133S146 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn observe_terrane_union_carrier_assignment(use_number: bool) {
    let mut original: TerraneUnionF133S73;
    let mut destination: TerraneUnionF133S146;
    original = TerraneUnionF133S73::Arm1(String::from("alpha"));
    if use_number {
        original = TerraneUnionF133S73::Arm0(terrane_int_support::Int::from(7_i128));
    }
    destination = TerraneUnionF133S146::Arm1(String::from("old"));
    let _ = &destination;
    destination = match (&original).clone() {
        TerraneUnionF133S73::Arm0(value) => TerraneUnionF133S146::Arm0(value),
        TerraneUnionF133S73::Arm1(value) => TerraneUnionF133S146::Arm1(value),
    };
    original = TerraneUnionF133S73::Arm1(String::from("changed"));
    if matches!(&&destination, TerraneUnionF133S146::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &destination {
            TerraneUnionF133S146::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&* match &destination {
            TerraneUnionF133S146::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            &terrane_int_support::Int::from(1_i128)))
        );
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &original {
        TerraneUnionF133S73::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF133S73::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn test_scenario_terrane_union_carrier_assignment() {
    observe_terrane_union_carrier_assignment(false);
    observe_terrane_union_carrier_assignment(true);
}
// Source: tests/integration/url-values/case.trn
// Namespace: conformance/url-values
fn test_scenario_terrane_conformance_url_values() {
    let parsed: UrlResult;
    let value: Url;
    let relative: UrlResult;
    parsed = parse_url(
        String::from("https://user:pass@bücher.example:443/a?x=1&x=2#f"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&parsed.failed));
    value = parsed.value.clone();
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&value.scheme),
        terrane_scalar_support::scalar_text(&value.host),
        terrane_scalar_support::scalar_text(&value.port),
        terrane_scalar_support::scalar_text(&value.path)
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.string()));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.username),
        terrane_scalar_support::scalar_text(&value.password)
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.query.count));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.query.get(String::from("x"))),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.query
        .get_all(String::from("x")).length()))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.fragment),
        terrane_scalar_support::scalar_text(&value.origin)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        "user:pass@"; value.serialized.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        "user:pass@"; value.display.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    relative = value.resolve(String::from("../b?q=hello%20world"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&relative.value.host),
        terrane_scalar_support::scalar_text(&relative.value.path),
        terrane_scalar_support::scalar_text(&relative.value.query.get(String::from("q")))
    );
}
// Source: tests/integration/value-reassignment/case.trn
// Namespace: value-reassignment
fn test_scenario_terrane_value_reassignment() {
    let mut value: terrane_int_support::Int;
    let next: i64;
    value = terrane_int_support::Int::from(1_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
    next = 2;
    value = terrane_int_support::Int::from(next as i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
    value = terrane_int_support::Int::from(3_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            test_scenario_terrane_adaptive_fixed_comparison();
        }
        "1" => {
            test_scenario_terrane_adaptive_union_field();
        }
        "2" => {
            test_scenario_terrane_borrowed_scalar_comparisons();
        }
        "3" => {
            test_scenario_terrane_bounded_arithmetic_families();
        }
        "4" => {
            test_scenario_terrane_bounded_class_field_capture();
        }
        "5" => {
            test_scenario_terrane_bounded_list_capture();
        }
        "6" => {
            test_scenario_terrane_bounded_local_shared_alias();
        }
        "7" => {
            test_scenario_terrane_bounded_local_shared_list();
        }
        "8" => {
            test_scenario_terrane_bounded_local_shared_reuse();
        }
        "9" => {
            test_scenario_terrane_bounded_optional_capture();
        }
        "10" => {
            test_scenario_terrane_bounded_owner_capture();
        }
        "11" => {
            test_scenario_terrane_bounded_shared_capture();
        }
        "12" => {
            test_scenario_terrane_bounded_two_interfaces();
        }
        "13" => {
            if let Err(error) = test_scenario_terrane_byte_index_and_slice() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "14" => {
            if let Err(error) = test_scenario_terrane_bytes_and_legacy_digests() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "15" => {
            if let Err(error) = test_scenario_terrane_bytes_variadic_concat() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "16" => {
            if let Err(error) = test_scenario_terrane_bytes_views_encoding() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "17" => {
            test_scenario_terrane_call_callable_alias();
        }
        "18" => {
            test_scenario_terrane_call_list_borrow_contents();
        }
        "19" => {
            test_scenario_terrane_call_string_alias_borrow();
        }
        "20" => {
            test_scenario_terrane_call_string_different_owners();
        }
        "21" => {
            test_scenario_terrane_call_string_field_borrow();
        }
        "22" => {
            test_scenario_terrane_call_string_later_read();
        }
        "23" => {
            test_scenario_terrane_call_string_named_borrow();
        }
        "24" => {
            test_scenario_terrane_call_typed_callable();
        }
        "25" => {
            test_scenario_terrane_callable_exact_result_control();
        }
        "26" => {
            test_scenario_terrane_callable_field_alias();
        }
        "27" => {
            test_scenario_terrane_callable_field_direct();
        }
        "28" => {
            test_scenario_terrane_callable_field_reference();
        }
        "29" => {
            test_scenario_terrane_callable_field_this();
        }
        "30" => {
            test_scenario_terrane_canonical_union_alias();
        }
        "31" => {
            if let Err(error) = test_values_terrane_checked_coercion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "32" => {
            test_scenario_terrane_collection_identity_lifetime();
        }
        "33" => {
            test_scenario_terrane_collection_lookup_errors();
        }
        "34" => {
            test_scenario_terrane_collection_property_get_call();
        }
        "35" => {
            test_scenario_terrane_collection_property_method_calls();
        }
        "36" => {
            test_scenario_terrane_collections_value_semantics();
        }
        "37" => {
            test_scenario_terrane_complete_float_surface();
        }
        "38" => {
            test_scenario_terrane_concrete_consuming_local_capture();
        }
        "39" => {
            test_scenario_terrane_concrete_returned_shared_capture();
        }
        "40" => {
            test_scenario_terrane_concrete_two_capture_class();
        }
        "41" => {
            test_scenario_terrane_concrete_two_capture_string();
        }
        "42" => {
            test_scenario_terrane_consuming_callable_field_take();
        }
        "43" => {
            test_scenario_terrane_consuming_method_reference_snapshot();
        }
        "44" => {
            if let Err(error) = test_scenario_terrane_contextual_constant_arithmetic() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "45" => {
            test_scenario_terrane_contextual_floating_literals();
        }
        "46" => {
            if let Err(error) = test_scenario_terrane_core_regex() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "47" => {
            test_scenario_terrane_declared_union_reaching_arm();
        }
        "48" => {
            test_scenario_terrane_destination_empty_collections();
        }
        "49" => {
            test_scenario_terrane_exact_constant_membership();
        }
        "50" => {
            test_scenario_terrane_exact_float_narrowing();
        }
        "51" => {
            if let Err(error) = test_values_terrane_explicit_numeric_coercions() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "52" => {
            test_scenario_terrane_fixed_integer_float_destination();
        }
        "53" => {
            test_scenario_terrane_float_narrowing_special_values();
        }
        "54" => {
            test_scenario_terrane_foundational_float_math();
        }
        "55" => {
            if let Err(error) = test_scenario_terrane_function_value_name_collisions() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "56" => {
            test_scenario_terrane_generic_optional_owned_return();
        }
        "57" => {
            test_scenario_terrane_generic_owned_parameter();
        }
        "58" => {
            test_scenario_terrane_generic_return_borrowed_twice();
        }
        "59" => {
            test_scenario_terrane_generic_return_owned_once();
        }
        "60" => {
            __terrane_run(async move {
                if let Err(error) = test_scenario_terrane_initialized_async_callable_field()
                    .await
                {
                    let skipped = error.descriptor_name() == "/core/testing::test-skip";
                    __terrane_test_record(
                        if skipped { "skipped" } else { "failed" },
                        &error,
                    );
                    std::process::exit(if skipped { 102 } else { 101 });
                }
            });
        }
        "61" => {
            test_scenario_terrane_initialized_callable_field_alias();
        }
        "62" => {
            if let Err(error) = test_values_terrane_integer_coercions() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "63" => {
            if let Err(error) = test_scenario_terrane_integer_euclidean() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "64" => {
            if let Err(error) = test_scenario_terrane_integer_move() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "65" => {
            if let Err(error) = test_scenario_terrane_interpolation_key_reference_destinations() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "66" => {
            test_scenario_terrane_iterator_protocol();
        }
        "67" => {
            __terrane_run(async move {
                test_scenario_terrane_layout_calls_async().await;
            });
        }
        "68" => {
            test_scenario_terrane_list_ref_then_owned_local();
        }
        "69" => {
            test_scenario_terrane_list_ref_then_owned();
        }
        "70" => {
            test_scenario_terrane_local_collection_capture();
        }
        "71" => {
            test_scenario_terrane_local_loop_callback_capture();
        }
        "72" => {
            test_scenario_terrane_local_scalar_union();
        }
        "73" => {
            test_scenario_terrane_local_shared_discarded_argument();
        }
        "74" => {
            test_scenario_terrane_local_union_alias_carrier();
        }
        "75" => {
            test_scenario_terrane_local_union_compatible_boundary();
        }
        "76" => {
            test_scenario_terrane_logical_comparisons();
        }
        "77" => {
            test_scenario_terrane_lossy_float_narrowing();
        }
        "78" => {
            test_scenario_terrane_map_entry_destructuring();
        }
        "79" => {
            if let Err(error) = test_scenario_terrane_map_key_removal() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "80" => {
            test_values_terrane_mixed_width_arithmetic();
        }
        "81" => {
            test_values_terrane_mixed_width_comparisons();
        }
        "82" => {
            test_scenario_terrane_mutable_receiver_field_snapshot();
        }
        "83" => {
            test_scenario_terrane_mutable_reference_capture_reuse();
        }
        "84" => {
            __terrane_run(async move {
                test_scenario_terrane_narrowed_optional_async_callable().await;
            });
        }
        "85" => {
            test_scenario_terrane_narrowed_optional_callable();
        }
        "86" => {
            test_scenario_terrane_narrowed_optional_consuming_callable();
        }
        "87" => {
            test_scenario_terrane_narrowed_optional_mutable_callable();
        }
        "88" => {
            test_scenario_terrane_narrowed_optional_throwing_callable();
        }
        "89" => {
            test_scenario_terrane_negative_wrapped_shift();
        }
        "90" => {
            test_scenario_terrane_nested_call_and_receiver_loans();
        }
        "91" => {
            test_scenario_terrane_none_value_display();
        }
        "92" => {
            if let Err(error) = test_scenario_terrane_null_byte_literal() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "93" => {
            if let Err(error) = test_values_terrane_numeric_destination_conversion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "94" => {
            test_values_terrane_numeric_literals();
        }
        "95" => {
            test_scenario_terrane_optional_enum_payload();
        }
        "96" => {
            test_scenario_terrane_optional_none_guard_flow();
        }
        "97" => {
            test_scenario_terrane_optional_scalar_union();
        }
        "98" => {
            test_scenario_terrane_owned_consuming_string_capture();
        }
        "99" => {
            test_scenario_terrane_owned_copy_collection_capture();
        }
        "100" => {
            test_scenario_terrane_parse_radix_families();
        }
        "101" => {
            test_scenario_terrane_pipeline_borrowed_callbacks();
        }
        "102" => {
            if let Err(error) = test_scenario_terrane_quoted_interpolation() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "103" => {
            if let Err(error) = test_renderer_bindings() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "104" => {
            __terrane_run(async move {
                test_scenario_terrane_required_async_callable_field().await;
            });
        }
        "105" => {
            test_scenario_terrane_required_mutable_callable_field();
        }
        "106" => {
            if let Err(error) = test_scenario_terrane_required_throwing_callable_field() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "107" => {
            if let Err(error) = test_scenario_terrane_scalar_reference_transparency() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "108" => {
            if let Err(error) = test_scenario_terrane_sequential_pipeline() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "109" => {
            test_scenario_terrane_shared_reference_capture_reassign();
        }
        "110" => {
            test_scenario_terrane_stable_list_sorting();
        }
        "111" => {
            if let Err(error) = test_composition() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "112" => {
            test_generic_terrane_string_display_generic();
        }
        "113" => {
            test_scenario_terrane_string_ref_then_owned();
        }
        "114" => {
            test_scenario_terrane_string_result_list_iteration();
        }
        "115" => {
            test_scenario_terrane_string_value_parameter_reuse();
        }
        "116" => {
            if let Err(error) = test_scenario_terrane_text_families() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "117" => {
            test_scenario_terrane_truth_protocol();
        }
        "118" => {
            test_scenario_terrane_tuple_type_boundaries();
        }
        "119" => {
            if let Err(error) = test_scenario_terrane_typed_catch_bindings() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "120" => {
            test_scenario_terrane_typed_destinations();
        }
        "121" => {
            test_scenario_terrane_typed_document_decoding();
        }
        "122" => {
            test_scenario_terrane_typed_document_encoding();
        }
        "123" => {
            test_scenario_terrane_typed_float_output();
        }
        "124" => {
            test_scenario_terrane_typed_json_boundaries();
        }
        "125" => {
            test_boundaries();
        }
        "126" => {
            test_custom();
        }
        "127" => {
            test_destinations();
        }
        "128" => {
            test_generic_terrane_typed_serialization_generic();
        }
        "129" => {
            test_matrix();
        }
        "130" => {
            test_scenario_terrane_typed_tuple_decode_shapes();
        }
        "131" => {
            test_scenario_terrane_union_carrier_assignment();
        }
        "132" => {
            test_scenario_terrane_conformance_url_values();
        }
        "133" => {
            test_scenario_terrane_value_reassignment();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
