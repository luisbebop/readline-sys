//! [2.3.7 History Expansion](https://goo.gl/OHS0L3)
//!
//! These functions implement history expansion.
use libc::{c_char, c_int, c_void, free};
use readline::util;
use std::ffi::{CStr, CString};
use std::ptr;

mod ext_expand {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn history_expand(s: *mut c_char, out: *mut *mut c_char) -> c_int;
        pub fn get_history_event(s: *const c_char, idx: *mut c_int, delim: c_int) -> *mut c_char;
        pub fn history_tokenize(s: *const c_char) -> *mut *mut c_char;
        pub fn history_arg_extract(first: c_int, last: c_int, s: *const c_char) -> *mut c_char;
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
/// assert!(out == "ls -al");
/// ```
pub fn expand(s: &str) -> Result<(isize, String), ::HistoryError> {
    ::history::mgmt::init();

    unsafe {
        let ptr = try!(CString::new(s)).into_raw();
        let mut output_ptr: *mut c_char = ptr::null_mut();
        let res = ext_expand::history_expand(ptr, &mut output_ptr);
        // Memory safety, grabbing back *mut char
        let _ = CString::from_raw(ptr);

        if output_ptr.is_null() {
            Err(::HistoryError::new("NullPointer", "Null pointer returned from history_expand!"))
        } else {
            let out = CStr::from_ptr(output_ptr).to_string_lossy().into_owned();
            util::free(output_ptr as *mut c_void);
            Ok((res as isize, out))
        }
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
/// assert!(evt == "ls -al");
/// assert!(idx == 3);
/// ```
pub fn get_event(s: &str, idx: &mut i32, delim: Option<char>) -> Result<String, ::HistoryError> {
    ::history::mgmt::init();
    let ptr = try!(CString::new(s)).as_ptr();
    let ch = match delim {
        Some(c) => c as c_int,
        None => 0 as c_int,
    };

    unsafe {
        let char_ptr = ext_expand::get_history_event(ptr, idx as *mut c_int, ch);

        if char_ptr.is_null() {
            Err(::HistoryError::new("History Error", "Null pointer returned!"))
        } else {
            let out = CStr::from_ptr(char_ptr).to_string_lossy().into_owned();
            //free(char_ptr as *mut c_void);
            Ok(out)
        }
    }
}

/// Return an array of tokens parsed out of string `s`, much as the shell might. The tokens are
/// split on the characters in the `history_word_delimiters` variable, and shell quoting conventions
/// are obeyed.
///
/// # Examples
///
/// ```
/// use rl_sys::history::expand;
///
/// let res = expand::tokenize("one two three 'a b c' <def>").unwrap();
/// // ["one", "two", "three", "'a b c'", "<", "def", ">"]
/// assert!(res.len() == 7);
/// assert!(res[0] == "one");
/// assert!(res[6] == ">");
/// ```
pub fn tokenize(s: &str) -> Result<Vec<String>, ::HistoryError> {
    ::history::mgmt::init();
    let ptr = try!(CString::new(s)).as_ptr();
    let mut res = Vec::new();

    unsafe {
        // Returns a char **.  The last entry is 0x0.
        let arr_ptr = ext_expand::history_tokenize(ptr);

        if arr_ptr.is_null() {
            Err(::HistoryError::new("History Error", "Null pointer returned!"))
        } else {
            // Loop through the char** offsets until 0x0 is found, then break.  The pointers point
            // to *mut chars (string), so use CStr to convert them.  free the string from readline
            // after conversion.
            for i in 0.. {
                let curr_ptr = *arr_ptr.offset(i);
                if curr_ptr.is_null() {
                    break;
                } else {
                    res.push(CStr::from_ptr(curr_ptr).to_string_lossy().into_owned());
                    free(curr_ptr as *mut c_void);
                }
            }

            // free the char ** pointer afer use.
            free(arr_ptr as *mut c_void);

            Ok(res)
        }
    }
}

/// Extract a string segment consisting of the `first` through `last` arguments present in string
/// `s`. Arguments are split using `history_tokenize`. If either `first` or `last` is < 0, then make
/// that arg count from the right (subtract from the number of tokens, so that `first` = -1 means
/// the next to last token on the line). If `first` and `last` are 36 (ASCII '$') the last arg from
/// string `s` is used.
///
/// # Examples
///
/// ```
/// use rl_sys::history::expand;
///
/// let mut res = expand::arg_extract("one two three 'a b c' <def>", 0, 1).unwrap();
/// assert!(res == "one two");
/// // 36 is used to represent ASCII '$'.
/// res = expand::arg_extract("one two three", 36, 36).unwrap();
/// println!("{:?}", res);
/// assert!(res == "three");
/// ```
pub fn arg_extract(s: &str, first: i32, last: i32) -> Result<String, ::HistoryError> {
    ::history::mgmt::init();
    let ptr = try!(CString::new(s)).as_ptr();
    unsafe {
        let char_ptr = ext_expand::history_arg_extract(first, last, ptr);

        if char_ptr.is_null() {
            Err(::HistoryError::new("History Error", "Null pointer returned!"))
        } else {
            let out = CStr::from_ptr(char_ptr).to_string_lossy().into_owned();
            free(char_ptr as *mut c_void);
            Ok(out)
        }
    }
}
