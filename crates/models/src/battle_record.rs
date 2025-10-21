use std::fmt::Display;

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

impl Display for BattleRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "对局 {} - ID {}",
            self.time
                .format(format_description!(
                    "[year]-[month]-[day]T[hour]:[minute]:[second]"
                ))
                .unwrap(),
            self.id
        )?;
        // 首行长度固定，分隔线与首行长度相同
        writeln!(f, "================================================")?;
        writeln!(
            f,
            "地图：{} - {}\t干员：{}",
            self.map.as_str(),
            self.level.as_str(),
            self.operator.as_str(),
        )?;
        writeln!(
            f,
            "{}{}对局时长：{} 秒",
            self.escape_result.as_str(),
            // 根据撤离结果调整制表符数量，保证输出对齐
            match self.escape_result {
                EscapeResult::EscapeSuccess | EscapeResult::MidwayExit => "\t\t",
                _ => "\t",
            },
            self.duration_seconds,
        )?;
        writeln!(
            f,
            "击杀：干员 {} / AI {}",
            self.kill_operators_count, self.kill_bots_count,
        )?;
        writeln!(f, "带出价值：{}", self.escape_value)?;
        writeln!(f, "净收益：{}", self.escape_value)?;

        if self.teammates.is_empty() {
            writeln!(f, "无队友")?;
        } else {
            writeln!(f, "队友信息：")?;
            for teammate in &self.teammates {
                write!(f, "    {}", teammate)?;
            }
        }

        Ok(())
    }
}

impl Display for Teammate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "队友：{}  对局时长：{} 秒  {}  击杀：干员 {} / AI {}  带出价值：{}",
            self.operator.as_str(),
            self.duration_seconds,
            self.escape_result.as_str(),
            self.kill_operators_count,
            self.kill_bots_count,
            self.escape_value
        )
    }
}
