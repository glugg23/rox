use crate::chunk::Chunk;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    String(Box<String>),
    Function(Box<Function>),
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ObjectType::String(s) => write!(f, "{}", s),
            ObjectType::Function(func) => write!(f, "{}", func),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    arity: i32,
    chunk: Chunk,
    name: Box<String>,
}

impl Function {
    pub fn new() -> Self {
        Function {
            arity: 0,
            name: Box::new("".to_owned()),
            chunk: Chunk::new(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.name == Box::new("".to_owned()) {
            write!(f, "<script>")
        } else {
            write!(f, "<fn {}>", self.name)
        }
    }
}
