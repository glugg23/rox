use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        matches!(self, Value::Nil | Value::Boolean(false))
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_falsey() {
        assert!(Value::Nil.is_falsey());
        assert!(Value::Boolean(false).is_falsey());
    }

    #[test]
    fn value_is_truthy() {
        assert_eq!(Value::Boolean(true).is_falsey(), false);
        assert_eq!(Value::Number(0.0).is_falsey(), false);
    }

    #[test]
    fn value_into_valid_number() {
        let result: f64 = Value::Number(1.0).into();

        assert_eq!(result, 1.0);
    }

    #[test]
    #[should_panic]
    fn value_into_nil_panics() {
        let _: f64 = Value::Nil.into();
    }

    #[test]
    #[should_panic]
    fn value_into_bool_panics() {
        let _: f64 = Value::Boolean(false).into();
    }
}
