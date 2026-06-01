# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_tio_global_optspecs
	string join \n h/help V/version
end

function __fish_tio_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_tio_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_tio_using_subcommand
	set -l cmd (__fish_tio_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c tio -n "__fish_tio_needs_command" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_needs_command" -s V -l version -d 'Print version'
complete -c tio -n "__fish_tio_needs_command" -f -a "list" -d 'List connected devices'
complete -c tio -n "__fish_tio_needs_command" -f -a "monitor" -d 'Live sensor data display'
complete -c tio -n "__fish_tio_needs_command" -f -a "health" -d 'Live timing and rate diagnostics'
complete -c tio -n "__fish_tio_needs_command" -f -a "dump" -d 'Dump raw packets from a device'
complete -c tio -n "__fish_tio_needs_command" -f -a "log" -d 'Log samples to a file'
complete -c tio -n "__fish_tio_needs_command" -f -a "rpc" -d 'Execute a device RPC'
complete -c tio -n "__fish_tio_needs_command" -f -a "upgrade" -d 'Upgrade device firmware'
complete -c tio -n "__fish_tio_needs_command" -f -a "proxy" -d 'Multiplex a sensor over TCP'
complete -c tio -n "__fish_tio_needs_command" -f -a "test" -d 'Run a simulated sine wave Twinleaf device over UDP'
complete -c tio -n "__fish_tio_needs_command" -f -a "completions" -d 'Generate shell completions for tio'
complete -c tio -n "__fish_tio_using_subcommand list" -s a -l all -d 'Include serial ports with unknown VID/PID'
complete -c tio -n "__fish_tio_using_subcommand list" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand monitor" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand monitor" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand monitor" -l fps -r
complete -c tio -n "__fish_tio_using_subcommand monitor" -s c -l colors -r
complete -c tio -n "__fish_tio_using_subcommand monitor" -l depth -d 'Routing depth limit (default: unlimited)' -r
complete -c tio -n "__fish_tio_using_subcommand monitor" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand health" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand health" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l jitter-window -d 'Seconds for jitter calculation window (>= 1)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l ppm-warn -d 'Warning threshold in parts per million (>= 0)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l ppm-err -d 'Error threshold in parts per million (>= 0)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l streams -d 'Comma-separated stream IDs to monitor (e.g., 0,1,5)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l fps -d 'UI refresh rate for heartbeat animation and stale detection (1–60)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l stale-ms -d 'Mark streams as stale after this many milliseconds without data (>= 1)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -s n -l event-log-size -d 'Maximum number of events to keep in history (>= 1)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -l event-display-lines -d 'Number of event lines to show (>= 3)' -r
complete -c tio -n "__fish_tio_using_subcommand health" -s q -l quiet -d 'Suppress the footer help text'
complete -c tio -n "__fish_tio_using_subcommand health" -s w -l warnings-only -d 'Only show warning and error events in the log'
complete -c tio -n "__fish_tio_using_subcommand health" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand health" -s V -l version -d 'Print version'
complete -c tio -n "__fish_tio_using_subcommand dump" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand dump" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand dump" -l depth -d 'Routing depth limit (default: unlimited)' -r
complete -c tio -n "__fish_tio_using_subcommand dump" -s d -l data -d 'Show parsed data samples'
complete -c tio -n "__fish_tio_using_subcommand dump" -s m -l meta -d 'Show metadata on boundaries'
complete -c tio -n "__fish_tio_using_subcommand dump" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -s f -d 'Output log file path' -r
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -l depth -d 'Routing depth (only used in --raw mode)' -r
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -l duration -d 'Stop after this wall-clock duration (e.g. 30s, 5m, 2h)' -r
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -s u -d 'Unbuffered output (flush every packet)'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -l raw -d 'Raw mode: skip metadata request and dump all packets'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -f -a "meta" -d 'Log metadata to a file. See "tio log meta --help" for more options'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -f -a "dump" -d 'Dump data from binary log file(s)'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -f -a "inspect" -d 'Summarize the contents of binary log file(s)'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -f -a "csv" -d 'Convert binary log data to CSV'
complete -c tio -n "__fish_tio_using_subcommand log; and not __fish_seen_subcommand_from meta dump inspect csv hdf" -f -a "hdf" -d 'Convert binary log files to HDF5 format'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from meta" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from meta" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from meta" -s f -d 'Output metadata file path' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from meta" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from meta" -f -a "reroute" -d 'Reroute metadata packets in a metadata file'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from dump" -s s -l sensor -d 'Sensor path in the sensor tree (e.g., /, /0, /0/1)' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from dump" -l depth -d 'Routing depth limit (default: unlimited)' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from dump" -s d -l data -d 'Show parsed data samples'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from dump" -s m -l meta -d 'Show metadata on boundaries'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from dump" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from inspect" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from csv" -s s -d 'Sensor route in the device tree (default: /)' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from csv" -s o -d 'Output filename prefix' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from csv" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s o -d 'Output file path (defaults to input filename with .h5 extension)' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s g -l glob -d 'Filter streams using a glob pattern (e.g. "/*/vector")' -r
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s l -l split -d 'How to organize runs in the output (none=flat, stream=per-stream, device=per-device, global=all-shared)' -r -f -a "none\t'No run splitting - flat structure: /{route}/{stream}/{datasets}'
stream\t'Each stream has independent run counter'
device\t'All streams on a device share run counter'
global\t'All streams globally share run counter'"
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s p -l policy -d 'When to detect discontinuities (continuous=any gap, monotonic=only time backward)' -r -f -a "continuous\t'Split on any discontinuity (gaps, rate changes, etc.)'
monotonic\t'Only split when time goes backward (allows gaps)'"
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s c -l compress -d 'Enable deflate compression (saves space, slows down write significantly)'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s d -l debug -d 'Enable debug output for glob matching'
complete -c tio -n "__fish_tio_using_subcommand log; and __fish_seen_subcommand_from hdf" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -s t -l req-type -d 'RPC request type' -r -f -a "u8\t''
u16\t''
u32\t''
u64\t''
i8\t''
i16\t''
i32\t''
i64\t''
f32\t''
f64\t''
string\t''"
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -s T -l rep-type -d 'RPC reply type' -r -f -a "u8\t''
u16\t''
u32\t''
u64\t''
i8\t''
i16\t''
i32\t''
i64\t''
f32\t''
f64\t''
string\t''"
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -s d -l debug -d 'Enable debug output'
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -a "list" -d 'List available RPCs on the device'
complete -c tio -n "__fish_tio_using_subcommand rpc; and not __fish_seen_subcommand_from list dump" -a "dump" -d 'Dump RPC data from the device'
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from list" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from list" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from list" -l name-only -d 'Output only names, not permissions or types'
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from dump" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from dump" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from dump" -l capture -d 'Trigger a capture before dumping'
complete -c tio -n "__fish_tio_using_subcommand rpc; and __fish_seen_subcommand_from dump" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand upgrade" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand upgrade" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand upgrade" -s y -l yes -d 'Skip confirmation prompt'
complete -c tio -n "__fish_tio_using_subcommand upgrade" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s p -l port -d 'TCP port to listen on for clients' -r
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s s -l subtree -d 'Sensor subtree to look at' -r
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s t -l timestamp -d 'Timestamp format' -r
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s T -l timeout -d 'Time limit for sensor reconnection attempts (seconds)' -r
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s k -l kick-slow -d 'Kick off slow clients instead of dropping traffic'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s v -l verbose -d 'Verbose output'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s d -l debug -d 'Debugging output'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -l dump -d 'Dump packet traffic except sample data/metadata or heartbeats'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -l dump-data -d 'Dump sample data traffic'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -l dump-meta -d 'Dump sample metadata traffic'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -l dump-hb -d 'Dump heartbeat traffic'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s a -l auto -d 'Deprecated; running without -s <url> now auto-detects by default'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s e -l enumerate -d 'Deprecated; use `tio list` instead'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -s V -l version -d 'Print version'
complete -c tio -n "__fish_tio_using_subcommand proxy; and not __fish_seen_subcommand_from nmea" -a "nmea" -d 'Bridge Twinleaf sensor data to NMEA TCP stream'
complete -c tio -n "__fish_tio_using_subcommand proxy; and __fish_seen_subcommand_from nmea" -s r -l root -d 'Sensor root address' -r -f
complete -c tio -n "__fish_tio_using_subcommand proxy; and __fish_seen_subcommand_from nmea" -s s -l sensor -d 'Sensor path in the sensor tree' -r
complete -c tio -n "__fish_tio_using_subcommand proxy; and __fish_seen_subcommand_from nmea" -s p -l port -d 'TCP port to listen on' -r
complete -c tio -n "__fish_tio_using_subcommand proxy; and __fish_seen_subcommand_from nmea" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand test" -l samplerate -d 'Sample rate in Hz' -r
complete -c tio -n "__fish_tio_using_subcommand test" -l frequency -d 'Initial sine wave frequency in Hz' -r
complete -c tio -n "__fish_tio_using_subcommand test" -l amplitude -d 'Initial sine wave amplitude in V' -r
complete -c tio -n "__fish_tio_using_subcommand test" -l noise -d 'Initial white noise level in V/sqrt(Hz)' -r
complete -c tio -n "__fish_tio_using_subcommand test" -l segment-seconds -d 'Segment duration in seconds' -r
complete -c tio -n "__fish_tio_using_subcommand test" -l port -d 'UDP port to listen on' -r
complete -c tio -n "__fish_tio_using_subcommand test" -s h -l help -d 'Print help'
complete -c tio -n "__fish_tio_using_subcommand test" -s V -l version -d 'Print version'
complete -c tio -n "__fish_tio_using_subcommand completions" -s h -l help -d 'Print help (see more with \'--help\')'
