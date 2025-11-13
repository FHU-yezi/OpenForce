use std::fmt::Display;

use serde::Serialize;

use crate::constants::{escape_result::EscapeResult, operator::Operator};

#[derive(Debug, Serialize)]
pub struct BattleRecordTeammate {
    pub operator: Operator,
    pub escape_result: EscapeResult,
    pub duration_seconds: u16,
    pub kill_operators_count: u16,
    pub kill_bots_count: u16,
    pub escape_value: u32,
}

impl Display for BattleRecordTeammate {
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
