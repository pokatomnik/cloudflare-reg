use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::{cli::cli::Cli, services::cloudflare::cloudflare::Cloudflare};

mod cli;
mod services;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Handle subcommands first
    if let Some(command) = cli.command {
        match command {
            crate::cli::cli::Commands::Completions { shell } => {
                // Generate completion for the specified shell and print to stdout
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();
                generate(shell, &mut cmd, bin_name.as_str(), &mut std::io::stdout());
                return Ok(());
            }
        }
    }

    let cloudflare_client = Cloudflare::new(cli.proxy);
    let registration = cloudflare_client.register()?;
    let response_str = registration.to_string();

    println!("{}", response_str);

    Ok(())
}
