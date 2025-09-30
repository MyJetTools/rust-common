use rust_extensions::ShortString;

use crate::{country_code::CountryCode, phone_codes::PhoneCode};

pub struct PhoneNumber {
    pub number: ShortString,
    country_code: Option<PhoneCode>,
}

impl PhoneNumber {
    pub fn from_phone_number_and_country_code(number: &str, country_code: CountryCode) -> Self {
        let number = ShortString::from_str(number).unwrap();
        let phone_code = PhoneCode::from_country_code(country_code);
        Self {
            number,
            country_code: Some(phone_code),
        }
    }

    pub fn from_phone_number(number: &str) -> Self {
        let number = ShortString::from_str(number).unwrap();
        Self {
            number,
            country_code: None,
        }
    }

    pub fn get_country_code(&self) -> Option<PhoneCode> {
        self.country_code
    }

    pub fn get_international_phone_number(&self) -> ShortString {
        match self.country_code {
            Some(phone_code) => {
                if self.number.starts_with("0") {
                    let mut result = ShortString::from_str(phone_code.phone_code).unwrap();

                    result.push_str(&self.number.as_str()[1..]);

                    return result;
                }
            }
            None => {}
        }

        self.number.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{country_code::CountryCode, phone_codes::PhoneNumber};

    #[test]
    fn test_bulgaria_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0896401234", CountryCode::BGR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+359896401234"
        );
    }

    // European Countries Tests
    #[test]
    fn test_albania_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0691234567", CountryCode::ALB);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+355691234567"
        );
    }

    #[test]
    fn test_andorra_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("07123456", CountryCode::AND);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3767123456"
        );
    }

    #[test]
    fn test_austria_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06991234567", CountryCode::AUT);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+436991234567"
        );
    }

    #[test]
    fn test_belarus_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0291234567", CountryCode::BLR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+375291234567"
        );
    }

    #[test]
    fn test_belgium_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0470123456", CountryCode::BEL);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+32470123456"
        );
    }

    #[test]
    fn test_bosnia_herzegovina_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::BIH);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+38761234567"
        );
    }

    #[test]
    fn test_croatia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0912345678", CountryCode::HRV);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+385912345678"
        );
    }

    #[test]
    fn test_czech_republic_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0721234567", CountryCode::CZE);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+420721234567"
        );
    }

    #[test]
    fn test_denmark_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("01234567", CountryCode::DNK);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+451234567"
        );
    }

    #[test]
    fn test_estonia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("051234567", CountryCode::EST);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+37251234567"
        );
    }

    #[test]
    fn test_finland_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0401234567", CountryCode::FIN);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+358401234567"
        );
    }

    #[test]
    fn test_france_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0123456789", CountryCode::FRA);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+33123456789"
        );
    }

    #[test]
    fn test_germany_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("03012345678", CountryCode::DEU);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+493012345678"
        );
    }

    #[test]
    fn test_greece_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06912345678", CountryCode::GRC);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+306912345678"
        );
    }

    #[test]
    fn test_hungary_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0612345678", CountryCode::HUN);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+36612345678"
        );
    }

    #[test]
    fn test_iceland_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06123456", CountryCode::ISL);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3546123456"
        );
    }

    #[test]
    fn test_ireland_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0851234567", CountryCode::IRL);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+353851234567"
        );
    }

    #[test]
    fn test_italy_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0612345678", CountryCode::ITA);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+39612345678"
        );
    }

    #[test]
    fn test_latvia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06123456", CountryCode::LVA);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3716123456"
        );
    }

    #[test]
    fn test_liechtenstein_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("078123456", CountryCode::LIE);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+42378123456"
        );
    }

    #[test]
    fn test_lithuania_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06123456", CountryCode::LTU);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3706123456"
        );
    }

    #[test]
    fn test_luxembourg_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::LUX);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+35261234567"
        );
    }

    #[test]
    fn test_north_macedonia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("071234567", CountryCode::MKD);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+38971234567"
        );
    }

    #[test]
    fn test_malta_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("079123456", CountryCode::MLT);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+35679123456"
        );
    }

    #[test]
    fn test_moldova_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::MDA);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+37361234567"
        );
    }

    #[test]
    fn test_monaco_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::MCO);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+37761234567"
        );
    }

    #[test]
    fn test_montenegro_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::MNE);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+38261234567"
        );
    }

    #[test]
    fn test_netherlands_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0612345678", CountryCode::NLD);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+31612345678"
        );
    }

    #[test]
    fn test_norway_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06123456", CountryCode::NOR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+476123456"
        );
    }

    #[test]
    fn test_poland_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0612345678", CountryCode::POL);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+48612345678"
        );
    }

    #[test]
    fn test_portugal_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0912345678", CountryCode::PRT);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+351912345678"
        );
    }

    #[test]
    fn test_romania_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0712345678", CountryCode::ROU);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+40712345678"
        );
    }

    #[test]
    fn test_russia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0912345678", CountryCode::RUS);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+7912345678"
        );
    }

    #[test]
    fn test_san_marino_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06123456", CountryCode::SMR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3786123456"
        );
    }

    #[test]
    fn test_serbia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0612345678", CountryCode::SRB);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+381612345678"
        );
    }

    #[test]
    fn test_slovakia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0912345678", CountryCode::SVK);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+421912345678"
        );
    }

    #[test]
    fn test_slovenia_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::SVN);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+38661234567"
        );
    }

    #[test]
    fn test_spain_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("061234567", CountryCode::ESP);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3461234567"
        );
    }

    #[test]
    fn test_sweden_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0701234567", CountryCode::SWE);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+46701234567"
        );
    }

    #[test]
    fn test_switzerland_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0781234567", CountryCode::CHE);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+41781234567"
        );
    }

    #[test]
    fn test_turkey_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("05321234567", CountryCode::TUR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+905321234567"
        );
    }

    #[test]
    fn test_ukraine_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("0671234567", CountryCode::UKR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+380671234567"
        );
    }

    #[test]
    fn test_united_kingdom_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("07123456789", CountryCode::GBR);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+447123456789"
        );
    }

    #[test]
    fn test_vatican_city_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("06123456", CountryCode::VAT);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+3796123456"
        );
    }

    #[test]
    fn test_kosovo_local_number_to_international() {
        let phone_number =
            PhoneNumber::from_phone_number_and_country_code("044123456", CountryCode::XKX);
        assert_eq!(
            phone_number.get_international_phone_number().as_str(),
            "+38344123456"
        );
    }
}
