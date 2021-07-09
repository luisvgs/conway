extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;

pub mod expression;
use expression::*;

pub mod value;
use value::{Operator, Value};

pub mod parser;
use parser::*;

#[derive(Parser)]
#[grammar = "conway.pest"]
pub struct ConwayParser;

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Literal(Value),
    Expression(Expression),
}

pub fn get_operator(pair: pest::iterators::Pair<Rule>) -> Operator {
    match pair.as_str() {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        _ => unreachable!(),
    }
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
        Rule::Binary => {
            let lhs = build_ast_from_literal(pair.clone().into_inner().next().unwrap());
            let op = get_operator(pair.clone().into_inner().next().unwrap());
            let rhs = build_ast_from_literal(pair.clone().into_inner().next().unwrap());
            binary_parser(op, lhs, rhs)
        }
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
                Value::Str(s) => Value::Str(s.to_string()),
                Value::Int(i) => Value::Int(*i),
                Value::Boolean(b) => Value::Boolean(*b),
            }
            AstNode::Expression(Expression::Unary(Unary { op, child })) => {
                let child = self.eval(&child);
                match op {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                }
            }
            AstNode::Expression(Expression::Binary(Binary { op, lhs, rhs })) => {
                let lhs_ret = self.eval(&lhs);
                let rhs_ret = self.eval(&rhs);

                match op {
                    Operator::Plus => {
                        let x: i32 = match lhs_ret {
                            Value::Int(x) => x,
                            _ => panic!(),
                        };

                        let y: i32 = match rhs_ret {
                            Value::Int(y) => y,
                            _ => panic!(),
                        };

                        Value::Int(x + y)
                    }

                    Operator::Minus => {
                        let x: i32 = match lhs_ret {
                            Value::Int(x) => x,
                            _ => panic!(),
                        };

                        let y: i32 = match rhs_ret {
                            Value::Int(y) => y,
                            _ => panic!(),
                        };
                        Value::Int(x - y)
                    }
                }
            }
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
    fn got_true() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("true")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(true)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn got_false() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("false")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(false)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn integer_parses() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("1285")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(1285)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn negative_parses() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("-41")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(-41)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }
    #[test]
    fn binary_parses() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("55+13;")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(68)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }
}
