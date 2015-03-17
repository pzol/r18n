use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    detail: Option<String>
}

impl Error {
    pub fn new() -> Error {
        Error { detail: None }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "Error"
    }
}
