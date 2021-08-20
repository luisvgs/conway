use std::io::{self, Write};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::{ ErrorVariant,Error };
use pest::Parser;

#[cfg(test)]
pub mod conway_test;

pub mod interpreter;
use interpreter::*;

pub mod conway;
#[allow(unused_imports)]
use conway::*;

pub mod expression;
use expression::*;

pub mod environment;
use environment::*;

pub mod utils;
#[allow(unused_imports)]
use utils::*;

pub mod value;
use value::{Operator, Value};

pub mod ast;
use ast::*;

pub mod parser;
use parser::*;

pub type Result<T> = std::result::Result<T, Error<Rule>>;

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


pub fn build_ast_from_unary(literal: pest::iterators::Pair<Rule>) -> AstNode {
    let mut pair = literal.into_inner();
    let op = pair.next().unwrap();
    let child_pair = pair.next().unwrap();
    let child = build_ast_from_literal(child_pair);
    unary_parser(op, child)
}

pub fn block_parser(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let mut block = Vec::new();
    
    for node in pair.into_inner() {
        match node.as_rule() {
            Rule::Statement => block.push(Box::new(build_ast_from_expr(node))),
            _ => unreachable!(),
        }
    }

    AstNode::Block(Block { stmts: block })
}
/// Consumes a given Rule and returns its representation in the AST
pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Block => block_parser(pair),
        Rule::Statement => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Variable => variable_parser(pair),
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Unary => build_ast_from_unary(pair),
        Rule::ReAssign => assignment_parser(pair),
        Rule::Literal => build_ast_from_literal(pair),
        Rule::Print => print_parser(pair),
        Rule::Identifier => identifier_parser(pair),
        Rule::Assignment => assignment_parser(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn main() -> Result<()> {
    // let conway = Conway {};
    // conway.run();
    let unparsed_file = std::fs::read_to_string("conway.cy").expect("cannot read conway file");
    let astnode = parser(&unparsed_file).expect("unsuccessful parse");
    let mut int = Interpreter {
        env: Rc::new(RefCell::new(Environment::new())),
    };
    
    for node in astnode.into_iter() {
        println!("{}", int.eval(&node)?);
    }

    Ok(())
}
