use serde::*;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlatformBrand {
    Apple,
    Windows,
    Android,
    Linux,
}

impl PlatformBrand {
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
            PlatformBrand::Apple => "apple",
            PlatformBrand::Windows => "windows",
            PlatformBrand::Android => "android",
            PlatformBrand::Linux => "linux",
        }
    }
}
