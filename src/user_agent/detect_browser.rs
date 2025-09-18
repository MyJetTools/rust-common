use serde::*;

pub const CHROME: &'static str = "chrome";
pub const FIREFOX: &'static str = "firefox";
pub const OPERA: &'static str = "opera";
pub const SAFARI: &'static str = "safari";
pub const EDGE: &'static str = "edge";
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Browser {
    Unknown,
    Chrome,
    Firefox,
    Opera,
    Safari,
    Edge,
}
impl Browser {
    pub fn from_str(src: &str) -> Self {
        match src {
            OPERA => Self::Opera,
            CHROME => Self::Chrome,
            FIREFOX => Self::Firefox,
            SAFARI => Self::Safari,
            EDGE => Self::Edge,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Browser::Unknown => None,
            Browser::Chrome => Some(CHROME),
            Browser::Firefox => Some(FIREFOX),
            Browser::Opera => Some(OPERA),
            Browser::Safari => Some(SAFARI),
            Browser::Edge => Some(EDGE),
        }
    }

    pub fn from_user_agent(user_agent_lower_case: &str) -> Self {
        if user_agent_lower_case.contains("edg/") {
            return Self::Edge;
        }
        if user_agent_lower_case.contains("opr/") || user_agent_lower_case.contains("opera") {
            return Self::Opera;
        }
        if user_agent_lower_case.contains("firefox") {
            return Self::Firefox;
        }

        if user_agent_lower_case.contains("chrome") {
            return Self::Chrome;
        }
        if user_agent_lower_case.contains("safari") {
            return Self::Safari;
        }

        Self::Unknown
    }
}
