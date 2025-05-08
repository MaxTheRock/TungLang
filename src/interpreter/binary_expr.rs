use pest::iterators::Pairs;

use crate::interpreter::Interpreter;
use crate::parser::Rule;
use crate::value::Value;

impl Interpreter {
    pub(super) fn evaluate_binary_expr(&self, mut pairs: Pairs<Rule>) -> Value {
        let left = match pairs.next() {
            Some(pair) if pair.as_rule() == Rule::identifier => pair.as_str().trim(),
            _ => {
                eprintln!("Expected left operand");
                return Value::Number(0);
            }
        };

        let operator = match pairs.next() {
            Some(pair) if pair.as_rule() == Rule::operator => pair.as_str(),
            _ => {
                eprintln!("Expected operator");
                return Value::Number(0);
            }
        };

        let right = match pairs.next() {
            Some(pair) if pair.as_rule() == Rule::identifier => pair.as_str().trim(),
            _ => {
                eprintln!("Expected right operand");
                return Value::Number(0);
            }
        };

        // Get left value
        let left_val = match self.variables.get(left) {
            Some(value) => value.clone(),
            None => {
                eprintln!("Variable not found: {}", left);
                return Value::Number(0);
            }
        };

        // Get right value
        let right_val = match self.variables.get(right) {
            Some(value) => value.clone(),
            None => {
                eprintln!("Variable not found: {}", right);
                return Value::Number(0);
            }
        };

        // Handle operations based on types
        match (left_val, right_val) {
            // Number operations
            (Value::Number(left_num), Value::Number(right_num)) => match operator {
                "+" => Value::Number(left_num + right_num),
                "-" => Value::Number(left_num - right_num),
                "*" => Value::Number(left_num * right_num),
                "/" => {
                    if right_num == 0 {
                        eprintln!("Division by zero");
                        Value::Number(0)
                    } else {
                        Value::Number(left_num / right_num)
                    }
                }
                _ => {
                    eprintln!("Unknown operator for numbers: {}", operator);
                    Value::Number(0)
                }
            },

            // String operations
            (Value::String(left_str), Value::String(right_str)) => match operator {
                "+" => Value::String(format!("{}{}", left_str, right_str)),
                _ => {
                    eprintln!("Invalid operator for strings: {}", operator);
                    Value::String(String::new())
                }
            },

            // Mixed types
            (left_val, right_val) => {
                eprintln!(
                    "Type mismatch: Cannot perform {} operation between {} and {}",
                    operator,
                    left_val.type_name(),
                    right_val.type_name()
                );
                Value::Number(0)
            }
        }
    }
}
