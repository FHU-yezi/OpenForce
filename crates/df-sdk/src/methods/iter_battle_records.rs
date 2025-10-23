use crate::error::Error;
use crate::parsers::*;
use crate::sdk::DeltaForceSdk;
use async_stream::stream;
use models::battle_record::{BattleRecord, Teammate};
use serde_json::Value;
use std::pin::Pin;
use tokio_stream::Stream;

async fn get_battle_records_list(sdk: &DeltaForceSdk, page: u8) -> Result<Vec<Value>, Error> {
    Ok(sdk
        .send_api_request(&[
            ("iChartId", "450526"),
            ("iSubChartId", "450526"),
            ("sIdeToken", "PHq59Y"),
            ("type", "4"),
            ("page", &page.to_string()),
        ])
        .await?
        .as_array()
        .ok_or(Error::ParseError)?
        .clone())
}

async fn get_battle_record_details(
    sdk: &DeltaForceSdk,
    room_id: &str,
) -> Result<Vec<Value>, Error> {
    Ok(sdk
        .send_api_request(&[
            ("iChartId", "450471"),
            ("iSubChartId", "450471"),
            ("sIdeToken", "ylP3eG"),
            ("roomId", room_id),
            ("type", "2"),
        ])
        .await?
        .as_array()
        .ok_or(Error::ParseError)?
        .clone())
}

trait FromBattleRecordDetailsApi: Sized {
    fn from_battle_record_details_api(x: &Value) -> Result<Self, Error>;
}

impl FromBattleRecordDetailsApi for Teammate {
    fn from_battle_record_details_api(x: &Value) -> Result<Self, Error> {
        Ok(Teammate {
            operator: parse_operator_id(&x["ArmedForceId"])?,
            escape_result: parse_escape_result(&x["EscapeFailReason"])?,
            duration_seconds: parse_uint(&x["DurationS"])?,
            kill_operators_count: parse_uint(&x["KillCount"])?,
            kill_bots_count: parse_uint(&x["KillAICount"])?,
            // 未知原因导致此字段有小概率为 null，此时会导致解析异常
            // 我们没有可参考的信息对此值进行猜测（玩家和队友的带出价值没有相关性）
            // 因此，字段为 null 时按 0 处理
            escape_value: match &x["FinalPrice"].as_null() {
                Some(_) => 0,
                None => parse_str_then_number(&x["FinalPrice"])?,
            },
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

fn estimate_escape_value(net_profit: i32) -> u32 {
    // 如果净收益为负值，假设带出价值为 0
    // 否则，假设带出价值等于净收益，即零损失 & 损耗
    match net_profit {
        negative if negative < 0 => 0,
        non_negative => non_negative as u32,
    }
}

impl DeltaForceSdk {
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

                    let battle_details = match get_battle_record_details(&self, &room_id).await {
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
                            // 未知原因导致此字段有小概率为 null，此时会导致解析异常
                            // 此时基于净收益（flowCalGainedPrice）估计带出价值
                            match &y["FinalPrice"].as_null() {
                                Some(_) => escape_value = Some(estimate_escape_value(parse_int::<i32>(&x["flowCalGainedPrice"])?)),
                                None => {
                                    match parse_str_then_number(&y["FinalPrice"]) {
                                        Ok(value) => escape_value = Some(value),
                                        Err(e) => {
                                            yield Err(e);
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            match Teammate::from_battle_record_details_api(&y) {
                                Ok(teammate) => teammates.push(teammate),
                                Err(e) => {
                                    yield Err(e);
                                    break;
                                }
                            }
                        }
                    }

                    // 对于部分记录，对局详情数据可能为空
                    // 因此 battle_details 的解析循环不会执行，从而导致 escape_value 为 None
                    // 此时基于净收益（flowCalGainedPrice）估计带出价值
                    if escape_value.is_none() {
                        escape_value = Some(estimate_escape_value(parse_int::<i32>(&x["flowCalGainedPrice"])?));
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
