use constants::escape_result::EscapeResult;
use constants::level::Level;
use constants::map::Map;
use constants::operator::Operator;
use serde::Serialize;
use serde::Serializer;
use time::PrimitiveDateTime;
use time::macros::format_description;

#[derive(Debug, Serialize)]
pub struct BattleRecord {
    pub id: String,
    #[serde(serialize_with = "datetime_serializer")]
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

pub fn datetime_serializer<S>(dt: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = dt
        .format(format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second]"
        ))
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}
