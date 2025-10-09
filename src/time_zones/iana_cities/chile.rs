#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IanaChile {
    Continental,
    EasterIsland,
}

impl IanaChile {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Continental" => Some(Self::Continental),
            "EasterIsland" => Some(Self::EasterIsland),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Continental => "Continental",
            Self::EasterIsland => "EasterIsland",
        }
    }
}
