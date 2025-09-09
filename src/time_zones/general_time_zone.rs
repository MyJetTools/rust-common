use rust_extensions::{date_time::DateTimeAsMicroseconds, StrOrString};

use crate::{country_code::CountryCode, time_zones::TimeZoneGmtOffset};

#[derive(Debug, Clone, Copy)]
pub enum GeneralTimeZone {
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

impl GeneralTimeZone {
    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            "EEST" => Some(Self::EEST),
            "EET" => Some(Self::EET),
            "CEST" => Some(Self::CEST),
            "CET" => Some(Self::CET),
            "IDLW" => Some(Self::IDLW),
            "SST" => Some(Self::SST),
            "HST" => Some(Self::HST),
            "AKDT" => Some(Self::AKDT),
            "AKST" => Some(Self::AKST),
            "PDT" => Some(Self::PDT),
            "PST" => Some(Self::PST),
            "MDT" => Some(Self::MDT),
            "MST" => Some(Self::MST),
            "CDT" => Some(Self::CDT),
            "CST" => Some(Self::CST),
            "EDT" => Some(Self::EDT),
            "EST" => Some(Self::EST),
            "ADT" => Some(Self::ADT),
            "AST" => Some(Self::AST),
            "NDT" => Some(Self::NDT),
            "NST" => Some(Self::NST),
            "ART" => Some(Self::ART),
            "FNT" => Some(Self::FNT),
            "AZOST" => Some(Self::AZOST),
            "AZOT" => Some(Self::AZOT),
            "GMT" => Some(Self::GMT),
            "WEST" => Some(Self::WEST),
            "WET" => Some(Self::WET),
            "AFT" => Some(Self::AFT),
            "GST" => Some(Self::GST),
            "IRST" => Some(Self::IRST),
            "PKT" => Some(Self::PKT),
            "IST" => Some(Self::IST),
            "NPT" => Some(Self::NPT),
            "BST" => Some(Self::BST),
            "MMT" => Some(Self::MMT),
            "ICT" => Some(Self::ICT),
            "SGT" => Some(Self::SGT),
            "JST" => Some(Self::JST),
            "ACDT" => Some(Self::ACDT),
            "ACST" => Some(Self::ACST),
            "AEDT" => Some(Self::AEDT),
            "AEST" => Some(Self::AEST),
            "SBT" => Some(Self::SBT),
            "NZDT" => Some(Self::NZDT),
            "NZST" => Some(Self::NZST),
            "TOT" => Some(Self::TOT),
            _ => None,
        }
    }

    pub fn from_client_time(
        client_time: DateTimeAsMicroseconds,
        mut time_zone: TimeZoneGmtOffset,
        country_code: CountryCode,
    ) -> Self {
        let day_saving_time = super::is_summer_time::is_day_saving_time(client_time, country_code);

        time_zone.set_day_saving_time(day_saving_time);

        Self::create(time_zone, country_code)
    }

    pub fn create(time_zone: TimeZoneGmtOffset, country_code: CountryCode) -> Self {
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

    pub fn get_offset_in_minutes(&self) -> i32 {
        self.get_offset_in_seconds() / 60
    }

    pub fn get_offset_in_seconds(&self) -> i32 {
        match self {
            GeneralTimeZone::EEST => 10800,
            GeneralTimeZone::EET => 7200,
            GeneralTimeZone::CEST => 7200,
            GeneralTimeZone::CET => 3600,
            GeneralTimeZone::IDLW => -43200,
            GeneralTimeZone::SST => -39600,
            GeneralTimeZone::HST => -36000,
            GeneralTimeZone::AKDT => -28800,
            GeneralTimeZone::AKST => -32400,
            GeneralTimeZone::PDT => -25200,
            GeneralTimeZone::PST => -28800,
            GeneralTimeZone::MDT => -21600,
            GeneralTimeZone::MST => -25200,
            GeneralTimeZone::CDT => -18000,
            GeneralTimeZone::CST => -21600,
            GeneralTimeZone::EDT => -14400,
            GeneralTimeZone::EST => -18000,
            GeneralTimeZone::ADT => -10800,
            GeneralTimeZone::AST => -14400,
            GeneralTimeZone::NDT => -9000,
            GeneralTimeZone::NST => -12600,
            GeneralTimeZone::ART => -10800,
            GeneralTimeZone::FNT => -7200,
            GeneralTimeZone::AZOST => 0,
            GeneralTimeZone::AZOT => -3600,
            GeneralTimeZone::GMT => 0,
            GeneralTimeZone::WEST => 3600,
            GeneralTimeZone::WET => 0,
            GeneralTimeZone::AFT => 12600,
            GeneralTimeZone::GST => 14400,
            GeneralTimeZone::IRST => 16200,
            GeneralTimeZone::PKT => 18000,
            GeneralTimeZone::IST => 19800,
            GeneralTimeZone::NPT => 20700,
            GeneralTimeZone::BST => 21600,
            GeneralTimeZone::MMT => 23400,
            GeneralTimeZone::ICT => 25200,
            GeneralTimeZone::SGT => 28800,
            GeneralTimeZone::JST => 32400,
            GeneralTimeZone::ACDT => 37800,
            GeneralTimeZone::ACST => 34200,
            GeneralTimeZone::AEDT => 39600,
            GeneralTimeZone::AEST => 36000,
            GeneralTimeZone::SBT => 46800,
            GeneralTimeZone::NZDT => 46800,
            GeneralTimeZone::NZST => 43200,
            GeneralTimeZone::TOT => 46800,
            GeneralTimeZone::Unknown(seconds) => *seconds,
        }
    }

    pub fn as_str(&self) -> StrOrString<'static> {
        match self {
            GeneralTimeZone::EEST => "EEST".into(),
            GeneralTimeZone::EET => "EET".into(),
            GeneralTimeZone::CEST => "CEST".into(),
            GeneralTimeZone::CET => "CET".into(),
            GeneralTimeZone::IDLW => "IDLW".into(),
            GeneralTimeZone::SST => "SST".into(),
            GeneralTimeZone::HST => "HST".into(),
            GeneralTimeZone::AKDT => "AKDT".into(),
            GeneralTimeZone::AKST => "AKST".into(),
            GeneralTimeZone::PDT => "PDT".into(),
            GeneralTimeZone::PST => "PST".into(),
            GeneralTimeZone::MDT => "MDT".into(),
            GeneralTimeZone::MST => "MST".into(),
            GeneralTimeZone::CDT => "CDT".into(),
            GeneralTimeZone::CST => "CST".into(),
            GeneralTimeZone::EDT => "EDT".into(),
            GeneralTimeZone::EST => "EST".into(),
            GeneralTimeZone::ADT => "ADT".into(),
            GeneralTimeZone::AST => "AST".into(),
            GeneralTimeZone::NDT => "NDT".into(),
            GeneralTimeZone::NST => "NST".into(),
            GeneralTimeZone::ART => "ART".into(),
            GeneralTimeZone::FNT => "FNT".into(),
            GeneralTimeZone::AZOST => "AZOST".into(),
            GeneralTimeZone::AZOT => "AZOT".into(),
            GeneralTimeZone::GMT => "GMT".into(),
            GeneralTimeZone::WEST => "WEST".into(),
            GeneralTimeZone::WET => "WET".into(),
            GeneralTimeZone::AFT => "AFT".into(),
            GeneralTimeZone::GST => "GST".into(),
            GeneralTimeZone::IRST => "IRST".into(),
            GeneralTimeZone::PKT => "PKT".into(),
            GeneralTimeZone::IST => "IST".into(),
            GeneralTimeZone::NPT => "NPT".into(),
            GeneralTimeZone::BST => "BST".into(),
            GeneralTimeZone::MMT => "MMT".into(),
            GeneralTimeZone::ICT => "ICT".into(),
            GeneralTimeZone::SGT => "SGT".into(),
            GeneralTimeZone::JST => "JST".into(),
            GeneralTimeZone::ACDT => "ACDT".into(),
            GeneralTimeZone::ACST => "ACST".into(),
            GeneralTimeZone::AEDT => "AEDT".into(),
            GeneralTimeZone::AEST => "AEST".into(),
            GeneralTimeZone::SBT => "SBT".into(),
            GeneralTimeZone::NZDT => "NZDT".into(),
            GeneralTimeZone::NZST => "NZST".into(),
            GeneralTimeZone::TOT => "TOT".into(),
            GeneralTimeZone::Unknown(seconds) => {
                super::utils::seconds_to_string(*seconds).to_string().into()
            }
        }
    }

    pub fn as_full_name(&self) -> StrOrString<'static> {
        match self {
            GeneralTimeZone::EEST => "Eastern European Summer Time".into(),
            GeneralTimeZone::EET => "Eastern European Time".into(),
            GeneralTimeZone::CEST => "Central European Summer Time".into(),
            GeneralTimeZone::CET => "Central European Time".into(),
            GeneralTimeZone::IDLW => "International Date Line West".into(),
            GeneralTimeZone::SST => "Samoa Standard Time".into(),
            GeneralTimeZone::HST => "Hawaii Standard Time".into(),
            GeneralTimeZone::AKDT => "Alaska Daylight Time".into(),
            GeneralTimeZone::AKST => "Alaska Standard Time".into(),
            GeneralTimeZone::PDT => "Pacific Daylight Time".into(),
            GeneralTimeZone::PST => "Pacific Standard Time".into(),
            GeneralTimeZone::MDT => "Mountain Daylight Time".into(),
            GeneralTimeZone::MST => "Mountain Standard Time".into(),
            GeneralTimeZone::CDT => "Central Daylight Time".into(),
            GeneralTimeZone::CST => "Central Standard Time".into(),
            GeneralTimeZone::EDT => "Eastern Daylight Time".into(),
            GeneralTimeZone::EST => "Eastern Standard Time".into(),
            GeneralTimeZone::ADT => "Atlantic Daylight Time".into(),
            GeneralTimeZone::AST => "Atlantic Standard Time".into(),
            GeneralTimeZone::NDT => "Newfoundland Daylight Time".into(),
            GeneralTimeZone::NST => "Newfoundland Standard Time".into(),
            GeneralTimeZone::ART => "Argentina Time".into(),
            GeneralTimeZone::FNT => "Fernando de Noronha Time".into(),
            GeneralTimeZone::AZOST => "Azores Summer Time".into(),
            GeneralTimeZone::AZOT => "Azores Standard Time".into(),
            GeneralTimeZone::GMT => "Greenwich Mean Time".into(),
            GeneralTimeZone::WEST => "Western European Summer Time".into(),
            GeneralTimeZone::WET => "Western European Time".into(),
            GeneralTimeZone::AFT => "Afghanistan Time".into(),
            GeneralTimeZone::GST => "Gulf Standard Time".into(),
            GeneralTimeZone::IRST => "Iran Standard Time".into(),
            GeneralTimeZone::PKT => "Pakistan Standard Time".into(),
            GeneralTimeZone::IST => "India Standard Time".into(),
            GeneralTimeZone::NPT => "Nepal Time".into(),
            GeneralTimeZone::BST => "Bangladesh Standard Time".into(),
            GeneralTimeZone::MMT => "Myanmar Time".into(),
            GeneralTimeZone::ICT => "Indochina Time".into(),
            GeneralTimeZone::SGT => "Singapore Time".into(),
            GeneralTimeZone::JST => "Japan Standard Time".into(),
            GeneralTimeZone::ACDT => "Australian Central Daylight Time".into(),
            GeneralTimeZone::ACST => "Australian Central Standard Time".into(),
            GeneralTimeZone::AEDT => "Australian Eastern Daylight Time".into(),
            GeneralTimeZone::AEST => "Australian Eastern Standard Time".into(),
            GeneralTimeZone::SBT => "Solomon Islands Time".into(),
            GeneralTimeZone::NZDT => "New Zealand Daylight Time".into(),
            GeneralTimeZone::NZST => "New Zealand Standard Time".into(),
            GeneralTimeZone::TOT => "Tonga Time".into(),
            GeneralTimeZone::Unknown(seconds) => {
                super::utils::seconds_to_string(*seconds).to_string().into()
            }
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, GeneralTimeZone::Unknown(_))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_europe_summer_time() {
        use rust_extensions::date_time::DateTimeAsMicroseconds;

        let dt = DateTimeAsMicroseconds::create(2025, 08, 10, 2, 0, 0, 0);
        let time_zone = super::TimeZoneGmtOffset::try_from_str("UTC+2:00").unwrap();

        let named_time_zone =
            super::GeneralTimeZone::from_client_time(dt, time_zone, super::CountryCode::DEU);

        match named_time_zone {
            super::GeneralTimeZone::CEST => {}
            _ => panic!("Expected CEST, got {:?}", named_time_zone),
        }
    }

    #[test]
    fn test_europe_winter_time() {
        use rust_extensions::date_time::DateTimeAsMicroseconds;

        let dt = DateTimeAsMicroseconds::create(2025, 02, 10, 2, 0, 0, 0);
        let time_zone = super::TimeZoneGmtOffset::try_from_str("UTC+1:00").unwrap();

        let named_time_zone =
            super::GeneralTimeZone::from_client_time(dt, time_zone, super::CountryCode::DEU);

        match named_time_zone {
            super::GeneralTimeZone::CET => {}
            _ => panic!("Expected CET, got {:?}", named_time_zone),
        }
    }
}
