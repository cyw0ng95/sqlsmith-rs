#!/bin/bash

export CARGO_BUILD_JOBS=$(( $(nproc) * 2 ))
cargo build && \
    cargo test && \
    cargo run --bin server