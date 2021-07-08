#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{:?}", *x),
            Self::Str(s) => write!(f, "{:?}", *s),
            Self::Boolean(b) => write!(f, "{:?}", *b),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::Int(x) => Value::Int(-x),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;
    fn add(self, other: Value) -> Self {
        match self {
            Self::Int(x) => match other {
                Value::Int(y) => Value::Int(x + y),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Minus,
    Plus,
}
