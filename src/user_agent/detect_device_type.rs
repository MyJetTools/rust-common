use serde::*;

use crate::user_agent::UserAgentString;

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
    pub fn from_user_agent(user_agent: &UserAgentString) -> Self {
        // Mobile detection
        if user_agent.as_str().contains("mobile")
            || user_agent.as_str().contains("android")
            || user_agent.as_str().contains("iphone")
            || user_agent.as_str().contains("ipod")
            || user_agent.as_str().contains("windows phone")
            || user_agent.as_str().contains("blackberry")
        {
            return DeviceType::Mobile;
        }

        // Tablet detection
        if user_agent.as_str().contains("ipad")
            || user_agent.as_str().contains("tablet")
            || user_agent.as_str().contains("kindle")
            || user_agent.as_str().contains("surface")
            || user_agent.as_str().contains("playbook")
        {
            return DeviceType::Tablet;
        }

        // Desktop detection
        if user_agent.as_str().contains("windows nt")
            || user_agent.as_str().contains("macintosh")
            || user_agent.as_str().contains("linux") && !user_agent.as_str().contains("android")
            || user_agent.as_str().contains("x11")
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
