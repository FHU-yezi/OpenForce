pub mod battle_records;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ExportCommands {
    /// 导出对局记录（烽火地带）
    #[command(arg_required_else_help = false)]
    BattleRecords {
        /// 对局记录数量限制
        #[arg(short, long)]
        limit: Option<usize>,
    },
}

impl ExportCommands {
    pub async fn handle(self) {
        match self {
            ExportCommands::BattleRecords { limit } => {
                battle_records::export_battle_records(limit).await;
            }
        }
    }
}
