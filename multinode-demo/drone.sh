#!/usr/bin/env bash
#
# Starts an instance of morgan-tokenbot
#
here=$(dirname "$0")

# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

[[ -f "$MORGAN_CONFIG_DIR"/mint-keypair.json ]] || {
  echo "$MORGAN_CONFIG_DIR/mint-keypair.json not found, create it by running:"
  echo
  echo "  ${here}/setup.sh"
  exit 1
}

set -x
# shellcheck disable=SC2086 # Don't want to double quote $morgan_tokenbot
# exec $morgan_tokenbot --keypair "$MORGAN_CONFIG_DIR"/mint-keypair.json "$@"
trap 'kill "$pid" && wait "$pid"' INT TERM ERR

$morgan_tokenbot \
  --keypair "$MORGAN_CONFIG_DIR"/mint-keypair.json \
  "$@" \
  > >($drone_logger) 2>&1 &
pid=$!
wait "$pid"