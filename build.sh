#!/bin/bash

if command -v sccache &> /dev/null
then
    export RUSTC_WRAPPER=sccache
fi

cd view && {
    pnpm install && \
        pnpm run dev &
}

cargo build && \
    cargo test && \
    cargo run --bin server