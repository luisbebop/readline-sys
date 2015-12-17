//! This library provides native bindings for the [GNU readline library][1].
//!
//! [1]: https://cnswww.cns.cwru.edu/php/chet/readline/rltop.html
//!
//! The GNU Readline library provides a set of functions for use by applications
//! that allow users to edit command lines as they are typed in. Both Emacs and
//! vi editing modes are available. The Readline library includes additional
//! functions to maintain a list of previously-entered command lines, to recall
//! and perhaps reedit those lines, and perform csh-like history expansion on
//! previous commands.
//!
//! # Examples
//!
//! ```
//! use rl_sys::readline;
//! use rl_sys::history::{listmgmt, mgmt};
//!
//! loop {
//!     let input = match readline::readline("$ ") {
//!         Ok(Some(s)) => match &*s {
//!             "clear" => {
//!                 listmgmt::clear();
//!                 continue;
//!             }
//!             _ => s
//!         },
//!         Ok(None) => break,  // EOF encountered
//!         Err(e) => {
//!             println!("{}", e);
//!             continue;
//!         }
//!     };
//!     println!("{}", input);
//!
//!     // Add input to history.
//!     match listmgmt::add(&input) {
//!         Ok(_) => {},
//!         Err(e) => { println!("{:?}", e); },
//!     }
//! }
//!
//! mgmt::cleanup();
//! ```
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![deny(missing_docs)]
extern crate errno;
extern crate libc;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate sodium_sys;
extern crate time;

pub use error::{HistoryError, ReadlineError};
pub use version::version;

mod error;
pub mod history;
pub mod readline;
mod version;
