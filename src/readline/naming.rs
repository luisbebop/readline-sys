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
