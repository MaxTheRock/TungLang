// Handles arithmetic and logical operators for TungLang
use crate::value::Value;
use miette::Result;

/// Applies a binary operator to two Values
pub fn apply_operator(left: Value, right: Value, op: &str) -> Result<Value> {
    match (left.clone(), right.clone(), op) {
        // Arithmetic - Python-like behavior with auto-promotion to float
        (Value::Number(l), Value::Number(r), "+") => Ok(Value::Number(l + r)),
        (Value::Number(l), Value::Number(r), "-") => Ok(Value::Number(l - r)),
        (Value::Number(l), Value::Number(r), "*") => Ok(Value::Number(l * r)),
        (Value::Number(l), Value::Number(r), "/") => Ok(Value::Float(l as f64 / r as f64)), // Division always returns float in Python
        (Value::Number(l), Value::Number(r), "//") => Ok(Value::Number(l / r)), // Floor division
        (Value::Number(l), Value::Number(r), "%") => Ok(Value::Number(l % r)),
        (Value::Number(l), Value::Number(r), "**") => Ok(Value::Float((l as f64).powf(r as f64))), // Exponentiation
        
        // Mixed number and float operations (auto-promotion)
        (Value::Number(l), Value::Float(r), "+") => Ok(Value::Float(l as f64 + r)),
        (Value::Number(l), Value::Float(r), "-") => Ok(Value::Float(l as f64 - r)),
        (Value::Number(l), Value::Float(r), "*") => Ok(Value::Float(l as f64 * r)),
        (Value::Number(l), Value::Float(r), "/") => Ok(Value::Float(l as f64 / r)),
        (Value::Number(l), Value::Float(r), "//") => Ok(Value::Number((l as f64 / r).floor() as i64)),
        (Value::Number(l), Value::Float(r), "%") => Ok(Value::Float((l as f64) % r)),
        (Value::Number(l), Value::Float(r), "**") => Ok(Value::Float((l as f64).powf(r))),
        
        (Value::Float(l), Value::Number(r), "+") => Ok(Value::Float(l + r as f64)),
        (Value::Float(l), Value::Number(r), "-") => Ok(Value::Float(l - r as f64)),
        (Value::Float(l), Value::Number(r), "*") => Ok(Value::Float(l * r as f64)),
        (Value::Float(l), Value::Number(r), "/") => Ok(Value::Float(l / r as f64)),
        (Value::Float(l), Value::Number(r), "//") => Ok(Value::Number((l / r as f64).floor() as i64)),
        (Value::Float(l), Value::Number(r), "%") => Ok(Value::Float(l % r as f64)),
        (Value::Float(l), Value::Number(r), "**") => Ok(Value::Float(l.powf(r as f64))),
        
        (Value::Float(l), Value::Float(r), "+") => Ok(Value::Float(l + r)),
        (Value::Float(l), Value::Float(r), "-") => Ok(Value::Float(l - r)),
        (Value::Float(l), Value::Float(r), "*") => Ok(Value::Float(l * r)),
        (Value::Float(l), Value::Float(r), "/") => Ok(Value::Float(l / r)),
        (Value::Float(l), Value::Float(r), "//") => Ok(Value::Number((l / r).floor() as i64)),
        (Value::Float(l), Value::Float(r), "%") => Ok(Value::Float(l % r)),
        (Value::Float(l), Value::Float(r), "**") => Ok(Value::Float(l.powf(r))),
        // String concatenation and Python-like string operations
        (Value::String(l), Value::String(r), "+") => Ok(Value::String(l + &r)),
        (Value::String(l), Value::Number(r), "+") => Ok(Value::String(l + &r.to_string())),
        (Value::String(l), Value::Float(r), "+") => Ok(Value::String(l + &r.to_string())),
        (Value::String(l), Value::Boolean(r), "+") => Ok(Value::String(l + &r.to_string())),
        (Value::String(l), Value::Array(r), "+") => Ok(Value::String(l + &format!("{:?}", r))),
        (Value::String(l), Value::Dict(r), "+") => Ok(Value::String(l + &format!("{:?}", r))),
        (Value::String(l), Value::Undefined, "+") => Ok(Value::String(l + "undefined")),
        
        (Value::Number(l), Value::String(r), "+") => Ok(Value::String(l.to_string() + &r)),
        (Value::Float(l), Value::String(r), "+") => Ok(Value::String(l.to_string() + &r)),
        (Value::Boolean(l), Value::String(r), "+") => Ok(Value::String(l.to_string() + &r)),
        (Value::Array(l), Value::String(r), "+") => Ok(Value::String(format!("{:?}", l) + &r)),
        (Value::Dict(l), Value::String(r), "+") => Ok(Value::String(format!("{:?}", l) + &r)),
        (Value::Undefined, Value::String(r), "+") => Ok(Value::String("undefined".to_string() + &r)),
        
        // Python-like string repetition with * operator
        (Value::String(s), Value::Number(n), "*") => {
            if n <= 0 {
                Ok(Value::String("".to_string()))
            } else {
                Ok(Value::String(s.repeat(n as usize)))
            }
        },
        (Value::Number(n), Value::String(s), "*") => {
            if n <= 0 {
                Ok(Value::String("".to_string()))
            } else {
                Ok(Value::String(s.repeat(n as usize)))
            }
        },
        // Array concatenation and other Python-like array operations
        (Value::Array(mut l), Value::Array(r), "+") => {
            l.extend(r);
            Ok(Value::Array(l))
        },
        // Python-like array/item concatenation (adding an item to array)
        (Value::Array(mut l), right, "+") => {
            l.push(right);
            Ok(Value::Array(l))
        },
        (left, Value::Array(mut r), "+") => {
            r.insert(0, left);
            Ok(Value::Array(r))
        },
        // Python-like array multiplication (repeat arrays)
        (Value::Array(a), Value::Number(n), "*") => {
            if n <= 0 {
                Ok(Value::Array(vec![]))
            } else {
                let mut result = Vec::new();
                for _ in 0..n {
                    result.extend(a.clone());
                }
                Ok(Value::Array(result))
            }
        },
        (Value::Number(n), Value::Array(a), "*") => {
            if n <= 0 {
                Ok(Value::Array(vec![]))
            } else {
                let mut result = Vec::new();
                for _ in 0..n {
                    result.extend(a.clone());
                }
                Ok(Value::Array(result))
            }
        },
        // Equality
        (Value::Number(l), Value::Number(r), "==") => Ok(Value::Boolean(l == r)),
        (Value::Float(l), Value::Float(r), "==") => Ok(Value::Boolean(l == r)),
        (Value::String(l), Value::String(r), "==") => Ok(Value::Boolean(l == r)),
        (Value::Boolean(l), Value::Boolean(r), "==") => Ok(Value::Boolean(l == r)),
        // Inequality
        (Value::Number(l), Value::Number(r), "!=") => Ok(Value::Boolean(l != r)),
        (Value::Float(l), Value::Float(r), "!=") => Ok(Value::Boolean(l != r)),
        (Value::String(l), Value::String(r), "!=") => Ok(Value::Boolean(l != r)),
        (Value::Boolean(l), Value::Boolean(r), "!=") => Ok(Value::Boolean(l != r)),
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
            Ok(Value::Boolean(res))
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
            Ok(Value::Boolean(res))
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
            Ok(Value::Boolean(res))
        }
        // Logical
        (Value::Boolean(l), Value::Boolean(r), "&&") => Ok(Value::Boolean(l && r)),
        (Value::Boolean(l), Value::Boolean(r), "||") => Ok(Value::Boolean(l || r)),
        // Unary
        (Value::Boolean(l), Value::Undefined, "!") => Ok(Value::Boolean(!l)),
        (Value::Number(l), Value::Undefined, "-") => Ok(Value::Number(-l)),
        (Value::Float(l), Value::Undefined, "-") => Ok(Value::Float(-l)),
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
            Ok(Value::Boolean(result))
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
            Ok(Value::Boolean(result))
        },

        // Python-like 'in' operator for arrays and dicts
        (item, Value::Array(arr), "in") => {
            Ok(Value::Boolean(arr.contains(&item)))
        },
        (Value::String(key), Value::Dict(dict), "in") => {
            Ok(Value::Boolean(dict.contains_key(&key)))
        },
        
        // Python-like 'not in' operator for arrays and dicts
        (item, Value::Array(arr), "!in") => {
            Ok(Value::Boolean(!arr.contains(&item)))
        },
        (Value::String(key), Value::Dict(dict), "!in") => {
            Ok(Value::Boolean(!dict.contains_key(&key)))
        },
        
        // Fall through cases
        _ => {
            Err(miette::miette!(
                "Error: Unsupported operation '{}' between types {:?} and {:?}",
                op, left, right
            ))
        }
    }
}
