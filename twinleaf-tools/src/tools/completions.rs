use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use crate::{TioCli, CompletionsCli};
use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::Command,
};

/// Generate shell completion code and output to stdout

/// clap_complete already generates 'static' completion code for
/// Bash, Zsh, Fish, Elvish, and Powershell based on the built-in
/// CLI.  However, we can add 'dynamic' behaviour by editing the
/// completion scripts to include calls to `tio rpc list`, letting
/// us shell-complete RPC names.
///
/// We do this by reading using clap_complete to generate the
/// static completion code and write it to a file in the repo (see
/// twinleaf-tools/build-completions.sh). We read that file into a
/// Rust string, and then perform a series of search-and-replaces
/// to add the RPC name completion logic to `tio rpc` and `tio
/// capture`.
///
/// We then print this string to stdout, just like clap_complete
/// normally does, so that the user can source the output in their
/// shell rc. With `--install` / `--install-all`, the same script is
/// written to disk and the shell rc is updated instead.
///
/// This currently is only implemented for Bash and Zsh and there
/// are no plans to attempt to implement it for the other shells,
/// as each shell has its own unique and difficult completion logic

/// NOTE: A lot of this code contains exact strings from the current
/// CLI and can easily break upon changes as simple as adding
/// extra flags to rpc-related or even changing the docstrings
/// to tio rpc options. I haven't implemented a great way to get
/// around this, so it may take some manual updating if changes
/// are made in those regards. It should play nicely with CLI
/// changes to pretty much anything other than tio rpc though, as
/// it does not touch any of that.

pub fn run_completions(completions_cli: CompletionsCli) -> eyre::Result<()> {
    if completions_cli.install || completions_cli.install_all {
        return install_completions(
            completions_cli.shell,
            completions_cli.install_all,
            completions_cli.r#static,
        );
    }

    let shell = completions_cli
        .shell
        .expect("shell is required unless --install/--install-all");
    print!("{}", generate_script(shell, completions_cli.r#static)?);
    Ok(())
}

fn generate_script(shell: clap_complete::Shell, r#static: bool) -> eyre::Result<String> {
    match (r#static, shell) {
        (false, clap_complete::Shell::Bash) => generate_bash_dynamic(),
        (false, clap_complete::Shell::Zsh) => generate_zsh_dynamic(),
        (_, shell) => {
            let mut buf = Vec::new();
            clap_complete::generate(shell, &mut TioCli::command(), "tio", &mut buf);
            Ok(String::from_utf8(buf)?)
        }
    }
}

// Wrapper to ensure replaces fail loudly if they aren't one-to-one
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

    pub fn into_string(self) -> String {
        self.completions
    }
}

fn generate_bash_dynamic() -> eyre::Result<String> {
    let static_completions = include_str!("../../completion-scripts/tio_completions_static.bash");
    let mut completions = Completions::new(static_completions.to_string());

    // Replace COMP_WORDS with words from _get_comp_words_by_ref
    // Note that this may fail for non-typical scenarios w/ : and =
    // Also set up rpc_opt for completing rpc names/subcmds
    // And next for parsing tio rpc options w/ args
    completions.replace(
"
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ \"${BASH_VERSINFO[0]}\" -ge 4 ]]; then
        cur=\"$2\"
    else
        cur=\"${COMP_WORDS[COMP_CWORD]}\"
    fi
    prev=\"$3\"
    cmd=\"\"
    opts=\"\"

    for i in \"${COMP_WORDS[@]:0:COMP_CWORD}\"
",
"
    local i cur prev opts cmd words next rpc_opt
    COMPREPLY=()
    # $COMP_WORDS breaks on : & = which is bad for roots and flags;
    # This function fills $words with a version that doesn't
    _get_comp_words_by_ref -n := words
    if [[ \"${BASH_VERSINFO[0]}\" -ge 4 ]]; then
        cur=\"$2\"
    else
        cur=\"${words[COMP_CWORD]}\"
    fi
    prev=\"$3\"
    cmd=\"\"
    opts=\"\"
    next=false
    rpc_opt=false

    for i in \"${words[@]:0:COMP_CWORD}\"
"
    )?;


    // Treat RPC names as subcommands so we don't double-complete
    // Assume RPC names are all letters, numbers, and dots
    // Flags (which have dashes), sensor routes (which have slashes),
    // and root urls (which probably have ://) will not count
    // We use $next to avoid reading something like -t u8 as an rpc
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
                if $next; then
                    next=false
                elif [[ \"$i\" =~ ^(-r|-s|-t|-T|--root|--sensor|--rep-type|--req-type)$ ]]; then
                    next=true
                    rpc_opt=true
                elif [[ \"$i\" == -* ]]; then
                    rpc_opt=true
                else
                    cmd=\"tio__subcmd__rpc__subcmd__rpcname\"
                fi
                ;;
            tio__subcmd__rpc__subcmd__dump,*)
                if $next; then
                    next=false
                elif [[ \"$i\" =~ ^(-r|-s|--root|--sensor)$ ]]; then
                    next=true
                elif [[ \"$i\" != -* ]]; then
                    cmd=\"tio__subcmd__rpc__subcmd__dump__subcmd__rpcname\"
                fi
                ;;
            tio__subcmd__capture,*)
                if $next; then
                    next=false
                elif [[ \"$i\" =~ ^(-r|-s|--root|--sensor|--timeout)$ ]]; then
                    next=true
                elif [[ \"$i\" != -* ]]; then
                    cmd=\"tio__subcmd__capture__subcmd__rpcname\"
                fi
                ;;
    ")?;


    // Replace tio__subcmd__rpc's placeholder args
    // Don't add list and dump if we have an option after tio rpc
    // The helper function checks whether we are completion an option
    // or an rpc name, and only appends rpcs in the latter case
    completions.replace(
        "[RPC_NAME] [ARG] list dump\"\n",
"\"
            if ! $rpc_opt; then
                opts=\"$opts list dump\"
            fi
            opts=\"$opts $(_tio__helper__append_rpcs --name-only)\"
"
    )?;

    // Add rpcname as subcmd to suggest an arg instead of more rpcs
    // We append this case in front of the dump subcmd
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


    // Replace dump subcmd's mandatory <RPC_NAME> placeholder
    completions.replace(
        "<RPC_NAME>",
        "$(_tio__helper__append_rpcs --name-only)"
    )?;

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

    // Replace tio capture's [RPC_NAME] (only one left)
    completions.replace(
        "[RPC_NAME]",
        "$(_tio__helper__append_rpcs --name-only --capture-only)"
    )?;

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
    # Scan completed words; forward -r/-s/--root/--sensor
    for item in \"${words[@]}\"; do
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

    Ok(completions.into_string())
}

fn generate_zsh_dynamic() -> eyre::Result<String> {
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
",
"
(rpc)
if [[ \"$words[2]\" == -* ]]; then
_arguments \"${_arguments_options[@]}\" : \\
"
    )?;

    // If we are in -*, we want to call rpc names (not including
    // subcommands). It parses additional arguments from a slice
    // of line array: from 2 (first thing after "rpc") to -2 (last
    // completed option). We also pass the length of what we added
    // so it can slice it off.
    completions.replace(
"
'::rpc_name -- RPC name to execute:' \\
'::rpc_arg -- RPC argument value:' \\
\":: :_tio__subcmd__rpc_commands\" \\
\"*::: :->rpc\" \\
&& ret=0
",
"
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
"
    )?;


    // Change matching logic to look at last completed word
    completions.replace(
"
        words=($line[3] \"${words[@]}\")
        (( CURRENT += 1 ))
        curcontext=\"${curcontext%:*:*}:tio-rpc-command-$line[3]:\"
        case $line[3] in
            (list)
",
"
        words=($line[1] \"${words[@]}\")
        (( CURRENT += 1 ))
        curcontext=\"${curcontext%:*:*}:tio-rpc-command-$line[1]:\"
        case $line[1] in
            (list)
"
    )?;

    // Close the if/else opened due to checking whether we had
    // an option after tio rpc
    completions.replace(
"
        esac
    ;;
esac
;;
(capture)
",
"
        esac
    ;;
esac
fi
;;
(capture)
"
    )?;

    // Pass additional arguments to _tio__subcmd__rpc_commands
    completions.replace(
        "\":: :_tio__subcmd__rpc_commands\" \\",
        "\":: :_tio__subcmd__rpc_commands ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\"
    )?;

    // Remove rpc name from dump opts,
    // And use it as a hook to add case for last word being just "rpc"
    // If it is, we are at "tio rpc" and want to complete rpc names
    // We do a similar slicing thing to above using our saved _line
    // But we start from 3 to not pass the "dump"
    completions.replace(
        "':rpc_name -- RPC name to dump:' \\",
"\
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
'-h[Print help]' \\
'--help[Print help]' \\
':rpc_arg -- RPC argument value:' \\\
"
    )?;

    // Replace capture rpc name with dynamic completion, and pass in line
    completions.replace(
        "'::rpc_name -- Capture RPC name to execute:' \\",
        "\":: :_tio__subcmd__capture_rpc_names ${line[2,-2]} $(( ${#line[2,-2]} + 1 ))\" \\"
    )?;

    // Add functions to dynamically get rpc names, and add
    // them to the old rpc list/dump completion case
    completions.replace(
"
(( $+functions[_tio__subcmd__rpc_commands] )) ||
_tio__subcmd__rpc_commands() {
    local commands; commands=(
",

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
(( $+functions[_tio__subcmd__capture_rpc_names] )) ||
_tio__subcmd__capture_rpc_names() {
    local len=${@[-1]}
    local opts=( ${@[-$len,-2]} )
    set -- ${@:1:-$len}

    local commands
    _tio__helper__list_rpcs ${opts[@]} --name-only --capture-only
    commands=( \"${reply[@]}\" )
    _describe -t commands 'capture rpc names' commands \"$@\"
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
"
    )?;

    Ok(completions.into_string())
}

// ---------------------------------------------------------------------------
// Install helpers (ported from coworker's completions installer)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

impl SupportedShell {
    fn to_clap(self) -> Shell {
        match self {
            SupportedShell::Bash => Shell::Bash,
            SupportedShell::Zsh => Shell::Zsh,
            SupportedShell::Fish => Shell::Fish,
            SupportedShell::PowerShell => Shell::PowerShell,
        }
    }

    fn from_clap(shell: Shell) -> eyre::Result<Self> {
        match shell {
            Shell::Bash => Ok(SupportedShell::Bash),
            Shell::Zsh => Ok(SupportedShell::Zsh),
            Shell::Fish => Ok(SupportedShell::Fish),
            Shell::PowerShell => Ok(SupportedShell::PowerShell),
            Shell::Elvish => eyre::bail!("--install is not supported for elvish"),
            _ => eyre::bail!("--install is not supported for this shell"),
        }
    }
}

fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

// Detects the shell the user is currently running (based on $SHELL).
// Note: this reflects the login shell, not necessarily the active runtime shell.
fn detect_current_shell() -> Option<SupportedShell> {
    let shell = env::var("SHELL").ok()?;
    match shell.rsplit('/').next()? {
        "bash" => Some(SupportedShell::Bash),
        "zsh" => Some(SupportedShell::Zsh),
        "fish" => Some(SupportedShell::Fish),
        "pwsh" | "powershell" => Some(SupportedShell::PowerShell),
        _ => None,
    }
}

// Returns shells that exist on the system PATH.
// Used when `--install-all` is enabled or detection fails.
fn detect_installed_shells() -> Vec<SupportedShell> {
    let mut shells = Vec::new();

    if command_exists("bash") {
        shells.push(SupportedShell::Bash);
    }
    if command_exists("zsh") {
        shells.push(SupportedShell::Zsh);
    }
    if command_exists("fish") {
        shells.push(SupportedShell::Fish);
    }
    if command_exists("pwsh") || command_exists("powershell") {
        shells.push(SupportedShell::PowerShell);
    }

    shells
}

fn get_os() -> &'static str {
    env::consts::OS
}

fn bash_paths() -> (PathBuf, Option<PathBuf>) {
    let home = PathBuf::from(env::var("HOME").expect("HOME not set"));
    let mut rc = home.clone();
    rc.push(".bashrc");
    let os = get_os();

    if os == "macos" {
        let brew = PathBuf::from("/opt/homebrew/etc/bash_completion.d/");
        if brew.exists() {
            return (brew, Some(rc));
        }

        let intel = PathBuf::from("/usr/local/etc/bash_completion.d/");
        if intel.exists() {
            return (intel, Some(rc));
        }
    }

    let mut dir = home.clone();
    // Typo fix vs coworker fork: bash-completion/completions (not bash-completion.completions)
    dir.push(".local/share/bash-completion/completions/");
    (dir, Some(rc))
}

fn zsh_paths() -> (PathBuf, Option<PathBuf>) {
    let home = PathBuf::from(env::var("HOME").expect("HOME not set"));
    let mut dir = home.clone();
    dir.push(".zsh/completions/");
    let mut rc = home.clone();
    rc.push(".zshrc");
    (dir, Some(rc))
}

fn fish_paths() -> (PathBuf, Option<PathBuf>) {
    let mut home = PathBuf::from(env::var("HOME").expect("HOME not set"));
    home.push(".config/fish/completions/");
    (home, None)
}

fn powershell_paths() -> (PathBuf, Option<PathBuf>) {
    let output = Command::new("pwsh")
        .args(["-NoProfile", "-Command", "$PROFILE"])
        .output()
        .or_else(|_| {
            Command::new("powershell")
                .args(["-NoProfile", "-Command", "$PROFILE"])
                .output()
        });
    if let Ok(out) = output {
        if out.status.success() {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() {
                return (PathBuf::new(), Some(PathBuf::from(path)));
            }
        }
    }
    let mut fallback = PathBuf::from(env::var("HOME").unwrap());
    fallback.push("Documents/PowerShell/Microsoft.PowerShell_profile.ps1");
    (PathBuf::new(), Some(fallback))
}

// Appends shell configuration only if marker is not already present.
// Prevents duplicate entries when reinstalling completions.
fn update_config(rc_path: &PathBuf, marker: &str, lines: &[String]) -> std::io::Result<()> {
    let mut exists = String::new();

    if rc_path.exists() {
        let file = OpenOptions::new().read(true).open(rc_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            exists.push_str(&line);
            exists.push('\n');
        }
    }

    if exists.contains(marker) {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(rc_path)?;

    writeln!(file, "\n# >>> tio completions >>>")?;
    for l in lines {
        writeln!(file, "{}", l)?;
    }
    writeln!(file, "# <<<<<<")?;
    Ok(())
}

fn write_completion_script(
    shell: SupportedShell,
    r#static: bool,
    dir: &PathBuf,
) -> eyre::Result<()> {
    let clap_shell = shell.to_clap();
    // Use generate_to for static (and for shells without dynamic support).
    // For dynamic bash/zsh, write our patched script under the same filename
    // generate_to would have used.
    let use_dynamic = !r#static
        && matches!(shell, SupportedShell::Bash | SupportedShell::Zsh);

    if use_dynamic {
        let file_name = match shell {
            SupportedShell::Bash => "tio.bash".to_string(),
            SupportedShell::Zsh => "_tio".to_string(),
            _ => unreachable!(),
        };
        let path = dir.join(file_name);
        let script = generate_script(clap_shell, false)?;
        std::fs::write(&path, script)?;
    } else {
        let mut cmd = TioCli::command();
        generate_to(clap_shell, &mut cmd, "tio", dir)?;
    }
    Ok(())
}

// Installs completion scripts for a specific shell and updates shell configuration when required
fn install_shell_scripts(shell: SupportedShell, r#static: bool) -> eyre::Result<()> {
    match shell {
        SupportedShell::Bash => {
            let (dir, rc) = bash_paths();
            create_dir_all(&dir)?;
            write_completion_script(shell, r#static, &dir)?;
            if let Some(rc) = rc {
                update_config(
                    &rc,
                    "tio completions",
                    &[format!("source {}", dir.join("tio.bash").display())],
                )?;
            }
        }
        SupportedShell::Zsh => {
            let (dir, rc) = zsh_paths();
            create_dir_all(&dir)?;
            write_completion_script(shell, r#static, &dir)?;
            if let Some(rc) = rc {
                update_config(
                    &rc,
                    "tio completions",
                    &[
                        format!("fpath=({} $fpath)", dir.display()),
                        "autoload -Uz compinit && compinit".to_string(),
                    ],
                )?;
            }
        }
        SupportedShell::Fish => {
            let (dir, _) = fish_paths();
            create_dir_all(&dir)?;
            write_completion_script(shell, r#static, &dir)?;
        }
        SupportedShell::PowerShell => {
            let (_, rc) = powershell_paths();
            if let Some(rc) = rc {
                let mut completions_dir = PathBuf::from(env::var("HOME").unwrap());
                completions_dir.push(".config/powershell/completions");
                create_dir_all(&completions_dir)?;
                write_completion_script(shell, r#static, &completions_dir)?;
                let script_path = completions_dir.join("_tio.ps1");
                update_config(
                    &rc,
                    "tio completions",
                    &[format!(". \"{}\"", script_path.to_string_lossy())],
                )?;
            }
        }
    }
    Ok(())
}

fn target_shells(shell: Option<Shell>, install_all: bool) -> eyre::Result<Vec<SupportedShell>> {
    if let Some(shell) = shell {
        return Ok(vec![SupportedShell::from_clap(shell)?]);
    }

    if install_all {
        return Ok(detect_installed_shells());
    }

    Ok(detect_current_shell()
        .map(|s| vec![s])
        .unwrap_or_else(detect_installed_shells))
}

fn install_completions(
    shell: Option<Shell>,
    install_all: bool,
    r#static: bool,
) -> eyre::Result<()> {
    let targets = target_shells(shell, install_all)?;
    eyre::ensure!(
        !targets.is_empty(),
        "no supported shells detected; pass an explicit shell (e.g. `tio completions --install bash`)"
    );

    for shell in targets {
        install_shell_scripts(shell, r#static)?;
    }

    Ok(())
}
