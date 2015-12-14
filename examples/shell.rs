//! Example of a command line shell with history support
//!
//! Use the arrow keys to go forwards and backwards through the history.
//!
//! Currently supported commands:
//! `history -c` -> clear the history
//! `history -s n` -> stifle the history to n entries
//! `history -u` -> unstifle the history

extern crate rl_sys;

use rl_sys::{history, readline};
use std::process::Command;

fn main() {
    println!("welcome to shrl!");

    let mut counter = 0;
    loop {
        let prompt = format!("{}$ ", counter);
        let input: String = match readline(&prompt) {
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
        let _ = history::listmgmt::add_history(&input);

        if input.starts_with("history") {
            let argv: Vec<&str> = input.split_whitespace().collect();
            match argv.get(1).map(|s| &**s) {
                Some("-c") => history::listmgmt::clear_history(),
                Some("-s") => {
                    if let Some(s) = argv.get(2) {
                        if let Ok(n) = s.parse::<i32>() {
                            // Stifle the history so that only *n* entries will be stored.
                            history::listmgmt::stifle_history(n);
                        }
                    }
                }
                Some("-u") => {
                    history::listmgmt::unstifle_history();
                }
                Some(_) => println!("unrecognized history command"),
                None => {
                    println!("TODO: print the history.");
                }
            }
        } else {
            let output = match Command::new("sh").arg("-c").arg(input).output() {
                Ok(output) => output,
                Err(e) => {
                    println!("failed to execute process: {}", e);
                    continue;
                }
            };
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        counter += 1;
    }
}
