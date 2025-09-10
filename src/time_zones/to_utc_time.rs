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

    let has_dst = country_code.has_dst();

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

    // UTC-12 to UTC+14 IANA Time Zone Tests
    #[test]
    fn test_baker_island_utc_minus12() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Baker");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-21T00:00:00");
    }

    #[test]
    fn test_niue_utc_minus11() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Niue");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T23:00:00");
    }

    #[test]
    fn test_honolulu_utc_minus10() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Honolulu");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T22:00:00");
    }

    #[test]
    fn test_adak_utc_minus9() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "America/Adak");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T22:00:00");
    }

    #[test]
    fn test_pitcairn_utc_minus8() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Pitcairn");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T20:00:00");
    }

    #[test]
    fn test_phoenix_utc_minus7() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "America/Phoenix");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T19:00:00");
    }

    #[test]
    fn test_regina_utc_minus6() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "America/Regina");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T18:00:00");
    }

    #[test]
    fn test_guayaquil_utc_minus5() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "America/Guayaquil");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T17:00:00");
    }

    #[test]
    fn test_lapaz_utc_minus4() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "America/La_Paz");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T16:00:00");
    }

    #[test]
    fn test_buenos_aires_utc_minus3() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "America/Argentina/Buenos_Aires");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T15:00:00");
    }

    #[test]
    fn test_south_georgia_utc_minus2() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Atlantic/South_Georgia");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T14:00:00");
    }

    #[test]
    fn test_azores_utc_minus1() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Atlantic/Azores");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T13:00:00");
    }

    #[test]
    fn test_reykjavik_utc_zero() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Atlantic/Reykjavik");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T12:00:00");
    }

    #[test]
    fn test_algiers_utc_plus1() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Africa/Algiers");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T11:00:00");
    }

    #[test]
    fn test_johannesburg_utc_plus2() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Africa/Johannesburg");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T10:00:00");
    }

    #[test]
    fn test_moscow_utc_plus3() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Europe/Moscow");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T09:00:00");
    }

    #[test]
    fn test_dubai_utc_plus4() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Dubai");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T08:00:00");
    }

    #[test]
    fn test_karachi_utc_plus5() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Karachi");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T07:00:00");
    }

    #[test]
    fn test_kolkata_utc_plus5_30() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Kolkata");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T06:30:00");
    }

    #[test]
    fn test_dhaka_utc_plus6() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Dhaka");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T06:00:00");
    }

    #[test]
    fn test_jakarta_utc_plus7() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Jakarta");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T05:00:00");
    }

    #[test]
    fn test_shanghai_utc_plus8() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Shanghai");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T04:00:00");
    }

    #[test]
    fn test_eucla_utc_plus8_45() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Australia/Eucla");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T03:15:00");
    }

    #[test]
    fn test_tokyo_utc_plus9() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Asia/Tokyo");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T03:00:00");
    }

    #[test]
    fn test_brisbane_utc_plus10() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Australia/Brisbane");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T02:00:00");
    }

    #[test]
    fn test_noumea_utc_plus11() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Noumea");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T01:00:00");
    }

    #[test]
    fn test_fiji_utc_plus12() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Fiji");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-20T00:00:00");
    }

    #[test]
    fn test_tongatapu_utc_plus13() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Tongatapu");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-19T23:00:00");
    }

    #[test]
    fn test_kiritimati_utc_plus14() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-01-20T12:00:00").unwrap();
        let utc_time = super::to_utc_time(local_time, "Pacific/Kiritimati");
        assert_eq!(&utc_time.to_rfc3339()[..19], "2025-01-19T22:00:00");
    }
}
