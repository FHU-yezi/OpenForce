use crate::apis::battle_records::{get_battle_record_details_api, get_battle_records_list_api};
use crate::error::Error;
use crate::models::{battle_record::BattleRecord, battle_record_teammate::BattleRecordTeammate};
use crate::parsers::*;
use crate::sdk::DeltaForceSdk;
use async_stream::stream;
use serde_json::Value;
use std::pin::Pin;
use tokio_stream::Stream;

fn estimate_escape_value(net_profit: i32) -> u32 {
    // 如果净收益为负值，假设带出价值为 0
    // 否则，假设带出价值等于净收益，即零损失 & 损耗
    match net_profit {
        negative if negative < 0 => 0,
        non_negative => non_negative as u32,
    }
}

fn parse_teammate(data: &Value) -> Result<BattleRecordTeammate, Error> {
    Ok(BattleRecordTeammate {
        operator: parse_operator_id(&data["ArmedForceId"])?,
        escape_result: parse_escape_result(&data["EscapeFailReason"])?,
        duration_seconds: parse_uint(&data["DurationS"])?,
        kill_operators_count: parse_uint(&data["KillCount"])?,
        kill_bots_count: parse_uint(&data["KillAICount"])?,
        // 未知原因导致此字段有小概率为 null，此时会导致解析异常
        // 我们没有可参考的信息对此值进行猜测（玩家和队友的带出价值没有相关性）
        // 因此，字段为 null 时按 0 处理
        escape_value: match &data["FinalPrice"].as_null() {
            Some(_) => 0,
            None => parse_str_then_number(&data["FinalPrice"])?,
        },
    })
}

fn parse_battle_record(
    data: &Value,
    escape_value: u32,
    teammates: Vec<BattleRecordTeammate>,
) -> Result<BattleRecord, Error> {
    Ok(BattleRecord {
        id: parse_str(&data["RoomId"])?,
        time: parse_time(&data["dtEventTime"])?,
        map: parse_map_id_to_map(&data["MapId"])?,
        level: parse_map_id_to_level(&data["MapId"])?,
        operator: parse_operator_id(&data["ArmedForceId"])?,
        escape_result: parse_escape_result(&data["EscapeFailReason"])?,
        duration_seconds: parse_uint(&data["DurationS"])?,
        kill_operators_count: parse_uint(&data["KillCount"])?,
        kill_bots_count: parse_uint(&data["KillAICount"])?,
        escape_value,
        net_profit: parse_int(&data["flowCalGainedPrice"])?,
        teammates,
    })
}

impl DeltaForceSdk {
    pub async fn iter_battle_records(
        &self,
    ) -> Pin<Box<dyn Stream<Item = Result<BattleRecord, Error>> + Send + '_>> {
        Box::pin(stream! {
            let mut page: u8 = 1;
            loop {
                let battle_records = match get_battle_records_list_api(&self, page).await {
                    // 没有新的对局记录
                    Ok(x) if x.is_empty() => break,
                    Ok(x) => x,
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                };

                for battle_record in battle_records {
                    let room_id = match parse_str(&battle_record["RoomId"]) {
                        Ok(id) => id,
                        Err(e) => {
                            yield Err(e);
                            break;
                        }
                    };

                    let battle_details = match get_battle_record_details_api(&self, &room_id).await {
                        Ok(details) => details,
                        Err(e) => {
                            yield Err(e);
                            break;
                        }
                    };

                    let mut escape_value: Option<u32> = None;
                    let mut teammates = Vec::new();

                    for player_data in battle_details {
                        let is_current_user = match player_data["vopenid"].as_bool().ok_or(Error::ParseError) {
                            Ok(b) => b,
                            Err(e) => {
                                yield Err(e);
                                break;
                            }
                        };

                        if is_current_user {
                            // 未知原因导致此字段有小概率为 null，此时会导致解析异常
                            // 此时基于净收益（flowCalGainedPrice）估计带出价值
                            match &player_data["FinalPrice"].as_null() {
                                Some(_) => escape_value = Some(estimate_escape_value(parse_int::<i32>(&battle_record["flowCalGainedPrice"])?)),
                                None => {
                                    match parse_str_then_number(&player_data["FinalPrice"]) {
                                        Ok(value) => escape_value = Some(value),
                                        Err(e) => {
                                            yield Err(e);
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            match parse_teammate(&player_data) {
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
                        escape_value = Some(estimate_escape_value(parse_int(&battle_record["flowCalGainedPrice"])?));
                    }
                    match parse_battle_record(&battle_record, escape_value.unwrap(), teammates) {
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
