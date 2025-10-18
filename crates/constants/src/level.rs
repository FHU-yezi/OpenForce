#[derive(Debug)]
pub enum Level {
    常规,
    机密,
    绝密,
    前夜,
    永夜,
    终夜,
}

impl Level {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "常规" => Some(Level::常规),
            "机密" => Some(Level::机密),
            "绝密" => Some(Level::绝密),
            "前夜" => Some(Level::前夜),
            "永夜" => Some(Level::永夜),
            "终夜" => Some(Level::终夜),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Level::常规 => "常规",
            Level::机密 => "机密",
            Level::绝密 => "绝密",
            Level::前夜 => "前夜",
            Level::永夜 => "永夜",
            Level::终夜 => "终夜",
        }
    }

    pub fn from_map_id(x: u16) -> Option<Self> {
        match x {
            2201 | 2211 | 1901 | 1911 | 8101 => Some(Level::常规),
            1902 | 1912 | 2202 | 2212 | 3901 | 8102 => Some(Level::机密),
            3902 | 8103 | 8803 => Some(Level::绝密),
            2231 => Some(Level::前夜),
            2232 => Some(Level::永夜),
            // TODO: 缺少终夜（零号大坝）的地图 ID
            _ => None,
        }
    }
}
