#!/usr/bin/env bash

if [ "$#" -ne 1 ]; then
    echo "Error: Must provide the full path to the project to clean"
    exit 1
fi

./../../../interface/bpf/rust-utils/clean.sh "$PWD"/"$1"
