#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IanaIndian {
    Antananarivo,
    Chagos,
    Christmas,
    Cocos,
    Comoro,
    Kerguelen,
    Mahe,
    Maldives,
    Mauritius,
    Mayotte,
    Reunion,
}

impl IanaIndian {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Antananarivo" => Some(Self::Antananarivo),
            "Chagos" => Some(Self::Chagos),
            "Christmas" => Some(Self::Christmas),
            "Cocos" => Some(Self::Cocos),
            "Comoro" => Some(Self::Comoro),
            "Kerguelen" => Some(Self::Kerguelen),
            "Mahe" => Some(Self::Mahe),
            "Maldives" => Some(Self::Maldives),
            "Mauritius" => Some(Self::Mauritius),
            "Mayotte" => Some(Self::Mayotte),
            "Reunion" => Some(Self::Reunion),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Antananarivo => "Antananarivo",
            Self::Chagos => "Chagos",
            Self::Christmas => "Christmas",
            Self::Cocos => "Cocos",
            Self::Comoro => "Comoro",
            Self::Kerguelen => "Kerguelen",
            Self::Mahe => "Mahe",
            Self::Maldives => "Maldives",
            Self::Mauritius => "Mauritius",
            Self::Mayotte => "Mayotte",
            Self::Reunion => "Reunion",
        }
    }
}
