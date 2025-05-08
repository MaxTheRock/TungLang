use pest::iterators::Pairs;
use std::collections::HashMap;

use crate::parser::Rule;
use crate::value::Value;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, pairs: Pairs<Rule>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::file => {
                    // Process the file rule that contains statements
                    for statement_pair in pair.into_inner() {
                        match statement_pair.as_rule() {
                            Rule::statement => self.handle_statement(statement_pair.into_inner()),
                            Rule::EOI => (),
                            _ => {
                                println!("Unexpected rule in file: {:?}", statement_pair.as_rule())
                            }
                        }
                    }
                }
                _ => println!("Unexpected top-level rule: {:?}", pair.as_rule()),
            }
        }
    }

    fn handle_statement(&mut self, mut pairs: Pairs<Rule>) {
        if let Some(pair) = pairs.next() {
            match pair.as_rule() {
                Rule::assignment => self.handle_assignment(pair.into_inner()),
                Rule::function_call => self.handle_function_call(pair.into_inner()),
                _ => println!("Unexpected statement: {:?}", pair.as_rule()),
            }
        }
    }

    fn handle_assignment(&mut self, mut pairs: Pairs<Rule>) {
        let variable = match pairs.next() {
            Some(id_pair) if id_pair.as_rule() == Rule::identifier => {
                // Trim whitespace from the variable name
                id_pair.as_str().trim().to_string()
            }
            _ => {
                eprintln!("Expected identifier in assignment");
                return;
            }
        };

        let value = match pairs.next() {
            Some(val_pair) if val_pair.as_rule() == Rule::value => self.parse_value(val_pair),
            Some(other) => {
                eprintln!("Expected value, got: {:?}", other.as_rule());
                Value::Number(0)
            }
            None => {
                eprintln!("No value provided for assignment");
                Value::Number(0)
            }
        };

        self.variables.insert(variable, value);
    }

    fn parse_value(&self, pair: pest::iterators::Pair<Rule>) -> Value {
        let pair_rule = pair.as_rule();

        match pair_rule {
            Rule::value => {
                // Get the first inner pair which should be either string_literal or number
                let inner = pair.clone().into_inner().next();

                if let Some(inner_pair) = inner {
                    match inner_pair.as_rule() {
                        Rule::string_literal => {
                            // Get the inner string without quotes
                            let inner_string = inner_pair.into_inner().next().unwrap().as_str();
                            Value::String(inner_string.to_string())
                        }
                        Rule::number => match inner_pair.as_str().parse::<i64>() {
                            Ok(num) => Value::Number(num),
                            Err(_) => {
                                eprintln!("Failed to parse number: {}", inner_pair.as_str());
                                Value::Number(0)
                            }
                        },
                        Rule::string_content => {
                            let content = inner_pair.as_str();
                            Value::String(content.to_string())
                        }
                        _ => {
                            eprintln!("Unexpected value type: {:?}", inner_pair.as_rule());
                            Value::Number(0)
                        }
                    }
                } else {
                    eprintln!("Empty value");
                    Value::Number(0)
                }
            }
            _ => {
                eprintln!("Expected value, got {:?}", pair_rule);
                Value::Number(0)
            }
        }
    }

    fn evaluate_expression(&self, mut pairs: Pairs<Rule>) -> Value {
        if let Some(pair) = pairs.next() {
            match pair.as_rule() {
                Rule::identifier => {
                    // Trim whitespace from the variable name before lookup
                    let var_name = pair.as_str().trim();
                    match self.variables.get(var_name) {
                        Some(value) => value.clone(),
                        None => {
                            eprintln!("Variable not found: {}", var_name);
                            Value::Number(0)
                        }
                    }
                }
                Rule::value => self.parse_value(pair),
                Rule::binary_expr => self.evaluate_binary_expr(pair.into_inner()),
                _ => {
                    eprintln!("Unexpected expression type: {:?}", pair.as_rule());
                    Value::Number(0)
                }
            }
        } else {
            eprintln!("Empty expression");
            Value::Number(0)
        }
    }

    fn handle_function_call(&mut self, mut pairs: Pairs<Rule>) {
        let function_name = match pairs.next() {
            Some(id_pair) if id_pair.as_rule() == Rule::identifier => id_pair.as_str(),
            _ => {
                eprintln!("Expected function name");
                return;
            }
        };

        // Special handling for print function
        if function_name == "print" {
            // Get arguments
            let args: Vec<Value> = pairs
                .filter(|p| p.as_rule() == Rule::expression)
                .map(|p| self.evaluate_expression(p.into_inner()))
                .collect();

            // Print each argument
            for arg in args {
                println!("{}", arg);
            }
        } else {
            eprintln!("Unknown function: {}", function_name);
        }
    }

    fn evaluate_binary_expr(&self, mut pairs: Pairs<Rule>) -> Value {
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
