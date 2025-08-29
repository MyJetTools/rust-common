#[derive(Debug, Clone, Copy)]
pub enum IanaAustralia {
    Adelaide,
    Brisbane,
    Ecula,
    LordHowe,
    Perth,
    Sydney,
}

impl IanaAustralia {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Adelaide" => Some(Self::Adelaide),
            "Brisbane" => Some(Self::Brisbane),
            "Ecula" => Some(Self::Ecula),
            "Lord_Howe" => Some(Self::LordHowe),
            "Perth" => Some(Self::Perth),
            "Sydney" => Some(Self::Sydney),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Adelaide => "Adelaide",
            Self::Brisbane => "Brisbane",
            Self::Ecula => "Ecula",
            Self::LordHowe => "Lord_Howe",
            Self::Perth => "Perth",
            Self::Sydney => "Sydney",
        }
    }
}
