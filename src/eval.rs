use crate::value::Value;
use crate::Rule;
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
                    left = match (left, right, op_str) {
                        (Value::Number(l), Value::Number(r), "==") => Value::Number((l == r) as i64),
                        (Value::Number(l), Value::Number(r), "!=") => Value::Number((l != r) as i64),
                        (Value::Number(l), Value::Number(r), ">") => Value::Number((l > r) as i64),
                        (Value::Number(l), Value::Number(r), "<") => Value::Number((l < r) as i64),
                        (Value::Number(l), Value::Number(r), ">=") => Value::Number((l >= r) as i64),
                        (Value::Number(l), Value::Number(r), "<=") => Value::Number((l <= r) as i64),
                        _ => {
                            eprintln!(
                                "Error: Unsupported operation or type in expression: {}",
                                op_str
                            );
                            return Value::Undefined;
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
                    let raw = first.as_str();
                    // Remove leading/trailing quotes if present
                    let stripped = raw
                        .strip_prefix('"')
                        .and_then(|s| s.strip_suffix('"'))
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
                            let prompt: String = if let Some(Value::String(s)) = args.get(0) {
                                s.clone()
                            } else {
                                "".to_string()
                            };
                            print!("{}", prompt);
                            let mut input: String = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            Value::String(input.trim_end().to_string())
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
        _ => Value::Undefined,
    }
}
