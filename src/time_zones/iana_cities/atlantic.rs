#[derive(Debug, Clone, Copy)]
pub enum IanaAtlantic {
    Azores,
    Bermuda,
    Canary,
    CapeVerde,
    Faeroe,
    Faroe,
    Madeira,
    Reykjavik,
    SouthGeorgia,
    Stanley,
    StHelena,
}

impl IanaAtlantic {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Azores" => Some(Self::Azores),
            "Bermuda" => Some(Self::Bermuda),
            "Canary" => Some(Self::Canary),
            "Cape_Verde" => Some(Self::CapeVerde),
            "Faeroe" => Some(Self::Faeroe),
            "Faroe" => Some(Self::Faroe),
            "Madeira" => Some(Self::Madeira),
            "Reykjavik" => Some(Self::Reykjavik),
            "South_Georgia" => Some(Self::SouthGeorgia),
            "Stanley" => Some(Self::Stanley),
            "St_Helena" => Some(Self::StHelena),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Azores => "Azores",
            Self::Bermuda => "Bermuda",
            Self::Canary => "Canary",
            Self::CapeVerde => "Cape_Verde",
            Self::Faeroe => "Faeroe",
            Self::Faroe => "Faroe",
            Self::Madeira => "Madeira",
            Self::Reykjavik => "Reykjavik",
            Self::SouthGeorgia => "South_Georgia",
            Self::Stanley => "Stanley",
            Self::StHelena => "St_Helena",
        }
    }
}
