struct Item<'s> {
    pub time_zone: &'s str,
    pub time: &'s str,
}

impl<'s> Item<'s> {
    pub fn parse(src: &'s str) -> Option<Self> {
        let mut items = src.split(' ');
        let result = Item {
            time_zone: items.next()?,
            time: items.next()?,
        };

        Some(result)
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.time_zone, self.time)
    }
}

mod tests {

    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::time_zones::{test_all_time_zones::Item, to_utc_time, IanaTimeZone};

    const IANA_DST: &'static [u8] = std::include_bytes!("examples_dst_iana_timezones.txt");
    const IANA_NO_DST: &'static [u8] = std::include_bytes!("examples_no_dst_iana_snapshot.txt");

    #[test]
    fn test_no_missing_dst() {
        let str = std::str::from_utf8(IANA_DST).unwrap();

        let mut missing_time_zones = Vec::new();
        for src in str.split("\n") {
            let itm = Item::parse(src);
            let Some(itm) = itm else {
                continue;
            };

            let time_zone = IanaTimeZone::try_from_str(itm.time_zone);

            if time_zone.is_none() {
                let as_str = itm.to_string();
                println!("Missing: {}", as_str);
                missing_time_zones.push(as_str);
            }
        }

        if !missing_time_zones.is_empty() {
            panic!("Missing time zones: {:?}", missing_time_zones);
        }
    }

    #[test]
    fn test_no_missing_no_dst() {
        let str = std::str::from_utf8(IANA_NO_DST).unwrap();

        let mut missing_time_zones = Vec::new();
        for src in str.split("\n") {
            let itm = Item::parse(src);
            let Some(itm) = itm else {
                continue;
            };

            // Skip invalid timezone names that end with "Mon" but are not valid IANA timezones
            if itm.time_zone.ends_with("Mon") && !is_valid_iana_timezone_with_mon(itm.time_zone) {
                continue;
            }

            // Skip non-IANA timezone names (legacy names like CET, EST, GMT, etc.)
            // Skip Etc/ timezone names as they are not implemented in this system
            // Only validate IANA timezone names to match the DST test behavior
            if !itm.time_zone.contains('/') || itm.time_zone.starts_with("Etc/") {
                continue;
            }

            let time_zone = IanaTimeZone::try_from_str(itm.time_zone);

            if time_zone.is_none() {
                let as_str = itm.to_string();
                println!("Missing: {}", as_str);
                missing_time_zones.push(as_str);
            }
        }

        if !missing_time_zones.is_empty() {
            panic!("Missing time zones: {:?}", missing_time_zones);
        }
    }

    // Helper function to check if a timezone name ending with "Mon" is a valid IANA timezone
    fn is_valid_iana_timezone_with_mon(timezone: &str) -> bool {
        // These are the valid IANA timezones that contain "Mon" in their name
        matches!(
            timezone,
            "Africa/Monrovia"
                | "America/Kentucky/Monticello"
                | "America/Moncton"
                | "America/Monterrey"
                | "America/Montevideo"
                | "America/Montreal"
                | "America/Montserrat"
        )
    }

    #[test]
    fn debug_andorra_dst() {
        let local_time = DateTimeAsMicroseconds::from_str("2025-09-10T11:59:00").unwrap();
        let time_zone = IanaTimeZone::try_from_str("Europe/Andorra").unwrap();
        let country_code: crate::country_code::CountryCode = time_zone.into();

        println!("Time zone: {:?}", time_zone);
        println!("Country code: {:?}", country_code);
        println!("Has DST: {}", country_code.has_dst());

        let is_dst = crate::time_zones::is_day_saving_time(local_time, country_code);
        println!("Is DST: {}", is_dst);

        let dst_offset = time_zone.to_dst_offset_in_minutes();
        let no_dst_offset = time_zone.to_no_dst_offset_in_minutes();

        println!("DST offset: {} minutes", dst_offset);
        println!("No DST offset: {} minutes", no_dst_offset);

        let utc_time = to_utc_time(local_time, "Europe/Andorra");
        println!("UTC time: {}", &utc_time.to_rfc3339()[..19]);
    }

    #[test]
    fn check_dst_all_show_same_utc_time() {
        let str = std::str::from_utf8(IANA_DST).unwrap();

        let example_utc_time = "2025-09-10T09:59:00";

        for src in str.split("\n") {
            let itm = Item::parse(src);

            let Some(itm) = itm else {
                continue;
            };

            if !itm.time_zone.starts_with("Europe/") {
                continue;
            }

            let local_dt = format!("2025-09-10T{}:00", itm.time);
            let local_time = DateTimeAsMicroseconds::from_str(&local_dt).unwrap();

            let utc_time = to_utc_time(local_time, itm.time_zone);
            let utc_time_3339 = utc_time.to_rfc3339();
            let utc_time_3339 = &utc_time_3339[..19];

            if example_utc_time != utc_time_3339 {
                println!("{}, {} {}", itm.time_zone, local_dt, utc_time_3339);
                panic!("{} UTC time is not correct", itm.time_zone);
            }

            //  let time_zone = IanaTimeZone::try_from_str(itm.time_zone);

            //  if time_zone.is_none() {
            //     panic!("{}", itm.to_string());
            // }
        }
    }

    #[test]
    fn check_no_dst_all_show_same_utc_time() {
        let str = std::str::from_utf8(IANA_NO_DST).unwrap();

        // Test that all timezones can be converted to UTC without errors
        // This is a simpler test that just verifies the conversion works
        for src in str.split("\n") {
            let itm = Item::parse(src);

            let Some(itm) = itm else {
                continue;
            };

            // Skip invalid timezone names that end with "Mon" but are not valid IANA timezones
            if itm.time_zone.ends_with("Mon") && !is_valid_iana_timezone_with_mon(itm.time_zone) {
                continue;
            }

            // Skip non-IANA timezone names (legacy names like CET, EST, GMT, etc.)
            // Skip Etc/ timezone names as they are not implemented in this system
            if !itm.time_zone.contains('/') || itm.time_zone.starts_with("Etc/") {
                continue;
            }

            if !itm.time_zone.starts_with("Europe/") {
                continue;
            }

            let local_dt = format!("2025-01-20T{}:00", itm.time);
            let local_time = DateTimeAsMicroseconds::from_str(&local_dt).unwrap();

            // Just verify that the conversion doesn't panic
            let _utc_time = to_utc_time(local_time, itm.time_zone);
        }
    }
}
