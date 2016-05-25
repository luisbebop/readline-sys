//! [2.4.5 Allowing Undoing]
//! [2.4.5 allowing undoing]: https://goo.gl/ZemRqX
//!
//! Supporting the undo command is a painless thing, and makes your functions much more useful. It
//! is certainly easy to try something if you know you can undo it.
//!
//! If your function simply inserts text once, or deletes text once, and uses `rl_insert_text()` or
//! `rl_delete_text()` to do it, then undoing is already done for you automatically.
//!
//! If you do multiple insertions or multiple deletions, or any combination of these operations, you
//! should group them together into one operation. This is done with `rl_begin_undo_group()` and
//! `rl_end_undo_group()`.
//!
//! The types of events that can be undone are:
//!
//! ```c
//! enum undo_code { UNDO_DELETE, UNDO_INSERT, UNDO_BEGIN, UNDO_END };
//! ```
//!
//! Notice that `UNDO_DELETE` means to insert some text, and `UNDO_INSERT` means to delete some
//! text. That is, the undo code tells what to undo, not how to undo it. `UNDO_BEGIN` and
//! `UNDO_END` are tags added by `rl_begin_undo_group()` and `rl_end_undo_group()`.
use libc::c_int;
use self::UndoType::{Begin, Delete, End, Insert};
use std::ffi::CString;

/// Undo Event Types
#[derive(Debug, PartialEq)]
pub enum UndoType {
    /// Insert some text.
    Delete,
    /// Delete some text.
    Insert,
    /// Start an undo group (added by `rl_begin_undo_group()`)
    Begin,
    /// End an undo group (add by `rl_end_undo_group()`)
    End,
}

impl From<i32> for UndoType {
    fn from(i: i32) -> UndoType {
        if i == 0 {
            Delete
        } else if i == 1 {
            Insert
        } else if i == 2 {
            Begin
        } else if i == 3 {
            End
        } else {
            panic!("Unknown BindType!");
        }
    }
}

mod ext_undo {
    use libc::{c_char, c_int, c_uint};

    extern "C" {
        pub fn rl_begin_undo_group() -> c_int;
        pub fn rl_end_undo_group() -> c_int;
        pub fn rl_add_undo(undo: c_uint, start: c_int, end: c_int, text: *mut c_char) -> ();
        pub fn rl_free_undo_list() -> ();
        pub fn rl_do_undo() -> c_int;
        pub fn rl_modifying(start: c_int, end: c_int) -> c_int;
    }
}

/// Begins saving undo information in a group construct. The undo information usually comes from
/// calls to `rl_insert_text()` and `rl_delete_text()`, but could be the result of calls to
/// `rl_add_undo()`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{undo, util};
///
/// util::init();
///
/// assert!(undo::begin_undo_group() == 0);
/// assert!(undo::end_undo_group() == 0);
/// ```
pub fn begin_undo_group() -> i32 {
    unsafe { ext_undo::rl_begin_undo_group() }
}

/// Closes the current undo group started with `rl_begin_undo_group()`. There should be one call to
/// `rl_end_undo_group()` for each call to `rl_begin_undo_group()`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{undo, util};
///
/// util::init();
///
/// assert!(undo::begin_undo_group() == 0);
/// assert!(undo::end_undo_group() == 0);
/// ```
pub fn end_undo_group() -> i32 {
    unsafe { ext_undo::rl_end_undo_group() }
}

/// Remember how to undo an event (according to `what`).
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{undo, util};
/// use rl_sys::readline::undo::UndoType;
///
/// util::init();
///
/// assert!(undo::begin_undo_group() == 0);
/// assert!(undo::add_undo(UndoType::Delete, "I deleted this!").is_ok());
/// assert!(undo::end_undo_group() == 0);
/// ```
#[cfg_attr(feature = "clippy", allow(cast_possible_truncation, cast_possible_wrap))]
pub fn add_undo(what: UndoType, text: &str) -> Result<(), ::ReadlineError> {
    let ptr = try!(CString::new(text)).into_raw();

    unsafe {
        ext_undo::rl_add_undo(what as u32, 0, text.len() as c_int, ptr);
        let _ = CString::from_raw(ptr);
    }

    Ok(())
}

/// Free the existing undo list.  Note that this depends on history being initialized and used.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, mgmt};
/// use rl_sys::readline::{undo, util};
/// use rl_sys::readline::undo::UndoType;
///
/// util::init();
/// mgmt::init();
///
/// assert!(undo::begin_undo_group() == 0);
/// assert!(listmgmt::add("I deleted this!").is_ok());
/// assert!(undo::add_undo(UndoType::Insert, "I deleted this!").is_ok());
/// assert!(undo::end_undo_group() == 0);
/// undo::free_undo_list();
/// mgmt::cleanup();
/// ```
pub fn free_undo_list() -> () {
    unsafe { ext_undo::rl_free_undo_list() }
}

/// Undo the first thing on the undo list. Returns an Err if there was nothing to undo, Ok if
/// something was undone. Note that this depends on history being initialized and used.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, mgmt};
/// use rl_sys::readline::{undo, util};
/// use rl_sys::readline::undo::UndoType;
///
/// util::init();
/// mgmt::init();
///
/// assert!(undo::begin_undo_group() == 0);
/// assert!(listmgmt::add("I deleted this!").is_ok());
/// assert!(undo::add_undo(UndoType::Insert, "I deleted this!").is_ok());
/// assert!(undo::end_undo_group() == 0);
/// assert!(undo::do_undo().is_ok());
/// ```
pub fn do_undo() -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_undo::rl_do_undo();

        if res == 0 {
            Err(::ReadlineError::new("Undo Error", "There was nothing to undo!"))
        } else {
            Ok(res)
        }
    }
}

/// Tell Readline to save the text between `start` and `end` as a single undo unit. It is assumed
/// that you will subsequently modify that text.
///
/// If you neither insert nor delete text, but directly modify the existing text (e.g.,
/// change its case), call `rl_modifying()` once, just before you modify the text. You must supply
/// the indices of the text range that you are going to modify.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{undo, util, vars};
/// use std::ffi::CString;
///
/// util::init();
///
/// let buffer = CString::new("test").unwrap().into_raw();
/// unsafe {
///     vars::rl_line_buffer = buffer;
///     let _ = CString::from_raw(buffer);
///     assert!(undo::modifying(0, 1) == 0);
///     let new_buffer = CString::new("Test").unwrap().into_raw();
///     vars::rl_line_buffer = new_buffer;
///     let _ = CString::from_raw(new_buffer);
/// }
/// ```
pub fn modifying(start: i32, end: i32) -> i32 {
    unsafe { ext_undo::rl_modifying(start, end) }
}
