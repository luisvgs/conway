extern crate pest;
use super::*;
use std::rc::Rc;
use std::cell::RefCell;

// pub type ConwayResult<T> = std::result::Result<T, ConwayError>;

#[derive(Debug)]
pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn execute_block(&mut self, stmts: &Vec<Box<AstNode>>, env: Rc<RefCell<Environment>> ) -> Result<Value> {
        let previous = Rc::clone(&self.env);
        let mut value = Value::Nil;

        self.env = env;
        
        for stmt in stmts {
            value = self.eval(stmt)?;
        }

        self.env = previous;
        Ok(value)
    }
    pub fn eval(&mut self, node: &AstNode) -> Result<Value> {
        match node {
            AstNode::Block(Block { stmts }) => {
                let env_ref = Rc::clone(&self.env);

                self.execute_block(stmts, Rc::new(RefCell::new(Environment::with_ref(env_ref))))
            }
            AstNode::Print(Print { expr }) => {
                let evaluated_expression = self.eval(expr)?;
                Ok(evaluated_expression)
            }
            AstNode::Expression(Expression::Identifier(identifier)) => {
                Ok(self.env.borrow_mut().get_var(identifier.to_owned()))
            }
            AstNode::Expression(Expression::Variable(variable_name)) => {
                self.env.borrow_mut().define(variable_name.to_owned(), Value::Nil);
                Ok(Value::Nil)
            }
            AstNode::Expression(Expression::Assignment(Assignment { identifier, value })) => {
                let val = self.eval(value)?;
                if self.env.borrow().vals.contains_key(identifier) {
                    self.env.borrow_mut().vals.insert(identifier.to_owned(), val.clone());
                    Ok(val.clone())
                    // Ok(Value::Nothing)
                } else {  
                    self.env.borrow_mut().define(identifier.to_owned(), val.clone());
                    Ok(val.clone())
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
