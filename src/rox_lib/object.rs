use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    String(Box<String>),
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ObjectType::String(s) => write!(f, "{}", s),
        }
    }
}
