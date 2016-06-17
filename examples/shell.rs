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

//! Example of a command line shell with history support
//!
//! Use the arrow keys to go forwards and backwards through the history.
//!
//! Currently supported commands:
//!
//! * `history -c` -> clear the history
//! * `history -s n` -> stifle the history to n entries
//! * `history -u` -> unstifle the history
//! * `exit` -> exit the shell
//!
//! Other commands will be run via a subshell and the result output.
//!
extern crate rl_sys;
extern crate time;

use rl_sys::history::{expand, listinfo, listmgmt, mgmt};
use rl_sys::readline;
use std::process::Command;

fn main() {
    println!("welcome to shrl!");

    loop {
        let prompt = format!("{} $ ", time::now().rfc3339());
        let input: String = match readline::readline(&prompt) {
            Ok(Some(s)) => s,
            Ok(None) => break,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Ignore empty input.
        if input.is_empty() {
            continue;
        }

        // Add user input to history.
        listmgmt::add(&input).unwrap_or_else(|e| println!("{:?}", e));

        if input.starts_with("exit") {
            break;
        } else if input.starts_with("history") {
            let toks: Vec<String> = match expand::tokenize(&input) {
                Ok(t) => t,
                Err(e) => {
                    println!("{:?}", e);
                    Vec::new()
                }
            };

            let argv: Vec<&str> = toks.iter()
                                      .filter_map(|s| {
                                          if s == "history" {
                                              None
                                          } else {
                                              Some(s.as_ref())
                                          }
                                      })
                                      .collect();

            match argv.get(0) {
                Some(&"-c") => listmgmt::clear(),
                Some(&"-s") => {
                    if let Some(s) = argv.get(1) {
                        if let Ok(n) = s.parse::<i32>() {
                            // Stifle the history so that only *n* entries will be stored.
                            listmgmt::stifle(n);
                        }
                    }
                }
                Some(&"-u") => {
                    listmgmt::unstifle();
                }
                Some(&_) => println!("unrecognized history command"),
                None => {
                    println!("{:?}", listinfo::list());
                }
            }
        } else {
            match Command::new("sh").arg("-c").arg(input).output() {
                Ok(output) => {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                Err(e) => {
                    println!("failed to execute process: {}", e);
                    continue;
                }
            }

        }
    }

    mgmt::cleanup();
}
