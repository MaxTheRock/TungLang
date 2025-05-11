use crate::value::Value;
use crate::Rule;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;
use std::io::{self, Write};

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
                        }
                        (Value::Number(l), Value::Number(r), "!=") => {
                            Value::Number((l != r) as i32)
                        }
                        (Value::Number(l), Value::Number(r), ">") => Value::Number((l > r) as i32),
                        (Value::Number(l), Value::Number(r), "<") => Value::Number((l < r) as i32),
                        (Value::Number(l), Value::Number(r), ">=") => {
                            Value::Number((l >= r) as i32)
                        }
                        (Value::Number(l), Value::Number(r), "<=") => {
                            Value::Number((l <= r) as i32)
                        }
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
                    Value::Number(n)
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
                            io::stdout().flush().unwrap();
                            let mut input: String = String::new();
                            io::stdin().read_line(&mut input).unwrap();
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

pub fn execute_program(parsed: Pairs<Rule>, variables: &mut HashMap<String, Value>) {
    for pair in parsed {
        match pair.as_rule() {
            Rule::variable_declaration => {
                let mut inner: Pairs<Rule> = pair.into_inner();
                let var_name: String = inner.next().unwrap().as_str().to_string();
                let value: Value = evaluate_expression(inner.next().unwrap(), variables);
                if variables.contains_key(&var_name) {
                    eprintln!("Error: Variable '{}' is already declared.", var_name);
                } else {
                    variables.insert(var_name, value);
                }
            }
            Rule::print_statement => {
                let mut inner: Pairs<Rule> = pair.into_inner();
                let expr_pair: Pair<Rule> = inner.next().unwrap();
                let value: Value = evaluate_expression(expr_pair, variables);
                println!("{:?}", value);
            }
            Rule::if_statement => {
                execute_if_statement(pair, variables);
            }
            _ => {}
        }
    }
}

fn execute_block(block: Pair<Rule>, variables: &mut HashMap<String, Value>) {
    for statement in block.into_inner() {
        execute_program(statement.into_inner(), variables);
    }
}

pub fn execute_if_statement(pair: Pair<Rule>, variables: &mut HashMap<String, Value>) {
    let mut inner: Pairs<Rule> = pair.into_inner();
    let condition: Pair<Rule> = inner.next().unwrap();
    let condition_met: bool = match evaluate_expression(condition, variables) {
        Value::Number(n) => n != 0,
        Value::String(ref s) => !s.is_empty(),
        Value::Undefined => {
            eprintln!("Error: Undefined variable in condition");
            false
        }
    };

    if condition_met {
        let block: Pair<Rule> = inner.next().unwrap();
        execute_block(block, variables);
    } else {
        for elif_or_else in inner {
            match elif_or_else.as_rule() {
                Rule::elif_block => {
                    let mut elif_inner: Pairs<Rule> = elif_or_else.into_inner();
                    let elif_condition: Pair<Rule> = elif_inner.next().unwrap();
                    let elif_met: bool = match evaluate_expression(elif_condition, variables) {
                        Value::Number(n) => n != 0,
                        Value::String(ref s) => !s.is_empty(),
                        Value::Undefined => {
                            eprintln!("Error: Undefined variable in condition");
                            false
                        }
                    };
                    if elif_met {
                        let block: Pair<Rule> = elif_inner.next().unwrap();
                        execute_block(block, variables);
                        return;
                    }
                }
                Rule::else_block => {
                    let block: Pair<Rule> = elif_or_else.into_inner().next().unwrap();
                    execute_block(block, variables);
                    return;
                }
                _ => {}
            }
        }
    }
}
