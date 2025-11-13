use crate::constants::map::Map;
use serde_json::Value;

use crate::apis::room_password::get_room_password_list_api;
use crate::error::Error;
use crate::parsers::parse_str;
use crate::sdk::DeltaForceSdk;

fn parse_room_password(data: &Vec<Value>) -> Result<Vec<(Map, String)>, Error> {
    let mut result = Vec::new();

    for room_password in data {
        match room_password.as_object() {
            Some(x) => {
                let map_name = parse_str(&x["mapName"])?;
                result.push((
                    Map::from_str(&map_name)
                        .ok_or(Error::UnknownData(format!("未知的地图名称（{map_name}）")))?,
                    parse_str(&x["secret"])?,
                ));
            }
            None => return Err(Error::ParseError),
        }
    }

    // 按照 Map 枚举定义的顺序排序
    result.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(result)
}

impl DeltaForceSdk {
    pub async fn get_room_password(&self) -> Result<Vec<(Map, String)>, Error> {
        let room_password_list = get_room_password_list_api(self).await?;

        parse_room_password(&room_password_list)
    }
}
