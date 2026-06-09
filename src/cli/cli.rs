use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "cloudflare-reg")]
#[command(about = "Cloudflare WARP registration utility")]
#[command(version)]
pub(crate) struct Cli {
    #[arg(
        long = "proxy",
        short = 'p',
        help = "Proxy URI, example: socks5h: 127.0.0.1:1080"
    )]
    pub proxy: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Generate shell completions for a specific shell
    Completions {
        /// Target shell (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: Shell,
    },
}
