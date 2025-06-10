#!/bin/bash

RUN=0
show_help() {
    echo "Usage: $0 [-r] [-h]"
    echo "  -r: Run the program after building and testing."
    echo "  -h: Show this help message."
    exit 0
}

while getopts "rh" opt; do
    case $opt in
        r)
            RUN=1
            ;;
        h)
            show_help
            ;;
        *)
            show_help
            ;;
    esac
done

if command -v sccache &> /dev/null; then
    export RUSTC_WRAPPER=sccache
fi

if [ $RUN -eq 1 ]; then
    cd view && {
        pnpm install
        pnpm run dev &
    }
    cd ~-
fi

cargo build && \
    cargo test 

if [ $RUN -eq 1 ]; then
    cargo run --bin server
fi