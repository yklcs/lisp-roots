use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    ReadError(String),
    EvalError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::ReadError(e) => format!("read error: {}", e),
            Error::EvalError(e) => format!("eval error: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}
