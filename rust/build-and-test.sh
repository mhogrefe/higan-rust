#!/bin/bash
cargo update &&
cargo +nightly fmt --all &&
cargo test --release -- --nocapture &&
cargo build --release --target wasm32-unknown-unknown
