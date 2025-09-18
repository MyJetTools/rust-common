use serde::*;

use crate::user_agent::UserAgentString;

const APPLE: &'static str = "apple";
const WINDOWS: &'static str = "windows";
const ANDROID: &'static str = "android";
const LINUX: &'static str = "linux";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
        if user_agent.as_str().contains("android") {
            return Some(PlatformBrand::Android);
        }

        if user_agent.as_str().contains("windows") || user_agent.as_str().contains("win") {
            return Some(PlatformBrand::Windows);
        }

        if user_agent.as_str().contains("macintosh")
            || user_agent.as_str().contains("apple")
            || user_agent.as_str().contains("mac os x")
            || user_agent.as_str().contains("iphone")
            || user_agent.as_str().contains("ipod")
            || user_agent.as_str().contains("ipad")
        {
            return Some(PlatformBrand::Apple);
        }

        if user_agent.as_str().contains("linux") || user_agent.as_str().contains("x11") {
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
