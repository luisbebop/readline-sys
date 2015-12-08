# readline-sys
## Version
[![Crates.io](https://img.shields.io/crates/v/rl-sys.svg)](https://crates.io/crates/rl-sys)
[![Build
Status](https://travis-ci.org/rustyhorde/readline-sys.svg?branch=master)](https://travis-ci.org/rustyhorde/readline-sys)

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
rl-sys = "~0.3.0"
```

A simple implementation of `echo` using `rl_sys::readline`
```rust
extern crate rl_sys;

fn main() {
    loop {
        let response = match rl_sys::readline("$ ") {
            Ok(o) => match o {
                Some(s) => s,
                None    => break,
            },
            None    => break,
        };
        println!("{}", response);
    }
}
```

## License
Distributed under the [MIT License](LICENSE).
