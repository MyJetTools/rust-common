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

#[cfg(test)]
mod tests {
    use crate::user_agent::*;

    #[test]
    fn test_iphone_chrome() {
        let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 26_0_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/141.0.7390.26 Mobile/15E148 Safari/604.1";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Chrome));
        assert_eq!(user_agent.get_device_type(), DeviceType::Mobile);
        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }
}
