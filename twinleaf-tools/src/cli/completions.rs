use clap_complete::Shell;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = "\
Generate shell completions for tio.

Dynamic RPC name completions are currently supported for only Bash and Zsh.

Print a script to stdout (default):
  eval \"$(tio completions bash)\"
  eval \"$(tio completions zsh)\"
  tio completions fish | source
  tio completions powershell | Invoke-Expression

Or install into your shell config:
  tio completions --install
  tio completions --install bash
  tio completions --install-all

Pass --static / -s with any of the above for clap-only (non-dynamic) scripts.")]
pub struct CompletionsCli {
    /// Shell to generate or install completions for
    #[arg(
        value_enum,
        required_unless_present_any = ["install", "install_all"],
        conflicts_with = "install_all"
    )]
    pub shell: Option<Shell>,

    /// Generate static instead of dynamic completions
    #[arg(short = 's', long = "static")]
    pub r#static: bool,

    /// Install completions for the current login shell (or the given shell)
    #[arg(long, conflicts_with = "install_all")]
    pub install: bool,

    /// Install completions for all shells detected on PATH
    #[arg(long, conflicts_with = "install")]
    pub install_all: bool,
}
