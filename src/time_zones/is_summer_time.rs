use rust_extensions::date_time::{DateTimeAsMicroseconds, DateTimeStruct};

use crate::country_code::CountryCode;

pub fn is_day_saving_time(dt: DateTimeAsMicroseconds, country_code: CountryCode) -> bool {
    use crate::country_code::CountryCode::*;

    let dt_struct: DateTimeStruct = dt.into();

    match country_code {
        // USA, Canada: 2nd Sunday in March to 1st Sunday in November
        USA | CAN => {
            // DST start: 2nd Sunday in March
            let dst_start = nth_weekday_of_month(dt_struct.year, 3, 1, 2); // 1=Sunday, 2nd occurrence
            let dst_end = nth_weekday_of_month(dt_struct.year, 11, 1, 1);

            (dt >= dst_start) && (dt < dst_end)
        }
        // Europe: Last Sunday in March to Last Sunday in October
        GBR | FRA | DEU | ESP | ITA | NLD | BEL | CHE | AUT | DNK | NOR | SWE | FIN | IRL | PRT
        | POL | CZE | HUN | SVK | SVN | EST | LVA | LTU | GRC | ROU | BGR | HRV | CYP | MLT
        | LUX | ISL | AND | SRB | MDA | GIB | GGY | IMN | JEY | UKR | ALA | MCO | MNE | SMR
        | BIH | MKD | ALB | LIE | VAT => {
            let dst_start = last_weekday_of_month(dt_struct.year, 3, 0); // 0=Sunday
            let dst_end = last_weekday_of_month(dt_struct.year, 10, 0);
            (dt >= dst_start) && (dt < dst_end)
        }
        // UK (BST): Last Sunday in March to Last Sunday in October
        // Already covered by GBR
        // Australia (AEDT/ACDT): 1st Sunday in October to 1st Sunday in April
        AUS => {
            let dst_start = nth_weekday_of_month(dt_struct.year, 10, 0, 1); // 0=Sunday, 1st occurrence
            let dst_end = nth_weekday_of_month(dt_struct.year + 1, 4, 0, 1); // 0=Sunday, 1st occurrence (next year)
            if dt_struct.month >= 10 {
                dt >= dst_start
            } else if dt_struct.month < 4 {
                dt < dst_end
            } else {
                false
            }
        }
        // New Zealand: Last Sunday in September to 1st Sunday in April
        NZL => {
            let dst_start = last_weekday_of_month(dt_struct.year, 9, 0); // September, Sunday
            let dst_end = nth_weekday_of_month(dt_struct.year + 1, 4, 0, 1); // April, Sunday, next year
            if dt_struct.month >= 9 {
                dt >= dst_start
            } else if dt_struct.month < 4 {
                dt < dst_end
            } else {
                false
            }
        }
        // Default: no DST
        _ => false,
    }
}

use chrono::Datelike;

fn nth_weekday_of_month(year: i32, month: u32, weekday: u32, n: u32) -> DateTimeAsMicroseconds {
    // weekday: 0=Sunday, 1=Monday, ... 6=Saturday
    let mut count = 0;
    for day in 1..=31 {
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
            if date.weekday().num_days_from_sunday() == weekday {
                count += 1;
                if count == n {
                    return DateTimeAsMicroseconds::create(year, month, day, 2, 0, 0, 0);
                }
            }
        }
    }
    // fallback: last day of month
    DateTimeAsMicroseconds::create(year, month, 28, 2, 0, 0, 0)
}

fn last_weekday_of_month(year: i32, month: u32, weekday: u32) -> DateTimeAsMicroseconds {
    for day in (1..=31).rev() {
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
            if date.weekday().num_days_from_sunday() == weekday {
                return DateTimeAsMicroseconds::create(year, month, day, 2, 0, 0, 0);
            }
        }
    }
    // fallback: first day of month
    DateTimeAsMicroseconds::create(year, month, 1, 2, 0, 0, 0)
}

#[cfg(test)]
mod test {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    #[test]
    fn is_summer_time_eur() {
        let result = super::is_day_saving_time(
            DateTimeAsMicroseconds::create(2023, 2, 26, 2, 0, 0, 0),
            super::CountryCode::DEU,
        );

        assert_eq!(result, false);

        let result = super::is_day_saving_time(
            DateTimeAsMicroseconds::create(2025, 5, 30, 10, 0, 0, 0),
            super::CountryCode::DEU,
        );

        assert_eq!(result, true);
    }
}
