use clap::Parser;

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
}
