# Pong starter game for Amethyst game engine
A simple project I'm using to learn the Rust programming language following this tutorial:
https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-01.html

## Intall

Currently only works on a mac, you will have to install the latest version of Xcode

Make sure you're using the latest version of Rust
```sh
rustup update
```

```sh
cargo build
cargo run
```


## Rust things

### Imports

https://doc.rust-lang.org/std/keyword.use.html

external:
```rs
use path::to::item;
use my_library::some_function();
```

module:
```rs
mod include_me;
```

## Bug fixes fixes

### Amethyst - metal out of date
failed to run custom build command for `gfx-backend-metal v0.3.3`
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer