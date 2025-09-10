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
}
