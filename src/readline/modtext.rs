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

//! [2.4.7 Modifying Text]
//! [2.4.7 modifying text]: https://goo.gl/DLJ9Gn
use std::ffi::{CStr, CString};

mod ext_modtext {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn rl_insert_text(text: *const c_char) -> c_int;
        pub fn rl_delete_text(start: c_int, end: c_int) -> c_int;
        pub fn rl_copy_text(start: c_int, end: c_int) -> *const c_char;
        pub fn rl_kill_text(start: c_int, end: c_int) -> c_int;
        pub fn rl_push_macro_input(m: *const c_char) -> ();
    }
}

fn genresult(res: i32, message: &str) -> Result<i32, ::ReadlineError> {
    if res == 0 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Modtext Error", message))
    }
}

/// Insert text into the line at the current cursor position. Returns the number of characters
/// inserted.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{modtext, util};
///
/// util::init();
///
/// assert!(modtext::insert_text("inserted").is_ok());
/// ```
pub fn insert_text(text: &str) -> Result<i32, ::ReadlineError> {
    let cstext = try!(CString::new(text));

    unsafe {
        let res = ext_modtext::rl_insert_text(cstext.as_ptr());

        if res > 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Modtext Error", "Unable to insert text!"))
        }
    }
}

/// Delete the text between `start` and `end` in the current line. Returns the number of characters
/// deleted.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{modtext, util};
///
/// util::init();
///
/// assert!(modtext::insert_text("inserted").is_ok());
/// assert!(modtext::delete_text(2, 5).is_ok());
/// ```
pub fn delete_text(start: i32, end: i32) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_modtext::rl_delete_text(start, end);

        if res > 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Modtext Error", "Unable to delete text!"))
        }
    }
}

/// Return a copy of the text between `start` and `end` in the current line.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{modtext, util};
///
/// util::init();
///
/// assert!(modtext::insert_text("inserted").is_ok());
/// match modtext::copy_text(2, 5) {
///     Ok(s)  => assert!(s == "ser"),
///     Err(_) => assert!(false),
/// }
/// ```
pub fn copy_text(start: i32, end: i32) -> Result<String, ::ReadlineError> {
    unsafe {
        let str_ptr = ext_modtext::rl_copy_text(start, end);

        if str_ptr.is_null() {
            Err(::ReadlineError::new("Modtext Error", "Unable to copy text!"))
        } else {
            Ok(CStr::from_ptr(str_ptr).to_string_lossy().into_owned())
        }
    }
}

/// Copy the text between `start` and `end` in the current line to the kill ring, appending or
/// prepending to the last kill if the last command was a kill command. The text is deleted. If
/// start is less than end, the text is appended, otherwise prepended. If the last command was not
/// a kill, a new kill ring slot is used.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{modtext, util};
///
/// util::init();
///
/// assert!(modtext::insert_text("inserted").is_ok());
/// assert!(modtext::kill_text(5, 2).is_ok());
/// ```
pub fn kill_text(start: i32, end: i32) -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_modtext::rl_kill_text(start, end),
                  "Unable to kill text!")
    }
}

/// Cause macro `m` to be inserted into the line, as if it had been invoked by a key bound to a
/// macro. Not especially useful; use `rl_insert_text()` instead.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{modtext, util};
///
/// util::init();
///
/// assert!(modtext::push_macro_input("\\C-e | less\\C-m").is_ok());
/// ```
pub fn push_macro_input(m: &str) -> Result<(), ::ReadlineError> {
    let csm = try!(CString::new(m));
    unsafe { Ok(ext_modtext::rl_push_macro_input(csm.as_ptr())) }
}
