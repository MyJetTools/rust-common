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
        let src = user_agent.as_str();

        // On iOS every browser renders through WebKit and appends "Safari/..." to its
        // user agent, so the vendor token has to be matched before anything else.
        if src.contains("crios/") {
            return Self::Chrome.into();
        }
        if src.contains("fxios/") {
            return Self::Firefox.into();
        }
        if src.contains("edgios/") {
            return Self::Edge.into();
        }
        if src.contains("opios/") {
            return Self::Opera.into();
        }

        if src.contains("edg/") {
            return Self::Edge.into();
        }
        if src.contains("opr/") || src.contains("opera") {
            return Self::Opera.into();
        }
        if src.contains("firefox") {
            return Self::Firefox.into();
        }

        if src.contains("chrome") {
            return Self::Chrome.into();
        }
        if src.contains("safari") {
            return Self::Safari.into();
        }

        // iOS web views drop the "Safari/..." token but still carry "Mobile/<build>"
        if src.contains("mobile/") && src.contains("iphone") {
            return Self::Safari.into();
        }

        None
    }
}
