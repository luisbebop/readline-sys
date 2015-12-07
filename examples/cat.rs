//! A simple implementation of `cat` using `rl_sys::readline`

extern crate rl_sys;

fn main() {
    loop {
        let input: String = match rl_sys::readline("".to_owned()) {
            Some(s) => s,
            None => break,  // EOF, ctrl-d
        };
        println!("{}", input);
    }
}
