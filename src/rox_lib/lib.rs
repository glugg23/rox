use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

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
