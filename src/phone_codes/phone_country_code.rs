use crate::country_code::CountryCode;

#[derive(Debug, Clone, Copy)]
pub enum PhoneCountryCode {
    CountryCode(CountryCode),
    UniversalInternationalFreephone,
    InternationalSharedCost,
    GlobalMobileSatelliteSystem,
    InternationalNetworks,
    InmarsatSNAC,
}

impl PhoneCountryCode {
    pub fn unwrap_as_country_code(&self) -> CountryCode {
        match self {
            PhoneCountryCode::CountryCode(cc) => *cc,
            _ => panic!("Not a CountryCode variant. {:?}", self),
        }
    }
}

impl Into<PhoneCountryCode> for CountryCode {
    fn into(self) -> PhoneCountryCode {
        PhoneCountryCode::CountryCode(self)
    }
}
