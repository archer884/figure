use num::rational::ParseRatioError;
use std::error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(Box<error::Error>),
    Eval(&'static str),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::Parse(Box::new(e))
    }
}

impl From<ParseRatioError> for Error {
    fn from(e: ParseRatioError) -> Error {
        Error::Parse(Box::new(e))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(source) => write!(f, "Error in parsing: {}", source),
            Error::Eval(message) => write!(f, "Error in evaluation: {}", message),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Parse(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}
