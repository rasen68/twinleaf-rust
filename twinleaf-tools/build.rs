use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use std::{env, io};

include!("src/lib.rs");

fn main() -> Result<(), io::Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = MonitorCli::command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "tio-monitor", &outdir)?;
    }

    Ok(())
}
