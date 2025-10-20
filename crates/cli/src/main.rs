mod commands;
mod utils;

use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(about = "OpenFront CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    cli.command.handle().await;
}
