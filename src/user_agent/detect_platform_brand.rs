use serde::*;

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
    pub fn from_user_agent(src: &str) -> Option<Self> {
        if src.contains("android") {
            return Some(PlatformBrand::Android);
        }

        if src.contains("windows") || src.contains("win") {
            return Some(PlatformBrand::Windows);
        }

        if src.contains("macintosh")
            || src.contains("apple")
            || src.contains("mac os x")
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
