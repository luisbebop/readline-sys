//! A simple implementation of `cat` using `rl_sys::readline`

extern crate rl_sys;

fn main() {
    loop {
        let input: String = match rl_sys::readline("") {
            Ok(Some(s)) => s,
            Ok(None) => break,  // EOF, ctrl-d
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        println!("{}", input);
    }
}
