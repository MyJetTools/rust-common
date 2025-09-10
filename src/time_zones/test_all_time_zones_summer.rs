struct Item<'s> {
    pub time_zone: &'s str,
    pub time: &'s str,
}

impl<'s> Item<'s> {
    pub fn parse(src: &'s str) -> Self {
        let mut items = src.split(' ');
        Item {
            time_zone: items.next().unwrap(),
            time: items.next().unwrap(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.time_zone, self.time)
    }
}

mod tests {

    use crate::time_zones::{test_all_time_zones_summer::Item, IanaTimeZone};

    const DATA: &'static [u8] = std::include_bytes!("examples_dst_iana_timezones.txt");

    #[test]
    fn test_from_file() {
        let str = std::str::from_utf8(DATA).unwrap();

        for src in str.split("\n") {
            let itm = Item::parse(src);

            let time_zone = IanaTimeZone::try_from_str(itm.time_zone);

            if time_zone.is_none() {
                println!("{}", itm.to_string());
            }
        }
    }
}
