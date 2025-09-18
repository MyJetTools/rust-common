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
}
