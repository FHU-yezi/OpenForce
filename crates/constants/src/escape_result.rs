#[derive(Debug)]
pub enum EscapeResult {
    EscapeSuccess,
    KilledByOperator,
    KilledByBot,
    Suicide,
    MidwayExit,
    EscapeFailedUnknown,
}

impl EscapeResult {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "撤离成功" => Some(EscapeResult::EscapeSuccess),
            "撤离失败 - 干员击杀" => Some(EscapeResult::KilledByOperator),
            "撤离失败 - AI 击杀" => Some(EscapeResult::KilledByBot),
            "撤离失败 - 自杀" => Some(EscapeResult::Suicide),
            "中途退出" => Some(EscapeResult::MidwayExit),
            "撤离失败 - 未知原因" => Some(EscapeResult::EscapeFailedUnknown),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EscapeResult::EscapeSuccess => "撤离成功",
            EscapeResult::KilledByOperator => "撤离失败 - 干员击杀",
            EscapeResult::KilledByBot => "撤离失败 - AI 击杀",
            EscapeResult::Suicide => "撤离失败 - 自杀",
            EscapeResult::MidwayExit => "中途退出",
            EscapeResult::EscapeFailedUnknown => "撤离失败 - 未知原因",
        }
    }

    // TODO: 缺少部分 ID 对应的撤离结果
    pub fn from_escape_result_id(x: u8) -> Option<Self> {
        match x {
            1 => Some(EscapeResult::EscapeSuccess),
            2 => Some(EscapeResult::KilledByOperator),
            3 => Some(EscapeResult::KilledByBot),
            7 => Some(EscapeResult::MidwayExit),
            10 => Some(EscapeResult::Suicide),
            6 | 9 | 11 => Some(EscapeResult::EscapeSuccess),
            _ => None,
        }
    }
}
