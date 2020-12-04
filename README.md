Comlib
======
Library for participating in programming competitions in Rust.
The goal is to include utilities for most commonly needed scenarios in programming competitions, such as input reading, as well as to include many common algorithms implemented in a generic way.

Usage
-----
Often in programming competitions it is not possible to submit multiple files, let along to depend on external crates.
The easiest way to use this library is to include all of the relevant parts as dependencies:
```toml
[dependencies]
comlib-input = { git = "https://github.com/henkkuli/comlib" }
```
and then use [`cargo equip`](https://crates.io/crates/cargo-equip) to compile your code into a single file:
```bash
cargo equip --exclude rand --resolve-cfgs --remove docs --remove comments --rustfmt --minify libs --check --src src/main.rs -o submission.rs
```
