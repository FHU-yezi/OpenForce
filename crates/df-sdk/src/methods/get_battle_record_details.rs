use crate::models::{battle_record::BattleRecord, battle_record_teammate::BattleRecordTeammate};
use serde_json::Value;

use crate::apis::battle_records::get_battle_record_details_api;
use crate::error::Error;
use crate::parsers::{
    parse_escape_result, parse_map_id_to_level, parse_map_id_to_map, parse_operator_id,
    parse_str_then_number, parse_time, parse_uint,
};
use crate::sdk::DeltaForceSdk;

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
    room_id: &str,
    teammates: Vec<BattleRecordTeammate>,
) -> Result<BattleRecord, Error> {
    Ok(BattleRecord {
        id: room_id.to_string(),
        time: parse_time(&data["dtEventTime"])?,
        map: parse_map_id_to_map(&data["MapId"])?,
        level: parse_map_id_to_level(&data["MapId"])?,
        operator: parse_operator_id(&data["ArmedForceId"])?,
        escape_result: parse_escape_result(&data["EscapeFailReason"])?,
        duration_seconds: parse_uint(&data["DurationS"])?,
        kill_operators_count: parse_uint(&data["KillCount"])?,
        kill_bots_count: parse_uint(&data["KillAICount"])?,
        escape_value: parse_str_then_number(&data["FinalPrice"])?,
        // TODO: 该接口无净收益参数
        net_profit: parse_str_then_number(&data["FinalPrice"])?,
        teammates,
    })
}

impl DeltaForceSdk {
    pub async fn get_battle_record_details(&self, room_id: &str) -> Result<BattleRecord, Error> {
        let battle_record = get_battle_record_details_api(self, room_id).await?;
        if battle_record.len() == 0 {
            // TODO: 添加数据为空时的独立错误
            return Err(Error::ParseError);
        }

        let mut teammates = Vec::new();
        let mut current_user_data: Option<Value> = None;
        for player_data in battle_record {
            let is_current_user = player_data["vopenid"].as_bool().ok_or(Error::ParseError)?;

            if is_current_user {
                current_user_data = Some(player_data);
            } else {
                teammates.push(parse_teammate(&player_data)?);
            }
        }

        Ok(parse_battle_record(
            &current_user_data.unwrap(),
            room_id,
            teammates,
        )?)
    }
}
