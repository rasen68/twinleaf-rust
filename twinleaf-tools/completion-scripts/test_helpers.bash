
# Constants
RPC_LIST_FAILED="[RPC_LIST_FAILED]"
TIO_SIM_ADDRESS="udp4://127.0.0.1"
PORT_NUM=7855
FAILS=0

TIO_OPTS=( -r -s -h --root --sensor --help )
TIO_RPC_OPTS=( ${TIO_OPTS[@]} -t -T -d --req-type --rep-type --debug )
TIO_RPC_LIST_OPTS=( ${TIO_OPTS[@]} --name-only --capture-only )
TIO_RPC_DUMP_OPTS=( ${TIO_OPTS[@]} --capture )
TIO_CAPTURE_OPTS=( ${TIO_OPTS[@]} --timeout )
TIO_RPC_TYPES=( u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string )

TIO_SIM_RPCS=( dev.desc dev.firmware.upgrade dev.firmware.upload dev.metadata dev.name dev.stop rpc.hash rpc.id rpc.info rpc.list rpc.listinfo rpc.name test.amplitude test.capture test.enable test.frequency test.go test.noise test.status )
TIO_SIM_DEV_RPCS=( dev.desc dev.firmware.upgrade dev.firmware.upload dev.metadata dev.name dev.stop )

_tio_sim() {
	killall tio &>/dev/null
	PORT_NUM=$(( $PORT_NUM + 1 ))
	setsid tio simulate --port $PORT_NUM </dev/null &>/dev/null &
}

_fail_test() {
	echo "" >&2
	echo "FAILED: $*"
	echo "FAILED: $*" >&2
	echo "Expected: ${EXPECTED[*]}" >&2
	echo "Actual:   ${ACTUAL[*]}" >&2
	echo "" >&2
	FAILS=$(( $FAILS + 1 ))
}

# Test drive function
# Completes the current word as if you typed $@<TAB>
# Usage: set EXPECTED to an array of expected comp options
# (order doesn't matter), then call _comp_test tio rpc ...
# Note: Last argument is completed, include empty last
# argument "" if you want to complete next word
_comp_test() {
	local cmd="$1" func cur prev
	local -a ACTUAL

	# Lookup registered completer for this cmd and save to func
	if [[ $(complete -p "$cmd") =~ -F[[:space:]]+([^[:space:]]+) ]]; then
		func="${BASH_REMATCH[1]}"
	else
		echo "FAILED: $* (no -F completion for $cmd)" >&2
		return 1
	fi

	# Load command line to environment / args
	COMP_WORDS=( "$@" )
	COMP_CWORD=$((${#COMP_WORDS[@]} - 1))
	COMP_LINE="$*"
	COMP_POINT=${#COMP_LINE}
	cur="${COMP_WORDS[COMP_CWORD]}"
	prev=""
	if (( $COMP_CWORD > 0 )); then
		prev="${COMP_WORDS[COMP_CWORD - 1]}"
	fi
	COMPREPLY=()

	# Call our completion function
	"$func" "$cmd" "$cur" "$prev"

	# Sort arrays
	local ACTUAL
	mapfile -t ACTUAL < <(printf '%s\n' "${COMPREPLY[@]}" | sort)
	mapfile -t EXPECTED < <(printf '%s\n' "${EXPECTED[@]}" | sort)

	# Compare
	if [[ ${#ACTUAL[@]} != ${#EXPECTED[@]} ]]; then
		_fail_test $*
		return 1
	fi
	local i
	for ((i = 0; i < ${#ACTUAL[@]}; i++)); do
		if [[ "${ACTUAL[i]}" != "${EXPECTED[i]}" ]]; then
			_fail_test $*
			return 1
		fi
	done

	# Passed
	echo "Passed: $*"
	return 0
}
