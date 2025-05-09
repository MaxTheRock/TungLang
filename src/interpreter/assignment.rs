use pest::iterators::Pairs;

use crate::diagnostics::span_to_source_span;
use crate::interpreter::{Interpreter, InterpreterError};
use crate::parser::Rule;

impl Interpreter {
    pub(super) fn handle_variable_declaration(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), InterpreterError> {
        let id_pair = pairs.next();
        let variable = match id_pair {
            Some(ref id_pair) if id_pair.as_rule() == Rule::identifier => {
                // Trim whitespace from the variable name
                id_pair.as_str().trim().to_string()
            }
            _ => {
                let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
                return Err(InterpreterError::InvalidExpression(
                    "Expected identifier in declaration".to_string(),
                    span,
                ));
            }
        };

        // Check if variable already exists
        if self.variables.contains_key(&variable) {
            let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
            return Err(InterpreterError::VariableAlreadyDefined(
                format!("Variable '{}' has already been defined", variable),
                span,
            ));
        }

        let value = match pairs.next() {
            Some(val_pair) if val_pair.as_rule() == Rule::expression => {
                self.evaluate_expression(val_pair.into_inner())?
            }
            Some(val_pair) => {
                let span = span_to_source_span(val_pair.as_span());
                return Err(InterpreterError::InvalidExpression(
                    "Expected expression in variable declaration".to_string(),
                    Some(span),
                ));
            }
            None => {
                return Err(InterpreterError::InvalidExpression(
                    "Expected expression in variable declaration".to_string(),
                    None,
                ));
            }
        };

        self.variables.insert(variable, value);
        Ok(())
    }

    pub(super) fn handle_assignment(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), InterpreterError> {
        let id_pair = pairs.next();
        let variable = match id_pair {
            Some(ref id_pair) if id_pair.as_rule() == Rule::identifier => {
                // Trim whitespace from the variable name
                id_pair.as_str().trim().to_string()
            }
            _ => {
                let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
                return Err(InterpreterError::InvalidExpression(
                    "Expected identifier in assignment".to_string(),
                    span,
                ));
            }
        };

        // Check if variable exists
        if !self.variables.contains_key(&variable) {
            let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
            return Err(InterpreterError::VariableNotFound(
                format!("Cannot assign to undeclared variable '{}'. Use 'var' keyword to declare variables.", variable),
                span,
            ));
        }

        let value = match pairs.next() {
            Some(val_pair) if val_pair.as_rule() == Rule::expression => {
                self.evaluate_expression(val_pair.into_inner())?
            }
            Some(val_pair) => {
                let span = span_to_source_span(val_pair.as_span());
                return Err(InterpreterError::InvalidExpression(
                    "Expected expression in assignment".to_string(),
                    Some(span),
                ));
            }
            None => {
                return Err(InterpreterError::InvalidExpression(
                    "Expected expression in assignment".to_string(),
                    None,
                ));
            }
        };

        self.variables.insert(variable, value);
        Ok(())
    }
}
