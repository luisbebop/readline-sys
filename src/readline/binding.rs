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
use libc::c_char;
use readline::{CommandFunction, Keymap};
use self::BindType::{Func, Kmap, Macr};
use std::ffi::CString;
use std::path::Path;
use std::ptr;

/// Result type returned by all binding functions.
pub type BindResult = Result<i32, ::ReadlineError>;

/// Use for calls to `generic_bind`.
#[derive(Debug, PartialEq)]
pub enum BindType {
    /// Generate a function binding.
    Func(Option<CommandFunction>),
    /// Generate a keymap binding.
    Kmap(Keymap),
    /// Generate a macro binding.
    Macr(*const c_char),
}

impl From<i32> for BindType {
    fn from(i: i32) -> BindType {
        if i == 0 {
            Func(None)
        } else if i == 1 {
            Kmap(ptr::null_mut())
        } else if i == 2 {
            Macr(ptr::null())
        } else {
            panic!("Unknown BindType!");
        }
    }
}

mod ext_binding {
    use libc::{c_char, c_int};
    use readline::{CommandFunction, Keymap};

    extern "C" {
        pub fn rl_bind_key(key: c_int, f: CommandFunction) -> c_int;
        pub fn rl_bind_key_in_map(key: c_int, f: CommandFunction, map: Keymap) -> c_int;
        pub fn rl_bind_key_if_unbound(key: c_int, f: CommandFunction) -> c_int;
        pub fn rl_bind_key_if_unbound_in_map(key: c_int, f: CommandFunction, map: Keymap) -> c_int;
        pub fn rl_unbind_key(key: c_int) -> c_int;
        pub fn rl_unbind_key_in_map(key: c_int, map: Keymap) -> c_int;
        pub fn rl_unbind_function_in_map(f: CommandFunction, map: Keymap) -> c_int;
        pub fn rl_unbind_command_in_map(cmd: *const c_char, map: Keymap) -> c_int;
        pub fn rl_bind_keyseq(keyseq: *const c_char, f: CommandFunction) -> c_int;
        pub fn rl_bind_keyseq_in_map(keyseq: *const c_char,
                                     f: CommandFunction,
                                     map: Keymap)
                                     -> c_int;
        pub fn rl_set_key(keyseq: *const c_char, f: CommandFunction, map: Keymap) -> c_int;
        pub fn rl_bind_keyseq_if_unbound(keyseq: *const c_char, f: CommandFunction) -> c_int;
        pub fn rl_bind_keyseq_if_unbound_in_map(keyseq: *const c_char,
                                                f: CommandFunction,
                                                map: Keymap)
                                                -> c_int;
        pub fn rl_generic_bind(bind_type: c_int,
                               keyseq: *const c_char,
                               data: *mut c_char,
                               map: Keymap)
                               -> c_int;
        pub fn rl_parse_and_bind(line: *mut c_char) -> c_int;
        pub fn rl_read_init_file(filename: *const c_char) -> c_int;
    }
}

fn genresult(res: i32, err: &str) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// match binding::bind_key('\t', test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn bind_key(key: char, f: CommandFunction) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_in_map('\t', km, test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn bind_key_in_map(key: char, map: Keymap, f: CommandFunction) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// // Create an empty keymap to ensure ';' doesn't collide on first bind.
/// let keymap = keymap::create_empty().unwrap();
/// keymap::set(keymap);
///
/// match binding::bind_key_if_unbound(';', test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::bind_key_if_unbound(';', test_cmd_func) {
///     Ok(_)  => assert!(false),
///     Err(_) => assert!(true),
/// }
/// # }
/// ```
pub fn bind_key_if_unbound(key: char, f: CommandFunction) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::bind_key_if_unbound_in_map('\t', km, test_cmd_func) {
///     Ok(_)  => assert!(false),
///     Err(_) => assert!(true),
/// }
/// # }
/// ```
pub fn bind_key_if_unbound_in_map(key: char, map: Keymap, f: CommandFunction) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, test_cmd_func) {
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
pub fn unbind_key(key: char) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, test_cmd_func) {
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
pub fn unbind_key_in_map(key: char, map: Keymap) -> BindResult {
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap();
///
/// match binding::bind_key_if_unbound_in_map('\t', km, test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::unbind_function_in_map(test_cmd_func, km) {
///     Ok(res)  => assert!(res == 1),
///     Err(_) => assert!(false),
/// }
/// # }
/// ```
pub fn unbind_function_in_map(f: CommandFunction, map: Keymap) -> BindResult {
    let res = unsafe { ext_binding::rl_unbind_function_in_map(f, map) };
    if res == 1 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Binding Error", "Unable to unbind function!"))
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
pub fn unbind_command_in_map(cmd: &str, map: Keymap) -> BindResult {
    let cscmd = try!(CString::new(cmd));
    let res = unsafe {
        ext_binding::rl_unbind_command_in_map(cscmd.as_ptr(), map)
    };
    if res == 1 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Binding Error", "Unable to unbind command!"))
    }
}

/// Bind the key sequence represented by the string `keyseq` to the function `f`, beginning in the
/// current keymap. This makes new keymaps as necessary. The return value is non-zero if `keyseq` is
/// invalid.
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// match binding::bind_keyseq("C-z", test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn bind_keyseq(keyseq: &str, f: CommandFunction) -> BindResult {
    let cskeyseq = try!(CString::new(keyseq));
    unsafe {
        genresult(ext_binding::rl_bind_keyseq(cskeyseq.as_ptr(), f),
                  "Unable to bind key sequence!")
    }
}

/// Bind the key sequence represented by the string `keyseq` to the function `f`. This makes new
/// keymaps as necessary. Initial bindings are performed in `map`. The return value is non-zero if
/// keyseq is invalid.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
/// use std::ptr;
///
/// util::init();
///
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap_or(ptr::null_mut());
///
/// assert!(!km.is_null());
///
/// match binding::bind_keyseq_in_map("C-z", test_cmd_func, km) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn bind_keyseq_in_map(keyseq: &str, f: CommandFunction, map: Keymap) -> BindResult {
    let cskeyseq = try!(CString::new(keyseq));
    unsafe {
        genresult(ext_binding::rl_bind_keyseq_in_map(cskeyseq.as_ptr(), f, map),
                  "Unable to bind key sequence!")
    }
}

/// Equivalent to `rl_bind_keyseq_in_map`.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
/// use std::ptr;
///
/// util::init();
///
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap_or(ptr::null_mut());
///
/// assert!(!km.is_null());
///
/// match binding::set_key("C-z", test_cmd_func, km) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn set_key(keyseq: &str, f: CommandFunction, map: Keymap) -> BindResult {
    let cskeyseq = try!(CString::new(keyseq));
    unsafe {
        genresult(ext_binding::rl_set_key(cskeyseq.as_ptr(), f, map),
                  "Unable to bind key sequence!")
    }
}

/// Binds `keyseq` to function `f` if it is not already bound in the currently active keymap.
/// Returns non-zero in the case of an invalid `keyseq` or if `keyseq` is already bound.
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
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// match binding::bind_keyseq_if_unbound("C-z", test_cmd_func) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::bind_keyseq_if_unbound("C-z", test_cmd_func) {
///     Ok(_)  => assert!(false),
///     Err(_) => assert!(true),
/// }
/// # }
/// ```
pub fn bind_keyseq_if_unbound(keyseq: &str, f: CommandFunction) -> BindResult {
    let cskeyseq = try!(CString::new(keyseq));
    unsafe {
        genresult(ext_binding::rl_bind_keyseq_if_unbound(cskeyseq.as_ptr(), f),
                  "Unable to bind key sequence!")
    }
}

/// Binds `keyseq` to function `f` if it is not already bound in map. Returns non-zero in the case
/// of an invalid `keyseq` or if `keyseq` is already bound.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
/// use std::ptr;
///
/// util::init();
///
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap_or(ptr::null_mut());
///
/// assert!(!km.is_null());
///
/// match binding::bind_keyseq_if_unbound_in_map("C-z", test_cmd_func, km) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// match binding::bind_keyseq_if_unbound_in_map("C-z", test_cmd_func, km) {
///     Ok(_)  => assert!(false),
///     Err(_) => assert!(true),
/// }
/// # }
/// ```
pub fn bind_keyseq_if_unbound_in_map(keyseq: &str, f: CommandFunction, map: Keymap) -> BindResult {
    let cskeyseq = try!(CString::new(keyseq));
    unsafe {
        genresult(ext_binding::rl_bind_keyseq_if_unbound_in_map(cskeyseq.as_ptr(), f, map),
                  "Unable to bind key sequence!")
    }
}

/// Bind the key sequence represented by the string `keyseq` to the function `f`. type says what
/// kind of data is pointed to by data; this can be a function (`Func`), a macro (`Macr`), or a
/// keymap (`Kmap`). This makes new keymaps as necessary. The initial keymap in which to do
/// bindings is `map`.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{binding, keymap, util};
/// use rl_sys::readline::binding::BindType;
/// use std::ffi::CString;
/// use std::ptr;
///
/// util::init();
///
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// let km = keymap::create_empty().unwrap_or(ptr::null_mut());
///
/// assert!(!km.is_null());
///
/// match binding::generic_bind("C-z", BindType::Func(Some(test_cmd_func)), km) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// let km2 = keymap::create_empty().unwrap_or(ptr::null_mut());
/// assert!(!km2.is_null());
///
/// match binding::generic_bind("C-k", BindType::Kmap(km2), km) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// let macr = CString::new("\\C-e | less\\C-m").unwrap();
///
/// match binding::generic_bind("C-M-l", BindType::Macr(macr.as_ptr()), km) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn generic_bind(keyseq: &str, bind_type: BindType, map: Keymap) -> BindResult {
    let cskeyseq = try!(CString::new(keyseq));

    match bind_type {
        Func(func_ptr) => {
            let fn_ptr = if func_ptr.is_some() {
                func_ptr.expect("Unable to get function pointer") as *mut i8
            } else {
                ::std::ptr::null_mut() as *mut i8
            };
            unsafe {
                genresult(ext_binding::rl_generic_bind(0, cskeyseq.as_ptr(), fn_ptr, map),
                          "Unable to bind to function!")
            }
        }
        Kmap(km) => {
            unsafe {
                genresult(ext_binding::rl_generic_bind(1, cskeyseq.as_ptr(), km as *mut i8, map),
                          "Unable to bind to keymap!")
            }
        }
        Macr(m) => {
            unsafe {
                genresult(ext_binding::rl_generic_bind(2, cskeyseq.as_ptr(), m as *mut i8, map),
                          "Unable to bind to macro!")
            }
        }
    }
}

/// Parse `line` as if it had been read from the `inputrc` file and perform any key bindings and
/// variable assignments found (see section [1.3 Readline Init File]).
/// [1.3 readline init file]: https://goo.gl/VtaCdx
///
/// # Examples
///
/// ```rust
/// use rl_sys::readline::{binding, util};
///
/// util::init();
///
/// match binding::parse_and_bind("\"\\ew\":\"\\C-e # macro\"") {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
/// ```
pub fn parse_and_bind(line: &str) -> BindResult {
    unsafe {
        let ptr = try!(CString::new(line)).into_raw();
        let res = genresult(ext_binding::rl_parse_and_bind(ptr),
                            "Unable to parse and bind!");
        let _ = CString::from_raw(ptr);
        res
    }
}

/// Read keybindings and variable assignments from `path` (see section [1.3 Readline Init File]).
/// [1.3 readline init file]: https://goo.gl/VtaCdx
///
/// # Examples
///
/// ```rust
/// use rl_sys::readline::{binding, util};
/// use std::env;
/// use std::fs::{self, File};
/// use std::io::{Error, ErrorKind, Write};
///
/// # #[allow(dead_code)]
/// # fn foo() -> std::io::Result<()> {
///
/// util::init();
///
/// let home = try!(env::home_dir().ok_or(Error::new(ErrorKind::Other, "oh no!")));
/// let ifp = home.join(".myinputrc");
/// let mut f = try!(File::create(ifp.as_path()));
/// try!(f.write_all("\"\\ew\":\"\\C-e # macro\"".as_bytes()));
///
/// match binding::read_init_file(ifp.as_path()) {
///     Ok(res) => assert!(res == 0),
///     Err(_)  => assert!(false),
/// }
///
/// try!(fs::remove_file(ifp.as_path()));
/// # Ok(())
/// # }
/// ```
pub fn read_init_file(path: &Path) -> BindResult {
    let cspath = try!(CString::new(path.to_string_lossy().into_owned()));
    unsafe {
        genresult(ext_binding::rl_read_init_file(cspath.as_ptr()),
                  "Unable to read init file!")
    }
}
