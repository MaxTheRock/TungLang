// Handles arithmetic and logical operators for TungLang
use crate::value::Value;

/// Applies a binary operator to two Values
pub fn apply_operator(left: Value, right: Value, op: &str) -> Value {
    match (left.clone(), right.clone(), op) {
        // Arithmetic - Python-like behavior with auto-promotion to float
        (Value::Number(l), Value::Number(r), "+") => Value::Number(l + r),
        (Value::Number(l), Value::Number(r), "-") => Value::Number(l - r),
        (Value::Number(l), Value::Number(r), "*") => Value::Number(l * r),
        (Value::Number(l), Value::Number(r), "/") => Value::Float(l as f64 / r as f64), // Division always returns float in Python
        (Value::Number(l), Value::Number(r), "//") => Value::Number(l / r), // Floor division
        (Value::Number(l), Value::Number(r), "%") => Value::Number(l % r),
        (Value::Number(l), Value::Number(r), "**") => Value::Float((l as f64).powf(r as f64)), // Exponentiation
        
        // Mixed number and float operations (auto-promotion)
        (Value::Number(l), Value::Float(r), "+") => Value::Float(l as f64 + r),
        (Value::Number(l), Value::Float(r), "-") => Value::Float(l as f64 - r),
        (Value::Number(l), Value::Float(r), "*") => Value::Float(l as f64 * r),
        (Value::Number(l), Value::Float(r), "/") => Value::Float(l as f64 / r),
        (Value::Number(l), Value::Float(r), "//") => Value::Number((l as f64 / r).floor() as i64),
        (Value::Number(l), Value::Float(r), "%") => Value::Float((l as f64) % r),
        (Value::Number(l), Value::Float(r), "**") => Value::Float((l as f64).powf(r)),
        
        (Value::Float(l), Value::Number(r), "+") => Value::Float(l + r as f64),
        (Value::Float(l), Value::Number(r), "-") => Value::Float(l - r as f64),
        (Value::Float(l), Value::Number(r), "*") => Value::Float(l * r as f64),
        (Value::Float(l), Value::Number(r), "/") => Value::Float(l / r as f64),
        (Value::Float(l), Value::Number(r), "//") => Value::Number((l / r as f64).floor() as i64),
        (Value::Float(l), Value::Number(r), "%") => Value::Float(l % r as f64),
        (Value::Float(l), Value::Number(r), "**") => Value::Float(l.powf(r as f64)),
        
        (Value::Float(l), Value::Float(r), "+") => Value::Float(l + r),
        (Value::Float(l), Value::Float(r), "-") => Value::Float(l - r),
        (Value::Float(l), Value::Float(r), "*") => Value::Float(l * r),
        (Value::Float(l), Value::Float(r), "/") => Value::Float(l / r),
        (Value::Float(l), Value::Float(r), "//") => Value::Number((l / r).floor() as i64),
        (Value::Float(l), Value::Float(r), "%") => Value::Float(l % r),
        (Value::Float(l), Value::Float(r), "**") => Value::Float(l.powf(r)),
        // String concatenation and Python-like string operations
        (Value::String(l), Value::String(r), "+") => Value::String(l + &r),
        (Value::String(l), Value::Number(r), "+") => Value::String(l + &r.to_string()),
        (Value::String(l), Value::Float(r), "+") => Value::String(l + &r.to_string()),
        (Value::String(l), Value::Boolean(r), "+") => Value::String(l + &r.to_string()),
        (Value::String(l), Value::Array(r), "+") => Value::String(l + &format!("{:?}", r)),
        (Value::String(l), Value::Dict(r), "+") => Value::String(l + &format!("{:?}", r)),
        (Value::String(l), Value::Undefined, "+") => Value::String(l + "undefined"),
        
        (Value::Number(l), Value::String(r), "+") => Value::String(l.to_string() + &r),
        (Value::Float(l), Value::String(r), "+") => Value::String(l.to_string() + &r),
        (Value::Boolean(l), Value::String(r), "+") => Value::String(l.to_string() + &r),
        (Value::Array(l), Value::String(r), "+") => Value::String(format!("{:?}", l) + &r),
        (Value::Dict(l), Value::String(r), "+") => Value::String(format!("{:?}", l) + &r),
        (Value::Undefined, Value::String(r), "+") => Value::String("undefined".to_string() + &r),
        
        // Python-like string repetition with * operator
        (Value::String(s), Value::Number(n), "*") => {
            if n <= 0 {
                Value::String("".to_string())
            } else {
                Value::String(s.repeat(n as usize))
            }
        },
        (Value::Number(n), Value::String(s), "*") => {
            if n <= 0 {
                Value::String("".to_string())
            } else {
                Value::String(s.repeat(n as usize))
            }
        },
        // Array concatenation and other Python-like array operations
        (Value::Array(mut l), Value::Array(r), "+") => {
            l.extend(r);
            Value::Array(l)
        },
        // Python-like array/item concatenation (adding an item to array)
        (Value::Array(mut l), right, "+") => {
            l.push(right);
            Value::Array(l)
        },
        (left, Value::Array(mut r), "+") => {
            r.insert(0, left);
            Value::Array(r)
        },
        // Python-like array multiplication (repeat arrays)
        (Value::Array(a), Value::Number(n), "*") => {
            if n <= 0 {
                Value::Array(vec![])
            } else {
                let mut result = Vec::new();
                for _ in 0..n {
                    result.extend(a.clone());
                }
                Value::Array(result)
            }
        },
        (Value::Number(n), Value::Array(a), "*") => {
            if n <= 0 {
                Value::Array(vec![])
            } else {
                let mut result = Vec::new();
                for _ in 0..n {
                    result.extend(a.clone());
                }
                Value::Array(result)
            }
        },
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
        // Type conversion for comparison (Python allows comparing different numeric types)
        (Value::Number(l), Value::Float(r), op) 
            if matches!(op, "==" | "!=" | ">" | "<" | ">=" | "<=") => {
            let left = l as f64;
            let result = match op {
                "==" => left == r,
                "!=" => left != r,
                ">" => left > r,
                "<" => left < r,
                ">=" => left >= r,
                "<=" => left <= r,
                _ => unreachable!(),
            };
            Value::Boolean(result)
        },
        (Value::Float(l), Value::Number(r), op)
            if matches!(op, "==" | "!=" | ">" | "<" | ">=" | "<=") => {
            let right = r as f64;
            let result = match op {
                "==" => l == right,
                "!=" => l != right,
                ">" => l > right,
                "<" => l < right,
                ">=" => l >= right,
                "<=" => l <= right,
                _ => unreachable!(),
            };
            Value::Boolean(result)
        },

        // Python-like 'in' operator for arrays and dicts
        (item, Value::Array(arr), "in") => {
            Value::Boolean(arr.contains(&item))
        },
        (Value::String(key), Value::Dict(dict), "in") => {
            Value::Boolean(dict.contains_key(&key))
        },
        
        // Fall through cases
        _ => {
            eprintln!("Error: Unsupported operation '{}' between types {:?} and {:?}", op, left, right.clone());
            Value::Undefined
        }
    }
}
