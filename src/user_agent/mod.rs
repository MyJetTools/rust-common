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

    #[test]
    fn test_iphone_firefox() {
        let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_7_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/34.0 Mobile/15E148 Safari/605.1.15";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Firefox));
        assert_eq!(user_agent.get_device_type(), DeviceType::Mobile);
        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }

    #[test]
    fn test_iphone_edge() {
        let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_7_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 EdgiOS/46.3.13 Mobile/15E148 Safari/605.1.15";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Edge));
        assert_eq!(user_agent.get_device_type(), DeviceType::Mobile);
        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }

    #[test]
    fn test_iphone_opera() {
        let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) OPT/4.4.0 Mobile/15E148 Safari/605.1.15";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Opera));
        assert_eq!(user_agent.get_device_type(), DeviceType::Mobile);
        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }

    #[test]
    fn test_ipad_is_a_tablet() {
        // An iPad user agent carries "Mobile/<build>" - it must not be taken for a phone
        let user_agent = "Mozilla/5.0 (iPad; CPU OS 15_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.8 Mobile/15E148 Safari/604.1";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Safari));
        assert_eq!(user_agent.get_device_type(), DeviceType::Tablet);
        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }

    #[test]
    fn test_android_tablet_has_no_mobile_token() {
        let user_agent = "Mozilla/5.0 (Linux; Android 10; SM-T860) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Chrome));
        assert_eq!(user_agent.get_device_type(), DeviceType::Tablet);
        assert_eq!(
            user_agent.get_platform_brand(),
            Some(PlatformBrand::Android)
        );
    }

    #[test]
    fn test_apple_native_app_is_not_windows() {
        // CFNetwork sends "Darwin/<version>", which used to be matched as "win"
        let user_agent = "MyApp/1.0 CFNetwork/1333.0.4 Darwin/21.6.0";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }

    #[test]
    fn test_01() {
        let user_agent = "Mozilla/5.0 (Linux; Android 8.0; Pixel 2 Build/OPD3.170816.012) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.5431.1356 Mobile Safari/537.36";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Chrome));
        assert_eq!(user_agent.get_device_type(), DeviceType::Mobile);
        assert_eq!(
            user_agent.get_platform_brand(),
            Some(PlatformBrand::Android)
        );
    }

    #[test]
    fn test_user_agent_02() {
        let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 15_8_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Safari));
        assert_eq!(user_agent.get_device_type(), DeviceType::Mobile);
        assert_eq!(user_agent.get_platform_brand(), Some(PlatformBrand::Apple));
    }

    #[test]
    fn test_user_agent_03() {
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36";
        let user_agent = UserAgentString::new(user_agent);

        assert_eq!(user_agent.get_browser(), Some(Browser::Chrome));
        assert_eq!(user_agent.get_device_type(), DeviceType::Desktop);
        assert_eq!(
            user_agent.get_platform_brand(),
            Some(PlatformBrand::Windows)
        );
    }
}
