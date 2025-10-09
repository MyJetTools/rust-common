#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NorthDakota {
    Beulah,
    Center,
    NewSalem,
}

impl NorthDakota {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Beulah" => Some(Self::Beulah),
            "Center" => Some(Self::Center),
            "New_Salem" => Some(Self::NewSalem),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Beulah => "Beulah",
            Self::Center => "Center",
            Self::NewSalem => "New_Salem",
        }
    }
}
