use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

use crate::diagnostics::TungError;
use crate::parser::Rule;
use crate::value::Value;

pub fn evaluate_expression(
    mut pairs: Pairs<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, TungError> {
    // We expect at least one expression term
    match pairs.next() {
        Some(pair) => evaluate_expression_term(pair, variables),
        None => Err(TungError::InvalidExpression(
            "Empty expression".to_string(),
            None,
        )),
    }
}

fn evaluate_expression_term(
    pair: Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, TungError> {
    match pair.as_rule() {
        Rule::comparison => evaluate_comparison(pair.into_inner(), variables),
        Rule::term => evaluate_term(pair.into_inner(), variables),
        _ => Err(TungError::InvalidExpression(
            format!("Unexpected rule in expression: {:?}", pair.as_rule()),
            None,
        )),
    }
}

fn evaluate_comparison(
    mut pairs: Pairs<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, TungError> {
    // Get left term
    let left = match pairs.next() {
        Some(term) => evaluate_term(term.into_inner(), variables)?,
        None => {
            return Err(TungError::InvalidExpression(
                "Expected left operand in comparison".to_string(),
                None,
            ))
        }
    };

    // Get comparison operator
    let op = match pairs.next() {
        Some(op_pair) if op_pair.as_rule() == Rule::comparison_op => op_pair.as_str(),
        _ => {
            return Err(TungError::InvalidOperator(
                "Expected comparison operator".to_string(),
                None,
            ))
        }
    };

    // Get right term
    let right = match pairs.next() {
        Some(term) => evaluate_term(term.into_inner(), variables)?,
        None => {
            return Err(TungError::InvalidExpression(
                "Expected right operand in comparison".to_string(),
                None,
            ))
        }
    };

    // Perform the comparison
    match op {
        "==" => Ok(Value::Boolean(left == right)),
        "!=" => Ok(Value::Boolean(left != right)),
        "<" => compare_values(&left, &right, |a, b| a < b),
        "<=" => compare_values(&left, &right, |a, b| a <= b),
        ">" => compare_values(&left, &right, |a, b| a > b),
        ">=" => compare_values(&left, &right, |a, b| a >= b),
        _ => Err(TungError::InvalidOperator(
            format!("Unknown comparison operator: {}", op),
            None,
        )),
    }
}

fn compare_values<F>(left: &Value, right: &Value, compare_fn: F) -> Result<Value, TungError>
where
    F: Fn(&i64, &i64) -> bool,
{
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(compare_fn(l, r))),
        _ => Err(TungError::TypeMismatch(
            format!("Cannot compare {:?} and {:?}", left, right),
            None,
        )),
    }
}

fn evaluate_term(
    mut pairs: Pairs<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, TungError> {
    // Get the first factor
    let first_factor = match pairs.next() {
        Some(factor) => evaluate_factor(factor, variables)?,
        None => {
            return Err(TungError::InvalidExpression(
                "Expected a factor in term".to_string(),
                None,
            ))
        }
    };

    // Process any operations (+ - * /)
    let mut result = first_factor;
    while let Some(op_pair) = pairs.next() {
        // Get operator
        let op = op_pair.as_str();

        // Get next factor
        let factor = match pairs.next() {
            Some(factor) => evaluate_factor(factor, variables)?,
            None => {
                return Err(TungError::InvalidExpression(
                    "Expected a factor after operator".to_string(),
                    None,
                ))
            }
        };

        // Perform the operation
        result = match op {
            "+" => add_values(result, factor)?,
            "-" => subtract_values(result, factor)?,
            "*" => multiply_values(result, factor)?,
            "/" => divide_values(result, factor)?,
            _ => {
                return Err(TungError::InvalidOperator(
                    format!("Unknown operator: {}", op),
                    None,
                ))
            }
        };
    }

    Ok(result)
}

fn evaluate_factor(
    pair: Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, TungError> {
    match pair.as_rule() {
        Rule::factor => {
            // Get the inner part of the factor and evaluate that
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::value => evaluate_value(inner.into_inner().next().unwrap()),
                Rule::identifier => {
                    let var_name = inner.as_str();
                    variables
                        .get(var_name)
                        .cloned()
                        .ok_or_else(|| TungError::VariableNotFound(var_name.to_string(), None))
                }
                Rule::expression => evaluate_expression(inner.into_inner(), variables),
                _ => Err(TungError::InvalidExpression(
                    format!("Unexpected rule in factor inner: {:?}", inner.as_rule()),
                    None,
                )),
            }
        }
        Rule::value => evaluate_value(pair.into_inner().next().unwrap()),
        Rule::identifier => {
            let var_name = pair.as_str();
            variables
                .get(var_name)
                .cloned()
                .ok_or_else(|| TungError::VariableNotFound(var_name.to_string(), None))
        }
        Rule::expression => evaluate_expression(pair.into_inner(), variables),
        _ => Err(TungError::InvalidExpression(
            format!("Unexpected rule in factor: {:?}", pair.as_rule()),
            None,
        )),
    }
}

fn evaluate_value(pair: Pair<Rule>) -> Result<Value, TungError> {
    match pair.as_rule() {
        Rule::number => {
            let num_str = pair.as_str();
            let num = num_str.parse::<i64>().map_err(|_| {
                TungError::InvalidExpression(format!("Failed to parse number: {}", num_str), None)
            })?;
            Ok(Value::Number(num))
        }
        Rule::string_literal => {
            let text = pair.into_inner().next().unwrap().as_str();
            Ok(Value::String(text.to_string()))
        }
        Rule::boolean => {
            let bool_str = pair.as_str();
            match bool_str {
                "true" => Ok(Value::Boolean(true)),
                "false" => Ok(Value::Boolean(false)),
                _ => Err(TungError::InvalidExpression(
                    format!("Invalid boolean value: {}", bool_str),
                    None,
                )),
            }
        }
        _ => Err(TungError::InvalidExpression(
            format!("Unexpected value type: {:?}", pair.as_rule()),
            None,
        )),
    }
}

// Basic arithmetic operations
fn add_values(left: Value, right: Value) -> Result<Value, TungError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
        (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
        (l, r) => Err(TungError::TypeMismatch(
            format!("Cannot add {:?} and {:?}", l, r),
            None,
        )),
    }
}

fn subtract_values(left: Value, right: Value) -> Result<Value, TungError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
        (l, r) => Err(TungError::TypeMismatch(
            format!("Cannot subtract {:?} from {:?}", r, l),
            None,
        )),
    }
}

fn multiply_values(left: Value, right: Value) -> Result<Value, TungError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
        (l, r) => Err(TungError::TypeMismatch(
            format!("Cannot multiply {:?} by {:?}", l, r),
            None,
        )),
    }
}

fn divide_values(left: Value, right: Value) -> Result<Value, TungError> {
    match (left, right) {
        (Value::Number(_), Value::Number(0)) => Err(TungError::DivisionByZero(None)),
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
        (l, r) => Err(TungError::TypeMismatch(
            format!("Cannot divide {:?} by {:?}", l, r),
            None,
        )),
    }
}
