use crate::parser::Rule;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn evaluate_expression(pair: Pair<Rule>, variables: &HashMap<String, Value>) -> Value {
    match pair.as_rule() {
        Rule::expression | Rule::comparison | Rule::sum | Rule::term => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let mut left: Value = evaluate_expression(inner.next().unwrap(), variables);
            while let Some(op_pair) = inner.next() {
                let op_str: &str = op_pair.as_str();
                if let Some(right_pair) = inner.next() {
                    let right: Value = evaluate_expression(right_pair, variables);
                    left = match (left, right.clone(), op_str) {
                        // Arithmetic
                        (Value::Number(l), Value::Number(r), "+") => Value::Number(l + r),
                        (Value::Number(l), Value::Number(r), "-") => Value::Number(l - r),
                        (Value::Number(l), Value::Number(r), "*") => Value::Number(l * r),
                        (Value::Number(l), Value::Number(r), "/") => Value::Number(l / r),
                        (Value::Number(l), Value::Number(r), "%") => Value::Number(l % r),
                        // String concatenation
                        (Value::String(l), Value::String(r), "+") => Value::String(l + &r),
                        // Equality
                        (Value::Number(l), Value::Number(r), "==") => Value::Boolean(l == r),
                        (Value::String(l), Value::String(r), "==") => Value::Boolean(l == r),
                        (Value::Boolean(l), Value::Boolean(r), "==") => Value::Boolean(l == r),
                        // Inequality
                        (Value::Number(l), Value::Number(r), "!=") => Value::Boolean(l != r),
                        (Value::String(l), Value::String(r), "!=") => Value::Boolean(l != r),
                        (Value::Boolean(l), Value::Boolean(r), "!=") => Value::Boolean(l != r),
                        // Comparison
                        (Value::Number(l), Value::Number(r), op)
                            if [">", "<", ">=", "<="].contains(&op) =>
                        {
                            let res: bool = match op {
                                ">" => l > r,
                                "<" => l < r,
                                ">=" => l >= r,
                                "<=" => l <= r,
                                _ => unreachable!(),
                            };
                            Value::Boolean(res)
                        }
                        (Value::String(l), Value::String(r), op)
                            if [">", "<", ">=", "<="].contains(&op) =>
                        {
                            let res: bool = match op {
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
                        (Value::Boolean(l), _, "!") => Value::Boolean(!l),
                        (Value::Number(l), Value::Undefined, "-") => Value::Number(-l),
                        _ => {
                            eprintln!(
                                "Error: Unsupported operation or type in expression: {}",
                                op_str
                            );
                            Value::Undefined
                        }
                    };
                } else {
                    eprintln!("Error: Operator '{}' without right-hand operand", op_str);
                    return Value::Undefined;
                }
            }
            left
        }
        Rule::factor => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let first: Pair<Rule> = inner.next().unwrap();
            match first.as_rule() {
                Rule::string => {
                    let raw: &str = first.as_str();
                    let stripped: &str = raw
                        .strip_prefix('"')
                        .and_then(|s: &str| s.strip_suffix('"'))
                        .unwrap_or(raw);
                    Value::String(stripped.to_string())
                }
                Rule::number => {
                    let n: i32 = first.as_str().parse::<i32>().unwrap();
                    Value::Number(n as i64)
                }
                Rule::IDENTIFIER => {
                    let ident: &str = first.as_str();
                    if let Some(value) = variables.get(ident) {
                        value.clone()
                    } else {
                        eprintln!("Error: Undefined variable '{}'", ident);
                        Value::Undefined
                    }
                }
                Rule::function_call => {
                    let mut inner: Pairs<Rule> = first.into_inner();
                    let func_name: &str = inner.next().unwrap().as_str();
                    let args: Vec<Value> = inner
                        .map(|arg: Pair<Rule>| evaluate_expression(arg, variables))
                        .collect();
                    match func_name {
                        "input" => {
                            if let Some(Value::String(prompt)) = args.get(0) {
                                print!("{}", prompt);
                                use std::io::Write;
                                std::io::stdout().flush().unwrap();
                            }
                            let mut input: String = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            let input: &str = input.trim_end_matches(['\n', '\r']);
                            if let Ok(n) = input.parse::<i64>() {
                                Value::Number(n)
                            } else {
                                Value::String(input.to_string())
                            }
                        }
                        "int" => {
                            if let Some(val) = args.get(0) {
                                match val {
                                    Value::Number(n) => Value::Number(*n),
                                    Value::String(s) => s
                                        .parse::<i64>()
                                        .map(Value::Number)
                                        .unwrap_or(Value::Undefined),
                                    _ => Value::Undefined,
                                }
                            } else {
                                Value::Undefined
                            }
                        }
                        "str" => {
                            if let Some(val) = args.get(0) {
                                match val {
                                    Value::String(s) => Value::String(s.clone()),
                                    Value::Number(n) => Value::String(n.to_string()),
                                    Value::Boolean(b) => Value::String(b.to_string()),
                                    Value::Undefined => Value::String("undefined".to_string()),
                                }
                            } else {
                                Value::Undefined
                            }
                        }
                        _ => {
                            eprintln!("Unknown function: {}", func_name);
                            Value::Undefined
                        }
                    }
                }
                _ => Value::Undefined,
            }
        }
        Rule::while_statement => {
            // While statements are handled in the interpreter, not as expressions
            Value::Undefined
        }
        _ => Value::Undefined,
    }
}
