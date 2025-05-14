// Handles the TungLang abs() built-in function
use crate::value::Value;

/// Returns the absolute value of a number or float
pub fn std_abs(val: &Value) -> Value {
    match val {
        Value::Number(n) => Value::Number(n.abs()),
        Value::Float(f) => Value::Float(f.abs()),
        _ => Value::Undefined,
    }
}
