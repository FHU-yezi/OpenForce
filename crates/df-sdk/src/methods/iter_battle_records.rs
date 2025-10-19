use crate::error::Error;
use crate::parsers::{parse_int, parse_str, parse_str_then_number, parse_time, parse_uint};
use crate::sdk::DeltaForceSdk;
use crate::utils::extract_data;
use constants::{escape_result::EscapeResult, level::Level, map::Map, operator::Operator};
use models::battle_record::{BattleRecord, Teammate};

impl<'a> DeltaForceSdk<'a> {
    pub async fn iter_battle_records(&self, page: u8) -> Result<Vec<BattleRecord>, Error> {
        let mut url = self.endpoint.clone();
        url.query_pairs_mut()
            .append_pair("iChartId", "450526")
            .append_pair("iSubChartId", "450526")
            .append_pair("sIdeToken", "PHq59Y")
            .append_pair("type", "4")
            .append_pair("page", &page.to_string());

        let request = self.client.post(url).header(
            "Cookie",
            &self
                .credentials
                .as_ref()
                .ok_or(Error::MissingCredentials)?
                .to_cookies(),
        );

        let response = request.send().await.map_err(|e| Error::RequestError(e))?;

        let mut result = Vec::new();

        let data = extract_data(response).await?;
        for x in data.as_array().ok_or(Error::ParseError)? {
            println!("正在获取 {} 的对局详情", parse_str(&x["RoomId"]).unwrap());

            let mut details_url = self.endpoint.join("/ide/").unwrap();
            details_url
                .query_pairs_mut()
                .append_pair("iChartId", "450471")
                .append_pair("iSubChartId", "450471")
                .append_pair("sIdeToken", "ylP3eG")
                .append_pair("roomId", &parse_str(&x["RoomId"]).unwrap())
                .append_pair("type", "2");

            let details_request = self
                .client
                .post(details_url)
                .header("Cookie", &self.credentials.as_ref().unwrap().to_cookies());

            let details_response = details_request
                .send()
                .await
                .map_err(|e| Error::RequestError(e))?;

            let details_data = extract_data(details_response).await?;

            let mut escape_value: Option<u32> = None;
            let mut teammates = Vec::new();
            for y in details_data.as_array().ok_or(Error::ParseError)? {
                match y["vopenid"].as_bool().ok_or(Error::ParseError)? {
                    true => escape_value = Some(parse_str_then_number(&y["FinalPrice"]).unwrap()),
                    false => teammates.push(Teammate {
                        operator: Operator::from_operator_id(
                            parse_uint(&y["ArmedForceId"]).unwrap(),
                        )
                        .unwrap(),
                        escape_result: EscapeResult::from_escape_result_id(
                            parse_uint(&y["EscapeFailReason"]).unwrap(),
                        )
                        .unwrap(),
                        duration_seconds: parse_uint(&y["DurationS"]).unwrap(),
                        kill_operators_count: parse_uint(&y["KillCount"]).unwrap(),
                        kill_bots_count: parse_uint(&y["KillAICount"]).unwrap(),
                        escape_value: parse_str_then_number(&y["FinalPrice"]).unwrap(),
                    }),
                }
            }

            result.push(BattleRecord {
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
                kill_bots_count: parse_uint(&x["KillAICount"]).unwrap(),
                escape_value: match escape_value {
                    Some(x) => x,
                    None => return Err(Error::MissingData("未找到自己的对局数据".to_string())),
                },
                net_profit: parse_int(&x["flowCalGainedPrice"]).unwrap(),
                teammates,
            });
        }

        Ok(result)
    }
}
