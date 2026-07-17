use clap::CommandFactory;
use crate::{TioCli, CompletionsCli};

pub fn run_completions(completions_cli: CompletionsCli) -> eyre::Result<()> {
    match (completions_cli.r#static, completions_cli.shell) {
        (false, clap_complete::Shell::Bash) => generate_bash_dynamic(),
        (false, clap_complete::Shell::Zsh) => generate_zsh_dynamic(),
        (_, shell) => Ok(clap_complete::generate(shell, &mut TioCli::command(), "tio", &mut std::io::stdout())),
    }
}

struct Completions {
    completions: String,
}

impl Completions {
    pub fn new(completions: String) -> Self {
        Self { completions }
    }

    pub fn replace(&mut self, pattern: &str, replacement: &str) -> eyre::Result<&mut Self> {
        eyre::ensure!(self.completions.contains(pattern), "pattern {} not found in completions", pattern);
        eyre::ensure!(self.completions.matches(pattern).count() == 1, "pattern {} found multiple times in completions", pattern);
        self.completions = self.completions.replace(pattern, replacement);
        Ok(self)
    }

    pub fn print(&self) {
        print!("{}", self.completions);
    }
}

fn generate_bash_dynamic() -> eyre::Result<()> {
    let static_completions = include_str!("../../completion-scripts/tio_completions_static.bash");
    let mut completions = Completions::new(static_completions.to_string());

    // Treat RPC names as subcommands so we don't double-complete
    // Assume RPC names are all letters, numbers, and dots
    // Flags (which have dashes), sensor routes (which have slashes),
    // and root urls (which probably have ://) will not count
    completions.replace("
            tio__subcmd__rpc,list)
                cmd=\"tio__subcmd__rpc__subcmd__list\"
                ;;
    ",
    "
            tio__subcmd__rpc,list)
                cmd=\"tio__subcmd__rpc__subcmd__list\"
                ;;
            tio__subcmd__rpc,*)
                if [[ \"$i\" =~ \"[a-zA-Z0-9.]+\" ]]; then
                    cmd=\"tio__subcmd__rpc__subcmd__rpcname\"
                fi
                ;;
            tio__subcmd__rpc__subcmd__dump,*)
                if [[ \"$i\" =~ \"[a-zA-Z0-9.]+\" ]]; then
                    cmd=\"tio__subcmd__rpc__subcmd__dump__subcmd__rpcname\"
                fi
                ;;
            tio__subcmd__capture,*)
                if [[ \"$i\" =~ \"[a-zA-Z0-9.]+\" ]]; then
                    cmd=\"tio__subcmd__capture__subcmd__rpcname\"
                fi
				;;

    ")?;


    // Read dynamic RPC completions into options list
    // The helper function checks whether we are completion an option
    // or an rpc name, and only appends rpcs in the latter case
    completions.replace("
        tio__subcmd__rpc)
            opts=\"-r -s -t -T -d -h --root --sensor --req-type --rep-type --debug --help [RPC_NAME] [ARG] list dump\"
    ",
    "
        tio__subcmd__rpc)
			opts=\"-r -s -t -T -d -h --root --sensor --req-type --rep-type --debug --help list dump $(_tio__helper__append_rpcs --name-only)\"
    ")?;
    // Note: I personally like literally showing [RPC_LIST_FAILED] as an option to make it
    // clear what went wrong, but you could also just echo nothing since there is nothing to complete

    // Add rpcname as subcmd to suggest an arg instead of more rpc names
    // We do this by hooking on dump subcmd and appending in front of it
    // TODO: We could try to copy this from earlier in the string
    // Which would make this more readable and maintainable, but that sounds hard
    completions.replace("
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
    ")?;


    // Add dynamic completions to rpc dump
    completions.replace("
        tio__subcmd__rpc__subcmd__dump)
            opts=\"-r -s -h --root --sensor --capture --help <RPC_NAME>\"
    ",
    "
        tio__subcmd__rpc__subcmd__dump)
            opts=\"-r -s -h --root --sensor --capture --help $(_tio__helper__append_rpcs --name-only)\"
    ")?;

    // Add rpcname as subcmd to dump and capture
    // It doesn't really matter where these go so we'll put them before rpc list
    completions.replace("
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
    ")?;

    // Add dynamic completions to tio capture
    completions.replace("
        tio__subcmd__capture)
            opts=\"-r -s -h --root --sensor --timeout --help [RPC_NAME]\"
    ",
    "
        tio__subcmd__capture)
			opts=\"-r -s -h --root --sensor --timeout --help $(_tio__helper__append_rpcs --name-only --capture-only)\"
    ")?;

    // Helpers: gate RPC listing to positional completions, and forward -r/-s/--root/--sensor
    completions.replace("
if [[ \"${BASH_VERSINFO[0]}\" -eq 4 && \"${BASH_VERSINFO[1]}\" -ge 4 || \"${BASH_VERSINFO[0]}\" -gt 4 ]]; then
    complete -F _tio -o nosort -o bashdefault -o default tio
else
    complete -F _tio -o bashdefault -o default tio
fi
",
    "
_tio__helper__append_rpcs() {
	# Only fetch when completing a positional (RPC name), not a flag/value
	if [[ ${cur} == -* ]]; then
		return
	fi
	case \"${prev}\" in
		-r|--root|-s|--sensor|-t|--req-type|-T|--rep-type|--timeout)
			return
			;;
	esac
	local rpcs
	rpcs=\"$(_tio__helper__list_rpcs \"$@\")\"
	rpcs=\"${rpcs//$'\\n'/ }\" # replace newlines with spaces
	rpcs=\"${rpcs% }\"     # remove trailing whitespace
	echo \"$rpcs\"
}
_tio__helper__list_rpcs() {
	local opts=()
	local next=false
	local item
	# Scan completed words only; forward -r/-s/--root/--sensor
	for item in \"${COMP_WORDS[@]:0:COMP_CWORD}\"; do
        # Bash sometimes breaks --sensor=/0 into three tokens
        # Skip the middle = so /0 is the next of --sensor
		[[ \"$item\" == '=' ]] && continue;
		if $next; then
			opts+=( \"$item\" )
			next=false
		elif [[ \"$item\" =~ ^(--name-only|--capture-only|--root=.+|--sensor=.+|-s.+|-r.+)$ ]]; then
			opts+=( \"$item\" )
		elif [[ \"$item\" =~ ^(-r|-s|--root|--sensor)$ ]]; then
			next=true
			opts+=( \"$item\" )
		fi
	done
	opts+=( \"$@\" )
	tio rpc list \"${opts[@]}\" 2>/dev/null || echo '[RPC_LIST_FAILED]'
}

if [[ \"${BASH_VERSINFO[0]}\" -eq 4 && \"${BASH_VERSINFO[1]}\" -ge 4 || \"${BASH_VERSINFO[0]}\" -gt 4 ]]; then
    complete -F _tio -o nosort -o bashdefault -o default tio
else
    complete -F _tio -o bashdefault -o default tio
fi
")?;

    completions.print();
    Ok(())
}

fn generate_zsh_dynamic() -> eyre::Result<()> {
    let static_completions = include_str!("../../completion-scripts/tio_completions_static.zsh");
    let mut completions = Completions::new(static_completions.to_string());

    // Only use rpc's _arguments to append options if we already
    // have a dash (i.e. user is typing tio rpc -...). If this is
    // the case, we then go directly to the rpc name completer, since
    // there cannot be a subcommand after tio rpc -s /0 for example.
    // Otherwise, if they're typing tio rpc list/dump, we go to
    // _tio__subcmd__rpc_commands to generate rpc names and let
    // subcommands append their own _arguments if we get one
    completions.replace(
        "
(rpc)
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
'-h[Print help]' \\
'--help[Print help]' \\
'::rpc_name -- RPC name to execute:' \\
'::rpc_arg -- RPC argument value:' \\
\":: :_tio__subcmd__rpc_commands\" \\
\"*::: :->rpc\" \\
&& ret=0
",
        "
(rpc)
if [[ \"$words[2]\" == -* ]]; then
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
'-h[Print help]' \\
'--help[Print help]' \\
\":: :_tio__subcmd__rpc_names ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\
':rpc_arg -- RPC argument value:' \\
&& ret=0
else
# Save line to _line, which will come in handy later
local _line=( \"${line[@]}\" )
_arguments \"${_arguments_options[@]}\" : \\
\":: :_tio__subcmd__rpc_commands\" \\
\"*::: :->rpc\" \\
&& ret=0
",
    )?;

    // Change matching logic to look at last completed word
    completions.replace("
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
			(list)")?;

    // Close the if/else opened due to checking whether we had
    // an option after tio rpc
    completions.replace(
        "
        esac
    ;;
esac
;;
(capture)",
        "
        esac
    ;;
esac
fi
;;
(capture)",
    )?;

    // Pass additional arguments to _tio__subcmd__rpc_commands
    // We slice line from 2 (first thing after "rpc") to -2 (last completed option)
    // We also pass the length of what we added so it can slice it off
    completions.replace("\":: :_tio__subcmd__rpc_commands\" \\",
    "\":: :_tio__subcmd__rpc_commands ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\")?;

    // Remove rpc name from dump opts,
    // And use it as a hook to add case for last word being just "rpc"
    // If it is, we are at "tio rpc" and want to complete rpc names
    // We do a similar slicing thing to above using our saved _line
    // But we start from 3 to not pass the "dump"
    completions.replace("
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
':rpc_arg -- RPC argument value:' \\")?;

    // Replace capture rpc name with dynamic completion, and pass in line
    completions.replace("'::rpc_name -- Capture RPC name to execute:' \\",
    "\":: :_tio__subcmd__capture_rpc_names ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\")?;

    // Add functions to dynamically get rpc names,
    // Replacing old completion case for "tio rpc [list/dump/[RPC_NAME]"
    completions.replace("
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
		elif [[ \"$item\" =~ \"^(--name-only|--capture-only|--root=.+|--sensor=.+|-s.+|-r.+)$\" ]]; then
			opts+=( \"$item\" )
		elif [[ \"$item\" =~ \"^(-r|-s|--root|--sensor)$\" ]]; then
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
    _describe -t commands 'capture rpc names' commands \"$@\"")?;

    completions.print();
    Ok(())
}
