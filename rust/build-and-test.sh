#!/bin/bash
cargo update &&
cargo +nightly fmt &&
cargo test --release &&
cargo build --release --target wasm32-unknown-unknown
