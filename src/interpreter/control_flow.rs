use pest::iterators::Pairs;

use crate::diagnostics::TungError;
use crate::interpreter::expression::evaluate_expression;
use crate::interpreter::Interpreter;
use crate::parser::Rule;
use crate::value::Value;

impl Interpreter {
    pub(super) fn handle_if_statement(&mut self, mut pairs: Pairs<Rule>) -> Result<(), TungError> {
        // Get the control keyword (already processed in preprocessing)
        let _ = pairs.next();

        // Get the condition expression
        let condition = match pairs.next() {
            Some(expr) => evaluate_expression(expr.into_inner(), &self.variables)?,
            None => {
                return Err(TungError::InvalidExpression(
                    "Expected condition in if statement".to_string(),
                    None,
                ))
            }
        };

        // Skip the colon
        pairs.next();

        // Get the statements to execute if condition is true
        let statements = match pairs.next() {
            Some(stmts) => stmts,
            None => {
                return Err(TungError::InvalidStatement(
                    "Expected statements in if block".to_string(),
                    None,
                ))
            }
        };

        // Check if the condition is true
        match condition {
            Value::Boolean(true) => {
                // Execute the statements in the if block
                for stmt in statements.into_inner() {
                    if stmt.as_rule() == Rule::statement {
                        self.handle_statement(stmt.into_inner())?;
                    }
                }
            }
            Value::Boolean(false) => {
                // Condition is false, skip the if block
            }
            _ => {
                return Err(TungError::TypeMismatch(
                    "If condition must evaluate to a boolean".to_string(),
                    None,
                ))
            }
        }

        Ok(())
    }
}
