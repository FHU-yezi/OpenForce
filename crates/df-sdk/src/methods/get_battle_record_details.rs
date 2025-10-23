use models::battle_record::{BattleRecord, Teammate};
use serde_json::Value;

use crate::apis::battle_records::get_battle_record_details_api;
use crate::error::Error;
use crate::parsers::{
    parse_escape_result, parse_map_id_to_level, parse_map_id_to_map, parse_operator_id,
    parse_str_then_number, parse_time, parse_uint,
};
use crate::sdk::DeltaForceSdk;

trait FromBattleRecordDetailsApi: Sized {
    fn from_battle_record_details_api(
        x: &Value,
        room_id: &str,
        teammates: Vec<Teammate>,
    ) -> Result<Self, Error>;
}

impl FromBattleRecordDetailsApi for BattleRecord {
    fn from_battle_record_details_api(
        x: &Value,
        room_id: &str,
        teammates: Vec<Teammate>,
    ) -> Result<Self, Error> {
        Ok(BattleRecord {
            id: room_id.to_string(),
            time: parse_time(&x["dtEventTime"])?,
            map: parse_map_id_to_map(&x["MapId"])?,
            level: parse_map_id_to_level(&x["MapId"])?,
            operator: parse_operator_id(&x["ArmedForceId"])?,
            escape_result: parse_escape_result(&x["EscapeFailReason"])?,
            duration_seconds: parse_uint(&x["DurationS"])?,
            kill_operators_count: parse_uint(&x["KillCount"])?,
            kill_bots_count: parse_uint(&x["KillAICount"])?,
            escape_value: parse_str_then_number(&x["FinalPrice"])?,
            // TODO: 该接口无净收益参数
            net_profit: parse_str_then_number(&x["FinalPrice"])?,
            teammates,
        })
    }
}

fn parse_teammate(x: &Value) -> Result<Teammate, Error> {
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

impl DeltaForceSdk {
    pub async fn get_battle_record_details(&self, room_id: &str) -> Result<BattleRecord, Error> {
        let battle_record_details = get_battle_record_details_api(self, room_id).await?;
        if battle_record_details.len() == 0 {
            // TODO: 添加数据为空时的独立错误
            return Err(Error::ParseError);
        }

        let mut teammates = Vec::new();
        let mut player_data: Option<Value> = None;
        for x in battle_record_details {
            let is_player = x["vopenid"].as_bool().ok_or(Error::ParseError)?;

            if is_player {
                player_data = Some(x);
            } else {
                teammates.push(parse_teammate(&x)?);
            }
        }

        Ok(BattleRecord::from_battle_record_details_api(
            &player_data.unwrap(),
            room_id,
            teammates,
        )?)
    }
}
