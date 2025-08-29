#[derive(Debug, Clone, Copy)]
pub enum IanaAtlantic {
    Azores,
    Bermuda,
    CapeVerde,
    Faeroe,
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
            "Cape_Verde" => Some(Self::CapeVerde),
            "Faeroe" => Some(Self::Faeroe),
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
            Self::CapeVerde => "Cape_Verde",
            Self::Faeroe => "Faeroe",
            Self::Reykjavik => "Reykjavik",
            Self::SouthGeorgia => "South_Georgia",
            Self::Stanley => "Stanley",
            Self::StHelena => "St_Helena",
        }
    }
}
