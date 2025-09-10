use rust_common::country_code::CountryCode;
use rust_common::time_zones::{to_utc_time, IanaTimeZone};
use rust_extensions::date_time::DateTimeAsMicroseconds;

fn main() {
    let local_time = DateTimeAsMicroseconds::from_str("2025-09-10T11:59:00").unwrap();
    let time_zone = IanaTimeZone::try_from_str("Europe/Andorra").unwrap();
    let country_code: CountryCode = time_zone.into();

    println!("Time zone: {:?}", time_zone);
    println!("Country code: {:?}", country_code);
    println!("Has DST: {}", country_code.has_dst());

    let is_dst = rust_common::time_zones::is_day_saving_time(local_time, country_code);
    println!("Is DST: {}", is_dst);

    let dst_offset = time_zone.to_dst_offset_in_minutes();
    let no_dst_offset = time_zone.to_no_dst_offset_in_minutes();

    println!("DST offset: {} minutes", dst_offset);
    println!("No DST offset: {} minutes", no_dst_offset);

    let utc_time = to_utc_time(local_time, time_zone);
    println!("UTC time: {}", &utc_time.to_rfc3339()[..19]);
}
