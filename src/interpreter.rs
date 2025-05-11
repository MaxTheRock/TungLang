use crate::Rule;
use crate::eval::evaluate_expression;
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
            let mut inner = pair.into_inner();
            let var_name = inner.next().unwrap().as_str().to_string();
            let value = evaluate_expression(inner.next().unwrap(), variables);
            variables.insert(var_name, value);
        }
        Rule::print_statement => {
            let mut inner = pair.into_inner();
            let value = evaluate_expression(inner.next().unwrap(), variables);
            match value {
                Value::String(s) => println!("{}", s),
                Value::Number(n) => println!("{}", n),
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

fn execute_if_statement(pair: Pair<Rule>, variables: &mut HashMap<String, Value>) -> miette::Result<()> {
    let mut inner = pair.into_inner();
    let condition = inner.next().unwrap();
    let condition_met = match evaluate_expression(condition, variables) {
        Value::Number(n) => n != 0,
        Value::String(ref s) => !s.is_empty(),
        Value::Undefined => false,
    };
    if condition_met {
        let block = inner.next().unwrap();
        execute_block(block, variables)?;
        return Ok(());
    } else {
        let mut found = false;
        for elif_or_else in inner {
            match elif_or_else.as_rule() {
                Rule::elif_block => {
                    let mut elif_inner = elif_or_else.into_inner();
                    let elif_condition = elif_inner.next().unwrap();
                    let elif_met = match evaluate_expression(elif_condition, variables) {
                        Value::Number(n) => n != 0,
                        Value::String(ref s) => !s.is_empty(),
                        Value::Undefined => false,
                    };
                    if elif_met {
                        let block = elif_inner.next().unwrap();
                        execute_block(block, variables)?;
                        found = true;
                        break;
                    }
                }
                Rule::else_block => {
                    let block = elif_or_else.into_inner().next().unwrap();
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
