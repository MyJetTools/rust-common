#[derive(Debug, Clone, Copy)]
pub enum IanaArctic {
    Longyearbyen,
}

impl IanaArctic {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Longyearbyen" => Some(Self::Longyearbyen),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Longyearbyen => "Longyearbyen",
        }
    }
}
