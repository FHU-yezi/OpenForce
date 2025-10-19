pub mod export;

use clap::Subcommand;
use export::ExportCommands;

#[derive(Subcommand)]
pub enum Commands {
    /// 导出数据
    Export {
        #[command(subcommand)]
        export_command: ExportCommands,
    },
}

impl Commands {
    pub async fn handle(self) {
        match self {
            Commands::Export { export_command } => {
                export_command.handle().await;
            }
        }
    }
}
