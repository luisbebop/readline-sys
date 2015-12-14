//! [2.4 History Variables](https://cnswww.cns.cwru.edu/php/chet/readline/history.html#SEC17)
use libc::{c_char, c_int};
use ::error::HistoryError;
use std::ffi::{CStr, CString};

extern {
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
    // static mut history_search_delimiter_chars: *mut c_char;
    // static mut history_no_expand_chars: *mut c_char;
    static mut history_quotes_inhibit_expansion: c_int;
    //static mut history_inhibit_expansion_function: func ptr;
}

/// If non-zero, timestamps are written to the history file, so they can be preserved between
/// sessions. The default value is 0, meaning that timestamps are not saved.
///
/// The current timestamp format uses the value of `history_comment_char` to delimit timestamp
/// entries in the history file. If that variable does not have a value (the default),
/// timestamps will not be written.
pub fn set_write_timestamps(v: u8) {
    unsafe { history_write_timestamps = v as c_int };
}

/// Get the current value of the `history_write_timestamps` variable.
pub fn get_write_timestamps() -> u8 {
    unsafe { history_write_timestamps as u8 }
}

/// The character that introduces a history event. The default is `!'. Setting this to 0
/// inhibits history expansion.
pub fn set_expansion_char(c: char) {
    unsafe { history_expansion_char = c as c_char };
}

/// Get the current value of the `history_expansion_char` variable.
pub fn get_expansion_char() -> char {
    unsafe { (history_expansion_char as u8) as char }
}

/// The character that invokes word substitution if found at the start of a line. The default
/// is `^'.
pub fn set_subst_char(c: char) {
    unsafe { history_subst_char = c as c_char };
}

/// Get the current value of the `history_subst_char` variable.
pub fn get_subst_char() -> char {
    unsafe { (history_subst_char as u8) as char }
}

/// During tokenization, if this character is seen as the first character of a word, then it
/// and all subsequent characters up to a newline are ignored, suppressing history expansion for
/// the remainder of the line. This is disabled by default.
pub fn set_comment_char(c: char) {
    unsafe { history_comment_char = c as c_char };
}

/// Get the current value of the `history_comment_char` variable.
pub fn get_comment_char() -> char {
    unsafe { (history_comment_char as u8) as char }
}

/// Set the list of word delimiter characters.  Note, this will replace any existing list.
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

/// Get the list of word delimiter characters.
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

/// If non-zero, single-quoted words are not scanned for the history expansion character. The
/// default value is 0.
pub fn set_quotes_inhibit_expansion(v: i8) {
    unsafe { history_quotes_inhibit_expansion = v as c_int };
}

/// Get the current value of the `history_quotes_inhibit_expansion` variable.
pub fn get_quotes_inhibit_expansion() -> i8 {
    unsafe { history_quotes_inhibit_expansion as i8 }
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
    fn test_quotes_inhibit_expansion() {
        assert_eq!(get_quotes_inhibit_expansion(), 0);
        set_quotes_inhibit_expansion(1);
        assert_eq!(get_quotes_inhibit_expansion(), 1);
        set_quotes_inhibit_expansion(0);
    }
}
