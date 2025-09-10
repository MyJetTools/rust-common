#[derive(Debug, Clone, Copy)]
pub enum IanaAustralia {
    Adelaide,
    Brisbane,
    BrokenHill,
    Canberra,
    Currie,
    Darwin,
    Eucla,

    Hobart,
    Lindeman,
    LordHowe,
    Melbourne,
    Perth,
    Sydney,
    LHI,
    North,
    NSW,
    Queensland,
    South,
    Tasmania,
    Victoria,
    West,
    Yancowinna,
}

impl IanaAustralia {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Adelaide" => Some(Self::Adelaide),
            "Brisbane" => Some(Self::Brisbane),
            "Darwin" => Some(Self::Darwin),
            "Eucla" => Some(Self::Eucla),
            "Hobart" => Some(Self::Hobart),
            "Lindeman" => Some(Self::Lindeman),
            "Lord_Howe" => Some(Self::LordHowe),
            "Melbourne" => Some(Self::Melbourne),
            "Perth" => Some(Self::Perth),
            "Sydney" => Some(Self::Sydney),
            "Broken_Hill" => Some(Self::BrokenHill),
            "Canberra" => Some(Self::Canberra),
            "Currie" => Some(Self::Currie),
            "LHI" => Some(Self::LHI),
            "North" => Some(Self::North),
            "NSW" => Some(Self::NSW),
            "Queensland" => Some(Self::Queensland),
            "South" => Some(Self::South),
            "Tasmania" => Some(Self::Tasmania),
            "Victoria" => Some(Self::Victoria),
            "West" => Some(Self::West),
            "Yancowinna" => Some(Self::Yancowinna),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Adelaide => "Adelaide",
            Self::Brisbane => "Brisbane",
            Self::BrokenHill => "Broken_Hill",
            Self::Canberra => "Canberra",
            Self::Currie => "Currie",
            Self::Darwin => "Darwin",
            Self::Eucla => "Eucla",
            Self::Hobart => "Hobart",
            Self::Lindeman => "Lindeman",
            Self::LordHowe => "Lord_Howe",
            Self::LHI => "LHI",
            Self::Melbourne => "Melbourne",
            Self::North => "North",
            Self::NSW => "NSW",
            Self::Perth => "Perth",
            Self::Queensland => "Queensland",
            Self::South => "South",
            Self::Sydney => "Sydney",
            Self::Tasmania => "Tasmania",
            Self::Victoria => "Victoria",
            Self::West => "West",
            Self::Yancowinna => "Yancowinna",
        }
    }
}
