use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod scanner;
pub mod vm;

type Value = f64;

#[derive(Debug)]
pub struct RoxError {
    pub message: String,
    pub line: i32,
}

impl RoxError {
    pub fn new(message: &str, line: i32) -> Self {
        RoxError {
            message: message.to_string(),
            line,
        }
    }
}

impl Display for RoxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "'{}' on line {}.", self.message, self.line)
    }
}

impl Error for RoxError {}
