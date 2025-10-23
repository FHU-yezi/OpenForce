use serde::Serialize;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Map {
    Dam,
    Forest,
    Brakkesh,
    SpaceCity,
    // TODO: 非官方翻译
    TidalPrison,
}

impl Map {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "零号大坝" => Some(Map::Dam),
            "长弓溪谷" => Some(Map::Forest),
            "巴克什" => Some(Map::Brakkesh),
            "航天基地" => Some(Map::SpaceCity),
            "潮汐监狱" => Some(Map::TidalPrison),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Map::Dam => "零号大坝",
            Map::Forest => "长弓溪谷",
            Map::Brakkesh => "巴克什",
            Map::SpaceCity => "航天基地",
            Map::TidalPrison => "潮汐监狱",
        }
    }

    pub fn from_map_id(x: u16) -> Option<Self> {
        match x {
            2201 | 2202 | 2211 | 2212 | 2231 | 2232 => Some(Map::Dam),
            1901 | 1902 | 1911 | 1912 => Some(Map::Forest),
            8101 | 8102 | 8103 => Some(Map::Brakkesh),
            3901 | 3902 => Some(Map::SpaceCity),
            8803 => Some(Map::TidalPrison),
            _ => None,
        }
    }
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
