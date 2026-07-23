use serde::*;

use crate::user_agent::UserAgentString;

const APPLE: &'static str = "apple";
const WINDOWS: &'static str = "windows";
const ANDROID: &'static str = "android";
const LINUX: &'static str = "linux";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlatformBrand {
    Apple,
    Windows,
    Android,
    Linux,
}

impl PlatformBrand {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            APPLE => Self::Apple.into(),
            WINDOWS => Self::Windows.into(),
            ANDROID => Self::Android.into(),
            LINUX => Self::Linux.into(),
            _ => None,
        }
    }
    pub fn from_user_agent(user_agent: &UserAgentString) -> Option<Self> {
        let src = user_agent.as_str();

        if src.contains("android") {
            return Some(PlatformBrand::Android);
        }

        // "win" on its own also matches "Darwin", which every native Apple app sends
        // through CFNetwork, so only the unambiguous Windows tokens are matched here.
        if src.contains("windows") || src.contains("win64") || src.contains("win32") {
            return Some(PlatformBrand::Windows);
        }

        if src.contains("macintosh")
            || src.contains("mac os x")
            || src.contains("darwin")
            || src.contains("iphone")
            || src.contains("ipod")
            || src.contains("ipad")
        {
            return Some(PlatformBrand::Apple);
        }

        if src.contains("linux") || src.contains("x11") {
            return Some(PlatformBrand::Linux);
        }

        None
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PlatformBrand::Apple => APPLE,
            PlatformBrand::Windows => WINDOWS,
            PlatformBrand::Android => ANDROID,
            PlatformBrand::Linux => LINUX,
        }
    }
}
