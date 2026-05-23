
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tio' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tio'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'tio' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List connected devices')
            [CompletionResult]::new('monitor', 'monitor', [CompletionResultType]::ParameterValue, 'Live sensor data display')
            [CompletionResult]::new('health', 'health', [CompletionResultType]::ParameterValue, 'Live timing and rate diagnostics')
            [CompletionResult]::new('dump', 'dump', [CompletionResultType]::ParameterValue, 'Dump raw packets from a device')
            [CompletionResult]::new('log', 'log', [CompletionResultType]::ParameterValue, 'Log samples to a file')
            [CompletionResult]::new('rpc', 'rpc', [CompletionResultType]::ParameterValue, 'Execute a device RPC')
            [CompletionResult]::new('upgrade', 'upgrade', [CompletionResultType]::ParameterValue, 'Upgrade device firmware')
            [CompletionResult]::new('proxy', 'proxy', [CompletionResultType]::ParameterValue, 'Multiplex a sensor over TCP')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Run a simulated sine wave Twinleaf device over UDP')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions for tio')
            break
        }
        'tio;list' {
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Include serial ports with unknown VID/PID')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'Include serial ports with unknown VID/PID')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;monitor' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--fps', '--fps', [CompletionResultType]::ParameterName, 'fps')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--colors', '--colors', [CompletionResultType]::ParameterName, 'colors')
            [CompletionResult]::new('--depth', '--depth', [CompletionResultType]::ParameterName, 'Routing depth limit (default: unlimited)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;health' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--jitter-window', '--jitter-window', [CompletionResultType]::ParameterName, 'Seconds for jitter calculation window (>= 1)')
            [CompletionResult]::new('--ppm-warn', '--ppm-warn', [CompletionResultType]::ParameterName, 'Warning threshold in parts per million (>= 0)')
            [CompletionResult]::new('--ppm-err', '--ppm-err', [CompletionResultType]::ParameterName, 'Error threshold in parts per million (>= 0)')
            [CompletionResult]::new('--streams', '--streams', [CompletionResultType]::ParameterName, 'Comma-separated stream IDs to monitor (e.g., 0,1,5)')
            [CompletionResult]::new('--fps', '--fps', [CompletionResultType]::ParameterName, 'UI refresh rate for heartbeat animation and stale detection (1–60)')
            [CompletionResult]::new('--stale-ms', '--stale-ms', [CompletionResultType]::ParameterName, 'Mark streams as stale after this many milliseconds without data (>= 1)')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Maximum number of events to keep in history (>= 1)')
            [CompletionResult]::new('--event-log-size', '--event-log-size', [CompletionResultType]::ParameterName, 'Maximum number of events to keep in history (>= 1)')
            [CompletionResult]::new('--event-display-lines', '--event-display-lines', [CompletionResultType]::ParameterName, 'Number of event lines to show (>= 3)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress the footer help text')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress the footer help text')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Only show warning and error events in the log')
            [CompletionResult]::new('--warnings-only', '--warnings-only', [CompletionResultType]::ParameterName, 'Only show warning and error events in the log')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'tio;dump' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--depth', '--depth', [CompletionResultType]::ParameterName, 'Routing depth limit (default: unlimited)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Show parsed data samples')
            [CompletionResult]::new('--data', '--data', [CompletionResultType]::ParameterName, 'Show parsed data samples')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Show metadata on boundaries')
            [CompletionResult]::new('--meta', '--meta', [CompletionResultType]::ParameterName, 'Show metadata on boundaries')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;log' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output log file path')
            [CompletionResult]::new('--depth', '--depth', [CompletionResultType]::ParameterName, 'Routing depth (only used in --raw mode)')
            [CompletionResult]::new('--duration', '--duration', [CompletionResultType]::ParameterName, 'Stop after this wall-clock duration (e.g. 30s, 5m, 2h)')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Unbuffered output (flush every packet)')
            [CompletionResult]::new('--raw', '--raw', [CompletionResultType]::ParameterName, 'Raw mode: skip metadata request and dump all packets')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('meta', 'meta', [CompletionResultType]::ParameterValue, 'Log metadata to a file. See "tio log meta --help" for more options')
            [CompletionResult]::new('dump', 'dump', [CompletionResultType]::ParameterValue, 'Dump data from binary log file(s)')
            [CompletionResult]::new('inspect', 'inspect', [CompletionResultType]::ParameterValue, 'Summarize the contents of binary log file(s)')
            [CompletionResult]::new('csv', 'csv', [CompletionResultType]::ParameterValue, 'Convert binary log data to CSV')
            [CompletionResult]::new('hdf', 'hdf', [CompletionResultType]::ParameterValue, 'Convert binary log files to HDF5 format')
            break
        }
        'tio;log;meta' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output metadata file path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('reroute', 'reroute', [CompletionResultType]::ParameterValue, 'Reroute metadata packets in a metadata file')
            break
        }
        'tio;log;meta;reroute' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'New device route (e.g., /0/1)')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'New device route (e.g., /0/1)')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output metadata file path (defaults to <input>_rerouted.tio)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output metadata file path (defaults to <input>_rerouted.tio)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;log;dump' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree (e.g., /, /0, /0/1)')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree (e.g., /, /0, /0/1)')
            [CompletionResult]::new('--depth', '--depth', [CompletionResultType]::ParameterName, 'Routing depth limit (default: unlimited)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Show parsed data samples')
            [CompletionResult]::new('--data', '--data', [CompletionResultType]::ParameterName, 'Show parsed data samples')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Show metadata on boundaries')
            [CompletionResult]::new('--meta', '--meta', [CompletionResultType]::ParameterName, 'Show metadata on boundaries')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;log;inspect' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;log;csv' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor route in the device tree (default: /)')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output filename prefix')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;log;hdf' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file path (defaults to input filename with .h5 extension)')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'Filter streams using a glob pattern (e.g. "/*/vector")')
            [CompletionResult]::new('--glob', '--glob', [CompletionResultType]::ParameterName, 'Filter streams using a glob pattern (e.g. "/*/vector")')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'How to organize runs in the output (none=flat, stream=per-stream, device=per-device, global=all-shared)')
            [CompletionResult]::new('--split', '--split', [CompletionResultType]::ParameterName, 'How to organize runs in the output (none=flat, stream=per-stream, device=per-device, global=all-shared)')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'When to detect discontinuities (continuous=any gap, monotonic=only time backward)')
            [CompletionResult]::new('--policy', '--policy', [CompletionResultType]::ParameterName, 'When to detect discontinuities (continuous=any gap, monotonic=only time backward)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Enable deflate compression (saves space, slows down write significantly)')
            [CompletionResult]::new('--compress', '--compress', [CompletionResultType]::ParameterName, 'Enable deflate compression (saves space, slows down write significantly)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Enable debug output for glob matching')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'Enable debug output for glob matching')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'tio;rpc' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'RPC request type')
            [CompletionResult]::new('--req-type', '--req-type', [CompletionResultType]::ParameterName, 'RPC request type')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'RPC reply type')
            [CompletionResult]::new('--rep-type', '--rep-type', [CompletionResultType]::ParameterName, 'RPC reply type')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Enable debug output')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'Enable debug output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List available RPCs on the device')
            [CompletionResult]::new('dump', 'dump', [CompletionResultType]::ParameterValue, 'Dump RPC data from the device')
            break
        }
        'tio;rpc;list' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--name-only', '--name-only', [CompletionResultType]::ParameterName, 'Output only names, not permissions or types')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;rpc;dump' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--capture', '--capture', [CompletionResultType]::ParameterName, 'Trigger a capture before dumping')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;upgrade' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'Skip confirmation prompt')
            [CompletionResult]::new('--yes', '--yes', [CompletionResultType]::ParameterName, 'Skip confirmation prompt')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;proxy' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'TCP port to listen on for clients')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'TCP port to listen on for clients')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor subtree to look at')
            [CompletionResult]::new('--subtree', '--subtree', [CompletionResultType]::ParameterName, 'Sensor subtree to look at')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Timestamp format')
            [CompletionResult]::new('--timestamp', '--timestamp', [CompletionResultType]::ParameterName, 'Timestamp format')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Time limit for sensor reconnection attempts (seconds)')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'Time limit for sensor reconnection attempts (seconds)')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'Kick off slow clients instead of dropping traffic')
            [CompletionResult]::new('--kick-slow', '--kick-slow', [CompletionResultType]::ParameterName, 'Kick off slow clients instead of dropping traffic')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Verbose output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Debugging output')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'Debugging output')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'Dump packet traffic except sample data/metadata or heartbeats')
            [CompletionResult]::new('--dump-data', '--dump-data', [CompletionResultType]::ParameterName, 'Dump sample data traffic')
            [CompletionResult]::new('--dump-meta', '--dump-meta', [CompletionResultType]::ParameterName, 'Dump sample metadata traffic')
            [CompletionResult]::new('--dump-hb', '--dump-hb', [CompletionResultType]::ParameterName, 'Dump heartbeat traffic')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Deprecated; running without -s <url> now auto-detects by default')
            [CompletionResult]::new('--auto', '--auto', [CompletionResultType]::ParameterName, 'Deprecated; running without -s <url> now auto-detects by default')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Deprecated; use `tio list` instead')
            [CompletionResult]::new('--enumerate', '--enumerate', [CompletionResultType]::ParameterName, 'Deprecated; use `tio list` instead')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('nmea', 'nmea', [CompletionResultType]::ParameterValue, 'Bridge Twinleaf sensor data to NMEA TCP stream')
            break
        }
        'tio;proxy;nmea' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('--root', '--root', [CompletionResultType]::ParameterName, 'Sensor root address')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('--sensor', '--sensor', [CompletionResultType]::ParameterName, 'Sensor path in the sensor tree')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'TCP port to listen on')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'TCP port to listen on')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tio;test' {
            [CompletionResult]::new('--samplerate', '--samplerate', [CompletionResultType]::ParameterName, 'Sample rate in Hz')
            [CompletionResult]::new('--frequency', '--frequency', [CompletionResultType]::ParameterName, 'Initial sine wave frequency in Hz')
            [CompletionResult]::new('--amplitude', '--amplitude', [CompletionResultType]::ParameterName, 'Initial sine wave amplitude in V')
            [CompletionResult]::new('--noise', '--noise', [CompletionResultType]::ParameterName, 'Initial white noise level in V/sqrt(Hz)')
            [CompletionResult]::new('--segment-seconds', '--segment-seconds', [CompletionResultType]::ParameterName, 'Segment duration in seconds')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'UDP port to listen on')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'tio;completions' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
