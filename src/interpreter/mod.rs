mod assignment;
mod control_flow;
mod expression;
mod function_call;

use pest::iterators::Pairs;
use std::collections::HashMap;

use crate::diagnostics::TungError;
use crate::parser::Rule;
use crate::value::Value;

// Use our new error type from diagnostics
pub type InterpreterError = TungError;

pub struct Interpreter {
    variables: HashMap<String, Value>,
    source: Option<String>, // Store the source code for error reporting
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            source: None,
        }
    }

    pub fn with_source(source: String) -> Self {
        Interpreter {
            variables: HashMap::new(),
            source: Some(source),
        }
    }

    pub fn interpret(&mut self, pairs: Pairs<Rule>) -> Result<(), InterpreterError> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::file => {
                    // Process the file rule that contains statements
                    for statement_pair in pair.into_inner() {
                        match statement_pair.as_rule() {
                            Rule::statement => {
                                self.handle_statement(statement_pair.into_inner())?;
                            }
                            Rule::EOI => (),
                            _ => {
                                return Err(InterpreterError::InvalidStatement(
                                    format!(
                                        "Unexpected rule in file: {:?}",
                                        statement_pair.as_rule()
                                    ),
                                    None,
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(InterpreterError::InvalidStatement(
                        format!("Unexpected top-level rule: {:?}", pair.as_rule()),
                        None,
                    ));
                }
            }
        }
        Ok(())
    }

    fn handle_statement(&mut self, mut pairs: Pairs<Rule>) -> Result<(), InterpreterError> {
        if let Some(pair) = pairs.next() {
            match pair.as_rule() {
                Rule::assignment => {
                    self.handle_assignment(pair.into_inner())?;
                    Ok(())
                }
                Rule::function_call => {
                    self.handle_function_call(pair.into_inner())?;
                    Ok(())
                }
                Rule::if_statement => {
                    self.handle_if_statement(pair.into_inner())?;
                    Ok(())
                }
                _ => Err(InterpreterError::InvalidStatement(
                    format!(
                        "Unexpected statement type: {:?}. Expected assignment, function call, or if statement.", 
                        pair.as_rule()
                    ),
                    None,
                )),
            }
        } else {
            Err(InterpreterError::InvalidStatement(
                "Empty statement encountered. Each statement should perform an action.".to_string(),
                None,
            ))
        }
    }
}
