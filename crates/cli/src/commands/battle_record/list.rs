use df_sdk::sdk::DeltaForceSdk;
use models::battle_record::BattleRecord;
use time::PrimitiveDateTime;
use tokio_stream::StreamExt;

use crate::error::Error;

fn serialize(data: &BattleRecord, pretty: bool) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(data)
    } else {
        serde_json::to_string(data)
    }
}

fn process_record(
    record: BattleRecord,
    since: Option<PrimitiveDateTime>,
    pretty: bool,
) -> Result<bool, Error> {
    // 检查时间限制
    if let Some(dt_limit) = since {
        if record.time < dt_limit {
            return Ok(false); // 停止处理更多记录
        }
    }

    // 序列化记录
    match serialize(&record, pretty) {
        Ok(serialized_string) => {
            println!("{}", serialized_string);
            Ok(true) // 继续处理更多记录
        }
        Err(e) => Err(Error::SerializeError(e)),
    }
}

pub async fn list(
    sdk: DeltaForceSdk,
    limit: Option<usize>,
    since: Option<PrimitiveDateTime>,
    pretty: bool,
) {
    let mut stream = sdk.iter_battle_records().await;
    if let Some(x) = limit {
        stream = Box::pin(stream.take(x));
    }

    while let Some(item) = stream.next().await {
        match item {
            Ok(record) => match process_record(record, since, pretty) {
                Ok(should_continue) => {
                    if !should_continue {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }
}
