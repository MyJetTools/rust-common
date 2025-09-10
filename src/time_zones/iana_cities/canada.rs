#[derive(Debug, Clone, Copy)]
pub enum IanaCanada {
    Atlantic,
    Central,
    Eastern,
    Mountain,
    Newfoundland,
    Pacific,
    Saskatchewan,
    Yukon,
}

impl IanaCanada {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Atlantic" => Some(Self::Atlantic),
            "Central" => Some(Self::Central),
            "Eastern" => Some(Self::Eastern),
            "Mountain" => Some(Self::Mountain),
            "Newfoundland" => Some(Self::Newfoundland),
            "Pacific" => Some(Self::Pacific),
            "Saskatchewan" => Some(Self::Saskatchewan),
            "Yukon" => Some(Self::Yukon),

            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Atlantic => "Atlantic",
            Self::Central => "Central",
            Self::Eastern => "Eastern",
            Self::Mountain => "Mountain",
            Self::Newfoundland => "Newfoundland",
            Self::Pacific => "Pacific",
            Self::Saskatchewan => "Saskatchewan",
            Self::Yukon => "Yukon",
        }
    }
}
