extern crate pest;
use super::*;

#[derive(Debug)]
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn eval(&mut self, node: &AstNode) -> Result<Value> {
        match node {
            AstNode::Print(Print { expr }) => {
                let evaluated_expression = self.eval(expr)?;
                Ok(evaluated_expression)
            }
            AstNode::Expression(Expression::Identifier(identifier)) => {
                Ok(self.env.get_var(identifier.to_owned()))
            }
            AstNode::Expression(Expression::Variable(variable_name)) => {
                self.env.define(variable_name.to_owned(), Value::Nil);
                Ok(Value::Nil)
            }
            AstNode::Expression(Expression::Assignment(Assignment { identifier, value })) => {
                let val = self.eval(value)?;
                if self.env.vals.contains_key(identifier) {
                    self.env.vals.insert(identifier.to_owned(), val);
                    Ok(Value::Nothing)
                } else {  
                    self.env.define(identifier.to_owned(), val);
                    Ok(Value::Nil)
                }
            }
            AstNode::Literal(l) => match l {
                Value::Str(s) => Ok(Value::Str(s.to_string())),
                Value::Int(i) => Ok(Value::Int(*i)),
                Value::Boolean(b) => Ok(Value::Boolean(*b)),
                _ => unreachable!(),
            },
            AstNode::Expression(Expression::Unary(Unary { op, child })) => {
                let child = self.eval(&child)?;
                match op {
                    Operator::Plus => Ok(child),
                    Operator::Minus => Ok(-child),
                    Operator::Bang => Ok(!child),
                }
            }
            _ => unreachable!(),
        }
    }
}
