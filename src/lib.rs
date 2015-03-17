#![feature(custom_derive, path, io, collections, core, libc)]
extern crate collections;
extern crate toml;
extern crate libc;

mod error;

use error::Error;

use std::fs::File;
use std::ffi::CString;
use libc::{c_char, c_int, size_t};
use std::borrow::ToOwned;
use std::path::AsPath;
use std::io::{ self, Read };
use std::error::FromError;
use std::fmt;
use std::error::Error as StdError;
use std::str;

extern {
    // fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(into: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
}

#[derive(Debug)]
pub struct R18n {
    dict: toml::Value
}

#[derive(Debug, PartialEq)]
pub enum LookupError {
    NotFound,
    NotASlice,
    NotAString,
    CountNotFound,
    InputNulError
}

impl fmt::Display for  LookupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for LookupError {
    fn description(&self) -> &str {
        use LookupError as E;
        match *self {
            E::NotFound => "not found",
            E::NotASlice => "not a slice",
            E::NotAString => "not a string",
            E::CountNotFound => "to few elements",
            E::InputNulError => "string contains forbidden nul"
        }
    }
}

impl R18n {
    pub fn t<'a>(&'a self, code: &'a str) -> Result<&'a str, LookupError> {
        match self.dict.lookup(code) {
            None => return Err(LookupError::NotFound),
            Some(v) => Ok(match v.as_str() {
                Some(s) => s,
                None    => return Err(LookupError::NotAString)
            })
        }
    }

    pub fn c(&self, code: &str, count: usize) -> Result<String, LookupError> {
        let v = match self.dict.lookup(code) {
            None => return Err(LookupError::NotFound),
            Some(v) => match v.as_slice() {
                None => return Err(LookupError::NotASlice),
                Some(v) => v
            }
        };

        let idx = if count == 10 { 0 } else { std::cmp::min(v.len(), count) - 1 };

        let s = match v[idx].as_str() {
            None => return Err(LookupError::CountNotFound),
            Some(s) => s
        };

        let input = CString::new(s).unwrap();

        let mut output = [0u8; 8192];

        let end = unsafe {
            snprintf(output.as_mut_ptr() as *mut _, output.len() as size_t, input.as_ptr() as *const _, count)
        } as usize;

        // let end = std::cmp::min(output.len() - 1, end as usize);

        let slice = std::str::from_utf8(&output[..end]).unwrap();
        Ok(slice.to_owned())
    }
}

impl FromError<Vec<toml::ParserError>> for Error{
    fn from_error(err: Vec<toml::ParserError>) -> Error {
        Error::new()
    }
}

impl FromError<io::Error> for Error{
    fn from_error(err: io::Error) -> Error {
        Error::new()
    }
}

pub fn load<P: AsPath + ?Sized>(path: &P) -> Result<R18n, Error> {
    let mut f = try!(File::open(path));
    let mut toml = String::new();
    try!(f.read_to_string(&mut toml));

    toml.parse()
}

impl str::FromStr for R18n {
    type Err = Error;
    fn from_str(s: &str) -> Result<R18n, Error> {
        let value : toml::Value = try!(s.parse());
        Ok(R18n { dict: value })
    }
}
