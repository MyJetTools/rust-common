#[derive(Debug, Clone, Copy)]
pub enum IanaUS {
    Alaska,
    Aleutian,
    Arizona,
    Central,
    EastIndiana,
    Eastern,
    Hawaii,
    IndianaStarke,
    Michigan,
    Mountain,
    Pacific,
    Samoa,
}

impl IanaUS {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Alaska" => Some(Self::Alaska),
            "Aleutian" => Some(Self::Aleutian),
            "Arizona" => Some(Self::Arizona),
            "Central" => Some(Self::Central),
            "East-Indiana" => Some(Self::EastIndiana),
            "Eastern" => Some(Self::Eastern),
            "Hawaii" => Some(Self::Hawaii),
            "Indiana-Starke" => Some(Self::IndianaStarke),
            "Michigan" => Some(Self::Michigan),
            "Mountain" => Some(Self::Mountain),
            "Pacific" => Some(Self::Pacific),
            "Samoa" => Some(Self::Samoa),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alaska => "Alaska",
            Self::Aleutian => "Aleutian",
            Self::Arizona => "Arizona",
            Self::Central => "Central",
            Self::EastIndiana => "East-Indiana",
            Self::Eastern => "Eastern",
            Self::Hawaii => "Hawaii",
            Self::IndianaStarke => "Indiana-Starke",
            Self::Michigan => "Michigan",
            Self::Mountain => "Mountain",
            Self::Pacific => "Pacific",
            Self::Samoa => "Samoa",
        }
    }
}
