use crate::stdlib::StdLib;
use crate::parser::Rule;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;
use crate::eval::evaluate_expression::evaluate_expression;

pub fn run_program(parsed: Pairs<Rule>) -> miette::Result<()> {
    let mut variables: HashMap<String, Value> = HashMap::new();
    let stdlib: StdLib = StdLib::new();
    for pair in parsed {
        execute_statement(pair, &mut variables, &stdlib)?;
    }
    Ok(())
}

fn execute_statement(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<()> {
    match pair.as_rule() {
        Rule::variable_declaration => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let var_name: String = inner.next().unwrap().as_str().to_string();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib);
            variables.insert(var_name, value);
        }
        Rule::assignment => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let var_name: String = inner.next().unwrap().as_str().to_string();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib);
            if variables.contains_key(&var_name) {
                variables.insert(var_name, value);
            } else {
                return Err(miette::miette!("Assignment to undefined variable '{}'.", var_name));
            }
        }
        Rule::print_statement => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib);
            match value {
                Value::String(s) => println!("{}", s),
                Value::Number(n) => println!("{}", n),
                Value::Float(f) => println!("{}", f),
                Value::Boolean(b) => println!("{}", b),
                Value::Array(arr) => println!("{:?}", arr),
                Value::Dict(map) => println!("{:?}", map),
                Value::Undefined => println!("undefined"),
            }
        }
        Rule::if_statement => {
            execute_if_statement(pair, variables, stdlib)?;
        }
        Rule::while_statement => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let condition: Pair<Rule> = inner.next().unwrap();
            let block: Pair<Rule> = inner.next().unwrap();
            while is_truthy(evaluate_expression(condition.clone(), variables, stdlib)) {
                let mut local_vars: HashMap<String, Value> = variables.clone();
                execute_block(block.clone(), &mut local_vars, stdlib)?;
                for (k, v) in local_vars.iter() {
                    if variables.contains_key(k) {
                        variables.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_truthy(value: Value) -> bool {
    match value {
        Value::Number(n) => n != 0,
        Value::Float(f) => f != 0.0,
        Value::String(ref s) => !s.is_empty(),
        Value::Boolean(b) => b,
        Value::Array(ref arr) => !arr.is_empty(),
        Value::Dict(ref map) => !map.is_empty(),
        Value::Undefined => false,
    }
}

fn execute_if_statement(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<()> {
    let mut inner: Pairs<Rule> = pair.into_inner();
    let condition: Pair<Rule> = inner.next().unwrap();
    let block: Pair<Rule> = inner.next().unwrap();
    let condition_met: bool = is_truthy(evaluate_expression(condition, variables, stdlib));
    if condition_met {
        let mut local_vars: HashMap<String, Value> = variables.clone();
        execute_block(block, &mut local_vars, stdlib)?;
        for (k, v) in local_vars.iter() {
            if variables.contains_key(k) {
                variables.insert(k.clone(), v.clone());
            }
        }
        return Ok(());
    } else {
        for elif_or_else in inner {
            match elif_or_else.as_rule() {
                Rule::elif_block => {
                    let mut elif_inner: Pairs<Rule> = elif_or_else.into_inner();
                    let elif_condition: Pair<Rule> = elif_inner.next().unwrap();
                    let elif_block: Pair<Rule> = elif_inner.next().unwrap();
                    let elif_met: bool = is_truthy(evaluate_expression(elif_condition, variables, stdlib));
                    if elif_met {
                        let mut local_vars: HashMap<String, Value> = variables.clone();
                        execute_block(elif_block, &mut local_vars, stdlib)?;
                        for (k, v) in local_vars.iter() {
                            if variables.contains_key(k) {
                                variables.insert(k.clone(), v.clone());
                            }
                        }
                        return Ok(());
                    }
                }
                Rule::else_block => {
                    let else_block: Pair<Rule> = elif_or_else.into_inner().next().unwrap();
                    let mut local_vars: HashMap<String, Value> = variables.clone();
                    execute_block(else_block, &mut local_vars, stdlib)?;
                    for (k, v) in local_vars.iter() {
                        if variables.contains_key(k) {
                            variables.insert(k.clone(), v.clone());
                        }
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn execute_block(
    block: Pair<Rule>,
    variables: &mut HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<()> {
    let mut local_vars: HashMap<String, Value> = variables.clone();
    for statement in block.into_inner() {
        execute_statement(statement, &mut local_vars, stdlib)?;
    }
    for (k, v) in local_vars.iter() {
        if variables.contains_key(k) {
            variables.insert(k.clone(), v.clone());
        }
    }
    Ok(())
}
