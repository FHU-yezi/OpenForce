#[derive(Debug)]
pub enum Operator {
    红狼,
    威龙,
    无名,
    疾风,
    蜂医,
    蛊,
    牧羊人,
    乌鲁鲁,
    深蓝,
    露娜,
    骇爪,
    银翼,
}

impl Operator {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "红狼" => Some(Operator::红狼),
            "威龙" => Some(Operator::威龙),
            "无名" => Some(Operator::无名),
            "疾风" => Some(Operator::疾风),
            "蜂医" => Some(Operator::蜂医),
            "蛊" => Some(Operator::蛊),
            "牧羊人" => Some(Operator::牧羊人),
            "乌鲁鲁" => Some(Operator::乌鲁鲁),
            "深蓝" => Some(Operator::深蓝),
            "露娜" => Some(Operator::露娜),
            "骇爪" => Some(Operator::骇爪),
            "银翼" => Some(Operator::银翼),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::红狼 => "红狼",
            Operator::威龙 => "威龙",
            Operator::无名 => "无名",
            Operator::疾风 => "疾风",
            Operator::蜂医 => "蜂医",
            Operator::蛊 => "蛊",
            Operator::牧羊人 => "牧羊人",
            Operator::乌鲁鲁 => "乌鲁鲁",
            Operator::深蓝 => "深蓝",
            Operator::露娜 => "露娜",
            Operator::骇爪 => "骇爪",
            Operator::银翼 => "银翼",
        }
    }

    pub fn from_operator_id(x: u16) -> Option<Self> {
        match x {
            10007 => Some(Operator::红狼),
            10010 => Some(Operator::威龙),
            10011 => Some(Operator::无名),
            10012 => Some(Operator::疾风),
            20003 => Some(Operator::蜂医),
            20004 => Some(Operator::蛊),
            30008 => Some(Operator::牧羊人),
            30009 => Some(Operator::乌鲁鲁),
            30010 => Some(Operator::深蓝),
            40005 => Some(Operator::露娜),
            40010 => Some(Operator::骇爪),
            40011 => Some(Operator::银翼),
            _ => None,
        }
    }
}
