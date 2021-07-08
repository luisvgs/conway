extern crate pest;
use super::{AstNode, Expression};
use crate::value::*;

pub fn string_parser(string_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Str(string_node.as_str().into()))
}

pub fn number_parser(number_node: Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Int(number_node.as_str().parse::<i32>().unwrap()))
}

pub fn unary_parser(pair: pest::iterators::Pair<Rule>, child: AstNode) -> AstNode {
    AstNode::Expression(Expression::Unary {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    })
}
