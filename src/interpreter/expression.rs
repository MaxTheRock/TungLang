use miette::SourceSpan;
use pest::iterators::{Pair, Pairs};

use crate::diagnostics::span_to_source_span;
use crate::interpreter::{Interpreter, InterpreterError};
use crate::parser::Rule;
use crate::value::Value;

impl Interpreter {
    pub(super) fn evaluate_expression(
        &self,
        mut pairs: Pairs<Rule>,
    ) -> Result<Value, InterpreterError> {
        if let Some(pair) = pairs.next() {
            // Get span for error reporting
            let span = pair.as_span();
            let source_span = Some(span_to_source_span(span));

            match pair.as_rule() {
                Rule::identifier => {
                    // Trim whitespace from the variable name before lookup
                    let var_name = pair.as_str().trim();
                    match self.variables.get(var_name) {
                        Some(value) => Ok(value.clone()),
                        None => Err(InterpreterError::VariableNotFound(
                            format!("Variable '{}' has not been defined", var_name),
                            source_span,
                        )),
                    }
                }
                Rule::value => self.parse_value(pair),
                Rule::comparison => self.evaluate_comparison_expr(pair.into_inner()),
                Rule::term => self.evaluate_term(pair.into_inner()),
                Rule::factor => self.evaluate_factor(pair),
                Rule::expression => self.evaluate_expression(pair.into_inner()),
                _ => Err(InterpreterError::InvalidExpression(
                    format!("Unexpected expression type: {:?}", pair.as_rule()),
                    source_span,
                )),
            }
        } else {
            Err(InterpreterError::InvalidExpression(
                "Empty expression found where a value was expected".to_string(),
                None,
            ))
        }
    }

    // Evaluate factors (identifiers, values, parenthesized expressions)
    pub(super) fn evaluate_factor(&self, pair: Pair<Rule>) -> Result<Value, InterpreterError> {
        let span = Some(span_to_source_span(pair.as_span()));
        let inner = pair.into_inner().next();
        match inner {
            Some(inner_pair) => {
                let inner_span = Some(span_to_source_span(inner_pair.as_span()));
                match inner_pair.as_rule() {
                    Rule::identifier => self.evaluate_expression(Pairs::single(inner_pair)),
                    Rule::value => self.parse_value(inner_pair),
                    Rule::expression => self.evaluate_expression(inner_pair.into_inner()),
                    _ => Err(InterpreterError::InvalidExpression(
                        format!("Unexpected factor type: {:?}", inner_pair.as_rule()),
                        inner_span,
                    )),
                }
            }
            None => Err(InterpreterError::InvalidExpression(
                "Empty factor".to_string(),
                span,
            )),
        }
    }

    // Evaluate terms (factors with operators)
    pub(super) fn evaluate_term(&self, mut pairs: Pairs<Rule>) -> Result<Value, InterpreterError> {
        let first_pair = pairs.peek();
        let span = first_pair.map(|p| span_to_source_span(p.as_span()));

        let mut result = match pairs.next() {
            Some(factor) => self.evaluate_factor(factor)?,
            None => {
                return Err(InterpreterError::InvalidExpression(
                    "Expected factor".to_string(),
                    span,
                ))
            }
        };

        // Process operators and additional factors
        while let (Some(op), Some(factor)) = (pairs.next(), pairs.next()) {
            let op_str = op.as_str();
            let op_span = Some(span_to_source_span(op.as_span()));
            let right = self.evaluate_factor(factor)?;

            result = self.apply_binary_operation(result, op_str, right, op_span)?;
        }

        Ok(result)
    }

    // Evaluate comparison expressions like x == 5, y <= 10, etc.
    pub(super) fn evaluate_comparison_expr(
        &self,
        mut pairs: Pairs<Rule>,
    ) -> Result<Value, InterpreterError> {
        let first_pair = pairs.peek();
        let span = first_pair.map(|p| span_to_source_span(p.as_span()));

        let left_term = match pairs.next() {
            Some(pair) => self.evaluate_term(pair.into_inner())?,
            None => {
                return Err(InterpreterError::InvalidExpression(
                    "Expected left term in comparison".to_string(),
                    span,
                ));
            }
        };

        let op_pair = pairs.next();
        let op_span = op_pair.as_ref().map(|p| span_to_source_span(p.as_span()));

        let operator = match op_pair {
            Some(pair) if pair.as_rule() == Rule::comparison_op => pair.as_str(),
            _ => {
                return Err(InterpreterError::InvalidExpression(
                    "Expected comparison operator".to_string(),
                    span,
                ));
            }
        };

        let right_term = match pairs.next() {
            Some(pair) => self.evaluate_term(pair.into_inner())?,
            None => {
                return Err(InterpreterError::InvalidExpression(
                    "Expected right term in comparison".to_string(),
                    span,
                ));
            }
        };

        // Handle comparisons based on types
        match (left_term, right_term) {
            // Number comparisons
            (Value::Number(left_num), Value::Number(right_num)) => {
                let result = match operator {
                    "==" => left_num == right_num,
                    "!=" => left_num != right_num,
                    "<" => left_num < right_num,
                    ">" => left_num > right_num,
                    "<=" => left_num <= right_num,
                    ">=" => left_num >= right_num,
                    _ => {
                        return Err(InterpreterError::InvalidOperator(
                            format!("Unknown comparison operator: {}", operator),
                            op_span,
                        ));
                    }
                };
                Ok(Value::Boolean(result))
            }
            // String comparisons
            (Value::String(left_str), Value::String(right_str)) => {
                let result = match operator {
                    "==" => left_str == right_str,
                    "!=" => left_str != right_str,
                    "<" => left_str < right_str,
                    ">" => left_str > right_str,
                    "<=" => left_str <= right_str,
                    ">=" => left_str >= right_str,
                    _ => {
                        return Err(InterpreterError::InvalidOperator(
                            format!("Unknown comparison operator: {}", operator),
                            op_span,
                        ));
                    }
                };
                Ok(Value::Boolean(result))
            }
            // Boolean comparisons
            (Value::Boolean(left_bool), Value::Boolean(right_bool)) => {
                let result = match operator {
                    "==" => left_bool == right_bool,
                    "!=" => left_bool != right_bool,
                    _ => {
                        return Err(InterpreterError::InvalidOperator(
                            format!("Invalid comparison operator for booleans: {}", operator),
                            op_span,
                        ));
                    }
                };
                Ok(Value::Boolean(result))
            }
            // Mixed types
            (left_val, right_val) => Err(InterpreterError::TypeMismatch(
                format!(
                    "Cannot compare {} with {} using {}",
                    left_val.type_name(),
                    right_val.type_name(),
                    operator
                ),
                op_span,
            )),
        }
    }

    pub(super) fn parse_value(&self, pair: Pair<Rule>) -> Result<Value, InterpreterError> {
        let span = Some(span_to_source_span(pair.as_span()));
        let inner = pair.into_inner().next();

        if let Some(inner_pair) = inner {
            let inner_span = Some(span_to_source_span(inner_pair.as_span()));
            match inner_pair.as_rule() {
                Rule::string_literal => {
                    // Get the inner string without quotes
                    let inner_string = match inner_pair.into_inner().next() {
                        Some(content) => content.as_str(),
                        None => "",
                    };
                    Ok(Value::String(inner_string.to_string()))
                }
                Rule::number => match inner_pair.as_str().parse::<i64>() {
                    Ok(num) => Ok(Value::Number(num)),
                    Err(_) => Err(InterpreterError::InvalidExpression(
                        format!("Failed to parse number: {}", inner_pair.as_str()),
                        inner_span,
                    )),
                },
                Rule::string_content => {
                    let content = inner_pair.as_str();
                    Ok(Value::String(content.to_string()))
                }
                Rule::boolean => {
                    let bool_val = match inner_pair.as_str() {
                        "true" => true,
                        "false" => false,
                        _ => {
                            return Err(InterpreterError::InvalidExpression(
                                format!("Invalid boolean value: {}", inner_pair.as_str()),
                                inner_span,
                            ));
                        }
                    };
                    Ok(Value::Boolean(bool_val))
                }
                _ => Err(InterpreterError::InvalidExpression(
                    format!("Unexpected value type: {:?}", inner_pair.as_rule()),
                    inner_span,
                )),
            }
        } else {
            Err(InterpreterError::InvalidExpression(
                "Empty value".to_string(),
                span,
            ))
        }
    }

    pub(super) fn apply_binary_operation(
        &self,
        left: Value,
        operator: &str,
        right: Value,
        span: Option<SourceSpan>,
    ) -> Result<Value, InterpreterError> {
        // Handle operations based on types
        match (left, right) {
            // Number operations
            (Value::Number(left_num), Value::Number(right_num)) => match operator {
                "+" => Ok(Value::Number(left_num + right_num)),
                "-" => Ok(Value::Number(left_num - right_num)),
                "*" => Ok(Value::Number(left_num * right_num)),
                "/" => {
                    if right_num == 0 {
                        Err(InterpreterError::DivisionByZero(span))
                    } else {
                        Ok(Value::Number(left_num / right_num))
                    }
                }
                _ => Err(InterpreterError::InvalidOperator(
                    format!("Unknown operator for numbers: {}", operator),
                    span,
                )),
            },

            // String operations
            (Value::String(left_str), Value::String(right_str)) => match operator {
                "+" => Ok(Value::String(format!("{}{}", left_str, right_str))),
                _ => Err(InterpreterError::InvalidOperator(
                    format!("Invalid operator for strings: {}", operator),
                    span,
                )),
            },

            // Mixed types
            (left_val, right_val) => Err(InterpreterError::TypeMismatch(
                format!(
                    "Cannot perform {} operation between {} and {}",
                    operator,
                    left_val.type_name(),
                    right_val.type_name()
                ),
                span,
            )),
        }
    }
}
