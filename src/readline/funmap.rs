//! [2.4.4 Associating Function Names and Bindings]
//! [2.4.4 associating function names and bindings]: https://goo.gl/CrXrWQ
//!
//! These functions allow you to find out what keys invoke named functions and the functions invoked
//! by a particular key sequence. You may also associate a new function name with an arbitrary
//! function.
use readline::{CommandFunction, Keymap};
use readline::binding::BindType;
use std::ffi::{CStr, CString};
use std::ptr;

mod ext_funmap {
    use libc::{c_char, c_int};
    use readline::{CommandFunction, Keymap};

    extern "C" {
        pub fn rl_named_function(name: *const c_char) -> Option<CommandFunction>;
        pub fn rl_function_of_keyseq(keyseq: *const c_char,
                                     map: Keymap,
                                     bind_type: *mut c_int)
                                     -> Option<CommandFunction>;
        pub fn rl_invoking_keyseqs(f: CommandFunction) -> *mut *mut c_char;
        pub fn rl_invoking_keyseqs_in_map(f: CommandFunction, map: Keymap) -> *mut *mut c_char;
        pub fn rl_function_dumper(readable: c_int) -> ();
        pub fn rl_list_funmap_names() -> ();
        pub fn rl_add_funmap_entry(name: *const c_char, f: CommandFunction) -> c_int;
    }
}

/// Return the function with name `name`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{funmap, util};
///
/// util::init();
///
/// assert!(funmap::named_function("self-insert").is_ok())
/// ```
pub fn named_function(name: &str) -> Result<CommandFunction, ::ReadlineError> {
    let csname = try!(CString::new(name));
    let func_ptr = unsafe { ext_funmap::rl_named_function(csname.as_ptr()) };
    if func_ptr.is_none() {
        Err(::ReadlineError::new("Funmap Error", "Unable to find name function!"))
    } else {
        Ok(func_ptr.expect("Unable to get function pointer"))
    }
}

/// Return the function invoked by `keyseq` in keymap `map`. If `map` is None, the current keymap is
/// used. If `add_type` is true, the type of the object is returned (one of `Func`, `Kmap`, or
/// `Macr`).
///
/// # Examples
///
/// ```
/// use rl_sys::readline::binding::BindType;
/// use rl_sys::readline::{funmap, util};
///
/// util::init();
///
/// match funmap::function_of_keyseq("1", None, true) {
///     Ok((_, Some(t))) => assert!(t == BindType::from(0)),
///     Ok((_, None))    => assert!(false),
///     Err(_)           => assert!(false),
/// }
/// ```
pub fn function_of_keyseq
    (keyseq: &str,
     map: Option<Keymap>,
     add_type: bool)
     -> Result<(Option<CommandFunction>, Option<BindType>), ::ReadlineError> {

    let cskeyseq = try!(CString::new(keyseq));
    let km = match map {
        Some(km) => km,
        None => ptr::null_mut(),
    };
    let bind_type: *mut i32 = if add_type { &mut 1 } else { ptr::null_mut() };
    let func_ptr = unsafe { ext_funmap::rl_function_of_keyseq(cskeyseq.as_ptr(), km, bind_type) };
    if func_ptr.is_none() {
        Err(::ReadlineError::new("Funmap Error",
                                 "Unable to get function associated with keyseq!"))
    } else if add_type {
        Ok((func_ptr, Some(BindType::from(unsafe { *bind_type }))))
    } else {
        Ok((func_ptr, None))
    }
}

/// Return an array of strings representing the key sequences used to invoke function in the current
/// keymap.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{funmap, util};
///
/// util::init();
///
/// let cmd = funmap::named_function("self-insert").unwrap();
/// let names = funmap::invoking_keyseqs(cmd).unwrap();
/// assert!(names.len() > 0);
/// ```
pub fn invoking_keyseqs(f: CommandFunction) -> Result<Vec<String>, ::ReadlineError> {
    unsafe {
        let arr_ptr = ext_funmap::rl_invoking_keyseqs(f);

        if arr_ptr.is_null() {
            Err(::ReadlineError::new("Funmap Error", "Unable to find invoking key seqs!"))
        } else {
            let mut entries = Vec::new();
            for i in 0.. {
                let entry_ptr = *arr_ptr.offset(i as isize);
                if entry_ptr.is_null() {
                    break;
                } else {
                    entries.push(CStr::from_ptr(entry_ptr).to_string_lossy().into_owned());
                }
            }
            Ok(entries)
        }
    }
}

/// Return an array of strings representing the key sequences used to invoke function in the current
/// keymap.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{funmap, keymap, util};
///
/// util::init();
///
/// let cmd = funmap::named_function("self-insert").unwrap();
/// let km = keymap::get().unwrap();
/// let names = funmap::invoking_keyseqs_in_map(cmd, km).unwrap();
/// assert!(names.len() > 0);
/// ```
pub fn invoking_keyseqs_in_map(f: CommandFunction,
                               map: Keymap)
                               -> Result<Vec<String>, ::ReadlineError> {
    unsafe {
        let arr_ptr = ext_funmap::rl_invoking_keyseqs_in_map(f, map);

        if arr_ptr.is_null() {
            Err(::ReadlineError::new("Funmap Error", "Unable to find invoking key seqs in map!"))
        } else {
            let mut entries = Vec::new();
            for i in 0.. {
                let entry_ptr = *arr_ptr.offset(i as isize);
                if entry_ptr.is_null() {
                    break;
                } else {
                    entries.push(CStr::from_ptr(entry_ptr).to_string_lossy().into_owned());
                }
            }
            Ok(entries)
        }
    }
}

/// Print the readline function names and the key sequences currently bound to them to
/// `rl_outstream`. If `readable` is true, the list is formatted in such a way that it can be made
/// part of an `inputrc` file and re-read.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{funmap, util};
///
/// util::init();
///
/// funmap::function_dumper(true);
/// ```
pub fn function_dumper(readable: bool) -> () {
    let i = if readable { 1 } else { 0 };
    unsafe { ext_funmap::rl_function_dumper(i) }
}

/// Print the names of all bindable Readline functions to `rl_outstream`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{funmap, util};
///
/// util::init();
///
/// funmap::list_funmap_names();
/// ```
pub fn list_funmap_names() -> () {
    unsafe { ext_funmap::rl_list_funmap_names() }
}

/// Add `name` to the list of bindable Readline command names, and make function `f` the function to
/// be called when `name` is invoked.
///
/// # Examples
///
/// ```rust
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::c_int;
/// use rl_sys::readline::{funmap, util};
///
/// util::init();
///
/// extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
///   0
/// }
///
/// match funmap::add_funmap_entry("test-cmd", test_cmd_func) {
///     Ok(res) => assert!(res >= 0),
///     Err(_)  => assert!(false),
/// }
/// # }
/// ```
pub fn add_funmap_entry(name: &str, cmd: CommandFunction) -> Result<i32, ::ReadlineError> {
    let csname = try!(CString::new(name));
    let res = unsafe { ext_funmap::rl_add_funmap_entry(csname.as_ptr(), cmd) };
    if res >= 0 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Funmap Error", "Unable to add funmap entry!"))
    }
}

#[cfg(test)]
mod test {
    use readline::util;
    use readline::binding::BindType;
    use super::*;

    // #[test]
    // fn test_function_dumper() {
    //     util::init();
    //     function_dumper(true);
    // }
    //
    // #[test]
    // fn test_list_funmap_names() {
    //     util::init();
    //     list_funmap_names();
    // }

    #[test]
    fn test_function_of_keyseq() {
        util::init();
        match function_of_keyseq("1", None, true) {
            Ok((_, Some(t))) => assert!(t == BindType::from(0)),
            Ok((_, None)) => assert!(false),
            Err(_) => assert!(false),
        }

        match function_of_keyseq("2", None, false) {
            Ok((_, Some(_))) => assert!(false),
            Ok((_, None)) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_add_funmap_entry() {
        use libc::c_int;

        #[no_mangle]
        #[allow(private_no_mangle_fns)]
        extern "C" fn test_cmd_func(_count: c_int, _key: c_int) -> c_int {
            0
        }

        util::init();

        match add_funmap_entry("test-cmd", test_cmd_func) {
            Ok(res) => assert!(res >= 0),
            Err(_) => assert!(false),
        }
    }
}
