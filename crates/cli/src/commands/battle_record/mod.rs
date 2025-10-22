pub mod list;

use clap::{Subcommand, ValueEnum};
use df_sdk::sdk::DeltaForceSdk;
use list::list;

use crate::utils::parse_datetime;

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Default,
    Json,
    JsonPretty,
}

#[derive(Subcommand)]
pub enum BattleRecordCommands {
    /// 列出对局记录
    #[command(arg_required_else_help = false)]
    List {
        /// 输出格式
        #[arg(long, value_enum, default_value_t = OutputFormat::Default)]
        format: OutputFormat,

        /// 输出结果数量
        #[arg(long)]
        limit: Option<usize>,

        /// 仅输出该日期后的对局记录
        #[arg(long)]
        since: Option<String>,
    },
}

impl BattleRecordCommands {
    pub async fn handle(self, sdk: DeltaForceSdk) {
        match self {
            BattleRecordCommands::List {
                format,
                limit,
                since,
            } => match since {
                Some(since) => {
                    if let Some(parsed_since) = parse_datetime(&since) {
                        list(sdk, format, limit, Some(parsed_since)).await;
                    } else {
                        eprintln!("解析 --since 参数失败")
                    }
                }
                None => list(sdk, format, limit, None).await,
            },
        }
    }
}
