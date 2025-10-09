#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IanaArgentina {
    BuenosAires,
    Catamarca,
    ComodRivadavia,
    Cordoba,
    Jujuy,
    LaRioja,
    Mendoza,
    RioGallegos,
    Salta,
    SanJuan,
    SanLuis,
    Tucuman,
    Ushuaia,
}

impl IanaArgentina {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Buenos_Aires" => Some(Self::BuenosAires),
            "Catamarca" => Some(Self::Catamarca),
            "ComodRivadavia" => Some(Self::ComodRivadavia),
            "Cordoba" => Some(Self::Cordoba),
            "Jujuy" => Some(Self::Jujuy),
            "La_Rioja" => Some(Self::LaRioja),
            "Mendoza" => Some(Self::Mendoza),
            "Rio_Gallegos" => Some(Self::RioGallegos),
            "Salta" => Some(Self::Salta),
            "San_Juan" => Some(Self::SanJuan),
            "San_Luis" => Some(Self::SanLuis),
            "Tucuman" => Some(Self::Tucuman),
            "Ushuaia" => Some(Self::Ushuaia),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::BuenosAires => "Buenos_Aires",
            Self::Catamarca => "Catamarca",
            Self::ComodRivadavia => "ComodRivadavia",
            Self::Cordoba => "Cordoba",
            Self::Jujuy => "Jujuy",
            Self::LaRioja => "La_Rioja",
            Self::Mendoza => "Mendoza",
            Self::RioGallegos => "Rio_Gallegos",
            Self::Salta => "Salta",
            Self::SanJuan => "San_Juan",
            Self::SanLuis => "San_Luis",
            Self::Tucuman => "Tucuman",
            Self::Ushuaia => "Ushuaia",
        }
    }
}
