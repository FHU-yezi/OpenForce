mod commands;

use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "ofcli")]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    cli.command.handle().await;
}
