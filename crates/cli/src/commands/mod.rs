pub mod battle_record;

use battle_record::BattleRecordCommands;
use clap::Subcommand;
use df_sdk::sdk::DeltaForceSdk;

#[derive(Subcommand)]
pub enum Commands {
    /// 对局记录
    BattleRecord {
        #[command(subcommand)]
        battle_record_command: BattleRecordCommands,
    },
}

impl Commands {
    pub async fn handle(self, sdk: DeltaForceSdk) {
        match self {
            Commands::BattleRecord {
                battle_record_command,
            } => {
                battle_record_command.handle(sdk).await;
            }
        }
    }
}
