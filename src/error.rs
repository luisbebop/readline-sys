// Copyright 2015-2016 Jason Ozias
//
// This file is part of rl-sys.
//
// rl-sys is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rl-sys is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rl-sys.  If not, see <http://www.gnu.org/licenses/>.

//! rl-sys API Error Handling
use std::fmt;

#[derive(Debug)]
/// Represents an error that has occurred within the Readline API.
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
    pub fn new<T>(desc: &str, detail: T) -> ReadlineError
        where T: fmt::Display
    {
        ReadlineError {
            desc: String::from(desc),
            detail: format!("{}", detail),
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

#[derive(Debug)]
/// Represents an error that has occurred within the History API.
pub struct HistoryError {
    desc: String,
    detail: String,
}

impl HistoryError {
    /// Create a HistoryError struct from the given description and detail.
    pub fn new<T>(desc: &str, detail: T) -> HistoryError
        where T: fmt::Display
    {
        HistoryError {
            desc: String::from(desc),
            detail: format!("{}", detail),
        }
    }
}

/// Implemented as 'self.desc: self.detail'.
impl fmt::Display for HistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.desc, self.detail)
    }
}

impl From<::std::ffi::NulError> for HistoryError {
    fn from(e: ::std::ffi::NulError) -> HistoryError {
        HistoryError::new("NulError", e)
    }
}

impl From<::std::str::Utf8Error> for HistoryError {
    fn from(e: ::std::str::Utf8Error) -> HistoryError {
        HistoryError::new("FromUtf8Error", e)
    }
}

impl From<::std::num::ParseIntError> for HistoryError {
    fn from(e: ::std::num::ParseIntError) -> HistoryError {
        HistoryError::new("ParseIntError", e)
    }
}
