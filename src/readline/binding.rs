//! [2.4.3 Binding Keys]
//! [2.4.3 binding keys]: https://goo.gl/fmY0qd
//!
//! Key sequences are associated with functions through the keymap. Readline has several internal
//! keymaps: `emacs_standard_keymap`, `emacs_meta_keymap`, `emacs_ctlx_keymap`,
//! `vi_movement_keymap`, and `vi_insertion_keymap`. `emacs_standard_keymap` is the default, and the
//! examples in this manual assume that.
//!
//! Since `readline()` installs a set of default key bindings the first time it is called, there is
//! always the danger that a custom binding installed before the first call to `readline()` will be
//! overridden. An alternate mechanism is to install custom key bindings in an initialization
//! function assigned to the `rl_startup_hook` variable (see section [2.3 Readline Variables]).
//! [2.3 readline variables]: https://goo.gl/E1D6om
use readline::{CommandFunction, Keymap};
use std::ffi::CString;

mod ext_binding {
    use libc::{c_char, c_int};
    use readline::{CommandFunction, Keymap};

    extern "C" {
        pub fn rl_bind_key(key: c_int, f: *mut Option<CommandFunction>) -> c_int;
        pub fn rl_bind_key_in_map(key: c_int,
                                  f: *mut Option<CommandFunction>,
                                  map: Keymap)
                                  -> c_int;
        pub fn rl_bind_key_if_unbound(key: c_int, f: *mut Option<CommandFunction>) -> c_int;
        pub fn rl_bind_key_if_unbound_in_map(key: c_int,
                                             f: *mut Option<CommandFunction>,
                                             map: Keymap)
                                             -> c_int;
        pub fn rl_unbind_key(key: c_int) -> c_int;
        pub fn rl_unbind_key_in_map(key: c_int, map: Keymap) -> c_int;
        pub fn rl_unbind_function_in_map(f: *mut Option<CommandFunction>, map: Keymap) -> c_int;
        pub fn rl_unbind_command_in_map(cmd: *const c_char, map: Keymap) -> c_int;
    }
}

fn genresult(res: i32, err: &str) -> Result<i32, ::ReadlineError> {
    if res == 0 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Bind Error", err))
    }
}

/// Binds `key` to `f` in the currently active keymap. Returns non-zero in the case of an invalid
/// key.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// match binding::bind_key('\t', &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn bind_key(key: char, f: *mut Option<CommandFunction>) -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_bind_key(key as i32, f),
                  "Unable to bind key!")
    }
}

/// Bind `key` to function `f` in `map`. Returns non-zero in the case of an invalid key.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_in_map('\t', km, &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn bind_key_in_map(key: char,
                       map: Keymap,
                       f: *mut Option<CommandFunction>)
                       -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_bind_key_in_map(key as i32, f, map),
                  "Unable to bind key in map!")
    }
}

/// Binds `key` to function `f` if it is not already bound in the currently active keymap. Returns
/// non-zero in the case of an invalid `key` or if `key` is already bound.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// #[no_mangle]
/// #[allow(private_no_mangle_fns)]
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// // Create an empty keymap to ensure ';' doesn't collide on first bind.
/// let keymap = keymap::create_empty().unwrap();
/// keymap::set(keymap);
///
/// match binding::bind_key_if_unbound(';', &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::bind_key_if_unbound(';', &mut Some(test_cmd_func)) {
///     Ok(_)  => assert!(false),
///     Err(_) => assert!(true),
/// }
/// # }
/// ```
pub fn bind_key_if_unbound(key: char,
                           f: *mut Option<CommandFunction>)
                           -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_bind_key_if_unbound(key as i32, f),
                  "Unable to bind key!")
    }
}

/// Binds `key` to function `f` if it is not already bound in `map`. Returns non-zero in the case of
/// an invalid `key` or if `key` is already bound.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::bind_key_if_unbound_in_map('\t', km, &mut Some(test_cmd_func)) {
///     Ok(_)  => assert!(false),
///     Err(_) => assert!(true),
/// }
/// # }
/// ```
pub fn bind_key_if_unbound_in_map(key: char,
                                  map: Keymap,
                                  f: *mut Option<CommandFunction>)
                                  -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_bind_key_if_unbound_in_map(key as i32, f, map),
                  "Unable to bind key in map!")
    }
}

/// Bind `key` to the null function in the currently active keymap. Returns non-zero in case of
/// error.
/// Binds `key` to function `f` if it is not already bound in `map`. Returns non-zero in the case of
/// an invalid `key` or if `key` is already bound.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::unbind_key('\t') {
///     Ok(res)  => assert!(res == 0),
///     Err(_) => assert!(false),
/// }
/// # }
/// ```
pub fn unbind_key(key: char) -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_unbind_key(key as i32),
                  "Unable to unbind key!")
    }
}

/// Bind `key` to the null function in `map`. Returns non-zero in case of error.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::unbind_key_in_map('\t', km) {
///     Ok(res)  => assert!(res == 0),
///     Err(_) => assert!(false),
/// }
/// # }
/// ```
pub fn unbind_key_in_map(key: char, map: Keymap) -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_unbind_key_in_map(key as i32, map),
                  "Unable to unbind key in map!")
    }
}

/// Unbind all keys that execute function in map.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(count: c_int, key: c_int) -> c_int {
///   println!("{:?}, {:?}", count, key);
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, &mut Some(test_cmd_func)) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::unbind_function_in_map(&mut Some(test_cmd_func), km) {
///     Ok(res)  => assert!(res == 0),
///     Err(_) => assert!(false),
/// }
/// # }
/// ```
pub fn unbind_function_in_map(f: *mut Option<CommandFunction>,
                              map: Keymap)
                              -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_binding::rl_unbind_function_in_map(f, map),
                  "Unable to unbind key in map!")
    }
}

/// Unbind all keys that are bound to `cmd` in map.
///
/// # Examples
///
/// ```rust
/// use rl_sys::readline::{binding, keymap, util};
///
/// util::init();
///
/// let km = keymap::get().unwrap();
///
/// match binding::unbind_command_in_map("kill-line", km) {
///     Ok(res)  => assert!(res == 1),
///     Err(_) => assert!(false),
/// }
/// ```
pub fn unbind_command_in_map(cmd: &str, map: Keymap) -> Result<i32, ::ReadlineError> {
    unsafe {
        let ptr = try!(CString::new(cmd)).as_ptr();
        let res = ext_binding::rl_unbind_command_in_map(ptr, map);

        if res == 1 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Binding Error", "Unable to unbind command!"))
        }
    }
}
