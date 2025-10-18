// 非官方翻译（官方为 Easy / Normal / Hard）
#[derive(Debug)]
pub enum Level {
    Normal,
    Secret,
    TopSecret,
    NormalNight,
    SecretNight,
    TopSecretNight,
}

impl Level {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "常规" => Some(Level::Normal),
            "机密" => Some(Level::Secret),
            "绝密" => Some(Level::TopSecret),
            "前夜" => Some(Level::NormalNight),
            "永夜" => Some(Level::SecretNight),
            "终夜" => Some(Level::TopSecretNight),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Normal => "常规",
            Level::Secret => "机密",
            Level::TopSecret => "绝密",
            Level::NormalNight => "前夜",
            Level::SecretNight => "永夜",
            Level::TopSecretNight => "终夜",
        }
    }

    pub fn from_map_id(x: u16) -> Option<Self> {
        match x {
            2201 | 2211 | 1901 | 1911 | 8101 => Some(Level::Normal),
            1902 | 1912 | 2202 | 2212 | 3901 | 8102 => Some(Level::Secret),
            3902 | 8103 | 8803 => Some(Level::TopSecret),
            2231 => Some(Level::NormalNight),
            2232 => Some(Level::SecretNight),
            // TODO: 缺少终夜（零号大坝）的地图 ID
            _ => None,
        }
    }
}
