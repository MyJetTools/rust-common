use crate::country_code::phone_codes::dicts::USA_PREFIX;

use super::super::*;
use super::*;
use rust_extensions::ShortString;

lazy_static::lazy_static! {
    pub static ref PHONE_CODES: PhoneCodes = PhoneCodes::new();
}

pub struct PhoneCodes {
    nanp: Vec<(&'static str, PhoneCountryCode)>,

    codes_2_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_2_2: Vec<(&'static str, PhoneCountryCode)>,

    codes_3_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_3_2: Vec<(&'static str, PhoneCountryCode)>,

    codes_4_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_4_2: Vec<(&'static str, PhoneCountryCode)>,

    codes_5_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_5_2: Vec<(&'static str, PhoneCountryCode)>,

    codes_6_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_6_2: Vec<(&'static str, PhoneCountryCode)>,

    codes_8_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_8_2: Vec<(&'static str, PhoneCountryCode)>,

    codes_9_3: Vec<(&'static str, PhoneCountryCode)>,
    codes_9_2: Vec<(&'static str, PhoneCountryCode)>,
}

impl PhoneCodes {
    pub fn new() -> Self {
        Self {
            nanp: super::dicts::create_country_code_nanp(),
            codes_2_3: super::dicts::create_2_3(),
            codes_2_2: super::dicts::create_2_2(),
            codes_3_3: super::dicts::create_3_3(),
            codes_3_2: super::dicts::create_3_2(),
            codes_4_3: super::dicts::create_4_3(),
            codes_4_2: super::dicts::create_4_2(),
            codes_5_3: super::dicts::create_5_3(),
            codes_5_2: super::dicts::create_5_2(),
            codes_6_3: super::dicts::create_6_3(),
            codes_6_2: super::dicts::create_6_2(),

            codes_8_3: super::dicts::create_8_3(),
            codes_8_2: super::dicts::create_8_2(),

            codes_9_3: super::dicts::create_9_3(),
            codes_9_2: super::dicts::create_9_2(),
        }
    }

    pub fn find_by_phone(&self, number: &str) -> Option<PhoneNumber> {
        if number.len() < 7 {
            return None;
        }

        let the_number = format_number(number)?;

        match &the_number[..2] {
            USA_PREFIX => {
                let nanp_prefix = &the_number.as_str()[..6];
                if let Some(code) = find(self.nanp.iter(), nanp_prefix) {
                    return Some(PhoneNumber {
                        country_code: code.1.clone(),
                        phone_code: code.0,
                    });
                }

                return Some(PhoneNumber {
                    country_code: CountryCode::USA.into(),
                    phone_code: super::dicts::USA_PREFIX,
                });
            }
            "+2" => {
                return find_3_or_2(
                    self.codes_2_3.iter(),
                    self.codes_2_2.iter(),
                    the_number.as_str(),
                );
            }
            "+3" => {
                return find_3_or_2(
                    self.codes_3_3.iter(),
                    self.codes_3_2.iter(),
                    the_number.as_str(),
                );
            }
            "+4" => {
                return find_3_or_2(
                    self.codes_4_3.iter(),
                    self.codes_4_2.iter(),
                    the_number.as_str(),
                );
            }
            "+5" => {
                return find_3_or_2(
                    self.codes_5_3.iter(),
                    self.codes_5_2.iter(),
                    the_number.as_str(),
                );
            }
            "+6" => {
                return find_3_or_2(
                    self.codes_6_3.iter(),
                    self.codes_6_2.iter(),
                    the_number.as_str(),
                );
            }
            "+7" => {
                return Some(PhoneNumber {
                    country_code: CountryCode::RUS.into(),
                    phone_code: "+7",
                });
            }

            "+8" => {
                return find_3_or_2(
                    self.codes_8_3.iter(),
                    self.codes_8_2.iter(),
                    the_number.as_str(),
                );
            }

            "+9" => {
                return find_3_or_2(
                    self.codes_9_3.iter(),
                    self.codes_9_2.iter(),
                    the_number.as_str(),
                );
            }

            _ => {}
        }

        None
    }
}

pub struct PhoneNumber {
    pub country_code: PhoneCountryCode,
    pub phone_code: &'static str,
}

impl PhoneNumber {
    pub fn as_str(&self) -> ShortString {
        let mut result = match self.country_code {
            PhoneCountryCode::CountryCode(cc) => {
                match crate::country_code::names::EN_NAMES.get(&cc) {
                    Some(v) => {
                        let mut result = ShortString::from_str(*v).unwrap();
                        result.push(' ');
                        result
                    }
                    None => ShortString::new_empty(),
                }
            }
            PhoneCountryCode::UniversalInternationalFreephone => {
                ShortString::from_str("Universal International Freephone ").unwrap()
            }
            PhoneCountryCode::InternationalSharedCost => {
                ShortString::from_str("International Shared Cost ").unwrap()
            }
            PhoneCountryCode::GlobalMobileSatelliteSystem => {
                ShortString::from_str("Global Mobile Satellite System ").unwrap()
            }
            PhoneCountryCode::InternationalNetworks => {
                ShortString::from_str("International Networks ").unwrap()
            }
            PhoneCountryCode::InmarsatSNAC => ShortString::from_str("Inmarsat SNAC ").unwrap(),
        };

        result.push_str(self.phone_code);

        result
    }
}

fn find_3_or_2<'s>(
    src_3: impl IntoIterator<Item = &'s (&'static str, PhoneCountryCode)>,
    src_2: impl IntoIterator<Item = &'s (&'static str, PhoneCountryCode)>,
    the_number: &str,
) -> Option<PhoneNumber> {
    let prefix_3 = &the_number[..4];
    if let Some(code) = find(src_3, prefix_3) {
        return Some(PhoneNumber {
            country_code: code.1.clone(),
            phone_code: code.0,
        });
    }

    let prefix_2 = &the_number[..3];
    if let Some(code) = find(src_2, prefix_2) {
        return Some(PhoneNumber {
            country_code: code.1.clone(),
            phone_code: code.0,
        });
    }

    None
}

fn find<'s>(
    src: impl IntoIterator<Item = &'s (&'static str, PhoneCountryCode)>,
    prefix: &str,
) -> Option<(&'static str, PhoneCountryCode)> {
    for itm in src {
        if itm.0 == prefix {
            return Some((itm.0, itm.1));
        }
    }

    None
}

fn format_number(number: &str) -> Option<ShortString> {
    let sub_str = if number.starts_with("00") {
        &number[2..]
    } else if number.starts_with('+') {
        &number[1..]
    } else {
        return None;
    };

    let mut the_number = ShortString::from_str("+").unwrap();

    for n in sub_str.chars() {
        if n.is_ascii_digit() {
            the_number.push(n);
        }
    }

    Some(the_number)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_bgr() {
        let result = super::PHONE_CODES.find_by_phone("+359896401234").unwrap();

        assert!(result
            .country_code
            .unwrap_as_country_code()
            .equals_to(super::CountryCode::BGR),);
        assert_eq!(result.phone_code, "+359");

        println!("Result: {}", result.as_str());
    }

    #[test]
    fn test_rus() {
        let result = super::PHONE_CODES
            .find_by_phone("+7-495 974-35-81")
            .unwrap();

        assert!(result
            .country_code
            .unwrap_as_country_code()
            .equals_to(super::CountryCode::RUS),);
        assert_eq!(result.phone_code, "+7");

        println!("Result: {}", result.as_str());
    }

    #[test]
    fn test_ua() {
        let result = super::PHONE_CODES
            .find_by_phone("+38 (048) 716-42-89 (176)")
            .unwrap();

        assert!(result
            .country_code
            .unwrap_as_country_code()
            .equals_to(super::CountryCode::UKR),);
        assert_eq!(result.phone_code, "+380");

        println!("Result: {}", result.as_str());
    }
}
