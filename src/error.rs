//! rl-sys API Error Handling
use std::fmt;

#[derive(Debug)]
/// Represents an error that has occurred within the API.
pub struct ReadlineError {
    desc: String,
    detail: String,
}

/// Implemented as 'self.desc: self.detail'.
impl fmt::Display for ReadlineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.desc, self.detail)
    }
}

impl ReadlineError {
    /// Create a ReadlineError struct from the given description and detail.
    pub fn new<T>(desc: &str, detail: T) -> ReadlineError where T: fmt::Debug {
        ReadlineError {
            desc: String::from(desc),
            detail: format!("{:?}", detail),
        }
    }
}

impl From<::std::ffi::NulError> for ReadlineError {
    fn from(e: ::std::ffi::NulError) -> ReadlineError {
        ReadlineError::new("NulError", e)
    }
}

impl From<::std::str::Utf8Error> for ReadlineError {
    fn from(e: ::std::str::Utf8Error) -> ReadlineError {
        ReadlineError::new("FromUtf8Error", e)
    }
}

impl From<::std::io::Error> for ReadlineError {
    fn from(e: ::std::io::Error) -> ReadlineError {
        ReadlineError::new("I/O Error", e)
    }
}
