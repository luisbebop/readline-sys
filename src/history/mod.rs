//! Readline History API
use libc::{c_char, c_int, c_void};
use std::clone::Clone;
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

#[repr(C)]
#[derive(Copy)]
/// A history entry.  This includes the line, the optional timestamp, and optional application
/// specific data.
pub struct HistoryEntry {
    pub line: *mut c_char,
    pub timestamp: *mut c_char,
    pub data: HistoryData,
}

impl Clone for HistoryEntry {
    fn clone(&self) -> Self {
        *self
    }
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
#[derive(Copy)]
/// The history state.  This includes a pointer the the history entries array, and offset to the
/// current position in the history, the history length, the history size, and any history flags
/// that have been set.
pub struct HistoryState {
    pub entries: *mut *mut HistoryEntry,
    pub offset: c_int,
    pub length: c_int,
    pub size: c_int,
    pub flags: c_int,
}

impl Clone for HistoryState {
    fn clone(&self) -> Self {
        *self
    }
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
