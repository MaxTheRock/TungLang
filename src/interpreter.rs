use crate::eval::evaluate_expression;
use crate::parser::Rule;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn run_program(parsed: Pairs<Rule>) -> miette::Result<()> {
    let mut variables: HashMap<String, Value> = HashMap::new();
    for pair in parsed {
        execute_statement(pair, &mut variables)?;
    }
    Ok(())
}

fn execute_statement(pair: Pair<Rule>, variables: &mut HashMap<String, Value>) -> miette::Result<()> {
    match pair.as_rule() {
        Rule::variable_declaration => {
            let mut inner: pest::iterators::Pairs<Rule> = pair.into_inner();
            let var_name: String = inner.next().unwrap().as_str().to_string();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables);
            variables.insert(var_name, value);
        }
        Rule::print_statement => {
            let mut inner: pest::iterators::Pairs<Rule> = pair.into_inner();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables);
            match value {
                Value::String(s) => println!("{}", s),
                Value::Number(n) => println!("{}", n),
                Value::Boolean(b) => println!("{}", b),
                Value::Undefined => println!("undefined"),
            }
        }
        Rule::if_statement => {
            execute_if_statement(pair, variables)?;
        }
        _ => {}
    }
    Ok(())
}

fn is_truthy(value: Value) -> bool {
    match value {
        Value::Number(n) => n != 0,
        Value::String(ref s) => !s.is_empty(),
        Value::Boolean(b) => b,
        Value::Undefined => false,
    }
}

fn execute_if_statement(pair: Pair<Rule>, variables: &mut HashMap<String, Value>) -> miette::Result<()> {
    let mut inner: pest::iterators::Pairs<Rule> = pair.into_inner();
    let condition: Pair<Rule> = inner.next().unwrap();
    let condition_met: bool = is_truthy(evaluate_expression(condition, variables));
    if condition_met {
        let block: Pair<Rule> = inner.next().unwrap();
        execute_block(block, variables)?;
        return Ok(());
    } else {
        let mut found: bool = false;
        for elif_or_else in inner {
            match elif_or_else.as_rule() {
                Rule::elif_block => {
                    let mut elif_inner: pest::iterators::Pairs<Rule> = elif_or_else.into_inner();
                    let elif_condition: Pair<Rule> = elif_inner.next().unwrap();
                    let elif_met: bool = is_truthy(evaluate_expression(elif_condition, variables));
                    if elif_met {
                        let block: Pair<Rule> = elif_inner.next().unwrap();
                        execute_block(block, variables)?;
                        found = true;
                        break;
                    }
                }
                Rule::else_block => {
                    let block: Pair<Rule> = elif_or_else.into_inner().next().unwrap();
                    execute_block(block, variables)?;
                    found = true;
                    break;
                }
                _ => {}
            }
        }
        if found {
            return Ok(());
        }
    }
    Ok(())
}

fn execute_block(block: Pair<Rule>, variables: &mut HashMap<String, Value>) -> miette::Result<()> {
    for statement in block.into_inner() {
        execute_statement(statement, variables)?;
    }
    Ok(())
}
