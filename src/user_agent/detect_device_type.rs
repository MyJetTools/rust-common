use serde::*;

const MOBILE: &'static str = "mobile";
const TABLET: &'static str = "tablet";
const DESKTOP: &'static str = "desktop";
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
}

impl DeviceType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            MOBILE => Self::Mobile.into(),
            TABLET => Self::Tablet.into(),
            DESKTOP => Self::Desktop.into(),
            _ => None,
        }
    }
    pub fn from_user_agent(ua: &str) -> Self {
        // Mobile detection
        if ua.contains("mobile")
            || ua.contains("android")
            || ua.contains("iphone")
            || ua.contains("ipod")
            || ua.contains("windows phone")
            || ua.contains("blackberry")
        {
            return DeviceType::Mobile;
        }

        // Tablet detection
        if ua.contains("ipad")
            || ua.contains("tablet")
            || ua.contains("kindle")
            || ua.contains("surface")
            || ua.contains("playbook")
        {
            return DeviceType::Tablet;
        }

        // Desktop detection
        if ua.contains("windows nt")
            || ua.contains("macintosh")
            || ua.contains("linux") && !ua.contains("android")
            || ua.contains("x11")
        {
            return DeviceType::Desktop;
        }

        // Default to desktop for unknown or ambiguous User-Agents
        DeviceType::Desktop
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceType::Mobile => MOBILE,
            DeviceType::Tablet => TABLET,
            DeviceType::Desktop => DESKTOP,
        }
    }
}
