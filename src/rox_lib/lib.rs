use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod scanner;
pub mod vm;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => panic!("Value {} is not a number", self),
        }
    }
}

#[derive(Debug)]
pub struct RoxError {
    pub message: String,
    pub token: String,
    pub line: i32,
}

impl RoxError {
    pub fn new(message: &str, token: String, line: i32) -> Self {
        RoxError {
            message: message.to_string(),
            token,
            line,
        }
    }
}

impl Display for RoxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error at '{}': {}",
            self.line, self.token, self.message
        )
    }
}

impl Error for RoxError {}
