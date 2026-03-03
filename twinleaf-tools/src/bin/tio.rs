use std::process::ExitCode;
use twinleaf_tools::{TioCli, Commands};
use clap::Parser;
use twinleaf_tools::utils::{
    tio_proxy::run_proxy,
    tio_monitor::run_monitor, 
    tio_health::run_health,  
    tio_tool::{
        list_rpcs, 
        rpc, 
        rpc_dump, 
        dump, 
        log, 
        log_dump, 
        log_metadata,
        log_csv, 
        log_hdf, 
        firmware_upgrade, 
    }
};

fn main() -> ExitCode {
    let cli = TioCli::parse();

    //TODO: Work on exit code logic  
    let result = match cli.command {
        Commands::Proxy(proxy_cli) => run_proxy(proxy_cli), //will need return
        Commands::Monitor{
            tio, 
            all, 
            fps, 
            colors
        } => run_monitor(tio, all, fps, colors), 
        Commands::Health(health_cli) => run_health(health_cli), 
        Commands::RpcList { tio } => list_rpcs(&tio),
        Commands::Rpc {
            tio,
            rpc_name,
            rpc_arg,
            req_type,
            rep_type,
            debug,
        } => rpc(&tio, rpc_name, rpc_arg, req_type, rep_type, debug),
        Commands::RpcDump {
            tio,
            rpc_name,
            capture,
        } => rpc_dump(&tio, rpc_name, capture),
        Commands::Dump {
            tio,
            data,
            meta,
            depth,
        } => dump(&tio, data, meta, depth),
        Commands::Log {
            tio,
            file,
            unbuffered,
            raw,
            depth,
        } => log(&tio, file, unbuffered, raw, depth),
        Commands::LogMetadata { tio, file } => log_metadata(&tio, file),
        Commands::LogDump {
            files,
            data,
            meta,
            sensor,
            depth,
        } => log_dump(files, data, meta, sensor, depth),
        Commands::LogCsv {
            stream,
            files,
            sensor,
            metadata,
            output,
        } => log_csv(stream, files, sensor, metadata, output),
        Commands::LogHdf {
            files,
            output,
            filter,
            compress,
            debug,
            split_level,
            split_policy,
        } => log_hdf(
            files,
            output,
            filter,
            compress,
            debug,
            split_level,
            split_policy,
        ),
        Commands::FirmwareUpgrade { tio, firmware_path } => firmware_upgrade(&tio, firmware_path),
        _ => {Ok({})}
    };

    if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        eprintln!("FAILED");
        ExitCode::FAILURE
    }
}
