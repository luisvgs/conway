use std::io::{self, Write};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;

pub mod conway;
use conway::*;

pub mod expression;
use expression::*;

pub mod environment;
use environment::*;

pub mod value;
use value::{Operator, Value};

pub mod parser;
use parser::*;

#[derive(Parser)]
#[grammar = "conway.pest"]
pub struct ConwayParser;

/// Grammar's AST representation
#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Literal(Value),
    Expression(Expression),
}

/// Get operator utility function
pub fn get_operator(pair: pest::iterators::Pair<Rule>) -> Operator {
    match pair.as_str() {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "!" => Operator::Bang,
        _ => unreachable!(),
    }
}

pub fn build_ast_from_unary(literal: pest::iterators::Pair<Rule>) -> AstNode {
    let mut pair = literal.into_inner();
    let op = pair.next().unwrap();
    let child_pair = pair.next().unwrap();
    let child = build_ast_from_literal(child_pair);
    unary_parser(op, child)
}

fn parse_variable(expr: pest::iterators::Pair<Rule>) -> AstNode {
    let mut identifier = String::new();
    let mut val = Box::new(AstNode::Expression(Expression::Null));

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::Identifier => identifier = String::from(node.as_str()),
            Rule::Expr => {
                val = Box::new(build_ast_from_expr(node));
            }
            _ => unreachable!(),
        }
    }

    AstNode::Expression(Expression::Variable(Variable{
        identifier,
        value: val,
    }))
}

/// Consumes a given Rule and returns its representation in the AST
pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Statement => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Unary => build_ast_from_unary(pair),
        Rule::Literal => build_ast_from_literal(pair),
        // Rule::Binary => {
        //     let mut binary_expr = pair.into_inner();
        //     let lhs = build_ast_from_literal(binary_expr.next().unwrap());
        //     let op = get_operator(binary_expr.next().unwrap());
        //     let rhs = build_ast_from_literal(binary_expr.next().unwrap());
        //     binary_parser(op, lhs, rhs)
        // }
        Rule::Variable => parse_variable(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

/// Consumes a Rule, and run any of the following parsers if it matches.
pub fn build_ast_from_literal(literal: pest::iterators::Pair<Rule>) -> AstNode {
    let inner = literal.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::Str => string_parser(inner),
        Rule::Integer => number_parser(inner),
        Rule::Boolean => boolean_parser(inner),
        unknown => panic!("Unknown expression: {:?}", unknown),
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = ConwayParser::parse(Rule::Program, source)?;
    // dbg!(&pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::Statement => {
                ast.push(build_ast_from_expr(pair));
            }
            _ => {}
        }
    }
    Ok(ast)
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn eval(&mut self, node: &AstNode) -> Value {
        match node {
            AstNode::Expression(Expression::Variable( Variable { identifier, value })) => {
                let val = self.eval(value);
                self.env.define(identifier.clone(), val.clone());
                val
            }
            AstNode::Literal(l) => match l {
                Value::Str(s) => Value::Str(s.to_string()),
                Value::Int(i) => Value::Int(*i),
                Value::Boolean(b) => Value::Boolean(*b),
            },
            AstNode::Expression(Expression::Unary(Unary { op, child })) => {
                let child = self.eval(&child);
                match op {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                    Operator::Bang => !child,
                }
            }
            _ => unreachable!(), // AstNode::Expression(Expression::Binary(Binary { op, lhs, rhs })) => {
                                 //     let lhs_ret = self.eval(&lhs);
                                 //     let rhs_ret = self.eval(&rhs);

                                 //     match op {
                                 //         Operator::Plus => {
                                 //             let x: i32 = match lhs_ret {
                                 //                 Value::Int(x) => x,
                                 //                 _ => panic!(),
                                 //             };

                                 //             let y: i32 = match rhs_ret {
                                 //                 Value::Int(y) => y,
                                 //                 _ => panic!(),
                                 //             };

                                 //             Value::Int(x + y)
                                 //         }

                                 //         Operator::Minus => {
                                 //             let x: i32 = match lhs_ret {
                                 //                 Value::Int(x) => x,
                                 //                 _ => panic!(),
                                 //             };

                                 //             let y: i32 = match rhs_ret {
                                 //                 Value::Int(y) => y,
                                 //                 _ => panic!(),
                                 //             };
                                 //             Value::Int(x - y)
                                 //         }
                                 //         _ => unreachable!(),
                                 //     }
                                 // }
        }
    }
}

fn main() {
    let conway = Conway {};
    conway.run()
    // let unparsed_file = std::fs::read_to_string("conway.cy").expect("cannot read conway file");
    // println!("{:?}", int.env.get_var("var".to_string()));
}
