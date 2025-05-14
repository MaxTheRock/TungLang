use crate::parser::Rule;
use crate::value::Value;
use crate::stdlib::StdLib;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn evaluate_expression(
    pair: Pair<Rule>,
    variables: &HashMap<String, Value>,
    stdlib: &StdLib,
) -> Value {
    use crate::eval::operators::apply_operator;

    match pair.as_rule() {
        Rule::number => {
            let s: &str = pair.as_str();
            if s.contains('.') {
                Value::Float(s.parse::<f64>().unwrap())
            } else {
                Value::Number(s.parse::<i64>().unwrap())
            }
        }
        Rule::string => {
            let s: &str = pair.as_str();
            Value::String(s[1..s.len()-1].to_string())
        }
        Rule::IDENTIFIER => {
            let name: &str = pair.as_str();
            variables.get(name).cloned().unwrap_or(Value::Undefined)
        }
        Rule::function_call => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let func_name: &str = inner.next().unwrap().as_str();
            let args: Vec<Value> = inner.map(|p: Pair<Rule>| evaluate_expression(p, variables, stdlib)).collect();
            if let Some(func) = stdlib.get(func_name) {
                func(&args)
            } else {
                Value::Undefined
            }
        }
        Rule::comparison | Rule::sum | Rule::term => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let mut left: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib);
            while let Some(op_pair) = inner.next() {
                let op: &str = op_pair.as_str();
                let right: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib);
                left = apply_operator(left, right, op);
            }
            left
        }
        Rule::factor => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let first: Pair<Rule> = inner.next().unwrap();
            evaluate_expression(first, variables, stdlib)
        }
        Rule::array => {
            let elements: Vec<Value> = pair.into_inner().map(|p: Pair<Rule>| evaluate_expression(p, variables, stdlib)).collect();
            Value::Array(elements)
        }
        Rule::dict => {
            let mut map: HashMap<String, Value> = HashMap::new();
            for entry in pair.into_inner() {
                let mut kv: Pairs<Rule> = entry.into_inner();
                let k: String = kv.next().unwrap().as_str().to_string();
                let v: Value = evaluate_expression(kv.next().unwrap(), variables, stdlib);
                map.insert(k, v);
            }
            Value::Dict(map)
        }
        Rule::expression => {
            evaluate_expression(pair.into_inner().next().unwrap(), variables, stdlib)
        }
        _ => Value::Undefined
    }
}
