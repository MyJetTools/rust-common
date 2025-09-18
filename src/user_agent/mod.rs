mod detect_device_type;
pub use detect_device_type::*;
mod detect_platform_brand;
pub use detect_platform_brand::*;
mod detect_browser;
pub use detect_browser::*;
use rust_extensions::ShortString;

pub enum UserAgentString {
    AsString(String),
    AsShortString(ShortString),
}

impl UserAgentString {
    pub fn new(src: &str) -> Self {
        match ShortString::from_str_convert_to_lower_case(src) {
            Some(value) => Self::AsShortString(value),
            None => Self::AsString(src.to_lowercase()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            UserAgentString::AsString(value) => value.as_str(),
            UserAgentString::AsShortString(value) => value.as_str(),
        }
    }

    pub fn get_browser(&self) -> Option<Browser> {
        Browser::from_user_agent(self)
    }

    pub fn get_platform_brand(&self) -> Option<PlatformBrand> {
        PlatformBrand::from_user_agent(self)
    }

    pub fn get_device_type(&self) -> DeviceType {
        DeviceType::from_user_agent(self)
    }
}
