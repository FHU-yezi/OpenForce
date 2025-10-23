mod commands;
mod error;
mod utils;

use crate::commands::Commands;
use clap::{Args, Parser};
use df_sdk::sdk::DeltaForceSdk;

#[derive(Args)]
#[group(multiple = false)]
struct CookiesArgs {
    /// 从环境变量 OPENFORCE_COOKIES 读取 Cookies
    #[arg(long)]
    cookies_env: bool,
    /// 从指定文件读取 Cookies
    #[arg(long)]
    cookies_file: Option<String>,
    /// 从标准输入读取 Cookies
    #[arg(long)]
    cookies_stdin: bool,
}

#[derive(Parser)]
#[command(about = "OpenFront CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    cookies: CookiesArgs,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let credentials = match utils::get_credentials(&cli.cookies) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let mut sdk_builder = DeltaForceSdk::build();
    if let Some(credentials) = credentials {
        sdk_builder = sdk_builder.with_credentials(credentials);
    }
    let sdk = sdk_builder.build();

    cli.command.handle(sdk).await;
}
