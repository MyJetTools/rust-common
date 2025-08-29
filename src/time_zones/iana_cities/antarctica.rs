#[derive(Debug, Clone, Copy)]
pub enum IanaAntarctica {
    McMurdo,
}

impl IanaAntarctica {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "McMurdo" => Some(Self::McMurdo),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::McMurdo => "McMurdo",
        }
    }
}
