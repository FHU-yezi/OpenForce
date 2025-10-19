use crate::error::Error;
use crate::parsers::*;
use crate::sdk::DeltaForceSdk;
use crate::utils::extract_data;
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
            println!("正在获取 {} 的对局详情", parse_str(&x["RoomId"])?);

            let mut details_url = self.endpoint.join("/ide/").unwrap();
            details_url
                .query_pairs_mut()
                .append_pair("iChartId", "450471")
                .append_pair("iSubChartId", "450471")
                .append_pair("sIdeToken", "ylP3eG")
                .append_pair("roomId", &parse_str(&x["RoomId"])?)
                .append_pair("type", "2");

            let details_request = self.client.post(details_url).header(
                "Cookie",
                &self
                    .credentials
                    .as_ref()
                    .ok_or(Error::MissingCredentials)?
                    .to_cookies(),
            );

            let details_response = details_request
                .send()
                .await
                .map_err(|e| Error::RequestError(e))?;

            let details_data = extract_data(details_response).await?;

            let mut escape_value: Option<u32> = None;
            let mut teammates = Vec::new();
            for y in details_data.as_array().ok_or(Error::ParseError)? {
                match y["vopenid"].as_bool().ok_or(Error::ParseError)? {
                    true => escape_value = Some(parse_str_then_number(&y["FinalPrice"])?),
                    false => teammates.push(Teammate {
                        operator: parse_operator_id(&y["ArmedForceId"])?,
                        escape_result: parse_escape_result(&y["EscapeFailReason"])?,
                        duration_seconds: parse_uint(&y["DurationS"])?,
                        kill_operators_count: parse_uint(&y["KillCount"])?,
                        kill_bots_count: parse_uint(&y["KillAICount"])?,
                        escape_value: parse_str_then_number(&y["FinalPrice"])?,
                    }),
                }
            }

            result.push(BattleRecord {
                id: parse_str(&x["RoomId"])?,
                time: parse_time(&x["dtEventTime"])?,
                map: parse_map_id_to_map(&x["MapId"])?,
                level: parse_map_id_to_level(&x["MapId"])?,
                operator: parse_operator_id(&x["ArmedForceId"])?,
                escape_result: parse_escape_result(&x["EscapeFailReason"])?,
                duration_seconds: parse_uint(&x["DurationS"])?,
                kill_operators_count: parse_uint(&x["KillCount"])?,
                kill_bots_count: parse_uint(&x["KillAICount"])?,
                escape_value: match escape_value {
                    Some(x) => x,
                    None => return Err(Error::ParseError),
                },
                net_profit: parse_int(&x["flowCalGainedPrice"])?,
                teammates,
            });
        }

        Ok(result)
    }
}
