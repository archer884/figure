use crate::error::{Error, Result};
use num::rational::Rational64;
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(Rational64),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains('.') {
            Ok(Value::Float(s.parse()?))
        } else {
            Ok(Value::Integer(s.parse()?))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Value {
        match self {
            Value::Integer(left) => match rhs {
                Value::Integer(right) => Value::Integer(left + right),
                Value::Float(right) => Value::Float(Rational64::from(left) + right),
            }

            Value::Float(left) => match rhs {
                Value::Integer(right) => Value::Float(left + Rational64::from(right)),
                Value::Float(right) => Value::Float(left + right),
            }
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Value {
        match self {
            Value::Integer(left) => match rhs {
                Value::Integer(right) => Value::Integer(left / right),
                Value::Float(right) => Value::Float(Rational64::from(left) / right),
            }

            Value::Float(left) => match rhs {
                Value::Integer(right) => Value::Float(left / Rational64::from(right)),
                Value::Float(right) => Value::Float(left / right),
            }
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Value {
        match self {
            Value::Integer(left) => match rhs {
                Value::Integer(right) => Value::Integer(left * right),
                Value::Float(right) => Value::Float(Rational64::from(left) * right),
            }

            Value::Float(left) => match rhs {
                Value::Integer(right) => Value::Float(left * Rational64::from(right)),
                Value::Float(right) => Value::Float(left * right),
            }
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Value {
        match self {
            Value::Integer(left) => match rhs {
                Value::Integer(right) => Value::Integer(left - right),
                Value::Float(right) => Value::Float(Rational64::from(left) - right),
            }

            Value::Float(left) => match rhs {
                Value::Integer(right) => Value::Float(left - Rational64::from(right)),
                Value::Float(right) => Value::Float(left - right),
            }
        }
    }
}
