use pest::iterators::Pairs;

use crate::diagnostics::TungError;
use crate::interpreter::expression::evaluate_expression;
use crate::interpreter::Interpreter;
use crate::parser::Rule;

impl Interpreter {
    pub(super) fn handle_variable_declaration(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), TungError> {
        // Skip the var_keyword rule
        pairs.next();

        // Get the identifier
        let identifier = match pairs.next() {
            Some(id_pair) if id_pair.as_rule() == Rule::identifier => id_pair.as_str().to_string(),
            _ => {
                return Err(TungError::InvalidExpression(
                    "Expected identifier in declaration".to_string(),
                    None,
                ))
            }
        };

        // Expect the next token to be an expression
        let expression = match pairs.next() {
            Some(expr_pair) if expr_pair.as_rule() == Rule::expression => {
                evaluate_expression(expr_pair.into_inner(), &self.variables)?
            }
            _ => {
                return Err(TungError::InvalidExpression(
                    "Expected expression after identifier in declaration".to_string(),
                    None,
                ))
            }
        };

        // Store the variable in our environment
        self.variables.insert(identifier, expression);

        Ok(())
    }

    pub(super) fn handle_assignment(&mut self, mut pairs: Pairs<Rule>) -> Result<(), TungError> {
        // Get the identifier
        let identifier = match pairs.next() {
            Some(id_pair) if id_pair.as_rule() == Rule::identifier => id_pair.as_str().to_string(),
            _ => {
                return Err(TungError::InvalidExpression(
                    "Expected identifier in assignment".to_string(),
                    None,
                ))
            }
        };

        // Expect the next token to be an expression
        let expression = match pairs.next() {
            Some(expr_pair) if expr_pair.as_rule() == Rule::expression => {
                evaluate_expression(expr_pair.into_inner(), &self.variables)?
            }
            _ => {
                return Err(TungError::InvalidExpression(
                    "Expected expression after identifier in assignment".to_string(),
                    None,
                ))
            }
        };

        // Check if variable exists
        if !self.variables.contains_key(&identifier) {
            return Err(TungError::VariableNotFound(identifier, None));
        }

        // Update the variable in our environment
        self.variables.insert(identifier, expression);

        Ok(())
    }
}
