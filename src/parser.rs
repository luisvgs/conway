extern crate pest;
use super::*;
use crate::value::*;


#[derive(Parser)]
#[grammar = "conway.pest"]
pub struct ConwayParser;

pub fn parser(source: &str) -> Result<Vec<AstNode>> {
    let mut ast = vec![];

    let pairs = ConwayParser::parse(Rule::Program, source)?;
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

/// Parses the Rule to get a boolean Node
pub fn boolean_parser(bool_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Boolean(bool_node.as_str().parse::<bool>().unwrap()))
}

/// Parses the Rule to get a string Node
pub fn string_parser(string_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Str(string_node.as_str().into()))
}

/// Parses the Rule to get an integer Node
pub fn number_parser(number_node: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Literal(Value::Int(number_node.as_str().parse::<i32>().unwrap()))
}

/// Parses the Rule to get an unary expression Node
pub fn unary_parser(pair: pest::iterators::Pair<Rule>, child: AstNode) -> AstNode {
    AstNode::Expression(Expression::Unary(Unary {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "!" => Operator::Bang,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }))
}

pub fn variable_parser(expr: pest::iterators::Pair<Rule>) -> AstNode {
    let mut identifier = String::new();

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::Identifier => identifier = String::from(node.as_str()),
            _ => unreachable!(),
        }
    }

    AstNode::Expression(Expression::Variable(identifier))
}

pub fn assignment_parser(expr: pest::iterators::Pair<Rule>) -> AstNode {
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

    AstNode::Expression(Expression::Assignment(Assignment {
        identifier,
        value: val,
    }))
}

pub fn identifier_parser(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let str = &pair.as_str();
    AstNode::Expression(Expression::Identifier(String::from(&str[..])))
}

pub fn print_parser(expr: pest::iterators::Pair<Rule>) -> AstNode {
    let mut expression = Box::new(AstNode::Expression(Expression::Null));

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::Expr => {
                expression = Box::new(build_ast_from_expr(node));
            }
            _ => unreachable!(),
        }
    }

    AstNode::Print(Print { expr: expression })
}



//Parses the Rule to get a binary expression Node
// pub fn binary_parser(op: Operator, lhs: AstNode, rhs: AstNode) -> AstNode {
//     AstNode::Expression(Expression::Binary(Binary {
//         lhs: Box::new(lhs),
//         op,
//         rhs: Box::new(rhs),
//     }))
// }
