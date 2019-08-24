#!/usr/bin/env bash
#
# Run a minimal Morgan cluster.  Ctrl-C to exit.
#
# Before running this script ensure standard Morgan programs are available
# in the PATH, or that `cargo build --all` ran successfully
#
set -e

# Prefer possible `cargo build --all` binaries over PATH binaries
cd "$(dirname "$0")/"
PATH=$PWD/target/debug:$PATH

ok=true
for program in morgan-{drone,genesis,keygen,validator}; do
  $program -V || ok=false
done
$ok || {
  echo
  echo "Unable to locate required programs.  Try building them first with:"
  echo
  echo "  $ cargo build --all"
  echo
  exit 1
}

blockstreamSocket=/tmp/morgan-blockstream.sock # Default to location used by the block explorer
while [[ -n $1 ]]; do
  if [[ $1 = --blockstream ]]; then
    blockstreamSocket=$2
    shift 2
  else
    echo "Unknown argument: $1"
    exit 1
  fi
done

export RUST_LOG=${RUST_LOG:-morgan=info} # if RUST_LOG is unset, default to info
export RUST_BACKTRACE=1
dataDir=$PWD/target/"$(basename "$0" .sh)"

set -x
morgan-keygen -o "$dataDir"/config/leader-keypair.json
morgan-keygen -o "$dataDir"/config/leader-vote-account-keypair.json
morgan-keygen -o "$dataDir"/config/leader-stake-account-keypair.json
morgan-keygen -o "$dataDir"/config/drone-keypair.json
morgan-keygen -o "$dataDir"/config/leader-storage-account-keypair.json

leaderVoteAccountPubkey=$(\
  morgan-wallet \
    --keypair "$dataDir"/config/leader-vote-account-keypair.json  \
    address \
)

morgan-genesis \
  --difs 1000000000 \
  --bootstrap-leader-difs 10000000 \
  --difs-per-signature 1 \
  --hashes-per-tick sleep \
  --mint "$dataDir"/config/drone-keypair.json \
  --bootstrap-leader-keypair "$dataDir"/config/leader-keypair.json \
  --bootstrap-vote-keypair "$dataDir"/config/leader-vote-account-keypair.json \
  --bootstrap-stake-keypair "$dataDir"/config/leader-stake-account-keypair.json \
  --bootstrap-storage-keypair "$dataDir"/config/leader-storage-account-keypair.json \
  --ledger "$dataDir"/ledger

morgan-drone --keypair "$dataDir"/config/drone-keypair.json &
drone=$!

args=(
  --identity "$dataDir"/config/leader-keypair.json
  --storage-keypair "$dataDir"/config/leader-storage-account-keypair.json
  --voting-keypair "$dataDir"/config/leader-vote-account-keypair.json
  --vote-account "$leaderVoteAccountPubkey"
  --ledger "$dataDir"/ledger/
  --gossip-port 10001
  --rpc-port 10099
  --rpc-drone-address 127.0.0.1:11100
)
if [[ -n $blockstreamSocket ]]; then
  args+=(--blockstream "$blockstreamSocket")
fi
morgan-validator "${args[@]}" &
validator=$!

abort() {
  set +e
  kill "$drone" "$validator"
}
trap abort INT TERM EXIT

wait "$validator"
