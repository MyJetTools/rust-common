#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IanaIndiana {
    Indianapolis,
    Knox,
    Marengo,
    Petersburg,
    TellCity,
    Vevay,
    Vincennes,
    Winamac,
}

impl IanaIndiana {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Indianapolis" => Some(Self::Indianapolis),
            "Knox" => Some(Self::Knox),
            "Marengo" => Some(Self::Marengo),
            "Petersburg" => Some(Self::Petersburg),
            "Tell_City" => Some(Self::TellCity),
            "Vevay" => Some(Self::Vevay),
            "Vincennes" => Some(Self::Vincennes),
            "Winamac" => Some(Self::Winamac),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Indianapolis => "Indianapolis",
            Self::Knox => "Knox",
            Self::Marengo => "Marengo",
            Self::Petersburg => "Petersburg",
            Self::TellCity => "Tell_City",
            Self::Vevay => "Vevay",
            Self::Vincennes => "Vincennes",
            Self::Winamac => "Winamac",
        }
    }
}
