use df_sdk::models::battle_record::BattleRecord;
use df_sdk::sdk::DeltaForceSdk;

use crate::{commands::battle_record::OutputFormat, error::Error};

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

pub async fn get(sdk: DeltaForceSdk, room_id: String, format: OutputFormat) {
    let battle_record_details = match sdk.get_battle_record_details(&room_id).await {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    if let Err(e) = output(&battle_record_details, &format) {
        eprintln!("{}", e);
        return;
    }
}
