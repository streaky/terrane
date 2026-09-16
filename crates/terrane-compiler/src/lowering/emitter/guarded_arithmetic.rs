use super::super::prelude::*;
use num_bigint::BigInt;

#[derive(Clone, Copy)]
enum GuardedBranch {
    Then,
    Else,
}

#[derive(Clone)]
struct IntegerInterval {
    lower: BigInt,
    upper: BigInt,
}

struct AffineExpression {
    coefficient: BigInt,
    constant: BigInt,
    valid: IntegerInterval,
}

struct GuardedAssignment<'a> {
    target: &'a SyntaxNode,
    condition: &'a SyntaxNode,
    then_value: &'a SyntaxNode,
    else_value: &'a SyntaxNode,
    guarded_value: &'a SyntaxNode,
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
        let target = self.expression(plan.target);
        let condition = self.control_condition(plan.condition);
        let binding = self
            .local_typed_binding(plan.target)
            .expect("guarded target has a resolved local binding")
            .span;
        let other_value = match plan.guarded_branch {
            GuardedBranch::Then => plan.else_value,
            GuardedBranch::Else => plan.then_value,
        };
        let Some(other_fast) = self.statically_safe_division(other_value, binding, plan.scalar)
        else {
            return false;
        };
        let checked_then = self.expression(plan.then_value);
        let checked_else = self.expression(plan.else_value);
        let guarded_fast = self.unchecked_affine_expression(plan.guarded_value, plan.scalar);
        let (fast_then, fast_else) = match plan.guarded_branch {
            GuardedBranch::Then => (guarded_fast, other_fast),
            GuardedBranch::Else => (other_fast, guarded_fast),
        };
        let guard = integer_interval_guard(&target, plan.scalar, &plan.bounds, &plan.valid);
        let fast_then_name = format!("__terrane_guarded_then_{}", plan.condition.span.start);
        let fast_else_name = format!("__terrane_guarded_else_{}", plan.condition.span.start);
        let mask_name = format!("__terrane_guarded_mask_{}", plan.condition.span.start);
        let scalar_type = rust_type(plan.scalar);
        self.line(&format!(
            "{target} = if {guard} {{ let {fast_then_name} = {fast_then}; \
             let {fast_else_name} = {fast_else}; \
             let {mask_name} = 0_{scalar_type}.wrapping_sub(({condition}) as {scalar_type}); \
             {fast_else_name} ^ (({fast_then_name} ^ {fast_else_name}) & {mask_name}) }} \
             else if {condition} {{ {checked_then} }} else {{ {checked_else} }};"
        ));
        true
    }

    fn guarded_integer_assignment<'a>(
        &self,
        node: &'a SyntaxNode,
    ) -> Option<GuardedAssignment<'a>> {
        if self.debug_information {
            return None;
        }
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
        if binding.span != else_binding.span || self.global_storage(then_target).is_some() {
            return None;
        }
        let ValueType::Scalar(scalar) = binding.value_type else {
            return None;
        };
        let bounds = fixed_integer_bounds(scalar)?;
        let candidate = [
            (GuardedBranch::Else, else_value),
            (GuardedBranch::Then, then_value),
        ]
        .into_iter()
        .find_map(|(branch, value)| {
            let affine = self.affine_expression(value, binding.span, scalar, &bounds)?;
            (affine.coefficient != BigInt::from(0_u8)
                && affine.valid.upper >= BigInt::from(0_u8)
                && (affine.valid.lower > bounds.lower || affine.valid.upper < bounds.upper))
                .then_some((branch, value, affine.valid))
        })?;
        Some(GuardedAssignment {
            target: then_target,
            condition,
            then_value,
            else_value,
            guarded_value: candidate.1,
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

    fn affine_expression(
        &self,
        node: &SyntaxNode,
        binding: crate::Span,
        scalar: ScalarType,
        bounds: &IntegerInterval,
    ) -> Option<AffineExpression> {
        if node.kind == SyntaxKind::Name {
            let resolved = self.local_typed_binding(node)?;
            return (resolved.span == binding).then(|| AffineExpression {
                coefficient: BigInt::from(1_u8),
                constant: BigInt::from(0_u8),
                valid: bounds.clone(),
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
                constant: value,
                valid: bounds.clone(),
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
        let operator = self.source.text()[left.span.end..right.span.start].trim();
        if !matches!(operator, "+" | "-" | "*") {
            return None;
        }
        let left = self.affine_expression(left, binding, scalar, bounds)?;
        let right = self.affine_expression(right, binding, scalar, bounds)?;
        let valid = intersect_intervals(left.valid, right.valid)?;
        let (coefficient, constant) = match operator {
            "+" => (
                &left.coefficient + &right.coefficient,
                &left.constant + &right.constant,
            ),
            "-" => (
                &left.coefficient - &right.coefficient,
                &left.constant - &right.constant,
            ),
            "*" if left.coefficient == BigInt::from(0_u8) => (
                &right.coefficient * &left.constant,
                &right.constant * &left.constant,
            ),
            "*" if right.coefficient == BigInt::from(0_u8) => (
                &left.coefficient * &right.constant,
                &left.constant * &right.constant,
            ),
            _ => return None,
        };
        let operation_valid = affine_range(&coefficient, &constant, bounds)?;
        Some(AffineExpression {
            coefficient,
            constant,
            valid: intersect_intervals(valid, operation_valid)?,
        })
    }

    fn statically_safe_division(
        &mut self,
        node: &SyntaxNode,
        binding: crate::Span,
        scalar: ScalarType,
    ) -> Option<String> {
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
        let operator = self.source.text()[left.span.end..right.span.start].trim();
        if !matches!(operator, "/" | "%") {
            return None;
        }
        let Some(Ok(ContextualConstant::Integer(divisor))) =
            contextual_constant(self.source, right, scalar)
        else {
            return None;
        };
        if divisor == BigInt::from(0_u8) || divisor == BigInt::from(-1_i8) {
            return None;
        }
        let left = self.expression_as(left, ValueType::Scalar(scalar));
        let right = self.expression_as(right, ValueType::Scalar(scalar));
        Some(format!("({left} {operator} {right})"))
    }

    fn unchecked_affine_expression(&mut self, node: &SyntaxNode, scalar: ScalarType) -> String {
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
        {
            let operator = self.source.text()[left.span.end..right.span.start].trim();
            let left = self.unchecked_affine_expression(left, scalar);
            let right = self.unchecked_affine_expression(right, scalar);
            return format!("({left} {operator} {right})");
        }
        format!(
            "({} as {})",
            self.expression_as(node, ValueType::Scalar(scalar)),
            rust_type(scalar)
        )
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

fn integer_interval_guard(
    target: &str,
    scalar: ScalarType,
    bounds: &IntegerInterval,
    valid: &IntegerInterval,
) -> String {
    if let Some(unsigned) = unsigned_rust_type(scalar)
        && valid.lower <= BigInt::from(0_u8)
        && valid.upper >= BigInt::from(0_u8)
    {
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
