#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

make -C ../../../controllers/bpf/c/
cp ../../../controllers/bpf/c/out/noop.so .
