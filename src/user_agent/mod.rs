mod detect_device_type;
pub use detect_device_type::*;
mod detect_platform_brand;
pub use detect_platform_brand::*;
mod detect_browser;
pub use detect_browser::*;

pub struct UserAgentString(String);

impl UserAgentString {
    pub fn new(src: &str) -> Self {
        Self(src.to_lowercase())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
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
