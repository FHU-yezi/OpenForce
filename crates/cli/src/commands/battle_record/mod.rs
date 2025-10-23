pub mod get;
pub mod list;

use clap::Subcommand;
use df_sdk::sdk::DeltaForceSdk;
use get::get;
use list::list;
use time::PrimitiveDateTime;

use crate::OutputFormat;
use crate::utils::parse_datetime;

#[derive(Subcommand)]
pub enum BattleRecordCommands {
    /// 获取对局记录详情
    #[command(arg_required_else_help = false)]
    Get {
        /// 对局 ID
        room_id: String,
    },
    /// 列出对局记录
    #[command(arg_required_else_help = false)]
    List {
        /// 输出结果数量
        #[arg(long)]
        limit: Option<usize>,

        /// 仅输出该日期后的对局记录
        #[arg(long, value_parser = parse_datetime)]
        since: Option<PrimitiveDateTime>,
    },
}

impl BattleRecordCommands {
    pub async fn handle(self, sdk: DeltaForceSdk, format: OutputFormat) {
        match self {
            BattleRecordCommands::Get { room_id } => get(sdk, room_id, format).await,
            BattleRecordCommands::List { limit, since } => list(sdk, format, limit, since).await,
        }
    }
}
