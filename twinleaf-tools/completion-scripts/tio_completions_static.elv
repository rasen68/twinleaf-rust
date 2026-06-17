
use builtin;
use str;

set edit:completion:arg-completer[tio] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tio'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tio'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List connected devices'
            cand monitor 'Live sensor data display'
            cand health 'Live timing and rate diagnostics'
            cand dump 'Dump raw packets from a device'
            cand log 'Log samples to a file'
            cand rpc 'Execute a device RPC'
            cand upgrade 'Upgrade device firmware'
            cand proxy 'Multiplex a sensor over TCP'
            cand test 'Run a simulated sine wave Twinleaf device over UDP'
            cand completions 'Generate shell completions for tio'
        }
        &'tio;list'= {
            cand -a 'Include serial ports with unknown VID/PID'
            cand --all 'Include serial ports with unknown VID/PID'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;monitor'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand --fps 'fps'
            cand -c 'c'
            cand --colors 'colors'
            cand --depth 'Routing depth limit (default: unlimited)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;health'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand --jitter-window 'Seconds for jitter calculation window (>= 1)'
            cand --ppm-warn 'Warning threshold in parts per million (>= 0)'
            cand --ppm-err 'Error threshold in parts per million (>= 0)'
            cand --streams 'Comma-separated stream IDs to monitor (e.g., 0,1,5)'
            cand --fps 'UI refresh rate for heartbeat animation and stale detection (1–60)'
            cand --stale-ms 'Mark streams as stale after this many milliseconds without data (>= 1)'
            cand -n 'Maximum number of events to keep in history (>= 1)'
            cand --event-log-size 'Maximum number of events to keep in history (>= 1)'
            cand --event-display-lines 'Number of event lines to show (>= 3)'
            cand -q 'Suppress the footer help text'
            cand --quiet 'Suppress the footer help text'
            cand -w 'Only show warning and error events in the log'
            cand --warnings-only 'Only show warning and error events in the log'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'tio;dump'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand --depth 'Routing depth limit (default: unlimited)'
            cand -d 'Show parsed data samples'
            cand --data 'Show parsed data samples'
            cand -m 'Show metadata on boundaries'
            cand --meta 'Show metadata on boundaries'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;log'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand -f 'Output log file path'
            cand --depth 'Routing depth (only used in --raw mode)'
            cand --duration 'Stop after this wall-clock duration (e.g. 30s, 5m, 2h)'
            cand -u 'Unbuffered output (flush every packet)'
            cand --raw 'Raw mode: skip metadata request and dump all packets'
            cand -h 'Print help'
            cand --help 'Print help'
            cand meta 'Log metadata to a file. See "tio log meta --help" for more options'
            cand dump 'Dump data from binary log file(s)'
            cand inspect 'Summarize the contents of binary log file(s)'
            cand csv 'Convert binary log data to CSV'
            cand hdf 'Convert binary log files to HDF5 format'
        }
        &'tio;log;meta'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand -f 'Output metadata file path'
            cand -h 'Print help'
            cand --help 'Print help'
            cand reroute 'Reroute metadata packets in a metadata file'
        }
        &'tio;log;meta;reroute'= {
            cand -s 'New device route (e.g., /0/1)'
            cand --sensor 'New device route (e.g., /0/1)'
            cand -o 'Output metadata file path (defaults to <input>_rerouted.tio)'
            cand --output 'Output metadata file path (defaults to <input>_rerouted.tio)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;log;dump'= {
            cand -s 'Sensor path in the sensor tree (e.g., /, /0, /0/1)'
            cand --sensor 'Sensor path in the sensor tree (e.g., /, /0, /0/1)'
            cand --depth 'Routing depth limit (default: unlimited)'
            cand -d 'Show parsed data samples'
            cand --data 'Show parsed data samples'
            cand -m 'Show metadata on boundaries'
            cand --meta 'Show metadata on boundaries'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;log;inspect'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;log;csv'= {
            cand -s 'Sensor route in the device tree (default: /)'
            cand -o 'Output filename prefix'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;log;hdf'= {
            cand -o 'Output file path (defaults to input filename with .h5 extension)'
            cand -g 'Filter streams using a glob pattern (e.g. "/*/vector")'
            cand --glob 'Filter streams using a glob pattern (e.g. "/*/vector")'
            cand -l 'How to organize runs in the output (none=flat, stream=per-stream, device=per-device, global=all-shared)'
            cand --split 'How to organize runs in the output (none=flat, stream=per-stream, device=per-device, global=all-shared)'
            cand -p 'When to detect discontinuities (continuous=any gap, monotonic=only time backward)'
            cand --policy 'When to detect discontinuities (continuous=any gap, monotonic=only time backward)'
            cand -c 'Enable deflate compression (saves space, slows down write significantly)'
            cand --compress 'Enable deflate compression (saves space, slows down write significantly)'
            cand -d 'Enable debug output for glob matching'
            cand --debug 'Enable debug output for glob matching'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'tio;rpc'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand -t 'RPC request type'
            cand --req-type 'RPC request type'
            cand -T 'RPC reply type'
            cand --rep-type 'RPC reply type'
            cand -d 'Enable debug output'
            cand --debug 'Enable debug output'
            cand -h 'Print help'
            cand --help 'Print help'
            cand list 'List available RPCs on the device'
            cand dump 'Dump RPC data from the device'
        }
        &'tio;rpc;list'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand --name-only 'Output only names, not permissions or types'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;rpc;dump'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand --capture 'Trigger a capture before dumping'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;upgrade'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand -y 'Skip confirmation prompt'
            cand --yes 'Skip confirmation prompt'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;proxy'= {
            cand -p 'TCP port to listen on for clients'
            cand --port 'TCP port to listen on for clients'
            cand -s 'Sensor subtree to look at'
            cand --subtree 'Sensor subtree to look at'
            cand -t 'Timestamp format'
            cand --timestamp 'Timestamp format'
            cand -T 'Time limit for sensor reconnection attempts (seconds)'
            cand --timeout 'Time limit for sensor reconnection attempts (seconds)'
            cand -k 'Kick off slow clients instead of dropping traffic'
            cand --kick-slow 'Kick off slow clients instead of dropping traffic'
            cand -v 'Verbose output'
            cand --verbose 'Verbose output'
            cand -d 'Debugging output'
            cand --debug 'Debugging output'
            cand --dump 'Dump packet traffic except sample data/metadata or heartbeats'
            cand --dump-data 'Dump sample data traffic'
            cand --dump-meta 'Dump sample metadata traffic'
            cand --dump-hb 'Dump heartbeat traffic'
            cand -a 'Deprecated; running without -s <url> now auto-detects by default'
            cand --auto 'Deprecated; running without -s <url> now auto-detects by default'
            cand -e 'Deprecated; use `tio list` instead'
            cand --enumerate 'Deprecated; use `tio list` instead'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand nmea 'Bridge Twinleaf sensor data to NMEA TCP stream'
        }
        &'tio;proxy;nmea'= {
            cand -r 'Sensor root address'
            cand --root 'Sensor root address'
            cand -s 'Sensor path in the sensor tree'
            cand --sensor 'Sensor path in the sensor tree'
            cand -p 'TCP port to listen on'
            cand --port 'TCP port to listen on'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tio;test'= {
            cand --samplerate 'Sample rate in Hz'
            cand --frequency 'Initial sine wave frequency in Hz'
            cand --amplitude 'Initial sine wave amplitude in V'
            cand --noise 'Initial white noise level in V/sqrt(Hz)'
            cand --segment-seconds 'Segment duration in seconds'
            cand --port 'UDP port to listen on'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'tio;completions'= {
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
    ]
    $completions[$command]
}
