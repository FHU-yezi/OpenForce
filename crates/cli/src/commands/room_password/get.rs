use df_sdk::constants::map::Map;
use df_sdk::sdk::DeltaForceSdk;

use crate::OutputFormat;
use crate::error::Error;

fn output(room_password: &Vec<(Map, String)>, format: &OutputFormat) -> Result<(), Error> {
    match format {
        OutputFormat::Default => {
            for (map, password) in room_password {
                println!("{}：{}", map.as_str(), password);
            }
        }
        OutputFormat::Json | OutputFormat::JsonPretty => {
            let json_value = serde_json::Value::Array(
                room_password
                    .iter()
                    .map(|(map, password)| {
                        serde_json::json!({
                            "map": map.as_str(),
                            "password": password
                        })
                    })
                    .collect::<Vec<_>>(),
            );

            let json_str = match format {
                OutputFormat::Json => serde_json::to_string(&json_value),
                OutputFormat::JsonPretty => serde_json::to_string_pretty(&json_value),
                _ => unreachable!(),
            }
            .map_err(|e| Error::SerializeError(e))?;

            println!("{}", json_str);
        }
    }

    Ok(())
}

pub async fn get(sdk: DeltaForceSdk, format: OutputFormat) {
    let room_password = match sdk.get_room_password().await {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    if let Err(e) = output(&room_password, &format) {
        eprintln!("{}", e);
        return;
    }
}
