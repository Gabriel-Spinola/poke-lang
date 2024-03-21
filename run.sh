#!/bin/bash
# I dont know why but this script does not work (yet)

if [ $# -eq 0 ]; then
    echo "Usage: ./run_project.sh <path_to_project> <feature>"
    exit 1
fi

target_path="$1"

if [ $# -eq 1 ]; then
    echo "Usage: ./run_project.sh <path_to_project> <feature>"
    exit 1
fi

feature="$2"

cd poke || exit

if [ "$feature" == "lex" ]; then
    cargo run $target_path --features "debug_trace_lex_execution"
elif [ "$feature" == "vm" ]; then
    cargo run $target_path --features "debug_trace_execution"
elif [ "$feature" == "all" ]; then
    cargo run $target_path --all-features
else
    echo "Invalid feature. Available options: lex, vm, all"
    exit 1
fi
