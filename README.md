# TinyLJ 

[Github](https://github.com/zatrit/tinylj)

Fork of [luajit-rs](https://dreae.gitlab.io/luajit-rs/luajit)

Crate for interfacing with LuaJIT from Rust, for running high-performance Lua code that
can integrate with native-code written in rust.

## Getting Started

```rust
use tinylj::{c_int, State, lua_fn};

fn return_42(state: &mut State) -> c_int {
    state.push(42);

    1
}

pub fn main() {
    let mut state = State::new();
    state.open_libs();
    state.do_string(r#"print("Hello world!")"#);

    state.push(lua_fn!(return_42));
    state.set_global("return_42");
    state.do_string(r#"print(return_42())"#);
}
```