use rust_extensions::StrOrString;

const MIN_30: i32 = 30 * 60;
const MIN_45: i32 = 30 * 60;

pub const UTC_0: i32 = 0;
pub const UTC_1: i32 = 3600;
pub const UTC_2: i32 = 2 * 3600;
pub const UTC_3: i32 = 3 * 3600;
pub const UTC_4: i32 = 4 * 3600;
pub const UTC_5: i32 = 5 * 3600;
pub const UTC_5_30: i32 = 5 * 3600 + MIN_30;
pub const UTC_6: i32 = 6 * 3600;
pub const UTC_7: i32 = 7 * 3600;
pub const UTC_8: i32 = 8 * 3600;
pub const UTC_8_45: i32 = UTC_8 + MIN_45;
pub const UTC_9: i32 = 9 * 3600;
pub const UTC_9_30: i32 = UTC_9 + MIN_30;
pub const UTC_10: i32 = 10 * 3600;
pub const UTC_10_30: i32 = UTC_10 + MIN_30;
pub const UTC_11: i32 = 11 * 3600;
pub const UTC_12: i32 = 12 * 3600;
pub const UTC_13: i32 = 13 * 3600;
pub const UTC_14: i32 = 14 * 3600;

pub const UTC_MINUS_1: i32 = -UTC_1;
pub const UTC_MINUS_2: i32 = -UTC_2;
pub const UTC_MINUS_2_30: i32 = UTC_MINUS_2 - MIN_30; // UTC-3:30
pub const UTC_MINUS_3: i32 = -UTC_3;
pub const UTC_MINUS_3_30: i32 = UTC_MINUS_3 - MIN_30; // UTC-3:30
pub const UTC_MINUS_4: i32 = -UTC_4;
pub const UTC_MINUS_5: i32 = -UTC_5;
pub const UTC_MINUS_6: i32 = -UTC_6;
pub const UTC_MINUS_7: i32 = -UTC_7;
pub const UTC_MINUS_8: i32 = -UTC_8;
pub const UTC_MINUS_9: i32 = -UTC_9;
pub const UTC_MINUS_10: i32 = -UTC_10;
pub const UTC_MINUS_11: i32 = -UTC_11;
pub const UTC_MINUS_12: i32 = -UTC_12;

#[derive(Debug, Clone, Copy)]
pub enum TimeZone {
    UTC(super::TimeZoneGmtOffset),
    Iana(super::IanaTimeZone),
    General(super::GeneralTimeZone),
}

impl Into<TimeZone> for &'_ str {
    fn into(self) -> TimeZone {
        TimeZone::from_str(self)
    }
}

impl TimeZone {
    pub fn from_str(src: &str) -> Self {
        if let Some(iana) = super::IanaTimeZone::try_from_str(src) {
            return Self::Iana(iana);
        }

        if let Some(general) = super::GeneralTimeZone::try_from_str(src) {
            return Self::General(general);
        }

        Self::UTC(super::TimeZoneGmtOffset::try_from_str(src).unwrap())
    }

    pub fn as_str(&self) -> StrOrString<'static> {
        match self {
            Self::UTC(offset) => offset.to_string().into(),
            Self::Iana(iana) => iana.as_str().to_string().into(),
            Self::General(general) => general.as_str(),
        }
    }
}
