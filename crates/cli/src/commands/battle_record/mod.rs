pub mod list;

use clap::Subcommand;
use list::list;

#[derive(Subcommand)]
pub enum BattleRecordCommands {
    /// 列出对局记录
    #[command(arg_required_else_help = false)]
    List {
        /// 数量限制（默认为无限制）
        #[arg(long)]
        limit: Option<usize>,
    },
}

impl BattleRecordCommands {
    pub async fn handle(self) {
        match self {
            BattleRecordCommands::List { limit } => {
                list(limit).await;
            }
        }
    }
}
