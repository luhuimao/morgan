#!/usr/bin/env bash
set -e

cd "$(dirname "$0")"/..

cargo build --package morgan-install
export PATH=$PWD/target/debug:$PATH

echo "\`\`\`manpage"
morgan-install --help
echo "\`\`\`"
echo ""

commands=(init info deploy update run)

for x in "${commands[@]}"; do
    echo "\`\`\`manpage"
    morgan-install "${x}" --help
    echo "\`\`\`"
    echo ""
done
