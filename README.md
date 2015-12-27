# readline-sys
## Version
[![Crates.io](https://img.shields.io/crates/v/rl-sys.svg)](https://crates.io/crates/rl-sys)
[![Build
Status](https://travis-ci.org/rustyhorde/readline-sys.svg?branch=0.4.1)](https://travis-ci.org/rustyhorde/readline-sys)

Native bindings to
[libreadline](https://cnswww.cns.cwru.edu/php/chet/readline/rltop.html).

## Features
- thin wrappers around `readline` and `add_history`
- write history line to file: `add_history_persist`
- load history from file: `preload_history`
- library version: `version`

## API Documentation
[Rustdocs](https://rustyhorde.github.io/readline-sys/readline-sys/rl_sys/index.html)

## Usage
Add `rl-sys` as a dependency in `Cargo.toml`

```toml
[dependencies]
rl-sys = "~0.4.1"
```

A simple implementation of `cat` using `rl_sys::readline`
```rust
extern crate rl_sys;

fn main() {
    loop {
        let input: String = match rl_sys::readline("") {
            Ok(Some(s)) => s,
            Ok(None) => break,  // user entered ctrl-d
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        println!("{}", input);

        // Enables up/down arrow scrolling through history
        rl_sys::add_history(&input);
    }
}
```

Check out the more comprehensive `shell` example in the [examples directory](examples).

## Development
To work on this crate, remember to `git clone --recursive` or `git submodule
init && git submodule update`.

## License
Distributed under the [MIT License](LICENSE).
