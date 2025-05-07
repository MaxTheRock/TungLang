use crate::ast::{Expr, Operator, Statement};
use crate::error::{Result, TungError};
use miette::SourceSpan;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tung_lang.pest"]
pub struct TungLangParser;

// Track source code for error reporting
pub struct ParserContext {
    pub source: String,
}

impl ParserContext {
    pub fn new(source: String) -> Self {
        Self { source }
    }
}

// Parse Pest pairs into our AST
pub fn parse_expr(pair: Pair<Rule>, ctx: &ParserContext) -> Result<Expr> {
    match pair.as_rule() {
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner, ctx)
        }
        Rule::binary_expr => {
            let mut pairs = pair.into_inner();
            let left = parse_expr(pairs.next().unwrap(), ctx)?;
            let op_pair = pairs.next().unwrap();
            let op = match op_pair.as_str() {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                "==" => Operator::Equal,
                _ => {
                    return Err(TungError::Parse {
                        src: ctx.source.clone(),
                        span: get_span_from_pair(&op_pair),
                        message: format!("Unknown operator: {}", op_pair.as_str()),
                    }
                    .into())
                }
            };
            let right = parse_expr(pairs.next().unwrap(), ctx)?;

            Ok(Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        }
        Rule::term => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner, ctx)
        }
        Rule::string => {
            // Remove the quotation marks from the string
            let text = pair.as_str();
            if text.len() >= 2 {
                let text_without_quotes = &text[1..text.len() - 1];
                Ok(Expr::String(text_without_quotes.to_string()))
            } else {
                Err(TungError::Parse {
                    src: ctx.source.clone(),
                    span: get_span_from_pair(&pair),
                    message: "Empty string detected".to_string(),
                }
                .into())
            }
        }
        Rule::number => match pair.as_str().parse() {
            Ok(num) => Ok(Expr::Number(num)),
            Err(_) => Err(TungError::Parse {
                src: ctx.source.clone(),
                span: get_span_from_pair(&pair),
                message: "Invalid number format".to_string(),
            }
            .into()),
        },
        Rule::identifier => Ok(Expr::Identifier(pair.as_str().to_string())),
        _ => Err(TungError::Parse {
            src: ctx.source.clone(),
            span: get_span_from_pair(&pair),
            message: format!("Unexpected rule: {:?}", pair.as_rule()),
        }
        .into()),
    }
}

// Add debugging output to trace parsing process
pub fn parse_statement(pair: Pair<Rule>, ctx: &ParserContext) -> Result<Statement> {
    println!("Parsing statement: {:?}", pair.as_str());
    match pair.as_rule() {
        Rule::statement => {
            let inner = pair.into_inner().next().unwrap();
            parse_statement(inner, ctx)
        }
        Rule::assignment => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap().as_str().to_string();
            println!("Parsed assignment name: {}", name);
            let expr = parse_expr(pairs.next().unwrap(), ctx)?;
            println!("Parsed assignment expression: {:?}", expr);
            Ok(Statement::Assignment { name, value: expr })
        }
        Rule::print_stmt => {
            let expr_pair = pair.into_inner().next().unwrap();
            let expr = parse_expr(expr_pair, ctx)?;
            println!("Parsed print statement: {:?}", expr);
            Ok(Statement::Print(expr))
        }
        Rule::if_stmt => {
            let mut pairs = pair.into_inner();
            let condition = parse_expr(pairs.next().unwrap(), ctx)?;
            println!("Parsed if condition: {:?}", condition);

            let mut then_block = Vec::new();
            let mut else_block = Vec::new();

            // Parse the then block until we hit 'else' or 'endif'
            let mut current_block = &mut then_block;

            for p in pairs {
                match p.as_rule() {
                    Rule::else_keyword => {
                        current_block = &mut else_block;
                    }
                    Rule::statement => {
                        current_block.push(parse_statement(p, ctx)?);
                    }
                    _ => {}
                }
            }

            println!("Parsed if statement: then_block={:?}, else_block={:?}", then_block, else_block);
            Ok(Statement::If {
                condition,
                then_block,
                else_block,
            })
        }
        Rule::expression => {
            let expr = parse_expr(pair, ctx)?;
            println!("Parsed expression: {:?}", expr);
            Ok(Statement::Expression(expr))
        }
        _ => Err(TungError::Parse {
            src: ctx.source.clone(),
            span: get_span_from_pair(&pair),
            message: format!("Unexpected rule: {:?}", pair.as_rule()),
        }
        .into()),
    }
}

// Helper function to get a span from a Pest Pair
fn get_span_from_pair(pair: &Pair<Rule>) -> SourceSpan {
    let span = pair.as_span();
    let start = span.start();
    let length = span.end() - start;
    (start, length).into()
}
