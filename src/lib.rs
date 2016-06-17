// Copyright 2015-2016 Jason Ozias
//
// This file is part of rl-sys.
//
// rl-sys is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rl-sys is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rl-sys.  If not, see <http://www.gnu.org/licenses/>.

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
#[macro_use]
extern crate bitflags;
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
