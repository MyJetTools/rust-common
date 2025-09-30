use rust_extensions::ShortString;

pub struct SauPhoneNumber<'s>(&'s str);

impl<'s> SauPhoneNumber<'s> {
    pub fn new(src: &'s str) -> Self {
        SauPhoneNumber(src)
    }

    pub fn as_international_phone_number(&'s self) -> ShortString {
        let mut result = if self.0.starts_with("+") {
            ShortString::from_str("+").unwrap()
        } else if self.0.starts_with('0') {
            ShortString::from_str("+966").unwrap()
        } else {
            return ShortString::from_str(self.0).unwrap();
        };

        for c in self.0.chars().skip(1) {
            if c.is_ascii_digit() {
                result.push(c);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use crate::phone_codes::by_country::SauPhoneNumber;

    #[test]
    fn test_international_with_spaces() {
        let phone = SauPhoneNumber::new("+966 55 123 4567");

        assert_eq!(phone.as_international_phone_number(), "+966551234567");
    }

    #[test]
    fn test_local_to_international() {
        let phone = SauPhoneNumber::new("055 123 4567");

        assert_eq!(phone.as_international_phone_number(), "+966551234567");
    }
}
