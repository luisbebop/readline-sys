//! [2.4.6 Redisplay]
//! [2.4.6 redisplay]: https://goo.gl/i3PS2Q
use std::ffi::CString;

mod ext_redisplay {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn rl_redisplay() -> ();
        pub fn rl_forced_update_display() -> c_int;
        pub fn rl_on_new_line() -> c_int;
        pub fn rl_on_new_line_with_prompt() -> c_int;
        pub fn rl_reset_line_state() -> c_int;
        pub fn rl_crlf() -> c_int;
        pub fn rl_show_char(c: c_int) -> c_int;
        pub fn rl_message(message: *const c_char, ...) -> c_int;
        pub fn rl_clear_message() -> c_int;
        pub fn rl_save_prompt() -> ();
        pub fn rl_restore_prompt() -> ();
        pub fn rl_expand_prompt(prompt: *const c_char) -> c_int;
        pub fn rl_set_prompt(prompt: *const c_char) -> c_int;
    }
}

fn genresult(res: i32, message: &str) -> Result<i32, ::ReadlineError> {
    if res == 0 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Redisplay Error", message))
    }
}

/// Change what's displayed on the screen to reflect the current contents of `rl_line_buffer`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, undo, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     redisplay::redisplay();
///     let _ = CString::from_raw(buffer);
///     assert!(undo::modifying(0, 1) == 0);
///     let new_buffer = CString::new("Test").unwrap().into_raw();
///     vars::rl_line_buffer = new_buffer;
///     redisplay::redisplay();
///     let _ = CString::from_raw(new_buffer);
/// }
/// ```
pub fn redisplay() -> () {
    unsafe { ext_redisplay::rl_redisplay() }
}

/// Force the line to be updated and redisplayed, whether or not Readline thinks the screen display
/// is correct.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     assert!(redisplay::forced_update_display().is_ok());
///     let _ = CString::from_raw(buffer);
/// }
/// ```
pub fn forced_update_display() -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_redisplay::rl_forced_update_display(),
                  "Unable to force update display!")
    }
}

/// Tell the update functions that we have moved onto a new (empty) line, usually after outputting a
/// newline.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test\n").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     assert!(redisplay::on_new_line().is_ok());
///     let _ = CString::from_raw(buffer);
/// }
/// ```
pub fn on_new_line() -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_redisplay::rl_on_new_line(),
                  "Unable to set on new line!")
    }
}

/// Tell the update functions that we have moved onto a new line, with `rl_prompt` already
/// displayed. This could be used by applications that want to output the prompt string themselves,
/// but still need Readline to know the prompt string length for redisplay. It should be used after
/// setting `rl_already_prompted`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{self, redisplay, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test\n").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     print!("blah: ");
///     vars::rl_already_prompted = 1;
///     let _ = readline::readline("blah: ");
///     redisplay::redisplay();
///     print!("blah: ");
///     assert!(redisplay::on_new_line_with_prompt().is_ok());
///     vars::rl_already_prompted = 0;
///     let _ = CString::from_raw(buffer);
/// }
/// ```
pub fn on_new_line_with_prompt() -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_redisplay::rl_on_new_line_with_prompt(),
                  "Unable to set on new line with prompt!")
    }
}

/// Reset the display state to a clean state and redisplay the current line starting on a new line.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     assert!(redisplay::reset_line_state().is_ok());
///     let _ = CString::from_raw(buffer);
/// }
/// ```
pub fn reset_line_state() -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_redisplay::rl_reset_line_state(),
                  "Unable to reset line state!")
    }
}

/// Move the cursor to the start of the next screen line.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     assert!(redisplay::crlf().is_ok());
///     let _ = CString::from_raw(buffer);
/// }
/// ```
pub fn crlf() -> Result<i32, ::ReadlineError> {
    unsafe { genresult(ext_redisplay::rl_crlf(), "Unable to set crlf!") }
}

/// Display character `c` on `rl_outstream`. If Readline has not been set to display meta characters
/// directly, this will convert meta characters to a meta-prefixed key sequence. This is intended
/// for use by applications which wish to do their own redisplay.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// assert!(redisplay::show_char('a').is_ok());
/// ```
pub fn show_char(c: char) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_redisplay::rl_show_char(c as i32);

        if res > 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Redisplay Error", "Unable to show character!"))
        }
    }
}

/// The arguments are a format string as would be supplied to printf, possibly containing conversion
/// specifications such as `%d`, and any additional arguments necessary to satisfy the conversion
/// specifications. The resulting string is displayed in the echo area. The echo area is also used
/// to display numeric arguments and search strings. You should call `rl_save_prompt` to save the
/// prompt information before calling this function.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// redisplay::save_prompt();
/// assert!(redisplay::message("test message").is_ok());
/// redisplay::rl_restore_prompt();
/// assert!(redisplay::clear_message().is_ok());
/// ```
pub fn message(message: &str) -> Result<i32, ::ReadlineError> {
    let ptr = try!(CString::new(message)).as_ptr();
    unsafe { genresult(ext_redisplay::rl_message(ptr), "Unable to show message!") }
}

/// Clear the message in the echo area. If the prompt was saved with a call to `rl_save_prompt`
/// before the last call to `rl_message`, call `rl_restore_prompt` before calling this function.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// redisplay::save_prompt();
/// assert!(redisplay::message("test message").is_ok());
/// redisplay::rl_restore_prompt();
/// assert!(redisplay::clear_message().is_ok());
/// ```
pub fn clear_message() -> Result<i32, ::ReadlineError> {
    unsafe {
        genresult(ext_redisplay::rl_clear_message(),
                  "Unable to clear message!")
    }
}

/// Save the local Readline prompt display state in preparation for displaying a new message in the
/// message area with `rl_message()`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// redisplay::save_prompt();
/// assert!(redisplay::message("test message").is_ok());
/// redisplay::rl_restore_prompt();
/// assert!(redisplay::clear_message().is_ok());
/// ```
pub fn save_prompt() -> () {
    unsafe { ext_redisplay::rl_save_prompt() }
}

/// Restore the local Readline prompt display state saved by the most recent call to
/// `rl_save_prompt`.  If `rl_save_prompt` was called to save the prompt before a call to
/// `rl_message`, this function should be called before the corresponding call to
/// `rl_clear_message`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// redisplay::save_prompt();
/// assert!(redisplay::message("test message").is_ok());
/// redisplay::rl_restore_prompt();
/// assert!(redisplay::clear_message().is_ok());
/// ```
pub fn rl_restore_prompt() -> () {
    unsafe { ext_redisplay::rl_restore_prompt() }
}

/// Expand any special character sequences in prompt and set up the local Readline prompt redisplay
/// variables. This function is called by `readline()`. It may also be called to expand the primary
/// prompt if the `rl_on_new_line_with_prompt()` function or `rl_already_prompted` variable is used.
/// It returns the number of visible characters on the last line of the (possibly multi-line)
/// prompt. Applications may indicate that the prompt contains characters that take up no physical
/// screen space when displayed by bracketing a sequence of such characters with the special markers
/// `RL_PROMPT_START_IGNORE` and `RL_PROMPT_END_IGNORE` (declared in `readline.h'. This may be
/// used to embed terminal-specific escape sequences in prompts.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// assert!(redisplay::expand_prompt("test message").is_ok());
/// ```
pub fn expand_prompt(prompt: &str) -> Result<i32, ::ReadlineError> {
    let ptr = try!(CString::new(prompt)).as_ptr();
    unsafe {
        let res = ext_redisplay::rl_expand_prompt(ptr);

        if res > 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Redisplay Error", "Unable to expand prompt!"))
        }
    }
}

/// Make Readline use prompt for subsequent redisplay. This calls `rl_expand_prompt()` to expand
/// the prompt and sets `rl_prompt` to the result.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{redisplay, util};
///
/// util::init();
///
/// assert!(redisplay::set_prompt("NEW PROMPT: ").is_ok());
/// ```
pub fn set_prompt(prompt: &str) -> Result<i32, ::ReadlineError> {
    let ptr = try!(CString::new(prompt)).as_ptr();
    unsafe { genresult(ext_redisplay::rl_set_prompt(ptr), "Unable to set prompt!") }
}
