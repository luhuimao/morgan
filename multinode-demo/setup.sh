#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e
"$here"/clear-config.sh

# Create genesis ledger
$morgan_keybot -o "$SOLANA_CONFIG_DIR"/mint-keypair.json
$morgan_keybot -o "$SOLANA_CONFIG_DIR"/bootstrap-leader-keypair.json
$morgan_keybot -o "$SOLANA_CONFIG_DIR"/bootstrap-leader-vote-keypair.json
$morgan_keybot -o "$SOLANA_CONFIG_DIR"/bootstrap-leader-stake-keypair.json
$morgan_keybot -o "$SOLANA_CONFIG_DIR"/bootstrap-leader-storage-keypair.json

args=("$@")
default_arg --bootstrap-leader-keypair "$SOLANA_CONFIG_DIR"/bootstrap-leader-keypair.json
default_arg --bootstrap-vote-keypair "$SOLANA_CONFIG_DIR"/bootstrap-leader-vote-keypair.json
default_arg --bootstrap-stake-keypair "$SOLANA_CONFIG_DIR"/bootstrap-leader-stake-keypair.json
default_arg --bootstrap-storage-keypair "$SOLANA_CONFIG_DIR"/bootstrap-leader-storage-keypair.json
default_arg --ledger "$SOLANA_RSYNC_CONFIG_DIR"/ledger
default_arg --mint "$SOLANA_CONFIG_DIR"/mint-keypair.json
default_arg --difs 100000000000000
default_arg --hashes-per-tick sleep

$morgan_genesis "${args[@]}"

test -d "$SOLANA_RSYNC_CONFIG_DIR"/ledger
cp -a "$SOLANA_RSYNC_CONFIG_DIR"/ledger "$SOLANA_CONFIG_DIR"/bootstrap-leader-ledger
