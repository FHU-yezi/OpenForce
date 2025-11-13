use df_sdk::models::battle_record::BattleRecord;
use df_sdk::sdk::DeltaForceSdk;
use time::PrimitiveDateTime;
use tokio_stream::StreamExt;

use crate::commands::battle_record::OutputFormat;
use crate::error::Error;

fn should_output_and_continue(record: &BattleRecord, since: &Option<PrimitiveDateTime>) -> bool {
    if let Some(since) = since
        && record.time < *since
    {
        return false;
    }

    return true;
}

fn output(record: &BattleRecord, format: &OutputFormat) -> Result<(), Error> {
    let string = match format {
        OutputFormat::Default => record.to_string(),
        OutputFormat::Json => {
            serde_json::to_string(record).map_err(|e| Error::SerializeError(e))?
        }
        OutputFormat::JsonPretty => {
            serde_json::to_string_pretty(record).map_err(|e| Error::SerializeError(e))?
        }
    };

    println!("{}", string);
    Ok(())
}

pub async fn list(
    sdk: DeltaForceSdk,
    format: OutputFormat,
    limit: Option<usize>,
    since: Option<PrimitiveDateTime>,
) {
    let mut stream = sdk.iter_battle_records().await;
    if let Some(x) = limit {
        stream = Box::pin(stream.take(x));
    }

    while let Some(record) = stream.next().await {
        match record {
            Ok(record) => {
                if !should_output_and_continue(&record, &since) {
                    return;
                }

                if let Err(e) = output(&record, &format) {
                    eprintln!("{}", e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }
}
