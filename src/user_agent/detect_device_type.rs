use serde::*;

use crate::user_agent::UserAgentString;

const MOBILE: &'static str = "mobile";
const TABLET: &'static str = "tablet";
const DESKTOP: &'static str = "desktop";
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
}

impl DeviceType {
    pub fn is_desktop(&self) -> bool {
        match self {
            DeviceType::Desktop => true,
            _ => false,
        }
    }

    pub fn is_mobile(&self) -> bool {
        match self {
            DeviceType::Mobile => true,
            _ => false,
        }
    }

    pub fn is_tablet(&self) -> bool {
        match self {
            DeviceType::Tablet => true,
            _ => false,
        }
    }
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            MOBILE => Self::Mobile.into(),
            TABLET => Self::Tablet.into(),
            DESKTOP => Self::Desktop.into(),
            _ => None,
        }
    }
    pub fn from_user_agent(user_agent: &UserAgentString) -> Self {
        let src = user_agent.as_str();

        // Tablet detection has to run first: an iPad carries "Mobile/<build>" and every
        // Android tablet carries "Android", so the mobile rules below would swallow both.
        if src.contains("ipad")
            || src.contains("tablet")
            || src.contains("kindle")
            || src.contains("surface")
            || src.contains("playbook")
        {
            return DeviceType::Tablet;
        }

        // Android phones put "Mobile" into the user agent, Android tablets do not.
        if src.contains("android") {
            if src.contains("mobile") {
                return DeviceType::Mobile;
            }

            return DeviceType::Tablet;
        }

        // Mobile detection
        if src.contains("mobile")
            || src.contains("iphone")
            || src.contains("ipod")
            || src.contains("windows phone")
            || src.contains("blackberry")
        {
            return DeviceType::Mobile;
        }

        // Desktop detection
        if src.contains("windows nt")
            || src.contains("macintosh")
            || src.contains("linux")
            || src.contains("x11")
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
