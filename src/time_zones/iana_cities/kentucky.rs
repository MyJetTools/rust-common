#[derive(Debug, Clone, Copy)]
pub enum IanaKentucky {
    Louisville,
    Monticello,
}

impl IanaKentucky {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Louisville" => Some(Self::Louisville),
            "Monticello" => Some(Self::Monticello),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Louisville => "Louisville",
            Self::Monticello => "Monticello",
        }
    }
}
