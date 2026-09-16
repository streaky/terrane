use super::super::prelude::*;
use num_bigint::BigInt;

#[derive(Clone, Copy)]
enum GuardedBranch {
    Then,
    Else,
}

#[derive(Clone, Copy)]
enum AffineOperator {
    Add,
    Subtract,
    Multiply,
}

impl AffineOperator {
    fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
        }
    }
}

#[derive(Clone)]
enum AffineRender<'a> {
    Binding(&'a SyntaxNode),
    Constant(BigInt),
    Binary {
        operator: AffineOperator,
        left: Box<AffineRender<'a>>,
        right: Box<AffineRender<'a>>,
    },
}

#[derive(Clone, Copy)]
enum SafeDivisionOperator {
    Divide,
    Remainder,
}

impl SafeDivisionOperator {
    fn symbol(self) -> &'static str {
        match self {
            Self::Divide => "/",
            Self::Remainder => "%",
        }
    }
}

struct SafeDivision<'a> {
    left: &'a SyntaxNode,
    divisor: BigInt,
    operator: SafeDivisionOperator,
}

#[derive(Clone)]
struct IntegerInterval {
    lower: BigInt,
    upper: BigInt,
}

struct AffineExpression<'a> {
    coefficient: BigInt,
    constant: BigInt,
    valid: IntegerInterval,
    render: AffineRender<'a>,
}

struct GuardedAssignment<'a> {
    target: &'a SyntaxNode,
    condition: &'a SyntaxNode,
    then_value: &'a SyntaxNode,
    else_value: &'a SyntaxNode,
    guarded_render: AffineRender<'a>,
    other_fast: SafeDivision<'a>,
    guarded_branch: GuardedBranch,
    scalar: ScalarType,
    bounds: IntegerInterval,
    valid: IntegerInterval,
}

impl Emitter<'_> {
    pub(super) fn emit_guarded_integer_assignment(&mut self, node: &SyntaxNode) -> bool {
        let Some(plan) = self.guarded_integer_assignment(node) else {
            return false;
        };
        debug_assert!(
            plan.valid.lower >= BigInt::from(0_u8),
            "raw native division and remainder require a nonnegative fast region"
        );
        let target = self.expression(plan.target);
        let condition = self.control_condition(plan.condition);
        let checked_then = self.expression(plan.then_value);
        let checked_else = self.expression(plan.else_value);
        let guarded_fast = self.render_affine(&plan.guarded_render, plan.scalar);
        let other_fast = self.render_safe_division(&plan.other_fast, plan.scalar);
        let (fast_then, fast_else) = match plan.guarded_branch {
            GuardedBranch::Then => (guarded_fast, other_fast),
            GuardedBranch::Else => (other_fast, guarded_fast),
        };
        let guard = integer_interval_guard(&target, plan.scalar, &plan.bounds, &plan.valid);
        let scalar_type = rust_type(plan.scalar);
        self.line(&format!(
            "{target} = if {guard} {{ let __terrane_guarded_then = {fast_then}; \
             let __terrane_guarded_else = {fast_else}; \
             let __terrane_guarded_mask = 0_{scalar_type}.wrapping_sub(({condition}) as {scalar_type}); \
             __terrane_guarded_else ^ ((__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask) }} \
             else if {condition} {{ {checked_then} }} else {{ {checked_else} }};"
        ));
        true
    }

    fn guarded_integer_assignment<'a>(
        &self,
        node: &'a SyntaxNode,
    ) -> Option<GuardedAssignment<'a>> {
        let [condition, then_block, else_clause] = node.children.as_slice() else {
            return None;
        };
        if !pure_guard_condition(condition) {
            return None;
        }
        let [else_block] = else_clause.children.as_slice() else {
            return None;
        };
        let (then_target, then_value) = Self::single_assignment(then_block)?;
        let (else_target, else_value) = Self::single_assignment(else_block)?;
        if then_target.kind != SyntaxKind::Name || else_target.kind != SyntaxKind::Name {
            return None;
        }
        let binding = self.local_typed_binding(then_target)?;
        let else_binding = self.local_typed_binding(else_target)?;
        if binding.span != else_binding.span
            || self.global_storage(then_target).is_some()
            || self.reference_backed(binding)
            || self.async_mutable_captures.contains(self.text(then_target))
        {
            return None;
        }
        let ValueType::Scalar(scalar) = binding.value_type else {
            return None;
        };
        let bounds = fixed_integer_bounds(scalar)?;
        let candidate = [
            (GuardedBranch::Else, else_value, then_value),
            (GuardedBranch::Then, then_value, else_value),
        ]
        .into_iter()
        .find_map(|(branch, guarded_value, other_value)| {
            let affine = self.affine_expression(guarded_value, binding.span, scalar, &bounds)?;
            if affine.coefficient == BigInt::from(0_u8)
                || (affine.valid.lower <= bounds.lower && affine.valid.upper >= bounds.upper)
            {
                return None;
            }
            let valid = effective_fast_interval(scalar, affine.valid)?;
            let other_fast = self.safe_division(other_value, binding.span, scalar)?;
            Some((branch, affine.render, valid, other_fast))
        })?;
        Some(GuardedAssignment {
            target: then_target,
            condition,
            then_value,
            else_value,
            guarded_render: candidate.1,
            other_fast: candidate.3,
            guarded_branch: candidate.0,
            scalar,
            bounds,
            valid: candidate.2,
        })
    }

    fn single_assignment(block: &SyntaxNode) -> Option<(&SyntaxNode, &SyntaxNode)> {
        let [assignment] = block.children.as_slice() else {
            return None;
        };
        if assignment.kind != SyntaxKind::Assignment {
            return None;
        }
        let [target, value] = assignment.children.as_slice() else {
            return None;
        };
        Some((target, value))
    }

    fn affine_expression<'a>(
        &self,
        node: &'a SyntaxNode,
        binding: crate::Span,
        scalar: ScalarType,
        bounds: &IntegerInterval,
    ) -> Option<AffineExpression<'a>> {
        if node.kind == SyntaxKind::Name {
            let resolved = self.local_typed_binding(node)?;
            return (resolved.span == binding).then(|| AffineExpression {
                coefficient: BigInt::from(1_u8),
                constant: BigInt::from(0_u8),
                valid: bounds.clone(),
                render: AffineRender::Binding(node),
            });
        }
        if let Some(Ok(ContextualConstant::Integer(value))) =
            contextual_constant(self.source, node, scalar)
        {
            if value < bounds.lower || value > bounds.upper {
                return None;
            }
            return Some(AffineExpression {
                coefficient: BigInt::from(0_u8),
                constant: value.clone(),
                valid: bounds.clone(),
                render: AffineRender::Constant(value),
            });
        }
        let [left, right] = node.children.as_slice() else {
            return None;
        };
        if node.kind != SyntaxKind::BinaryExpression
            || self.value_type(node) != Some(ValueType::Scalar(scalar))
        {
            return None;
        }
        let operator = match self.source.text()[left.span.end..right.span.start].trim() {
            "+" => AffineOperator::Add,
            "-" => AffineOperator::Subtract,
            "*" => AffineOperator::Multiply,
            _ => return None,
        };
        let left = self.affine_expression(left, binding, scalar, bounds)?;
        let right = self.affine_expression(right, binding, scalar, bounds)?;
        let valid = intersect_intervals(left.valid, right.valid)?;
        let (coefficient, constant) = match operator {
            AffineOperator::Add => (
                &left.coefficient + &right.coefficient,
                &left.constant + &right.constant,
            ),
            AffineOperator::Subtract => (
                &left.coefficient - &right.coefficient,
                &left.constant - &right.constant,
            ),
            AffineOperator::Multiply if left.coefficient == BigInt::from(0_u8) => (
                &right.coefficient * &left.constant,
                &right.constant * &left.constant,
            ),
            AffineOperator::Multiply if right.coefficient == BigInt::from(0_u8) => (
                &left.coefficient * &right.constant,
                &left.constant * &right.constant,
            ),
            AffineOperator::Multiply => return None,
        };
        let operation_valid = affine_range(&coefficient, &constant, bounds)?;
        Some(AffineExpression {
            coefficient,
            constant,
            valid: intersect_intervals(valid, operation_valid)?,
            render: AffineRender::Binary {
                operator,
                left: Box::new(left.render),
                right: Box::new(right.render),
            },
        })
    }

    fn safe_division<'a>(
        &self,
        node: &'a SyntaxNode,
        binding: crate::Span,
        scalar: ScalarType,
    ) -> Option<SafeDivision<'a>> {
        let [left, right] = node.children.as_slice() else {
            return None;
        };
        if node.kind != SyntaxKind::BinaryExpression
            || self.value_type(node) != Some(ValueType::Scalar(scalar))
            || left.kind != SyntaxKind::Name
            || self.local_typed_binding(left)?.span != binding
        {
            return None;
        }
        let operator = match self.source.text()[left.span.end..right.span.start].trim() {
            "/" => SafeDivisionOperator::Divide,
            "%" => SafeDivisionOperator::Remainder,
            _ => return None,
        };
        let Some(Ok(ContextualConstant::Integer(divisor))) =
            contextual_constant(self.source, right, scalar)
        else {
            return None;
        };
        if divisor == BigInt::from(0_u8) || divisor == BigInt::from(-1_i8) {
            return None;
        }
        Some(SafeDivision {
            left,
            divisor,
            operator,
        })
    }

    fn render_affine(&mut self, render: &AffineRender<'_>, scalar: ScalarType) -> String {
        match render {
            AffineRender::Binding(node) => format!(
                "({} as {})",
                self.expression_as(node, ValueType::Scalar(scalar)),
                rust_type(scalar)
            ),
            AffineRender::Constant(value) => fixed_integer_literal(value, scalar),
            AffineRender::Binary {
                operator,
                left,
                right,
            } => {
                let left = self.render_affine(left, scalar);
                let right = self.render_affine(right, scalar);
                format!("({left} {} {right})", operator.symbol())
            }
        }
    }

    fn render_safe_division(&mut self, division: &SafeDivision<'_>, scalar: ScalarType) -> String {
        // The planned fast interval is nonnegative. Raw Rust division and remainder
        // are therefore equivalent to Terrane's Euclidean operations in this block.
        let left = self.expression_as(division.left, ValueType::Scalar(scalar));
        let right = fixed_integer_literal(&division.divisor, scalar);
        format!("({left} {} {right})", division.operator.symbol())
    }
}

fn pure_guard_condition(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::Name
            | SyntaxKind::Literal
            | SyntaxKind::UnaryOperator
            | SyntaxKind::BinaryExpression
            | SyntaxKind::UnaryExpression
            | SyntaxKind::GroupExpression
    ) && node.children.iter().all(pure_guard_condition)
}

fn effective_fast_interval(scalar: ScalarType, valid: IntegerInterval) -> Option<IntegerInterval> {
    let lower = valid.lower.max(BigInt::from(0_u8));
    let effective = IntegerInterval {
        lower,
        upper: valid.upper,
    };
    if unsigned_rust_type(scalar).is_some() {
        debug_assert!(effective.lower >= BigInt::from(0_u8));
    }
    (effective.lower <= effective.upper).then_some(effective)
}

fn integer_interval_guard(
    target: &str,
    scalar: ScalarType,
    bounds: &IntegerInterval,
    valid: &IntegerInterval,
) -> String {
    debug_assert!(valid.lower >= BigInt::from(0_u8));
    if let Some(unsigned) = unsigned_rust_type(scalar) {
        return format!("({target} as {unsigned}) <= {}_{unsigned}", valid.upper);
    }
    let mut clauses = Vec::new();
    if valid.lower > bounds.lower {
        clauses.push(format!(
            "{target} >= {}",
            integer_literal(&valid.lower, scalar, bounds)
        ));
    }
    if valid.upper < bounds.upper {
        clauses.push(format!(
            "{target} <= {}",
            integer_literal(&valid.upper, scalar, bounds)
        ));
    }
    clauses.join(" && ")
}

fn unsigned_rust_type(scalar: ScalarType) -> Option<&'static str> {
    match scalar {
        ScalarType::Int8 => Some("u8"),
        ScalarType::Int16 => Some("u16"),
        ScalarType::Int32 => Some("u32"),
        ScalarType::Int64 => Some("u64"),
        ScalarType::Int128 => Some("u128"),
        _ => None,
    }
}

fn fixed_integer_literal(value: &BigInt, scalar: ScalarType) -> String {
    let bounds = fixed_integer_bounds(scalar).expect("fixed integer literal has fixed bounds");
    integer_literal(value, scalar, &bounds)
}

fn integer_literal(value: &BigInt, scalar: ScalarType, bounds: &IntegerInterval) -> String {
    let rust_type = rust_type(scalar);
    if value == &bounds.lower {
        format!("{rust_type}::MIN")
    } else if value == &bounds.upper {
        format!("{rust_type}::MAX")
    } else if value < &BigInt::from(0_u8) {
        format!("({value}_{rust_type})")
    } else {
        format!("{value}_{rust_type}")
    }
}

fn intersect_intervals(left: IntegerInterval, right: IntegerInterval) -> Option<IntegerInterval> {
    let lower = left.lower.max(right.lower);
    let upper = left.upper.min(right.upper);
    (lower <= upper).then_some(IntegerInterval { lower, upper })
}

fn affine_range(
    coefficient: &BigInt,
    constant: &BigInt,
    bounds: &IntegerInterval,
) -> Option<IntegerInterval> {
    if coefficient == &BigInt::from(0_u8) {
        return (constant >= &bounds.lower && constant <= &bounds.upper).then(|| bounds.clone());
    }
    let (lower, upper) = if coefficient > &BigInt::from(0_u8) {
        (
            ceil_div(&(&bounds.lower - constant), coefficient),
            floor_div(&(&bounds.upper - constant), coefficient),
        )
    } else {
        let positive = -coefficient;
        (
            ceil_div(&(constant - &bounds.upper), &positive),
            floor_div(&(constant - &bounds.lower), &positive),
        )
    };
    intersect_intervals(bounds.clone(), IntegerInterval { lower, upper })
}

fn floor_div(numerator: &BigInt, positive_denominator: &BigInt) -> BigInt {
    let quotient = numerator / positive_denominator;
    let remainder = numerator % positive_denominator;
    if remainder < BigInt::from(0_u8) {
        quotient - 1_u8
    } else {
        quotient
    }
}

fn ceil_div(numerator: &BigInt, positive_denominator: &BigInt) -> BigInt {
    let quotient = numerator / positive_denominator;
    let remainder = numerator % positive_denominator;
    if remainder > BigInt::from(0_u8) {
        quotient + 1_u8
    } else {
        quotient
    }
}

fn fixed_integer_bounds(scalar: ScalarType) -> Option<IntegerInterval> {
    let (signed, bits) = match scalar {
        ScalarType::Int8 => (true, 8_u16),
        ScalarType::Int16 => (true, 16),
        ScalarType::Int32 => (true, 32),
        ScalarType::Int64 => (true, 64),
        ScalarType::Int128 => (true, 128),
        ScalarType::Uint8 => (false, 8),
        ScalarType::Uint16 => (false, 16),
        ScalarType::Uint32 => (false, 32),
        ScalarType::Uint64 => (false, 64),
        ScalarType::Uint128 => (false, 128),
        _ => return None,
    };
    let magnitude = BigInt::from(1_u8) << usize::from(bits - u16::from(signed));
    let (lower, upper) = if signed {
        (-&magnitude, magnitude - 1_u8)
    } else {
        (BigInt::from(0_u8), magnitude - 1_u8)
    };
    Some(IntegerInterval { lower, upper })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intermediate_bounds_narrow_an_affine_region() {
        let bounds = fixed_integer_bounds(ScalarType::Int8).unwrap();
        let multiplication = affine_range(&BigInt::from(3_u8), &BigInt::from(0_u8), &bounds)
            .expect("3 * value has an int8-safe interval");
        let addition = affine_range(&BigInt::from(3_u8), &BigInt::from(1_u8), &bounds)
            .expect("3 * value + 1 has an int8-safe interval");
        let valid = intersect_intervals(multiplication, addition).unwrap();
        assert_eq!(valid.lower, BigInt::from(-42_i8));
        assert_eq!(valid.upper, BigInt::from(42_i8));
    }

    #[test]
    fn signed_fast_intervals_are_clamped_to_nonnegative_inputs() {
        let valid = IntegerInterval {
            lower: BigInt::from(-42_i8),
            upper: BigInt::from(42_i8),
        };
        let effective = effective_fast_interval(ScalarType::Int8, valid).unwrap();
        assert_eq!(effective.lower, BigInt::from(0_u8));
        assert_eq!(effective.upper, BigInt::from(42_i8));
    }

    #[test]
    fn signed_division_rounds_interval_edges_outward() {
        assert_eq!(
            floor_div(&BigInt::from(-128_i16), &BigInt::from(3_u8)),
            BigInt::from(-43_i8)
        );
        assert_eq!(
            ceil_div(&BigInt::from(-128_i16), &BigInt::from(3_u8)),
            BigInt::from(-42_i8)
        );
    }
}
