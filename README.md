# Tauri Superapp

## Build

Building app requires the latest `stable` Rust toolchain, the `wasm32` target and `cargo-make`, `tauri-cli` and
`wasm-bindgen` build tools.

To install Rust and its toolchains/targets via [rustup](https://rustup.rs/), if it is not already installed, run:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

rustup toolchain install stable
rustup target add wasm32-unknown-unknown --toolchain stable
```

To install [cargo-make](https://github.com/sagiegurari/cargo-make), [tauri-cli](https://v2.tauri.app/start/create-project/) and
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), run:

```shell
cargo install --force cargo-make wasm-bindgen-cli
cargo install tauri-cli --version "^2.0.0" --locked
```

To build and run Tauri Superapp, use the following command:

```shell
cargo make run
```

Or for a debug build:

```shell
cargo make -p debug run
```

To run without building, use the following command:

```shell
cargo make start
```
