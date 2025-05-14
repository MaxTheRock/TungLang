// Handles the TungLang range() built-in function
use crate::value::Value;

/// Returns an array of numbers from start to end-1
pub fn std_range(args: &[Value]) -> Value {
    let (start, end) = match (args.get(0), args.get(1)) {
        (Some(Value::Number(s)), Some(Value::Number(e))) => (*s, *e),
        (Some(Value::Number(s)), None) => (0, *s),
        _ => (0, 0),
    };
    Value::Array((start..end).map(Value::Number).collect())
}
