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

    // Add logic to treat RPC namess as subcommands so we don't double-complete
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
    ");


    // Read dynamic RPC completions into options list
    // TODO: do we need --name-only or can we just do param sub? which is better?
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
			opts=\"-r -s -t -T -d -h --root --sensor --req-type --rep-type --debug [ARG]\"
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

    // Add rpcname as subcmd to dump for same reason as before
    // We hook on rpc list subcmd because that comes right after dump.. hopefully?
    let completions = completions.replace("
        tio__subcmd__rpc__subcmd__list)
    ",
    "
        tio__subcmd__rpc__subcmd__dump__subcmd__rpcname)
            opts=\"-r -s -h --root --sensor --capture\"
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
        tio__subcmd__rpc__subcmd__list)
    ");


    print!("{}", completions);
    Ok(())
}

fn generate_zsh_dynamic() -> eyre::Result<()> {
    let completions = include_str!("../../completion-scripts/tio_completions_dynamic.zsh");
    print!("{}", completions);
    Ok(())
}
