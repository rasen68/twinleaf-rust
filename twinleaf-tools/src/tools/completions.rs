use clap::CommandFactory;
use crate::{TioCli, CompletionsCli};

pub fn run_completions(completions_cli: CompletionsCli) -> eyre::Result<()> {
    match (completions_cli.r#static, completions_cli.shell) {
        (false, clap_complete::Shell::Bash) => generate_bash_dynamic(),
        (false, clap_complete::Shell::Zsh) => generate_zsh_dynamic(),
        (_, shell) => Ok(clap_complete::generate(shell, &mut TioCli::command(), "tio", &mut std::io::stdout())),
    }
}

fn generate_bash_dynamic() -> eyre::Result<()> {
    let completions = include_str!("../../completion-scripts/tio_completions_static.bash");

    // Add logic to treat RPC names as subcommands so we don't double-complete
    // Any word not starting with a - (other than list & dump) will be treated as an RPC
    let completions = completions.replace("
            tio__subcmd__rpc,list)
                cmd=\"tio__subcmd__rpc__subcmd__list\"
                ;;
    ",
    "
            tio__subcmd__rpc,list)
                cmd=\"tio__subcmd__rpc__subcmd__list\"
                ;;
            tio__subcmd__rpc,[^-]*)
                cmd=\"tio__subcmd__rpc__subcmd__rpcname\"
                ;;
            tio__subcmd__rpc__subcmd__dump,[^-]*)
                cmd=\"tio__subcmd__rpc__subcmd__dump__subcmd__rpcname\"
                ;;
            tio__subcmd__capture,[^-]*)
				cmd=\"tio__subcmd__capture__subcmd__rpcname\"
				;;

    ");


    // Read dynamic RPC completions into options list
    let completions = completions.replace("
        tio__subcmd__rpc)
            opts=\"-r -s -t -T -d -h --root --sensor --req-type --rep-type --debug --help [RPC_NAME] [ARG] list dump\"
    ",
    "
        tio__subcmd__rpc)
			local rpcs
			rpcs=\"$( tio rpc list --name-only 2>/dev/null || echo '[RPC_LIST_FAILED]')\"
			rpcs=\"${rpcs//\\\\n/ }\" # replace newlines with spaces
			rpcs=\"${rpcs% }\"     # remove trailing whitespace
			opts=\"-r -s -t -T -d -h --root --sensor --req-type --rep-type --debug --help list dump $rpcs\"
    ");

    // Add rpcname as subcmd to suggest an arg instead of more rpc names
    // We do this by hooking on dump subcmd and appending in front of it
    // TODO: We could try to copy this from earlier in the string
    // Which would make this more readable and maintainable, but that sounds hard
    let completions = completions.replace("
        tio__subcmd__rpc__subcmd__dump)
    ",
    "
        tio__subcmd__rpc__subcmd__rpcname)
			opts=\"-r -s -t -T -d -h --root --sensor --req-type --rep-type --debug --help [ARG]\"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )
                return 0
            fi
            case \"${prev}\" in
                --root)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                --sensor)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                --req-type)
                    COMPREPLY=($(compgen -W \"u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string\" -- \"${cur}\"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -W \"u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string\" -- \"${cur}\"))
                    return 0
                    ;;
                --rep-type)
                    COMPREPLY=($(compgen -W \"u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string\" -- \"${cur}\"))
                    return 0
                    ;;
                -T)
                    COMPREPLY=($(compgen -W \"u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string\" -- \"${cur}\"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )
            return 0
            ;;

        tio__subcmd__rpc__subcmd__dump)
    ");


    // Add dynamic completions to rpc dump
    // TODO: filter to only rpcs that are dumpable
    let completions = completions.replace("
        tio__subcmd__rpc__subcmd__dump)
            opts=\"-r -s -h --root --sensor --capture --help <RPC_NAME>\"
    ",
    "
        tio__subcmd__rpc__subcmd__dump)
			local rpcs
			rpcs=\"$( tio rpc list --name-only 2>/dev/null || echo '[RPC_LIST_FAILED]')\"
			rpcs=\"${rpcs//\\\\n/ }\" # replace newlines with spaces
			rpcs=\"${rpcs% }\"     # remove trailing whitespace
            opts=\"-r -s -h --root --sensor --capture --help $rpcs\"
    ");

    // Add rpcname as subcmd to dump and capture
    // It doesn't really matter where these go so we'll put them before rpc list
    let completions = completions.replace("
        tio__subcmd__rpc__subcmd__list)
    ",
    "
        tio__subcmd__rpc__subcmd__dump__subcmd__rpcname)
            opts=\"-r -s -h --root --sensor --capture --help\"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )
                return 0
            fi
            case \"${prev}\" in
                --root)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                --sensor)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )
            return 0
            ;;

        tio__subcmd__capture__subcmd__rpcname)
            opts=\"-r -s -h --root --sensor --timeout --help\"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )
                return 0
            fi
            case \"${prev}\" in
                --root)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                --sensor)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f \"${cur}\"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )
            return 0
            ;;
        tio__subcmd__rpc__subcmd__list)
    ");

    // Add dynamic completions to tio capture
    let completions = completions.replace("
        tio__subcmd__capture)
            opts=\"-r -s -h --root --sensor --timeout --help [RPC_NAME]\"
    ",
    "
        tio__subcmd__capture)
			local rpcs
			rpcs=\"$( tio rpc list --name-only --capture-only 2>/dev/null || echo '[RPC_LIST_FAILED]')\"
			rpcs=\"${rpcs//\\\\n/ }\" # replace newlines with spaces
			rpcs=\"${rpcs% }\"     # remove trailing whitespace
			opts=\"-r -s -h --root --sensor --timeout --help $rpcs\"
    ");

    // Add rpcname as subcmd to capture so we only suggest one rpc name
    // We hook on completions subcmd because that comes right after capture
    let completions = completions.replace("
        tio__subcmd__completions)
    ",
    "
        tio__subcmd__completions)
    ");


    print!("{}", completions);
    Ok(())
}

fn generate_zsh_dynamic() -> eyre::Result<()> {
    // NOTE: there is a known bug where completions break on rpc list/dump with
    // a sensor/route option, e.g. "tio rpc list -s /0 <TAB>".
    // This is because the rpc subcommand steals the sensor/route option and adds on
    // all its other options as well, which zsh doesn't like and causes a buggy output
    // This does not seem to be fixable because we want "tio rpc" to accept these options
    // before the rpc name, which also means it must accept them before list/dump
    let completions = include_str!("../../completion-scripts/tio_completions_static.zsh");

    // First remove rpc name and arg from rpc opts
    let completions = completions.replace("
'::rpc_name -- RPC name to execute:' \\
'::rpc_arg -- RPC argument value:' \\", "");

    // Change matching logic to look at last completed word
    let completions = completions.replace("
        words=($line[3] \"${words[@]}\")
        (( CURRENT += 1 ))
        curcontext=\"${curcontext%:*:*}:tio-rpc-command-$line[3]:\"
        case $line[3] in
            (list)",

    "
        words=($line[1] \"${words[@]}\")
		(( CURRENT += 1 ))
		curcontext=\"${curcontext%:*:*}:tio-rpc-command-$line[1]:\"
		case $line[1] in
			(list)");

    // Save line to _line, which will come in handy later
    let completions = completions.replace("
(rpc)
_arguments",

    "
(rpc)
local _line=( \"${line[@]}\" )
_arguments");

    // Pass additional arguments to _tio__subcmd__rpc_commands
    // We slice line from 2 (first thing after "rpc") to -2 (last completed option)
    // We also pass the length of what we added so it can slice it off
    let completions = completions.replace("\":: :_tio__subcmd__rpc_commands\" \\",
    "\":: :_tio__subcmd__rpc_commands ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\");

    // Remove rpc name from dump opts,
    // And use it as a hook to add case for last word being just "rpc"
    // If it is, we are at "tio rpc" and want to complete rpc names
    // We do a similar slicing thing to above using our saved _line
    // But we start from 3 to not pass the "dump"
    let completions = completions.replace("
':rpc_name -- RPC name to dump:' \\",

    "
\":: :_tio__subcmd__rpc_names ${_line[3,-2]} $(( ${#_line[3,-2]} + 1 ))\" \\
&& ret=0
;;
(rpc)
;;
([^-]*)
_arguments \"${_arguments_options[@]}\" : \\
'-r+[Sensor root address]:ROOT:_urls' \\
'--root=[Sensor root address]:ROOT:_urls' \\
'-s+[Sensor path in the sensor tree]:ROUTE:_default' \\
'--sensor=[Sensor path in the sensor tree]:ROUTE:_default' \\
'-t+[RPC request type]:REQ_TYPE:(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string)' \\
'--req-type=[RPC request type]:REQ_TYPE:(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string)' \\
'-T+[RPC reply type]:REP_TYPE:(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string)' \\
'--rep-type=[RPC reply type]:REP_TYPE:(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 string)' \\
'-d[Enable debug output]' \\
'--debug[Enable debug output]' \\
':rpc_arg -- RPC argument value:' \\");

    // Replace capture rpc name with dynamic completion, and pass in line
    let completions = completions.replace("'::rpc_name -- Capture RPC name to execute:' \\",
    "\":: :_tio__subcmd__capture_rpc_names ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\");

    // Add functions to dynamically get rpc names,
    // Replacing old completion case for "tio rpc [list/dump/[RPC_NAME]"
    let completions = completions.replace("
(( $+functions[_tio__subcmd__rpc_commands] )) ||
_tio__subcmd__rpc_commands() {
    local commands; commands=(
'list:List available RPCs on the device' \\
'dump:Dump RPC data from the device' \\
    )
    _describe -t commands 'tio rpc commands' commands \"$@\"",

    "
(( $+functions[_tio__helper__list_rpcs] )) ||
_tio__helper__list_rpcs() {
	local opts=()
	local next=false;
	for item in $@; do
		if $next; then
			opts+=( \"$item\" )
			next=false;
		elif [[ \"$item\" =~ \"--name-only|--capture-only|--root=.+|--sensor=.+|-s=.+|-r=.+\" ]]; then
			opts+=( \"$item\" )
		elif [[ \"$item\" =~ \"-r|-s|--root|--sensor\" ]]; then
			next=true;
			opts+=( \"$item\" )
		fi
	done
	IFS=$'\\n' reply=($(tio rpc list $opts 2>/dev/null || echo '[RPC_LIST_FAILED]'))
}
(( $+functions[_tio__subcmd__rpc_names] )) ||
_tio__subcmd__rpc_names() {
	# We've been passed an argument array of zsh stuff and then what we added
	# The last element is the length of what we added, we use that to get the rest
	# We use set -- to slice this off of $@ so that the zsh stuff can do its job
	local len=${@[-1]}
	local opts=( ${@[-$len,-2]} )
	set -- ${@:1:-$len}

    local commands
	_tio__helper__list_rpcs ${opts[@]} --name-only
	commands=( \"${reply[@]}\" )
    _describe -t commands 'rpc names' commands \"$@\"
}
(( $+functions[_tio__subcmd__rpc_commands] )) ||
_tio__subcmd__rpc_commands() {
	local len=${@[-1]}
	local opts=( ${@[-$len,-2]} )
	set -- ${@:1:-$len}

	local commands
	_tio__helper__list_rpcs ${opts[@]} --name-only
	commands=( \"${reply[@]}\" )
	commands=( \"${commands[@]}\"
'list:List available RPCs on the device' \\
'dump:Dump RPC data from the device' \\
    )
    _describe -t commands 'rpc names / tio rpc commands' commands \"$@\"
}
(( $+functions[_tio__subcmd__capture_rpc_names] )) ||
_tio__subcmd__capture_rpc_names() {
	local len=${@[-1]}
	local opts=( ${@[-$len,-2]} )
	set -- ${@:1:-$len}

	local commands
	_tio__helper__list_rpcs ${opts[@]} --name-only --capture-only
	commands=( \"${reply[@]}\" )
    _describe -t commands 'capture rpc names' commands \"$@\"");

    print!("{}", completions);
    Ok(())
}
