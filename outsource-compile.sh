#! /bin/bash

# Outsource compilation to a better machine which can compile it faster
# This is for release builds
# The user:host should be $1



mkdir -p target/release

tar -c --exclude target . | ssh $1 "mkdir -p ~/outsource-compile && cd ~/outsource-compile && tar -x && CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=\"/usr/local/bin/aarch64-unknown-linux-gnu-gcc\" cargo build --release --target aarch64-unknown-linux-musl && cat target/aarch64-unknown-linux-musl/release/kolloquy-offload-server" > target/release/kolloquy-offload-server
