use serde::*;

use crate::user_agent::UserAgentString;

pub const CHROME: &'static str = "chrome";
pub const FIREFOX: &'static str = "firefox";
pub const OPERA: &'static str = "opera";
pub const SAFARI: &'static str = "safari";
pub const EDGE: &'static str = "edge";
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Browser {
    Chrome,
    Firefox,
    Opera,
    Safari,
    Edge,
}
impl Browser {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            OPERA => Self::Opera.into(),
            CHROME => Self::Chrome.into(),
            FIREFOX => Self::Firefox.into(),
            SAFARI => Self::Safari.into(),
            EDGE => Self::Edge.into(),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Browser::Chrome => CHROME,
            Browser::Firefox => FIREFOX,
            Browser::Opera => OPERA,
            Browser::Safari => SAFARI,
            Browser::Edge => EDGE,
        }
    }

    pub fn from_user_agent(user_agent: &UserAgentString) -> Option<Self> {
        if user_agent.as_str().contains("edg/") {
            return Self::Edge.into();
        }
        if user_agent.as_str().contains("opr/") || user_agent.as_str().contains("opera") {
            return Self::Opera.into();
        }
        if user_agent.as_str().contains("firefox") {
            return Self::Firefox.into();
        }

        if user_agent.as_str().contains("chrome") {
            return Self::Chrome.into();
        }
        if user_agent.as_str().contains("safari") {
            return Self::Safari.into();
        }

        None
    }
}
