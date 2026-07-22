#!/usr/bin/env zsh
SCRIPT_DIR="$(cd "$(dirname "${(%):-%x}")" && pwd)"

# Load completion system
autoload -Uz compinit
compinit

# Load zpty
zmodload zsh/zpty

# Source test helpers and tio_completions_dynamic.zsh for _tio
source "$SCRIPT_DIR/test_helpers.sh" || exit 1
source "$SCRIPT_DIR/tio_completions_dynamic.zsh" || exit 1

# Kill all existing tio simulate/proxies
if pgrep ^tio$ >/dev/null; then
	killall tio
fi

### First test with no tio simulate, no RPCs findable ###
EXPECTED=( list dump $RPC_LIST_FAILED )
zcomptest tio rpc ""

EXPECTED=( )
zcomptest tio rpc rpc.name ""

EXPECTED=( $RPC_LIST_FAILED )
zcomptest tio rpc -s /0 ""

EXPECTED=( ${TIO_RPC_TYPES[@]} )
zcomptest tio rpc -t ""

EXPECTED=( ${TIO_RPC_OPTS[@]} )
zcomptest tio rpc -

EXPECTED=( $RPC_LIST_FAILED )
zcomptest tio capture ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} )
zcomptest tio capture -

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} )
zcomptest tio capture rpc.name ""

EXPECTED=( ${TIO_RPC_LIST_OPTS[@]} )
zcomptest tio rpc list ""

used=( -s )
EXPECTED=( ${(@)TIO_RPC_LIST_OPTS:|used} )
zcomptest tio rpc list -s /0 -

EXPECTED=( $RPC_LIST_FAILED )
zcomptest tio rpc dump ""

EXPECTED=( $RPC_LIST_FAILED )
zcomptest tio rpc dump -s /0 ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} )
zcomptest tio rpc dump -

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} )
zcomptest tio rpc dump rpc.name ""

### Now test --root functionality with tio simulates ###
### Each tests gets a fresh one to minimize delay ###
_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc --root $TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc -r $TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc -r$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc -s/0 -r$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_OPTS[@]} $RPC_LIST_FAILED )
zcomptest tio rpc -s/fake -r$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc dump --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_RPC_LIST_OPTS[@]} )
zcomptest tio rpc list --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

_tio_sim
EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
zcomptest tio capture --root=$TIO_SIM_ADDRESS:$PORT_NUM ""

### Connect to tio proxy to test remaining functionality ###
_tio_sim
setsid tio proxy $TIO_SIM_ADDRESS:$PORT_NUM </dev/null &>/dev/null &

EXPECTED=( ${TIO_RPC_OPTS[@]} list dump ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc ""

EXPECTED=( ${TIO_SIM_DEV_RPCS[@]} )
zcomptest tio rpc dev

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc dump ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
zcomptest tio capture ""

EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc -s/0 ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc dump -s/0 ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} test.capture )
zcomptest tio capture -s/0 ""

EXPECTED=( ${TIO_RPC_OPTS[@]} $RPC_LIST_FAILED )
zcomptest tio rpc -s /fake ""

EXPECTED=( ${TIO_CAPTURE_OPTS[@]} $RPC_LIST_FAILED )
zcomptest tio capture -s /fake ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} $RPC_LIST_FAILED )
zcomptest tio rpc dump -s /fake ""

EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc -d ""

EXPECTED=( ${TIO_RPC_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc -s /0 -t u8 ""

EXPECTED=( ${TIO_RPC_DUMP_OPTS[@]} ${TIO_SIM_RPCS[@]} )
zcomptest tio rpc dump -s /0 --capture ""

EXPECTED=( a${TIO_CAPTURE_OPTS[@]} test.capture )
zcomptest tio capture -s /0 --timeout 1 ""

# Shut down our remaining tio proxy
killall tio &>/dev/null || true

if (( $FAILS > 0 )); then
	echo "$FAILS test(s) failed" >&2
	exit 1
fi
echo "All dynamic completion tests passed"
exit 0
