use rust_extensions::{date_time::DateTimeAsMicroseconds, StrOrString};

use crate::{country_code::CountryCode, time_zones::TimeZone};

#[derive(Debug, Clone, Copy)]
pub enum NamedTimeZone {
    EEST, // UTC+03:00, Eastern European Summer Time
    EET,  // UTC+02:00, Eastern European Time

    CEST, // UTC+02:00, Central European Summer Time
    CET,  // UTC+01:00, Central European Time

    IDLW, // UTC-12:00, International Date Line West (no DST)
    SST,  // UTC-11:00, Samoa Standard Time (no DST)
    HST,  // UTC-10:00, Hawaii Standard Time (no DST)

    AKDT, // UTC-09:00, Alaska Daylight Time
    AKST, // UTC-08:00, Alaska Standard Time

    PDT, // UTC-07:00, Pacific Daylight Time
    PST, // UTC-08:00, Pacific Standard Time

    MDT, // UTC-07:00, Mountain Daylight Time
    MST, // UTC-07:00, Mountain Standard Time

    CDT, // UTC-06:00, Central Daylight Time
    CST, // UTC-06:00, Central Standard Time

    EDT, // UTC-05:00, Eastern Daylight Time
    EST, // UTC-05:00, Eastern Standard Time

    ADT, // UTC-04:00, Atlantic Daylight Time
    AST, // UTC-04:00, Atlantic Standard Time

    NDT, // UTC-03:30, Newfoundland Daylight Time
    NST, // UTC-03:30, Newfoundland Standard Time
    ART, // UTC-03:00, Argentina Time (no DST)
    FNT, // UTC-02:00, Fernando de Noronha Time (no DST)

    AZOST, // UTC+00:00, Azores Summer Time
    AZOT,  // UTC-01:00, Azores Standard Time

    GMT, // UTC+00:00, Greenwich Mean Time (no DST)

    WEST, // UTC+01:00, Western European Summer Time
    WET,  // UTC+00:00, Western European Time

    AFT,  // UTC+03:30, Afghanistan Time (no DST)
    GST,  // UTC+04:00, Gulf Standard Time (no DST)
    IRST, // UTC+04:30, Iran Standard Time (no DST)
    PKT,  // UTC+05:00, Pakistan Standard Time (no DST)
    IST,  // UTC+05:30, India Standard Time (no DST)
    NPT,  // UTC+05:45, Nepal Time (no DST)
    BST,  // UTC+06:00, Bangladesh Standard Time (no DST)
    MMT,  // UTC+06:30, Myanmar Time (no DST)
    ICT,  // UTC+07:00, Indochina Time (no DST)
    SGT,  // UTC+08:00, Singapore Time (no DST)
    JST,  // UTC+09:00, Japan Standard Time (no DST)

    ACDT, // UTC+10:30, Australian Central Daylight Time
    ACST, // UTC+09:30, Australian Central Standard Time

    AEDT, // UTC+11:00, Australian Eastern Daylight Time
    AEST, // UTC+10:00, Australian Eastern Standard Time

    SBT, // UTC+11:00, Solomon Islands Time (no DST)

    NZDT, // UTC+13:00, New Zealand Daylight Time
    NZST, // UTC+12:00, New Zealand Standard Time

    TOT, // UTC+13:00, Tonga Time (no DST)

    Unknown(i32),
}

impl NamedTimeZone {
    pub fn from_client_time(
        client_time: DateTimeAsMicroseconds,
        mut time_zone: TimeZone,
        country_code: CountryCode,
    ) -> Self {
        let day_saving_time = super::is_summer_time::is_day_saving_time(client_time, country_code);

        time_zone.set_day_saving_time(day_saving_time);

        Self::create(time_zone, country_code)
    }

    pub fn create(time_zone: TimeZone, country_code: CountryCode) -> Self {
        let seconds = time_zone.as_seconds();
        let is_day_saving_time = time_zone.get_day_saving_time();
        match (seconds, is_day_saving_time) {
            // Other common timezones
            (-43200, _) => Self::IDLW,     // UTC-12
            (-39600, _) => Self::SST,      // UTC-11
            (-36000, _) => Self::HST,      // UTC-10
            (-28800, true) => Self::AKDT,  // UTC-8:00 summer (Alaska Daylight Time)
            (-32400, false) => Self::AKST, // UTC-9:00 standard (Alaska Standard Time)
            (-25200, true) => Self::PDT,   // UTC-7:00 summer (Pacific Daylight Time)
            (-28800, false) => Self::PST,  // UTC-8:00 standard (Pacific Standard Time)
            (-21600, true) => Self::MDT,   // UTC-6:00 summer (Mountain Daylight Time)
            (-25200, false) => Self::MST,  // UTC-7:00 standard (Mountain Standard Time)
            (-18000, true) => Self::CDT,   // UTC-5:00 summer (Central Daylight Time)
            (-21600, false) => Self::CST,  // UTC-6:00 standard (Central Standard Time)
            (-14400, true) => Self::EDT,   // UTC-4:00 summer (Eastern Daylight Time)
            (-18000, false) => Self::EST,  // UTC-5:00 standard (Eastern Standard Time)
            (-10800, true) => Self::ADT,   // UTC-3:00 summer (Atlantic Daylight Time)
            (-14400, false) => Self::AST,  // UTC-4:00 standard (Atlantic Standard Time)
            (-9000, true) => Self::NDT,    // UTC-2:30 summer (Newfoundland Daylight Time)
            (-12600, false) => Self::NST,  // UTC-3:30 standard (Newfoundland Standard Time)
            (-10800, _) => Self::ART,      // UTC-3
            (-7200, _) => Self::FNT,       // UTC-2

            // Azores
            (0, true) => Self::AZOST,     // UTC+0 summer
            (-3600, false) => Self::AZOT, // UTC-1 standard

            // Europe
            (0, false) => Self::WET,    // UTC+0 standard
            (3600, true) => Self::WEST, // UTC+1 summer

            (7200, true) => Self::CEST, // UTC+2 summer
            (3600, false) => Self::CET, // UTC+1 standard

            (10800, true) => Self::EEST, // UTC+3 summer
            (7200, false) => Self::EET,  // UTC+2 standard

            // Afghanistan
            (12600, _) => Self::AFT,  // UTC+3:30
            (14400, _) => Self::GST,  // UTC+4
            (16200, _) => Self::IRST, // UTC+4:30
            (18000, _) => Self::PKT,  // UTC+5
            (19800, _) => Self::IST,  // UTC+5:30
            (20700, _) => Self::NPT,  // UTC+5:45
            (21600, _) => Self::BST,  // UTC+6
            (23400, _) => Self::MMT,  // UTC+6:30
            (25200, _) => Self::ICT,  // UTC+7
            (28800, _) => Self::SGT,  // UTC+8
            (32400, _) => Self::JST,  // UTC+9

            // Australia
            (37800, true) => Self::ACDT,  // UTC+10:30 summer
            (34200, false) => Self::ACST, // UTC+9:30 standard
            (39600, _) => match country_code {
                CountryCode::SLB
                | CountryCode::VUT
                | CountryCode::NCL
                | CountryCode::FSM
                | CountryCode::PNG => Self::SBT,
                _ => match is_day_saving_time {
                    true => Self::AEDT,
                    false => Self::AEST,
                },
            },

            // New Zealand
            (46800, true) => Self::NZDT,  // UTC+13 summer
            (43200, false) => Self::NZST, // UTC+12 standard

            // Tonga
            (46800, _) => Self::TOT, // UTC+13

            _ => Self::Unknown(seconds),
        }
    }

    pub fn as_str(&self) -> StrOrString<'static> {
        match self {
            NamedTimeZone::EEST => "EEST".into(),
            NamedTimeZone::EET => "EET".into(),
            NamedTimeZone::CEST => "CEST".into(),
            NamedTimeZone::CET => "CET".into(),
            NamedTimeZone::IDLW => "IDLW".into(),
            NamedTimeZone::SST => "SST".into(),
            NamedTimeZone::HST => "HST".into(),
            NamedTimeZone::AKDT => "AKDT".into(),
            NamedTimeZone::AKST => "AKST".into(),
            NamedTimeZone::PDT => "PDT".into(),
            NamedTimeZone::PST => "PST".into(),
            NamedTimeZone::MDT => "MDT".into(),
            NamedTimeZone::MST => "MST".into(),
            NamedTimeZone::CDT => "CDT".into(),
            NamedTimeZone::CST => "CST".into(),
            NamedTimeZone::EDT => "EDT".into(),
            NamedTimeZone::EST => "EST".into(),
            NamedTimeZone::ADT => "ADT".into(),
            NamedTimeZone::AST => "AST".into(),
            NamedTimeZone::NDT => "NDT".into(),
            NamedTimeZone::NST => "NST".into(),
            NamedTimeZone::ART => "ART".into(),
            NamedTimeZone::FNT => "FNT".into(),
            NamedTimeZone::AZOST => "AZOST".into(),
            NamedTimeZone::AZOT => "AZOT".into(),
            NamedTimeZone::GMT => "GMT".into(),
            NamedTimeZone::WEST => "WEST".into(),
            NamedTimeZone::WET => "WET".into(),
            NamedTimeZone::AFT => "AFT".into(),
            NamedTimeZone::GST => "GST".into(),
            NamedTimeZone::IRST => "IRST".into(),
            NamedTimeZone::PKT => "PKT".into(),
            NamedTimeZone::IST => "IST".into(),
            NamedTimeZone::NPT => "NPT".into(),
            NamedTimeZone::BST => "BST".into(),
            NamedTimeZone::MMT => "MMT".into(),
            NamedTimeZone::ICT => "ICT".into(),
            NamedTimeZone::SGT => "SGT".into(),
            NamedTimeZone::JST => "JST".into(),
            NamedTimeZone::ACDT => "ACDT".into(),
            NamedTimeZone::ACST => "ACST".into(),
            NamedTimeZone::AEDT => "AEDT".into(),
            NamedTimeZone::AEST => "AEST".into(),
            NamedTimeZone::SBT => "SBT".into(),
            NamedTimeZone::NZDT => "NZDT".into(),
            NamedTimeZone::NZST => "NZST".into(),
            NamedTimeZone::TOT => "TOT".into(),
            NamedTimeZone::Unknown(seconds) => super::utils::seconds_to_string(*seconds).into(),
        }
    }

    pub fn as_full_name(&self) -> StrOrString<'static> {
        match self {
            NamedTimeZone::EEST => "Eastern European Summer Time".into(),
            NamedTimeZone::EET => "Eastern European Time".into(),
            NamedTimeZone::CEST => "Central European Summer Time".into(),
            NamedTimeZone::CET => "Central European Time".into(),
            NamedTimeZone::IDLW => "International Date Line West".into(),
            NamedTimeZone::SST => "Samoa Standard Time".into(),
            NamedTimeZone::HST => "Hawaii Standard Time".into(),
            NamedTimeZone::AKDT => "Alaska Daylight Time".into(),
            NamedTimeZone::AKST => "Alaska Standard Time".into(),
            NamedTimeZone::PDT => "Pacific Daylight Time".into(),
            NamedTimeZone::PST => "Pacific Standard Time".into(),
            NamedTimeZone::MDT => "Mountain Daylight Time".into(),
            NamedTimeZone::MST => "Mountain Standard Time".into(),
            NamedTimeZone::CDT => "Central Daylight Time".into(),
            NamedTimeZone::CST => "Central Standard Time".into(),
            NamedTimeZone::EDT => "Eastern Daylight Time".into(),
            NamedTimeZone::EST => "Eastern Standard Time".into(),
            NamedTimeZone::ADT => "Atlantic Daylight Time".into(),
            NamedTimeZone::AST => "Atlantic Standard Time".into(),
            NamedTimeZone::NDT => "Newfoundland Daylight Time".into(),
            NamedTimeZone::NST => "Newfoundland Standard Time".into(),
            NamedTimeZone::ART => "Argentina Time".into(),
            NamedTimeZone::FNT => "Fernando de Noronha Time".into(),
            NamedTimeZone::AZOST => "Azores Summer Time".into(),
            NamedTimeZone::AZOT => "Azores Standard Time".into(),
            NamedTimeZone::GMT => "Greenwich Mean Time".into(),
            NamedTimeZone::WEST => "Western European Summer Time".into(),
            NamedTimeZone::WET => "Western European Time".into(),
            NamedTimeZone::AFT => "Afghanistan Time".into(),
            NamedTimeZone::GST => "Gulf Standard Time".into(),
            NamedTimeZone::IRST => "Iran Standard Time".into(),
            NamedTimeZone::PKT => "Pakistan Standard Time".into(),
            NamedTimeZone::IST => "India Standard Time".into(),
            NamedTimeZone::NPT => "Nepal Time".into(),
            NamedTimeZone::BST => "Bangladesh Standard Time".into(),
            NamedTimeZone::MMT => "Myanmar Time".into(),
            NamedTimeZone::ICT => "Indochina Time".into(),
            NamedTimeZone::SGT => "Singapore Time".into(),
            NamedTimeZone::JST => "Japan Standard Time".into(),
            NamedTimeZone::ACDT => "Australian Central Daylight Time".into(),
            NamedTimeZone::ACST => "Australian Central Standard Time".into(),
            NamedTimeZone::AEDT => "Australian Eastern Daylight Time".into(),
            NamedTimeZone::AEST => "Australian Eastern Standard Time".into(),
            NamedTimeZone::SBT => "Solomon Islands Time".into(),
            NamedTimeZone::NZDT => "New Zealand Daylight Time".into(),
            NamedTimeZone::NZST => "New Zealand Standard Time".into(),
            NamedTimeZone::TOT => "Tonga Time".into(),
            NamedTimeZone::Unknown(seconds) => super::utils::seconds_to_string(*seconds).into(),
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, NamedTimeZone::Unknown(_))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_europe_summer_time() {
        use rust_extensions::date_time::DateTimeAsMicroseconds;

        let dt = DateTimeAsMicroseconds::create(2025, 08, 10, 2, 0, 0, 0);
        let time_zone = super::TimeZone::try_from_str("UTC+2:00").unwrap();

        let named_time_zone =
            super::NamedTimeZone::from_client_time(dt, time_zone, super::CountryCode::DEU);

        match named_time_zone {
            super::NamedTimeZone::CEST => {}
            _ => panic!("Expected CEST, got {:?}", named_time_zone),
        }
    }

    #[test]
    fn test_europe_winter_time() {
        use rust_extensions::date_time::DateTimeAsMicroseconds;

        let dt = DateTimeAsMicroseconds::create(2025, 02, 10, 2, 0, 0, 0);
        let time_zone = super::TimeZone::try_from_str("UTC+1:00").unwrap();

        let named_time_zone =
            super::NamedTimeZone::from_client_time(dt, time_zone, super::CountryCode::DEU);

        match named_time_zone {
            super::NamedTimeZone::CET => {}
            _ => panic!("Expected CET, got {:?}", named_time_zone),
        }
    }
}
