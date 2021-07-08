extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;

pub mod value;
use value::{Value, Operator};

#[derive(Parser)]
#[grammar = "conway.pest"]
struct ConwayParser;

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
    AstNode::Expression(Expression::Unary {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    })
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Literal(Value),
    Expression(Expression),
}


#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unary {
        op: Operator,
        child: Box<AstNode>,
    },
}

pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Literal => build_ast_from_literal(pair.into_inner().next().unwrap()),
        Rule::Unary => match pair.as_rule() {
            Rule::Operator => {
                let mut pair = pair.into_inner();
                let op = pair.next().unwrap();
                let child = pair.next().unwrap();
                let child = build_ast_from_literal(child);
                unary_parser(op, child)
            }
            _ => unreachable!(),
        },
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

pub fn build_ast_from_literal(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Str => string_parser(pair),
        Rule::Integer => number_parser(pair),
        Rule::Boolean => boolean_parser(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = ConwayParser::parse(Rule::Program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::Expr => {
                ast.push(build_ast_from_expr(pair));
            }
            _ => {}
        }
    }
    Ok(ast)
}

pub struct Interpreter {}
impl Interpreter {
    pub fn eval(&self, node: &AstNode) -> Value {
        match node {
            AstNode::Literal(l) => match l {
                Value::Str(l) => Value::Str(l.to_string()),
                Value::Int(l) => Value::Int(*l),
                Value::Boolean(b) => Value::Boolean(*b),
            },
            AstNode::Expression(Expression::Unary { op, child }) => {
                let child = self.eval(&child);
                match op {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                }
            },
        }
    }
}

fn main() {
    let unparsed_file = std::fs::read_to_string("conway.cy").expect("cannot read conway file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    let int = Interpreter {};
    for node in astnode.into_iter() {
        println!("{}", int.eval(&node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numer_parses() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("133")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(133)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }
}
