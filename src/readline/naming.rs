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

//! [2.4.1 Naming a Function]
//! [2.4.1 naming a function]: https://goo.gl/MN1Yf9
//!
//! The user can dynamically change the bindings of keys while using Readline. This is done by
//! representing the function with a descriptive name. The user is able to type the descriptive name
//! when referring to the function. Thus, in an init file, one might find
//!
//! ```c
//! Meta-Rubout: backward-kill-word
//! ```
//!
//! This binds the keystroke `Meta-Rubout` to the function descriptively named `backward-kill-word`.
//! You, as the programmer, should bind the functions you write to descriptive names as well.
use readline::CommandFunction;
use std::ffi::CString;

mod ext_naming {
    use libc::{c_char, c_int};
    use readline::CommandFunction;

    extern "C" {
        pub fn rl_add_defun(name: *const c_char, f: *mut CommandFunction, key: c_int) -> c_int;
    }
}

/// Add `name` to the list of named functions. Make `f` be the function that gets called. If `key`
/// is not -1, then bind it to function using `rl_bind_key()`.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{naming, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// match naming::add_func("move-10-left", '\t', test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn add_func(name: &str, key: char, f: CommandFunction) -> Result<i32, ::ReadlineError> {
    unsafe {
        let csname = try!(CString::new(name));
        Ok(ext_naming::rl_add_defun(csname.as_ptr(), f as *mut CommandFunction, key as i32))
    }
}
