// Handles arithmetic and logical operators for TungLang
use crate::value::Value;

/// Applies a binary operator to two Values
pub fn apply_operator(left: Value, right: Value, op: &str) -> Value {
    match (left, right, op) {
        // Arithmetic
        (Value::Number(l), Value::Number(r), "+") => Value::Number(l + r),
        (Value::Number(l), Value::Number(r), "-") => Value::Number(l - r),
        (Value::Number(l), Value::Number(r), "*") => Value::Number(l * r),
        (Value::Number(l), Value::Number(r), "/") => Value::Number(l / r),
        (Value::Number(l), Value::Number(r), "%") => Value::Number(l % r),
        (Value::Float(l), Value::Float(r), "+") => Value::Float(l + r),
        (Value::Float(l), Value::Float(r), "-") => Value::Float(l - r),
        (Value::Float(l), Value::Float(r), "*") => Value::Float(l * r),
        (Value::Float(l), Value::Float(r), "/") => Value::Float(l / r),
        // String concatenation
        (Value::String(l), Value::String(r), "+") => Value::String(l + &r),
        // Equality
        (Value::Number(l), Value::Number(r), "==") => Value::Boolean(l == r),
        (Value::Float(l), Value::Float(r), "==") => Value::Boolean(l == r),
        (Value::String(l), Value::String(r), "==") => Value::Boolean(l == r),
        (Value::Boolean(l), Value::Boolean(r), "==") => Value::Boolean(l == r),
        // Inequality
        (Value::Number(l), Value::Number(r), "!=") => Value::Boolean(l != r),
        (Value::Float(l), Value::Float(r), "!=") => Value::Boolean(l != r),
        (Value::String(l), Value::String(r), "!=") => Value::Boolean(l != r),
        (Value::Boolean(l), Value::Boolean(r), "!=") => Value::Boolean(l != r),
        // Comparison
        (Value::Number(l), Value::Number(r), op)
            if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Value::Boolean(res)
        }
        (Value::Float(l), Value::Float(r), op)
            if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Value::Boolean(res)
        }
        (Value::String(l), Value::String(r), op)
            if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Value::Boolean(res)
        }
        // Logical
        (Value::Boolean(l), Value::Boolean(r), "&&") => Value::Boolean(l && r),
        (Value::Boolean(l), Value::Boolean(r), "||") => Value::Boolean(l || r),
        // Unary
        (Value::Boolean(l), Value::Undefined, "!") => Value::Boolean(!l),
        (Value::Number(l), Value::Undefined, "-") => Value::Number(-l),
        (Value::Float(l), Value::Undefined, "-") => Value::Float(-l),
        (Value::Array(_), _, _) | (Value::Dict(_), _, _) => Value::Undefined,
        _ => {
            eprintln!("Error: Unsupported operation or type in expression: {}", op);
            Value::Undefined
        }
    }
}
