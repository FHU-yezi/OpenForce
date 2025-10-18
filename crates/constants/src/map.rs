#[derive(Debug)]
pub enum Map {
    零号大坝,
    长弓溪谷,
    巴克什,
    航天基地,
    潮汐监狱,
}

impl Map {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "零号大坝" => Some(Map::零号大坝),
            "长弓溪谷" => Some(Map::长弓溪谷),
            "巴克什" => Some(Map::巴克什),
            "航天基地" => Some(Map::航天基地),
            "潮汐监狱" => Some(Map::潮汐监狱),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Map::零号大坝 => "零号大坝",
            Map::长弓溪谷 => "长弓溪谷",
            Map::巴克什 => "巴克什",
            Map::航天基地 => "航天基地",
            Map::潮汐监狱 => "潮汐监狱",
        }
    }

    pub fn from_map_id(x: u16) -> Option<Self> {
        match x {
            2201 | 2202 | 2211 | 2212 | 2231 | 2232 => Some(Map::零号大坝),
            1901 | 1902 | 1911 | 1912 => Some(Map::长弓溪谷),
            8101 | 8102 | 8103 => Some(Map::巴克什),
            3901 | 3902 => Some(Map::航天基地),
            8803 => Some(Map::潮汐监狱),
            _ => None,
        }
    }
}
