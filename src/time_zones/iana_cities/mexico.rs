#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IanaMexico {
    BajaNorte,
    BajaSur,
    General,
}

impl IanaMexico {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "BajaNorte" => Some(Self::BajaNorte),
            "BajaSur" => Some(Self::BajaSur),
            "General" => Some(Self::General),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BajaNorte => "BajaNorte",
            Self::BajaSur => "BajaSur",
            Self::General => "General",
        }
    }
}
