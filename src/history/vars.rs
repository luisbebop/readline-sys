//! [2.4 History Variables](https://goo.gl/la0XEf)
//!
//! This section describes the externally-visible variables exported by the GNU History Library.
use libc::{c_char, c_int, c_uint};
use error::HistoryError;
use std::ffi::{CStr, CString};

extern "C" {
    /// The logical offset of the first entry in the history list.
    pub static history_base: c_int;
    /// The number of entries currently stored in the history list.
    pub static history_length: c_int;
    /// The maximum number of history entries. This must be changed using `stifle_history()`.
    pub static history_max_entries: c_int;
    static mut history_write_timestamps: c_int;
    static mut history_expansion_char: c_char;
    static mut history_subst_char: c_char;
    static mut history_comment_char: c_char;
    static mut history_word_delimiters: *mut c_char;
    static mut history_search_delimiter_chars: *mut c_char;
    static mut history_no_expand_chars: *mut c_char;
    static mut history_quotes_inhibit_expansion: c_int;
    static mut history_inhibit_expansion_function: Option<extern "C" fn(*mut c_char, c_uint)
                                                                        -> c_int>;
}

/// If non-zero, timestamps are written to the history file, so they can be preserved between
/// sessions. The default value is 0, meaning that timestamps are not saved.
///
/// The current timestamp format uses the value of `history_comment_char` to delimit timestamp
/// entries in the history file. If that variable does not have a value (the default),
/// timestamps will not be written.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_write_timestamps(1);
/// ```
pub fn set_write_timestamps(v: u8) {
    unsafe { history_write_timestamps = v as c_int };
}

/// Get the current value of the `history_write_timestamps` variable.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_write_timestamps(1);
/// assert_eq!(vars::get_write_timestamps(), 1);
/// ```
pub fn get_write_timestamps() -> u8 {
    unsafe { history_write_timestamps as u8 }
}

/// The character that introduces a history event. The default is `!'. Setting this to 0
/// inhibits history expansion.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_expansion_char(':');
/// ```
pub fn set_expansion_char(c: char) {
    unsafe { history_expansion_char = c as c_char };
}

/// Get the current value of the `history_expansion_char` variable.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_expansion_char(':');
/// assert_eq!(vars::get_expansion_char(), ':');
/// ```
pub fn get_expansion_char() -> char {
    unsafe { (history_expansion_char as u8) as char }
}

/// The character that invokes word substitution if found at the start of a line. The default
/// is `^'.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_subst_char(':');
/// ```
pub fn set_subst_char(c: char) {
    unsafe { history_subst_char = c as c_char };
}

/// Get the current value of the `history_subst_char` variable.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_subst_char(':');
/// assert_eq!(vars::get_subst_char(), ':');
/// ```
pub fn get_subst_char() -> char {
    unsafe { (history_subst_char as u8) as char }
}

/// During tokenization, if this character is seen as the first character of a word, then it
/// and all subsequent characters up to a newline are ignored, suppressing history expansion for
/// the remainder of the line. This is disabled by default.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_comment_char(':');
/// ```
pub fn set_comment_char(c: char) {
    unsafe { history_comment_char = c as c_char };
}

/// Get the current value of the `history_comment_char` variable.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_comment_char(':');
/// assert_eq!(vars::get_comment_char(), ':');
/// ```
pub fn get_comment_char() -> char {
    unsafe { (history_comment_char as u8) as char }
}

/// Set the list of word delimiter characters.  Note, this will replace any existing list.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// let mut new_delims = Vec::new();
/// new_delims.extend([' ', '\t', '\n'].iter().cloned());
/// assert!(vars::set_word_delimiters(new_delims).is_ok());
/// ```
pub fn set_word_delimiters(chars: Vec<char>) -> Result<(), HistoryError> {
    let delims: String = chars.into_iter().collect();
    unsafe {
        let ptr = try!(CString::new(delims)).into_raw();
        history_word_delimiters = ptr;
    }
    Ok(())
}

/// Add a character from the list of word delimiter characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// assert!(vars::add_word_delimiter(':').is_ok());
/// ```
pub fn add_word_delimiter(c: char) -> Result<(), HistoryError> {
    unsafe {
        let ptr = history_word_delimiters;

        if ptr.is_null() {
            let mut chars_str: Vec<u8> = Vec::new();
            chars_str.push(c as u8);
            history_word_delimiters = CString::from_vec_unchecked(chars_str).into_raw();
            Ok(())
        } else {
            let mut all_chars = Vec::from(CStr::from_ptr(ptr).to_bytes());
            all_chars.push(c as u8);
            history_word_delimiters = CString::from_vec_unchecked(all_chars).into_raw();
            Ok(())
        }
    }
}

/// Remove a character from the list of word delimiter characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// assert!(vars::add_word_delimiter(':').is_ok());
/// assert!(vars::remove_word_delimiter(':').is_ok());
/// ```
pub fn remove_word_delimiter(c: char) -> Result<(), HistoryError> {
    unsafe {
        let ptr = history_word_delimiters;

        if ptr.is_null() {
            Err(HistoryError::new("Null Pointer", "Unable to read history_word_delimiters"))
        } else {
            let wds = Vec::from(CStr::from_ptr(ptr).to_bytes());
            let (_, rest): (Vec<_>, Vec<_>) = wds.into_iter().partition(|&n| n == (c as u8));

            history_word_delimiters = CString::from_vec_unchecked(rest).into_raw();
            Ok(())
        }
    }
}

/// Get the list of word delimiter characters. The characters that separate tokens for
/// `history_tokenize()`. The default value is `" \t\n;&()<>"`.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// let delims: String = vars::get_word_delimiters().unwrap().into_iter().collect();
/// println!("{}", delims);
/// ```
pub fn get_word_delimiters() -> Result<Vec<char>, HistoryError> {
    unsafe {
        let ptr = history_word_delimiters;

        if ptr.is_null() {
            Err(HistoryError::new("Null Pointer", "Unable to read word delimiters!"))
        } else {
            let all_chars = try!(CStr::from_ptr(ptr).to_str());
            Ok(all_chars.chars().collect())
        }
    }
}

/// Set the list of search delimiter characters.  Note, this will replace any existing list.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// let mut new_delims = Vec::new();
/// new_delims.extend([' ', '\t', '\n'].iter().cloned());
/// assert!(vars::set_search_delimiter_chars(new_delims).is_ok());
/// ```
pub fn set_search_delimiter_chars(chars: Vec<char>) -> Result<(), HistoryError> {
    let delims: String = chars.into_iter().collect();
    unsafe {
        let ptr = try!(CString::new(delims)).into_raw();
        history_search_delimiter_chars = ptr;
    }
    Ok(())
}

/// Add a character to the list of search delimiter characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// assert!(vars::add_search_delimiter_char(':').is_ok());
/// ```
pub fn add_search_delimiter_char(c: char) -> Result<(), HistoryError> {
    unsafe {
        let ptr = history_search_delimiter_chars;

        if ptr.is_null() {
            let mut chars_str: Vec<u8> = Vec::new();
            chars_str.push(c as u8);
            history_search_delimiter_chars = CString::from_vec_unchecked(chars_str).into_raw();
            Ok(())
        } else {
            let mut all_chars = Vec::from(CStr::from_ptr(ptr).to_bytes());
            all_chars.push(c as u8);
            history_search_delimiter_chars = CString::from_vec_unchecked(all_chars).into_raw();
            Ok(())
        }
    }
}

/// Remove a character from the list of search delimiter characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// assert!(vars::add_search_delimiter_char(':').is_ok());
/// assert!(vars::remove_search_delimiter_char(':').is_ok());
/// ```
pub fn remove_search_delimiter_char(c: char) -> Result<(), HistoryError> {
    unsafe {
        let ptr = history_search_delimiter_chars;

        if ptr.is_null() {
            Err(HistoryError::new("Null Pointer", "Unable to read search delimiter chars"))
        } else {
            let wds = Vec::from(CStr::from_ptr(ptr).to_bytes());
            let (_, rest): (Vec<_>, Vec<_>) = wds.into_iter().partition(|&n| n == (c as u8));

            history_search_delimiter_chars = CString::from_vec_unchecked(rest).into_raw();
            Ok(())
        }
    }
}

/// Get the list of search delimiter characters. The list of additional characters which can delimit
/// a history search string, in addition to space, TAB, ':' and '?' in the case of a substring
/// search. The default is empty.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// let delims: String = match vars::get_search_delimiter_chars() {
///     Ok(v) => {
///         v.into_iter().collect()
///     },
///     Err(_) => {
///         String::new()
///     },
/// };
/// println!("{}", delims);
/// ```
pub fn get_search_delimiter_chars() -> Result<Vec<char>, HistoryError> {
    unsafe {
        let ptr = history_search_delimiter_chars;

        if ptr.is_null() {
            Ok(Vec::new())
        } else {
            let all_chars = try!(CStr::from_ptr(ptr).to_str());
            Ok(all_chars.chars().collect())
        }
    }
}

/// Set the list of no expand characters.  Note, this will replace any existing list.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// let mut new_delims = Vec::new();
/// new_delims.extend([' ', '\t', '\n'].iter().cloned());
/// assert!(vars::set_no_expand_chars(new_delims).is_ok());
/// ```
pub fn set_no_expand_chars(chars: Vec<char>) -> Result<(), HistoryError> {
    let delims: String = chars.into_iter().collect();
    unsafe {
        let ptr = try!(CString::new(delims)).into_raw();
        history_no_expand_chars = ptr;
    }
    Ok(())
}

/// Add a character to the list of no expand characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// assert!(vars::add_no_expand_char(':').is_ok());
/// ```
pub fn add_no_expand_char(c: char) -> Result<(), HistoryError> {
    unsafe {
        let ptr = history_no_expand_chars;

        if ptr.is_null() {
            let mut chars_str: Vec<u8> = Vec::new();
            chars_str.push(c as u8);
            history_no_expand_chars = CString::from_vec_unchecked(chars_str).into_raw();
            Ok(())
        } else {
            let mut all_chars = Vec::from(CStr::from_ptr(ptr).to_bytes());
            all_chars.push(c as u8);
            history_no_expand_chars = CString::from_vec_unchecked(all_chars).into_raw();
            Ok(())
        }
    }
}

/// Remove a character from the list of no expand characters.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// assert!(vars::add_no_expand_char(':').is_ok());
/// assert!(vars::remove_no_expand_char(':').is_ok());
/// ```
pub fn remove_no_expand_char(c: char) -> Result<(), HistoryError> {
    unsafe {
        let ptr = history_no_expand_chars;

        if ptr.is_null() {
            Err(HistoryError::new("Null Pointer", "Unable to read search delimiter chars"))
        } else {
            let wds = Vec::from(CStr::from_ptr(ptr).to_bytes());
            let (_, rest): (Vec<_>, Vec<_>) = wds.into_iter().partition(|&n| n == (c as u8));

            history_no_expand_chars = CString::from_vec_unchecked(rest).into_raw();
            Ok(())
        }
    }
}

/// The list of characters which inhibit history expansion if found immediately following
/// `history_expansion_char`. The default is space, tab, newline, carriage return, and `='.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// let delims: String = match vars::get_no_expand_chars() {
///     Ok(v) => {
///         v.into_iter().collect()
///     },
///     Err(_) => {
///         String::new()
///     },
/// };
/// println!("{}", delims);
/// ```
pub fn get_no_expand_chars() -> Result<Vec<char>, HistoryError> {
    unsafe {
        let ptr = history_no_expand_chars;

        if ptr.is_null() {
            Ok(Vec::new())
        } else {
            let all_chars = try!(CStr::from_ptr(ptr).to_str());
            Ok(all_chars.chars().collect())
        }
    }
}

/// If non-zero, single-quoted words are not scanned for the history expansion character. The
/// default value is 0.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_quotes_inhibit_expansion(1);
/// ```
pub fn set_quotes_inhibit_expansion(v: i8) {
    unsafe { history_quotes_inhibit_expansion = v as c_int };
}

/// Get the current value of the `history_quotes_inhibit_expansion` variable.
///
/// # Examples
///
/// ```
/// use rl_sys::history::vars;
///
/// vars::set_quotes_inhibit_expansion(1);
/// assert_eq!(vars::get_quotes_inhibit_expansion(), 1);
/// ```
pub fn get_quotes_inhibit_expansion() -> i8 {
    unsafe { history_quotes_inhibit_expansion as i8 }
}

/// This should be set to a function of the following type:
/// `extern fn(*mut c_char, c_uint) -> c_int`.  The first argument is a string pointer
/// and the second is an int index into that string. It should return a non-zero value if the
/// history expansion starting at string[i] should not be performed; zero if the expansion should
/// be done. It is intended for use by applications like Bash that use the history expansion
/// character for additional purposes. By default, this variable is set to NULL.
///
/// # Examples
///
/// ```
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::{c_char, c_int, c_uint};
/// use rl_sys::history::vars;
///
/// extern fn blah(_: *mut c_char, _: c_uint) -> c_int {
///     0
/// }
///
/// vars::set_inhibit_expansion_function(Some(blah));
/// # }
/// ```
pub fn set_inhibit_expansion_function(f: Option<extern "C" fn(*mut c_char, c_uint) -> c_int>) {
    unsafe { history_inhibit_expansion_function = f }
}

/// Get the value of the inhibit expansion function.  This will be None if it is not set.
///
/// # Examples
///
/// ```
/// # extern crate libc;
/// # extern crate rl_sys;
/// # fn main() {
/// use libc::{c_char, c_int, c_uint};
/// use rl_sys::history::vars;
///
/// extern fn blah(_: *mut c_char, _: c_uint) -> c_int {
///     0
/// }
///
/// vars::set_inhibit_expansion_function(Some(blah));
/// assert!(vars::get_inhibit_expansion_function().is_some());
/// vars::set_inhibit_expansion_function(None);
/// assert!(vars::get_inhibit_expansion_function().is_none());
/// # }
/// ```
pub fn get_inhibit_expansion_function() -> Option<extern "C" fn(*mut c_char, c_uint) -> c_int> {
    unsafe { history_inhibit_expansion_function }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base() {
        ::history::mgmt::init();
        assert!(history_base >= 0);
    }

    #[test]
    fn test_length() {
        ::history::mgmt::init();
        assert!(history_length >= 0);
    }

    #[test]
    fn test_max_entries() {
        ::history::mgmt::init();
        assert!(history_max_entries >= 0);
    }

    #[test]
    fn test_write_timestamps() {
        ::history::mgmt::init();
        assert_eq!(get_write_timestamps(), 0);
        set_write_timestamps(1);
        assert_eq!(get_write_timestamps(), 1);
        set_write_timestamps(0);
    }

    #[test]
    fn test_expansion_char() {
        ::history::mgmt::init();
        assert_eq!(get_expansion_char(), '!');
        set_expansion_char('?');
        assert_eq!(get_expansion_char(), '?');
        set_expansion_char('!');
    }

    #[test]
    fn test_subst_char() {
        ::history::mgmt::init();
        assert_eq!(get_subst_char(), '^');
        set_subst_char('%');
        assert_eq!(get_subst_char(), '%');
        set_subst_char('^');
    }

    #[test]
    fn test_comment_char() {
        ::history::mgmt::init();
        set_comment_char(':');
        assert_eq!(get_comment_char(), ':');
        set_comment_char('\u{0}');
        assert_eq!(get_comment_char(), '\u{0}');
    }

    #[test]
    fn test_word_delimiter() {
        ::history::mgmt::init();
        add_word_delimiter(':').unwrap();
        let mut delims: String = get_word_delimiters().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n;&()|<>:");
        assert!(remove_word_delimiter(':').is_ok());
        assert!(remove_word_delimiter('|').is_ok());
        delims = get_word_delimiters().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n;&()<>");
        let mut new_delims = Vec::new();
        new_delims.extend([' ', '\t', '\n'].iter().cloned());
        assert!(set_word_delimiters(new_delims).is_ok());
        delims = get_word_delimiters().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n");
        let mut old_delims = Vec::new();
        old_delims.extend([' ', '\t', '\n', ';', '&', '(', ')', '|', '<', '>'].iter().cloned());
        assert!(set_word_delimiters(old_delims).is_ok());
        delims = get_word_delimiters().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n;&()|<>");
    }

    #[test]
    fn test_search_delimiter_chars() {
        ::history::mgmt::init();
        let schars = get_search_delimiter_chars();
        assert!(schars.is_ok());
        assert!(schars.unwrap().is_empty());
        let mut new_delims = Vec::new();
        new_delims.extend(['#'].iter().cloned());
        assert!(set_search_delimiter_chars(new_delims).is_ok());
        let mut delims: String = get_search_delimiter_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], "#");
        assert!(add_search_delimiter_char('@').is_ok());
        delims = get_search_delimiter_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], "#@");
        assert!(remove_search_delimiter_char('@').is_ok());
        delims = get_search_delimiter_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], "#");
        assert!(remove_search_delimiter_char('#').is_ok());
    }

    #[test]
    fn test_no_expand_chars() {
        ::history::mgmt::init();
        let nec = get_no_expand_chars();
        assert!(nec.is_ok());
        let mut delims: String = get_no_expand_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n\r=");
        assert!(add_no_expand_char('#').is_ok());
        delims = get_no_expand_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n\r=#");
        assert!(remove_no_expand_char('#').is_ok());
        delims = get_no_expand_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n\r=");
        let mut new_delims = Vec::new();
        new_delims.extend(['#'].iter().cloned());
        assert!(set_no_expand_chars(new_delims).is_ok());
        delims = get_no_expand_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], "#");
        let mut old_delims = Vec::new();
        old_delims.extend([' ', '\t', '\n', '\r', '='].iter().cloned());
        assert!(set_no_expand_chars(old_delims).is_ok());
        delims = get_no_expand_chars().unwrap().into_iter().collect();
        assert_eq!(&delims[..], " \t\n\r=");
    }

    #[test]
    fn test_quotes_inhibit_expansion() {
        assert_eq!(get_quotes_inhibit_expansion(), 0);
        set_quotes_inhibit_expansion(1);
        assert_eq!(get_quotes_inhibit_expansion(), 1);
        set_quotes_inhibit_expansion(0);
    }

    extern "C" fn blah(_: *mut ::libc::c_char, _: ::libc::c_uint) -> ::libc::c_int {
        0
    }

    #[test]
    fn test_inhibit_expansion_function() {
        assert!(get_inhibit_expansion_function().is_none());
        set_inhibit_expansion_function(Some(blah));
        assert!(get_inhibit_expansion_function().is_some());
        set_inhibit_expansion_function(None);
        assert!(get_inhibit_expansion_function().is_none());
    }
}
