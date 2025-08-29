use core::panic;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    country_code::IANA_TO_COUNTRY_CODE,
    time_zones::{IanaTimeZone, TimeZoneOffset},
};

pub fn to_utc_time(
    mut local_time: DateTimeAsMicroseconds,
    time_zone: &str,
) -> DateTimeAsMicroseconds {
    let offset = if let Some(offset) = TimeZoneOffset::try_from_str(time_zone) {
        offset.as_minutes() as i64
    } else {
        get_offset_from_iana(local_time, time_zone)
    };

    local_time.add_minutes(-offset);

    local_time
}

fn get_offset_from_iana(local_time: DateTimeAsMicroseconds, time_zone: &str) -> i64 {
    let iana_time_zone = IanaTimeZone::try_from_str(time_zone);

    let Some(iana_time_zone) = iana_time_zone else {
        panic!("Invalid iana time-zone: {}", time_zone);
    };

    let country_code = IANA_TO_COUNTRY_CODE.get(time_zone).cloned();

    let Some(country_code) = country_code else {
        panic!("Invalid time-zone: {}", time_zone);
    };

    let has_dst = match super::dst::DST.get(&country_code) {
        Some(value) => *value,
        None => {
            panic!("Can not detect dst by country: {:?}", country_code);
        }
    };

    let offset = if !has_dst {
        iana_time_zone.to_no_dst_offset_in_minutes()
    } else {
        let is_dst = super::is_day_saving_time(local_time, country_code);

        if is_dst {
            iana_time_zone.to_dst_offset_in_minutes()
        } else {
            iana_time_zone.to_no_dst_offset_in_minutes()
        }
    };

    offset as i64
}

#[cfg(test)]
mod test {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    #[test]
    fn test_zurich_to_utc_dst() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-08-29T12:00:00").unwrap();

        let utc_time = super::to_utc_time(local_time, "Europe/Zurich");

        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-08-29T10:00:00");
    }

    #[test]
    fn test_zurich_to_utc_no_dst() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();

        let utc_time = super::to_utc_time(local_time, "Europe/Zurich");

        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T11:00:00");
    }

    #[test]
    fn tes_utc2_to_utc_no_dst() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();

        let utc_time = super::to_utc_time(local_time, "UTC+2");

        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T10:00:00");
    }
}
