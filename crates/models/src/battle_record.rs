use constants::escape_result::EscapeResult;
use constants::level::Level;
use constants::map::Map;
use constants::operator::Operator;
use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(Debug, Serialize)]
pub struct BattleRecord {
    pub id: String,
    pub time: PrimitiveDateTime,
    pub map: Map,
    pub level: Level,
    pub operator: Operator,
    pub escape_result: EscapeResult,
    pub duration_seconds: u16,
    pub kill_operators_count: u16,
    pub kill_bots_count: u16,
    pub escape_value: u32,
    pub net_profit: i32,
    pub teammates: Vec<Teammate>,
}

#[derive(Debug, Serialize)]
pub struct Teammate {
    pub operator: Operator,
    pub escape_result: EscapeResult,
    pub duration_seconds: u16,
    pub kill_operators_count: u16,
    pub kill_bots_count: u16,
    pub escape_value: u32,
}
