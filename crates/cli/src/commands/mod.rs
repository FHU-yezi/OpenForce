pub mod battle_record;
pub mod room_password;

use battle_record::BattleRecordCommands;
use clap::Subcommand;
use df_sdk::sdk::DeltaForceSdk;
use room_password::RoomPasswordCommands;

#[derive(Subcommand)]
pub enum Commands {
    /// 对局记录
    BattleRecord {
        #[command(subcommand)]
        battle_record_command: BattleRecordCommands,
    },
    /// 房间密码
    RoomPassword {
        #[command(subcommand)]
        room_password_command: RoomPasswordCommands,
    },
}

impl Commands {
    pub async fn handle(self, sdk: DeltaForceSdk) {
        match self {
            Commands::BattleRecord {
                battle_record_command,
            } => battle_record_command.handle(sdk).await,
            Commands::RoomPassword {
                room_password_command,
            } => room_password_command.handle(sdk).await,
        }
    }
}
