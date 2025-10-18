use crate::parsers::{parse_str, parse_str_then_number, parse_time, parse_uint};
use crate::sdk::DeltaForceSdk;
use constants::{escape_result::EscapeResult, level::Level, map::Map, operator::Operator};
use models::battle_record::BattleRecord;
use serde_json::Value;

impl<'a> DeltaForceSdk<'a> {
    pub async fn iter_battle_records(&self) -> Result<Vec<BattleRecord>, String> {
        // 构建带有查询参数的 URL
        let mut url = self.base_url.join("/ide/").unwrap();
        url.query_pairs_mut()
            .append_pair("iChartId", "450526")
            .append_pair("iSubChartId", "450526")
            .append_pair("sIdeToken", "PHq59Y")
            .append_pair("type", "4")
            .append_pair("page", "1");

        let mut request = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded");

        if let Some(credentials) = &self.credentials {
            request = request.header("Cookie", credentials.to_cookies());
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("发送请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("请求失败，状态码: {}", response.status()));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| format!("解析数据失败: {}", e))?;

        // println!("{:#?}", data);

        Ok(data["jData"].as_object().unwrap()["data"]
            .as_array()
            .expect("data 数组不存在")
            .iter()
            .map(|x| BattleRecord {
                id: parse_str(&x["RoomId"]).unwrap(),
                time: parse_time(&x["dtEventTime"]).unwrap(),
                map: Map::from_map_id(parse_str_then_number(&x["MapId"]).unwrap()).unwrap(),
                level: Level::from_map_id(parse_str_then_number(&x["MapId"]).unwrap()).unwrap(),
                operator: Operator::from_operator_id(parse_uint(&x["ArmedForceId"]).unwrap())
                    .unwrap(),
                escape_result: EscapeResult::from_escape_result_id(
                    parse_uint(&x["EscapeFailReason"]).unwrap(),
                )
                .unwrap(),
                duration_seconds: parse_uint(&x["DurationS"]).unwrap(),
                kill_operators_count: parse_uint(&x["KillCount"]).unwrap(),
                kill_bots_count: parse_uint(&x["DurationS"]).unwrap(),
                escape_value: parse_str_then_number(&x["FinalPrice"]).unwrap(),
                // TODO
                net_profit: 0,
            })
            .collect())
    }
}
