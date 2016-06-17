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

//! [2.4.11 Miscellaneous Functions]
//! [2.4.11 miscellaneous functions]: https://goo.gl/2BieXb
use std::ffi::{CStr, CString};

mod ext_misc {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn rl_macro_dumper(readable: c_int) -> ();
        pub fn rl_variable_bind(name: *const c_char, value: *const c_char) -> c_int;
        pub fn rl_variable_value(name: *const c_char) -> *const c_char;
        pub fn rl_variable_dumper(readable: c_int) -> ();
        pub fn rl_set_paren_blink_timeout(us: c_int) -> c_int;
        pub fn rl_get_termcap(cap: *const c_char) -> *const c_char;
        pub fn rl_clear_history() -> ();
    }
}

/// Print the key sequences bound to macros and their values, using the current keymap, to
/// `rl_outstream`. If `readable` is true, the list is formatted in such a way that it can be made
/// part of an inputrc file and re-read.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{misc, util};
///
/// util::init();
///
/// misc::macro_dumper(true);
/// ```
pub fn macro_dumper(readable: bool) -> () {
    let read = if readable { 1 } else { 0 };

    unsafe { ext_misc::rl_macro_dumper(read) }
}

/// Make the Readline variable `name` have `value`. This behaves as if the readline command
/// `set variable value` had been executed in an inputrc file (see section
/// [1.3.1 Readline Init File Syntax]).
/// [1.3.1 readline init file syntax]: https://goo.gl/Ivqovs
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{misc, util};
///
/// util::init();
///
/// assert!(misc::variable_bind("comment-begin", "<").is_ok());
/// ```
pub fn variable_bind(name: &str, val: &str) -> Result<i32, ::ReadlineError> {
    let csname = try!(CString::new(name));
    let csval = try!(CString::new(val));

    unsafe { Ok(ext_misc::rl_variable_bind(csname.as_ptr(), csval.as_ptr())) }
}

/// Return a string representing the value of the Readline variable `name`. For boolean variables,
/// this string is either `on' or `off'.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{misc, util};
///
/// util::init();
///
/// match misc::variable_value("comment-begin") {
///     Ok(val) => assert!(val == "#"),
///     Err(_)  => assert!(false),
/// }
///
/// // boolean style
/// match misc::variable_value("output-meta") {
///     Ok(val) => assert!(val == "on"),
///     Err(_)  => assert!(false),
/// }
/// ```
pub fn variable_value(name: &str) -> Result<String, ::ReadlineError> {
    let csname = try!(CString::new(name));

    let val_ptr = unsafe { ext_misc::rl_variable_value(csname.as_ptr()) };
    if val_ptr.is_null() {
        Err(::ReadlineError::new("Misc Error",
                                 "Null pointer returned from rl_variable_value!"))
    } else {
        Ok(unsafe { CStr::from_ptr(val_ptr).to_string_lossy().into_owned() })
    }
}

/// Print the readline variable names and their current values to `rl_outstream`. If readable is
/// true, the list is formatted in such a way that it can be made part of an inputrc file and
/// re-read.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{misc, util};
///
/// util::init();
///
/// misc::variable_dumper(true);
/// ```
pub fn variable_dumper(readable: bool) -> () {
    let read = if readable { 1 } else { 0 };

    unsafe { ext_misc::rl_variable_dumper(read) }
}

/// Set the time interval (in microseconds) that Readline waits when showing a balancing character
/// when `blink-matching-paren` has been enabled.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{misc, util};
///
/// util::init();
///
/// assert!(misc::set_paren_blink_timeout(100000).is_ok());
/// ```
pub fn set_paren_blink_timeout(us: i32) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_misc::rl_set_paren_blink_timeout(us);

        if res >= 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Misc Error", "Unable to set paren blink timeout!"))
        }
    }
}

/// Retrieve the string value of the termcap capability `cap`. Readline fetches the termcap entry
/// for the current terminal name and uses those capabilities to move around the screen line and
/// perform other terminal-specific operations, like erasing a line. Readline does not use all of a
/// terminal's capabilities, and this function will return values for only those capabilities
/// Readline uses.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{misc, util};
///
/// util::init();
///
/// match misc::get_termcap("vb") {
///     Ok(s)  => assert!(!s.is_empty()),
///     Err(_) => assert!(false),
/// }
/// ```
pub fn get_termcap(cap: &str) -> Result<String, ::ReadlineError> {
    let cscap = try!(CString::new(cap));

    let cap_ptr = unsafe { ext_misc::rl_get_termcap(cscap.as_ptr()) };
    if cap_ptr.is_null() {
        Err(::ReadlineError::new("Misc Error", "rl_get_termcap returned a null pointer!"))
    } else {
        Ok(unsafe { CStr::from_ptr(cscap.as_ptr()).to_string_lossy().into_owned() })
    }
}

/// Clear the history list by deleting all of the entries, in the same manner as the History
/// library's `clear_history()` function. This differs from `clear_history` because it frees
/// private data Readline saves in the history list.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::misc;
///
/// misc::clear_history();
/// ```
pub fn clear_history() -> () {
    unsafe { ext_misc::rl_clear_history() }
}
