const MIN_30: i32 = 30 * 60;
const MIN_45: i32 = 30 * 60;

pub const UTC_0: i32 = 0;
pub const UTC_1: i32 = 3600;
pub const UTC_2: i32 = 2 * 3600;
pub const UTC_3: i32 = 3 * 3600;
pub const UTC_4: i32 = 4 * 3600;
pub const UTC_5: i32 = 5 * 3600;
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
pub struct TimeZone {
    seconds: i32,
    day_saving_time: bool,
}

impl TimeZone {
    pub fn try_from_str(src: &str) -> Option<Self> {
        let src = if src.starts_with("UTC") {
            &src[3..]
        } else {
            src
        };

        if src.starts_with('+') {
            let src = &src[1..];

            let (h, m) = parse_h_m(src)?;

            return Some(Self {
                seconds: h * 3600 + m * 60,
                day_saving_time: false,
            });
        }

        if src.starts_with('-') {
            let src = &src[1..];

            let (h, m) = parse_h_m(src)?;

            let result = h * 3600 + m * 60;
            return Some(Self {
                seconds: -result,
                day_saving_time: false,
            });
        }

        let (h, m) = parse_h_m(src)?;

        Some(Self {
            seconds: h * 3600 + m * 60,
            day_saving_time: false,
        })
    }

    pub fn to_string(&self) -> String {
        super::utils::seconds_to_string(self.seconds)
    }

    pub fn as_seconds(&self) -> i32 {
        self.seconds
    }

    pub fn get_day_saving_time(&self) -> bool {
        self.day_saving_time
    }

    pub fn set_day_saving_time(&mut self, value: bool) {
        self.day_saving_time = value;
    }

    #[cfg(feature = "country-code")]
    pub fn get_named_time_zone(
        &self,
        country_code: crate::country_code::CountryCode,
    ) -> super::NamedTimeZone {
        super::NamedTimeZone::create(*self, country_code)
    }
}

fn parse_h_m(src: &str) -> Option<(i32, i32)> {
    let mut parts = src.split(':');
    let hours = parts.next()?;
    let h: Result<i32, _> = hours.parse();
    let Ok(h) = h else {
        return None;
    };

    let min = match parts.next() {
        Some(min) => {
            let min: Result<i32, _> = min.parse();
            let Ok(min) = min else {
                return None;
            };

            min
        }
        None => 0,
    };

    Some((h, min))
}

#[cfg(test)]
mod tests {
    use crate::time_zones::TimeZone;

    #[test]
    fn parse_positive_tz() {
        let tz = TimeZone::try_from_str("+2:00").unwrap();
        assert_eq!(tz.as_seconds(), 7200);

        assert_eq!("UTC+2:00", tz.to_string());
    }

    #[test]
    fn parse_negative_tz() {
        let tz = TimeZone::try_from_str("-2:00").unwrap();
        assert_eq!(tz.as_seconds(), -7200);

        assert_eq!("UTC-2:00", tz.to_string());
    }

    #[test]
    fn parse_zero_tz() {
        let tz = TimeZone::try_from_str("0:00").unwrap();
        assert_eq!(tz.as_seconds(), 0);
        assert_eq!("UTC 0:00", tz.to_string());
    }
}
