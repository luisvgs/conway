use super::AstNode;
use crate::value::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unary(Unary),
    Variable(String),
    // Binary(Binary),
    Assignment(Assignment),
    Null,
    Identifier(String),
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

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Box<AstNode>>
}

#[derive(PartialEq, Debug, Clone)]
pub struct Print {
    pub expr: Box<AstNode>,
}
// #[derive(PartialEq, Debug, Clone)]
// pub struct Binary {
//     pub lhs: Box<AstNode>,
//     pub op: Operator,
//     pub rhs: Box<AstNode>,
// }
