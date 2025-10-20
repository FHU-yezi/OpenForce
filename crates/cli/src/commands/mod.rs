pub mod battle_record;

use battle_record::BattleRecordCommands;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// 对局记录
    BattleRecord {
        #[command(subcommand)]
        battle_record_command: BattleRecordCommands,
    },
}

impl Commands {
    pub async fn handle(self) {
        match self {
            Commands::BattleRecord {
                battle_record_command,
            } => {
                battle_record_command.handle().await;
            }
        }
    }
}
