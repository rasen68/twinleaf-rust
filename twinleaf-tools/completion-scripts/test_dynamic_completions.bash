#!/usr/bin/env bash
# Requires bash-completion installed under /usr/share (typical on Linux).
# Safe to run from any cwd; sources sibling scripts relative to this file.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Find bash_completion library for _get_comp_words_by_nref,
# and of course tio_completions_dynamic.bash for _tio
source /usr/share/bash-completion/bash_completion || exit 1
source "$SCRIPT_DIR/tio_completions_dynamic.bash" || exit 1
source "$SCRIPT_DIR/test_helpers.bash" || exit 1

# Kill all existing tio simulate/proxies
if pgrep ^tio$; then
	killall tio
fi

### First test with no tio simulate, no RPCs findable ###
EXPECTED=( ${TIO_RPC_OPTS[@]} list dump $RPC_LIST_FAILED )
_comp_test tio rpc ""

EXPECTED=( ${TIO_RPC_OPTS[@]} [ARG])
_comp_test tio rpc rpc.name ""

EXPECTED=( ${TIO_RPC_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio rpc -s /0 ""

EXPECTED=( ${TIO_RPC_TYPES[@]} )
_comp_test tio rpc -t ""

EXPECTED=( ${TIO_RPC_OPTS[@]} )
_comp_test tio rpc -

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio capture ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio capture -s /0 ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} )
_comp_test tio capture rpc.name ""

EXPECTED=( ${TIO_RPC_LIST_OPTS[@]} )
_comp_test tio rpc list ""

EXPECTED=( ${TIO_RPC_LIST_OPTS[@]} )
_comp_test tio rpc list -s /0 -

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio rpc dump ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio rpc dump -s /0 ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} )
_comp_test tio rpc dump rpc.name ""

### Now test --root functionality with tio simulates ###
### Each tests gets a fresh one to minimize delay ###
_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc --root $TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc -r $TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc -r$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc -s/0 -r$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio rpc -s/fake -r$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc dump --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_LIST_OPTS[@]} )
_comp_test tio rpc list --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
_comp_test tio capture --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

### Connect to tio proxy to test remaining functionality ###
_tio_sim
setsid tio proxy $TIO_SIM_ADDRESS:$PORT_NUM </dev/null &>/dev/null &

EXPECTED=( ${TIO_RPC_OPTS[@]} list dump ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc ""

EXPECTED=( ${TIO_SIM_DEV_RPCS[@]} )
_comp_test tio rpc dev

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc dump ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
_comp_test tio capture ""

EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc -s/0 ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc dump -s/0 ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
_comp_test tio capture -s/0 ""

EXPECTED=( ${TIO_RPC_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio rpc -s /fake ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio capture -s /fake ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} $RPC_LIST_FAILED )
_comp_test tio rpc dump -s /fake ""

EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc -d ""

EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc -s /0 -t u8 ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
_comp_test tio rpc dump -s /0 --capture ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
_comp_test tio capture -s /0 --timeout 1 ""

# Shut down our remaining tio proxy
killall tio
exit 0
