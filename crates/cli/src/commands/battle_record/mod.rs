pub mod list;

use clap::Subcommand;
use df_sdk::sdk::DeltaForceSdk;
use list::list;

use crate::utils::parse_datetime;

#[derive(Subcommand)]
pub enum BattleRecordCommands {
    /// 列出对局记录
    #[command(arg_required_else_help = false)]
    List {
        /// 输出结果数量
        #[arg(long)]
        limit: Option<usize>,

        /// 仅输出该日期后的对局记录
        #[arg(long)]
        since: Option<String>,

        /// 美化输出
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}

impl BattleRecordCommands {
    pub async fn handle(self, sdk: DeltaForceSdk) {
        match self {
            BattleRecordCommands::List {
                limit,
                since,
                pretty,
            } => match since {
                Some(since) => {
                    if let Some(parsed_since) = parse_datetime(&since) {
                        list(sdk, limit, Some(parsed_since), pretty).await;
                    } else {
                        eprintln!("解析 --since 参数失败")
                    }
                }
                None => list(sdk, limit, None, pretty).await,
            },
        }
    }
}
