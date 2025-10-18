use constants::escape_result::EscapeResult;
use constants::level::Level;
use constants::map::Map;
use constants::operator::Operator;
use time::OffsetDateTime;

pub struct BattleRecord<'a> {
    id: &'a str,
    time: OffsetDateTime,
    map: Map,
    level: Level,
    operator: Operator,
    escape_result: EscapeResult,
    duration_seconds: u16,
    kill_operators_count: u16,
    kill_bots_count: u16,
    escape_value: u32,
    net_profit: i32,
    // teammates: Vec<Teammate>,
}
