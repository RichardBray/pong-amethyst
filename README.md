# Pong starter game for Amethyst game engine
A simple project I'm using to learn the Rust programming language following this tutorial:
https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-01.html

## Install

Currently only works on a mac, you will have to install the latest version of Xcode

Make sure you're using the latest version of Rust
```sh
rustup update
```

```sh
cargo build
cargo run
```

## Helpful VS Code plugins

- Better TOML => https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
- Rust => https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
- vscode-ron => https://marketplace.visualstudio.com/items?itemName=a5huynh.vscode-ron
## Bug fixes fixes

### Amethyst - metal out of date
failed to run custom build command for `gfx-backend-metal v0.3.3`
```sh
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```