extern crate pest;
use super::*;
use crate::value::*;

pub fn boolean_parser(bool_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Boolean(bool_node.as_str().parse::<bool>().unwrap()))
}

pub fn string_parser(string_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Str(string_node.as_str().into()))
}

pub fn number_parser(number_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Int(number_node.as_str().parse::<i32>().unwrap()))
}

pub fn unary_parser(pair: pest::iterators::Pair<Rule>, child: AstNode) -> AstNode {
    AstNode::Expression(Expression::Unary(Unary {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }))
}

pub fn binary_parser(op: Operator, lhs: AstNode, rhs: AstNode) -> AstNode {
    AstNode::Expression(Expression::Binary(Binary {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }))
}
