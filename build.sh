#!/bin/bash

if command -v sccache &> /dev/null
then
    export RUSTC_WRAPPER=sccache
fi

cargo build && \
    cargo test && \
    cargo run --bin server