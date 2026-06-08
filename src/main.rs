use clap::Parser;

use crate::{cli::cli::Cli, services::cloudflare::cloudflare::Cloudflare};

mod cli;
mod services;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cloudflare_client = Cloudflare::new(cli.proxy);
    let registration = cloudflare_client.register()?;
    let response_str = registration.to_string();

    println!("{}", response_str);

    Ok(())
}
