use super::*;

/// Grammar's AST representation
#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Literal(Value),
    Expression(Expression),
    Print(Print),
}
