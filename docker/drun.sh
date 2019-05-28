#!/bin/bash

cd projects/source
rustup target add x86_64-unknown-linux-musl

cargo clean
cargo build --target x86_64-unknown-linux-musl --release
