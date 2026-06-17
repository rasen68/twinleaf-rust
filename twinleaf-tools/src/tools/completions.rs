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
    let completions = include_str!("../../completion-scripts/tio_completions_dynamic.bash");
    print!("{}", completions);
    Ok(())
}

fn generate_zsh_dynamic() -> eyre::Result<()> {
    let completions = include_str!("../../completion-scripts/tio_completions_dynamic.zsh");
    print!("{}", completions);
    Ok(())
}
