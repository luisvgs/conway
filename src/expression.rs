use super::AstNode;
use crate::value::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unary(Unary),
    // Binary(Binary),
    Assignment(Assignment),
    Null
}

#[derive(PartialEq, Debug, Clone)]
pub struct Assignment {
    pub identifier: String,
    pub value: Box<AstNode>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Unary {
    pub op: Operator,
    pub child: Box<AstNode>,
}

// #[derive(PartialEq, Debug, Clone)]
// pub struct Binary {
//     pub lhs: Box<AstNode>,
//     pub op: Operator,
//     pub rhs: Box<AstNode>,
// }
