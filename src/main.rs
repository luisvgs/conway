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
        _ => unreachable!(),
    }
}

pub fn build_ast_from_unary(literal: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = literal.into_inner().next().unwrap();
    println!("{:?}", pair);
    match pair.as_rule() {
        Rule::Operator => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_literal(child);
            unary_parser(op, child)
        },
        Rule::Unary => build_ast_from_expr(pair),
        Rule::Literal => build_ast_from_literal(pair),
        unknown => panic!("Unknown expression: {:?}", unknown),
    }
}

/// Consumes a given Rule and returns its representation in the AST
pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Unary => build_ast_from_unary(pair),
        // Rule::Binary => {
        //     let mut binary_expr = pair.into_inner();
        //     let lhs = build_ast_from_literal(binary_expr.next().unwrap());
        //     let op = get_operator(binary_expr.next().unwrap());
        //     let rhs = build_ast_from_literal(binary_expr.next().unwrap());
        //     binary_parser(op, lhs, rhs)
        // }
        // Rule::Assignment => {
        //     // Not tested yet.
        //     let mut assignment_expr = pair.into_inner();
        //     let _let_keyword = assignment_expr.next().unwrap();
        //     let identifier = assignment_expr.next().unwrap();
        //     AstNode::Expression(Expression::Assignment(Assignment {
        //         identifier: identifier.to_string(),
        //     }))
        // }
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

/// Interpreter just to hold the evaluation logic.
/// The eval function takes an AST node and returns its representation as a Conway value.
pub struct Interpreter {}
impl Interpreter {
    pub fn eval(&self, node: &AstNode) -> Value {
        match node {
            // AstNode::Expression(Expression::Assignment(Assignment { identifier })) => {
            //     Value::Str(identifier.to_string())
            // }
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
                }
            }
            // AstNode::Expression(Expression::Binary(Binary { op, lhs, rhs })) => {
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
            //     }
            // }
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

/// Some tests. Might need to separate them in a different file later.
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
