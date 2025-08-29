#[derive(Debug, Clone, Copy)]
pub enum IanaAntarctica {
    Casey,
    Davis,
    DumontDUrville,
    Macquarie,
    Mawson,
    McMurdo,
    Palmer,
    Rothera,
    Syowa,
    Troll,
    Vostok,
    SouthPole,
}

impl IanaAntarctica {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "Casey" => Some(Self::Casey),
            "Davis" => Some(Self::Davis),
            "DumontDUrville" => Some(Self::DumontDUrville),
            "Macquarie" => Some(Self::Macquarie),
            "Mawson" => Some(Self::Mawson),
            "McMurdo" => Some(Self::McMurdo),
            "Palmer" => Some(Self::Palmer),
            "Rothera" => Some(Self::Rothera),
            "Syowa" => Some(Self::Syowa),
            "Troll" => Some(Self::Troll),
            "Vostok" => Some(Self::Vostok),
            "South_Pole" => Some(Self::SouthPole),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Casey => "Casey",
            Self::Davis => "Davis",
            Self::DumontDUrville => "DumontDUrville",
            Self::Macquarie => "Macquarie",
            Self::Mawson => "Mawson",
            Self::McMurdo => "McMurdo",
            Self::Palmer => "Palmer",
            Self::Rothera => "Rothera",
            Self::Syowa => "Syowa",
            Self::Troll => "Troll",
            Self::Vostok => "Vostok",
            Self::SouthPole => "South_Pole",
        }
    }
}
