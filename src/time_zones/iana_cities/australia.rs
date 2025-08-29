#[derive(Debug, Clone, Copy)]
pub enum IanaAustralia {
    Adelaide,
    Brisbane,
    Darwin,
    Ecula,
    Hobart,
    Lindeman,
    LordHowe,
    Melbourne,
    Perth,
    Sydney,
}

impl IanaAustralia {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Adelaide" => Some(Self::Adelaide),
            "Brisbane" => Some(Self::Brisbane),
            "Darwin" => Some(Self::Darwin),
            "Ecula" => Some(Self::Ecula),
            "Hobart" => Some(Self::Hobart),
            "Lindeman" => Some(Self::Lindeman),
            "Lord_Howe" => Some(Self::LordHowe),
            "Melbourne" => Some(Self::Melbourne),
            "Perth" => Some(Self::Perth),
            "Sydney" => Some(Self::Sydney),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Adelaide => "Adelaide",
            Self::Brisbane => "Brisbane",
            Self::Darwin => "Darwin",
            Self::Ecula => "Ecula",
            Self::Hobart => "Hobart",
            Self::Lindeman => "Lindeman",
            Self::LordHowe => "Lord_Howe",
            Self::Melbourne => "Melbourne",
            Self::Perth => "Perth",
            Self::Sydney => "Sydney",
        }
    }
}
