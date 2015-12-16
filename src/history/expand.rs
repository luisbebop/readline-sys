//! [2.3.7 History Expansion](https://goo.gl/OHS0L3)
//!
//! These functions implement history expansion.
use libc::{c_char, c_int, c_void, free};
use std::ffi::{CStr, CString};

pub struct Expand {
    ptr: *mut c_char,
    pub output: String,
}

impl Drop for Expand {
    fn drop(&mut self) {
        unsafe { free(self.ptr as *mut c_void) };
    }
}

pub struct Event {
    ptr: *mut c_char,
    pub output: String,
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { free(self.ptr as *mut c_void) };
    }
}

mod ext_expand {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn history_expand(s: *mut c_char, out: *mut *mut c_char) -> c_int;
        pub fn get_history_event(s: *const c_char, idx: *mut c_int, delim: c_int) -> *mut c_char;
        pub fn history_tokenize(s: *const c_char) -> *mut *mut c_char;
    // pub fn history_arg_extract(arg1: c_int, arg2: c_int,
    //                            arg3: *const c_char) -> *mut c_char;
    }
}

/// Expand string, placing the result into `output`, a pointer to a string
/// (see section [1.1 History Expansion](https://goo.gl/aR8VB3)).
///
/// Returns:
///
/// * 0  - If no expansions took place (or, if the only change in the text was the removal of
/// escape characters preceding the history expansion character).
/// * 1  - If expansions did take place.
/// * -1 - If there was an error in expansion.
/// * 2 - If the returned line should be displayed, but not executed, as with the :p modifier (see
/// section [1.1.3 Modifiers](https://goo.gl/9HFpWN)).
///
/// If an error occurred in expansion, then output contains a descriptive error message.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, expand};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let (res, out) = expand::expand("!ls").unwrap();
/// assert!(res == 1);
/// assert!(out.output == "ls -al");
/// ```
pub fn expand(s: &str) -> Result<(isize, Expand), ::HistoryError> {
    use std::ptr;
    ::history::mgmt::init();

    unsafe {
        let ptr = try!(CString::new(s)).into_raw();
        let mut output_ptr: *mut c_char = ptr::null_mut();
        let res = ext_expand::history_expand(ptr, &mut output_ptr);
        let cstr = CStr::from_ptr(output_ptr);
        let out = Expand {
            ptr: output_ptr,
            output: cstr.to_string_lossy().into_owned(),
        };
        Ok((res as isize, out))
    }
}

/// Returns the text of the history event beginning at `s + *idx`. `*idx` is modified to point to
/// after the event specifier. At function entry, `*idx` points to the index into string where the
/// history event specification begins. `add_delim` is a character that is allowed to end the event
/// specification in addition to the "normal" terminating characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, expand};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let mut idx = 0;
/// let evt = expand::get_event("!ls:p", &mut idx, None).unwrap();
/// assert!(evt.output == "ls -al");
/// assert!(idx == 3);
/// ```
pub fn get_event(s: &str, idx: &mut i32, add_delim: Option<char>) -> Result<Event, ::HistoryError> {
    ::history::mgmt::init();
    let ptr = try!(CString::new(s)).as_ptr();
    let ch = match add_delim {
        Some(c) => c as c_int,
        None    => 0 as c_int,
    };

    unsafe {
        let char_ptr = ext_expand::get_history_event(ptr, idx as *mut c_int, ch);
        let cstr = CStr::from_ptr(char_ptr);
        let out = Event {
            ptr: char_ptr,
            output: cstr.to_string_lossy().into_owned(),
        };
        Ok(out)
    }
}

/// Return an array of tokens parsed out of string `s`, much as the shell might. The tokens are
/// split on the characters in the `history_word_delimiters` variable, and shell quoting conventions
/// are obeyed.
///
/// # Examples
///
/// ```
///
/// ```
pub fn tokenize(s: &str) -> Result<Vec<String>, ::HistoryError> {
    ::history::mgmt::init();
    let ptr = try!(CString::new(s)).as_ptr();

    unsafe {
        let arr_ptr = &mut *ext_expand::history_tokenize(ptr);
        println!("{:?}", arr_ptr);
        println!("{:?}", *arr_ptr.offset(1));
        Ok(Vec::new())
    }
}

pub fn arg_extract() {}
