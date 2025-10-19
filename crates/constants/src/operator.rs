use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Operator {
    DWolf,
    Vyron,
    // TODO: 非官方翻译
    Nameless,
    // TODO: 非官方翻译
    SwiftWind,
    Stinger,
    Toxik,
    Shepherd,
    Uluru,
    // TODO: 非官方翻译
    DeepBlue,
    Luna,
    HackClaw,
    // TODO: 非官方翻译
    SilverWing,
}

impl Operator {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "红狼" => Some(Operator::DWolf),
            "威龙" => Some(Operator::Vyron),
            "无名" => Some(Operator::Nameless),
            "疾风" => Some(Operator::SwiftWind),
            "蜂医" => Some(Operator::Stinger),
            "蛊" => Some(Operator::Toxik),
            "牧羊人" => Some(Operator::Shepherd),
            "乌鲁鲁" => Some(Operator::Uluru),
            "深蓝" => Some(Operator::DeepBlue),
            "露娜" => Some(Operator::Luna),
            "骇爪" => Some(Operator::HackClaw),
            "银翼" => Some(Operator::SilverWing),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::DWolf => "红狼",
            Operator::Vyron => "威龙",
            Operator::Nameless => "无名",
            Operator::SwiftWind => "疾风",
            Operator::Stinger => "蜂医",
            Operator::Toxik => "蛊",
            Operator::Shepherd => "牧羊人",
            Operator::Uluru => "乌鲁鲁",
            Operator::DeepBlue => "深蓝",
            Operator::Luna => "露娜",
            Operator::HackClaw => "骇爪",
            Operator::SilverWing => "银翼",
        }
    }

    pub fn from_operator_id(x: u16) -> Option<Self> {
        match x {
            10007 => Some(Operator::DWolf),
            10010 => Some(Operator::Vyron),
            10011 => Some(Operator::Nameless),
            10012 => Some(Operator::SwiftWind),
            20003 => Some(Operator::Stinger),
            20004 => Some(Operator::Toxik),
            30008 => Some(Operator::Shepherd),
            30009 => Some(Operator::Uluru),
            30010 => Some(Operator::DeepBlue),
            40005 => Some(Operator::Luna),
            40010 => Some(Operator::HackClaw),
            40011 => Some(Operator::SilverWing),
            _ => None,
        }
    }
}
