extern crate pest;
use super::*;
use crate::value::*;

/// Get operator utility function
pub fn get_operator(pair: pest::iterators::Pair<Rule>) -> Operator {
    match pair.as_str() {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "!" => Operator::Bang,
        _ => unreachable!(),
    }
}
