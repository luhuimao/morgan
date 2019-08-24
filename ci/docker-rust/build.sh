#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

docker build -t morganlabs/rust .

read -r rustc version _ < <(docker run morganlabs/rust rustc --version)
[[ $rustc = rustc ]]
docker tag morganlabs/rust:latest morganlabs/rust:"$version"
docker push morganlabs/rust:"$version"
docker push morganlabs/rust:latest
