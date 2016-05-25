//! GNU Readline History API
//!
//! Many programs read input from the user a line at a time. The GNU History library is able to keep
//! track of those lines, associate arbitrary data with each line, and utilize information from
//! previous lines in composing new ones.
//!
//! The programmer using the History library has available functions for remembering lines on a
//! history list, associating arbitrary data with a line, removing lines from the list, searching
//! through the list for a line containing an arbitrary text string, and referencing any line in the
//! list directly. In addition, a history expansion function is available which provides for a
//! consistent user interface across different programs.
//!
//! The user using programs written with the History library has the benefit of a consistent user
//! interface with a set of well-known commands for manipulating the text of previous lines and
//! using that text in new commands. The basic history manipulation commands are similar to the
//! history substitution provided by csh.
//!
//! If the programmer desires, he can use the Readline library, which includes some history
//! manipulation by default, and has the added advantage of command line editing.
//!
//! Before declaring any functions using any functionality the History library provides in other
//! code, an application writer should include the file <readline/history.h> in any file that uses
//! the History library's features. It supplies extern declarations for all of the library's public
//! functions and variables, and declares all of the public data structures.
use libc::{c_char, c_int, c_uint, c_void};
use std::default::Default;
use std::ffi::CStr;
use std::fmt;

pub mod expand;
pub mod histfile;
pub mod listinfo;
pub mod listmgmt;
pub mod mgmt;
pub mod move_;
pub mod search;
pub mod vars;

/// Application specific data attached to the history entry.
pub type HistoryData = *mut c_void;
/// Inhibit Expansion Function Type.
pub type InhibitExpansionFunc = Option<extern "C" fn(*mut c_char, c_uint) -> c_int>;

#[repr(C)]
#[derive(Clone, Copy)]
/// A history entry.
pub struct HistoryEntry {
    /// The line as a string.
    pub line: *mut c_char,
    /// An optional timestamp.
    pub timestamp: *mut c_char,
    /// Optional application specific data.
    pub data: HistoryData,
}

impl Default for HistoryEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl fmt::Debug for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = unsafe { CStr::from_ptr(self.line).to_string_lossy().into_owned() };
        let time = unsafe { CStr::from_ptr(self.timestamp).to_string_lossy().into_owned() };
        write!(f,
               "HistoryEntry {{ line: {}, timestamp: {}, data: {:?} }}",
               line,
               time,
               self.data)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
/// The history state.
pub struct HistoryState {
    /// A pointer to the history entries list.
    pub entries: *mut *mut HistoryEntry,
    /// An offset to the current position in the history.
    pub offset: c_int,
    /// The length of the history entries list.
    pub length: c_int,
    /// The size of the history entries list.
    pub size: c_int,
    /// Any history flags that have been set.
    pub flags: c_int,
}

impl Default for HistoryState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl fmt::Debug for HistoryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "HistoryState {{ entries: {:?}, offset: {:?}, length: {:?}, size: {:?}, flags: {:?}",
               self.entries,
               self.offset,
               self.length,
               self.size,
               self.flags)
    }
}
