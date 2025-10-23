pub mod get;

use clap::Subcommand;
use df_sdk::sdk::DeltaForceSdk;
use get::get;

#[derive(Subcommand)]
pub enum RoomPasswordCommands {
    /// 获取房间密码
    #[command(arg_required_else_help = false)]
    Get,
}

impl RoomPasswordCommands {
    pub async fn handle(self, sdk: DeltaForceSdk) {
        match self {
            RoomPasswordCommands::Get => get(sdk).await,
        }
    }
}
