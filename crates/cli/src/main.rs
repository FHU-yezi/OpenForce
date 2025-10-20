mod commands;
mod error;
mod utils;

use crate::commands::Commands;
use clap::Parser;
use df_sdk::sdk::DeltaForceSdk;

#[derive(Parser)]
#[command(about = "OpenFront CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Cookies
    #[arg(long)]
    cookies: Option<String>,

    /// 包含 Cookies 的文件
    #[arg(long)]
    cookies_file: Option<String>,

    /// 从标准输入中读取 Cookies
    #[arg(long)]
    cookies_stdin: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let credentials = match utils::get_credentials(&cli) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let sdk = DeltaForceSdk::build().with_credentials(credentials).build();

    cli.command.handle(sdk).await;
}
