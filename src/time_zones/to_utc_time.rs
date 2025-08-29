use core::panic;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    country_code::CountryCode,
    time_zones::{IanaTimeZone, TimeZone},
};

pub fn to_utc_time(
    mut local_time: DateTimeAsMicroseconds,
    time_zone: impl Into<TimeZone>,
) -> DateTimeAsMicroseconds {
    let time_zone: TimeZone = time_zone.into();
    let offset = match time_zone {
        TimeZone::UTC(offset) => offset.as_minutes(),
        TimeZone::Iana(iana) => get_offset_from_iana(local_time, iana),
        TimeZone::General(general) => general.get_offset_in_minutes(),
    };

    local_time.add_minutes(-offset as i64);
    local_time
}

fn get_offset_from_iana(local_time: DateTimeAsMicroseconds, time_zone: IanaTimeZone) -> i32 {
    let country_code: CountryCode = time_zone.into();

    let has_dst = match super::dst::DST.get(&country_code) {
        Some(value) => *value,
        None => {
            panic!("Can not detect dst by country: {:?}", country_code);
        }
    };

    let offset = if !has_dst {
        time_zone.to_no_dst_offset_in_minutes()
    } else {
        let is_dst = super::is_day_saving_time(local_time, country_code);

        if is_dst {
            time_zone.to_dst_offset_in_minutes()
        } else {
            time_zone.to_no_dst_offset_in_minutes()
        }
    };

    offset
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
