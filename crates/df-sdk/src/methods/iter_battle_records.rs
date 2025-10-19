use crate::error::Error;
use crate::parsers::*;
use crate::sdk::DeltaForceSdk;
use crate::utils::extract_data;
use async_stream::stream;
use models::battle_record::{BattleRecord, Teammate};
use serde_json::Value;
use std::pin::Pin;
use tokio_stream::Stream;

async fn get_battle_records_list<'a>(
    sdk: &DeltaForceSdk<'a>,
    page: u8,
) -> Result<Vec<Value>, Error> {
    let mut url = sdk.endpoint.clone();
    url.query_pairs_mut()
        .append_pair("iChartId", "450526")
        .append_pair("iSubChartId", "450526")
        .append_pair("sIdeToken", "PHq59Y")
        .append_pair("type", "4")
        .append_pair("page", &page.to_string());

    let request = sdk.client.post(url).header(
        "Cookie",
        &sdk.credentials
            .as_ref()
            .ok_or(Error::MissingCredentials)?
            .to_cookies(),
    );

    let response = request.send().await.map_err(|e| Error::RequestError(e))?;

    Ok(extract_data(response)
        .await?
        .as_array()
        .ok_or(Error::ParseError)?
        .clone())
}

async fn get_battle_record_detail<'a>(
    sdk: &DeltaForceSdk<'a>,
    room_id: &str,
) -> Result<Vec<Value>, Error> {
    let mut url = sdk.endpoint.join("/ide/").unwrap();
    url.query_pairs_mut()
        .append_pair("iChartId", "450471")
        .append_pair("iSubChartId", "450471")
        .append_pair("sIdeToken", "ylP3eG")
        .append_pair("roomId", room_id)
        .append_pair("type", "2");

    let request = sdk.client.post(url).header(
        "Cookie",
        sdk.credentials
            .as_ref()
            .ok_or(Error::MissingCredentials)?
            .to_cookies(),
    );

    let response = request.send().await.map_err(|e| Error::RequestError(e))?;

    Ok(extract_data(response)
        .await?
        .as_array()
        .ok_or(Error::ParseError)?
        .clone())
}

trait FromBattleRecordDetailApi: Sized {
    fn from_battle_record_detail_api(x: &Value) -> Result<Self, Error>;
}

impl FromBattleRecordDetailApi for Teammate {
    fn from_battle_record_detail_api(x: &Value) -> Result<Self, Error> {
        Ok(Teammate {
            operator: parse_operator_id(&x["ArmedForceId"])?,
            escape_result: parse_escape_result(&x["EscapeFailReason"])?,
            duration_seconds: parse_uint(&x["DurationS"])?,
            kill_operators_count: parse_uint(&x["KillCount"])?,
            kill_bots_count: parse_uint(&x["KillAICount"])?,
            escape_value: parse_str_then_number(&x["FinalPrice"])?,
        })
    }
}

trait FromBattleRecordsListApi: Sized {
    fn from_battle_records_list_api(
        x: &Value,
        escape_value: u32,
        teammates: Vec<Teammate>,
    ) -> Result<Self, Error>;
}

impl FromBattleRecordsListApi for BattleRecord {
    fn from_battle_records_list_api(
        x: &Value,
        escape_value: u32,
        teammates: Vec<Teammate>,
    ) -> Result<Self, Error> {
        Ok(BattleRecord {
            id: parse_str(&x["RoomId"])?,
            time: parse_time(&x["dtEventTime"])?,
            map: parse_map_id_to_map(&x["MapId"])?,
            level: parse_map_id_to_level(&x["MapId"])?,
            operator: parse_operator_id(&x["ArmedForceId"])?,
            escape_result: parse_escape_result(&x["EscapeFailReason"])?,
            duration_seconds: parse_uint(&x["DurationS"])?,
            kill_operators_count: parse_uint(&x["KillCount"])?,
            kill_bots_count: parse_uint(&x["KillAICount"])?,
            escape_value,
            net_profit: parse_int(&x["flowCalGainedPrice"])?,
            teammates,
        })
    }
}

impl<'a> DeltaForceSdk<'a> {
    pub async fn iter_battle_records(
        &self,
    ) -> Pin<Box<dyn Stream<Item = Result<BattleRecord, Error>> + Send + '_>> {
        Box::pin(stream! {
            let mut page: u8 = 1;
            loop {
                let battle_records_list = match get_battle_records_list(&self, page).await {
                    Ok(list) => list,
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                };
                // 没有新的对局记录
                if battle_records_list.is_empty() {
                    break;
                }

                for x in battle_records_list {
                    let room_id = match parse_str(&x["RoomId"]) {
                        Ok(id) => id,
                        Err(e) => {
                            yield Err(e);
                            break;
                        }
                    };

                    let battle_details = match get_battle_record_detail(&self, &room_id).await {
                        Ok(details) => details,
                        Err(e) => {
                            yield Err(e);
                            break;
                        }
                    };

                    let mut escape_value: Option<u32> = None;
                    let mut teammates = Vec::new();

                    for y in battle_details {
                        let is_player = match y["vopenid"].as_bool().ok_or(Error::ParseError) {
                            Ok(b) => b,
                            Err(e) => {
                                yield Err(e);
                                break;
                            }
                        };

                        if is_player {
                            match parse_str_then_number(&y["FinalPrice"]) {
                                Ok(value) => escape_value = Some(value),
                                Err(e) => {
                                    yield Err(e);
                                    break;
                                }
                            }
                        } else {
                            match Teammate::from_battle_record_detail_api(&y) {
                                Ok(teammate) => teammates.push(teammate),
                                Err(e) => {
                                    yield Err(e);
                                    break;
                                }
                            }
                        }
                    }

                    if escape_value.is_none() {
                        yield Err(Error::ParseError);
                        break;
                    }

                    match BattleRecord::from_battle_records_list_api(&x, escape_value.unwrap(), teammates) {
                        Ok(record) => yield Ok(record),
                        Err(e) => {
                            yield Err(e);
                            break;
                        },
                    }
                }

                page += 1;
            }
        })
    }
}
