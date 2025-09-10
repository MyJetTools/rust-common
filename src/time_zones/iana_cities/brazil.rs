#[derive(Debug, Clone, Copy)]
pub enum IanaBrazil {
    Acre,
    DeNoronha,
    East,
    West,
}

impl IanaBrazil {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Acre" => Some(Self::Acre),
            "DeNoronha" => Some(Self::DeNoronha),
            "East" => Some(Self::East),
            "West" => Some(Self::West),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Acre => "Acre",
            Self::DeNoronha => "DeNoronha",
            Self::East => "East",
            Self::West => "West",
        }
    }
}
