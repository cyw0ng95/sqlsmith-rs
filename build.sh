#!/bin/bash

cargo build && \
    cargo test && \
    cargo run --bin server