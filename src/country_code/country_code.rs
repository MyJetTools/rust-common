use std::collections::BTreeMap;

use super::ErrorParsingCountryCode;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CountryCode {
    AFG,
    ALA,
    ALB,
    DZA,
    ASM,
    AND,
    AGO,
    AIA,
    ATA,
    ATG,
    ARG,
    ARM,
    ABW,
    AUS,
    AUT,
    AZE,
    BHS,
    BHR,
    BGD,
    BRB,
    BLR,
    BEL,
    BLZ,
    BEN,
    BMU,
    BTN,
    BOL,
    BES,
    BIH,
    BWA,
    BVT,
    BRA,
    IOT,
    VGB,
    BRN,
    BGR,
    BFA,
    BDI,
    KHM,
    CMR,
    CAN,
    CPV,
    CYM,
    CAF,
    TCD,
    CHL,
    CHN,
    CXR,
    CCK,
    COL,
    COM,
    COK,
    CRI,
    HRV,
    CUB,
    CUW,
    CYP,
    CZE,
    COD,
    DNK,
    DJI,
    DMA,
    DOM,
    TLS,
    ECU,
    EGY,
    SLV,
    GNQ,
    ERI,
    EST,
    ETH,
    FLK,
    FRO,
    FJI,
    FIN,
    FRA,
    GUF,
    PYF,
    ATF,
    GAB,
    GMB,
    GEO,
    DEU,
    GHA,
    GIB,
    GRC,
    GRL,
    GRD,
    GLP,
    GUM,
    GTM,
    GGY,
    GIN,
    GNB,
    GUY,
    HTI,
    HMD,
    HND,
    HKG,
    HUN,
    ISL,
    IND,
    IDN,
    IRN,
    IRQ,
    IRL,
    IMN,
    ISR,
    ITA,
    CIV,
    JAM,
    JPN,
    JEY,
    JOR,
    KAZ,
    KEN,
    KIR,
    KWT,
    KGZ,
    LAO,
    LVA,
    LBN,
    LSO,
    LBR,
    LBY,
    LIE,
    LTU,
    LUX,
    MAC,
    MKD,
    MDG,
    MWI,
    MYS,
    MDV,
    MLI,
    MLT,
    MHL,
    MTQ,
    MRT,
    MUS,
    MYT,
    MEX,
    FSM,
    MDA,
    MCO,
    MNG,
    MNE,
    MSR,
    MAR,
    MOZ,
    MMR,
    NAM,
    NRU,
    NPL,
    NLD,
    NCL,
    NZL,
    NIC,
    NER,
    NGA,
    NIU,
    NFK,
    PRK,
    MNP,
    NOR,
    OMN,
    PAK,
    PLW,
    PSE,
    PAN,
    PNG,
    PRY,
    PER,
    PHL,
    PCN,
    POL,
    PRT,
    PRI,
    QAT,
    COG,
    REU,
    ROU,
    RUS,
    RWA,
    BLM,
    SHN,
    KNA,
    LCA,
    MAF,
    SPM,
    VCT,
    WSM,
    SMR,
    STP,
    SAU,
    SEN,
    SRB,
    SYC,
    SLE,
    SGP,
    SXM,
    SVK,
    SVN,
    SLB,
    SOM,
    ZAF,
    SGS,
    KOR,
    SSD,
    ESP,
    LKA,
    SDN,
    SUR,
    SJM,
    SWZ,
    SWE,
    CHE,
    SYR,
    TWN,
    TJK,
    TZA,
    THA,
    TGO,
    TKL,
    TON,
    TTO,
    TUN,
    TUR,
    TKM,
    TCA,
    TUV,
    VIR,
    UGA,
    UKR,
    ARE,
    GBR,
    USA,
    UMI,
    URY,
    UZB,
    VUT,
    VAT,
    VEN,
    VNM,
    WLF,
    ESH,
    YEM,
    ZMB,
    ZWE,
    XKX,
}

impl CountryCode {
    pub fn parse(src: &str) -> Result<Self, ErrorParsingCountryCode> {
        if src.len() == 3 {
            if let Some(country_code) = COUNTRIES_ISO_3_CODES.get(src) {
                return Ok(*country_code);
            }
        }

        if src.len() == 2 {
            if let Some(country_code) = COUNTRIES_ISO_2_CODES.get(src) {
                return Ok(*country_code);
            }
        }

        Err(ErrorParsingCountryCode {
            message: format!("Unknown country code: {}", src),
        })
    }

    pub fn equals_to(&self, other: CountryCode) -> bool {
        let src = *self as u16;
        let dest = other as u16;
        src == dest
    }

    #[deprecated(note = "please use `as_iso3_str` instead")]
    pub fn to_string_as_iso3(&self) -> &str {
        self.as_iso3_str()
    }

    pub fn as_iso3_str(&self) -> &'static str {
        for (key, value) in COUNTRIES_ISO_3_CODES.iter() {
            if self.equals_to(*value) {
                return key;
            }
        }

        panic!(
            "Somehow we can not find iso3 code for country code: {:?}",
            self
        );
    }

    #[deprecated(note = "please use `as_iso2_str` instead")]
    pub fn to_string_as_iso2(&self) -> &str {
        self.as_iso2_str()
    }

    pub fn as_iso2_str(&self) -> &str {
        for (key, value) in COUNTRIES_ISO_2_CODES.iter() {
            if self.equals_to(*value) {
                return key;
            }
        }

        panic!(
            "Somehow we can not find iso2 code for country code: {:?}",
            self
        );
    }

    #[cfg(feature = "time-zones")]
    pub fn get_iana_time_zone<'s>(
        &'s self,
        time_zone: crate::time_zones::TimeZoneGmtOffset,
        is_day_saving_time: bool,
    ) -> Option<crate::time_zones::IanaTimeZone> {
        crate::time_zones::IanaTimeZone::from_country_code(*self, time_zone, is_day_saving_time)
    }
}
lazy_static::lazy_static! {
    pub static ref COUNTRIES_ISO_3_CODES: BTreeMap<&'static str, CountryCode> = {
        let mut m = BTreeMap::new();

        m.insert("AFG", CountryCode::AFG);
        m.insert("ALA", CountryCode::ALA);
        m.insert("ALB", CountryCode::ALB);
        m.insert("DZA", CountryCode::DZA);
        m.insert("ASM", CountryCode::ASM);
        m.insert("AND", CountryCode::AND);
        m.insert("AGO", CountryCode::AGO);
        m.insert("AIA", CountryCode::AIA);
        m.insert("ATA", CountryCode::ATA);
        m.insert("ATG", CountryCode::ATG);
        m.insert("ARG", CountryCode::ARG);
        m.insert("ARM", CountryCode::ARM);
        m.insert("ABW", CountryCode::ABW);
        m.insert("AUS", CountryCode::AUS);
        m.insert("AUT", CountryCode::AUT);
        m.insert("AZE", CountryCode::AZE);
        m.insert("BHS", CountryCode::BHS);
        m.insert("BHR", CountryCode::BHR);
        m.insert("BGD", CountryCode::BGD);
        m.insert("BRB", CountryCode::BRB);
        m.insert("BLR", CountryCode::BLR);
        m.insert("BEL", CountryCode::BEL);
        m.insert("BLZ", CountryCode::BLZ);
        m.insert("BEN", CountryCode::BEN);
        m.insert("BMU", CountryCode::BMU);
        m.insert("BTN", CountryCode::BTN);
        m.insert("BOL", CountryCode::BOL);
        m.insert("BES", CountryCode::BES);
        m.insert("BIH", CountryCode::BIH);
        m.insert("BWA", CountryCode::BWA);
        m.insert("BVT", CountryCode::BVT);
        m.insert("BRA", CountryCode::BRA);
        m.insert("IOT", CountryCode::IOT);
        m.insert("VGB", CountryCode::VGB);
        m.insert("BRN", CountryCode::BRN);
        m.insert("BGR", CountryCode::BGR);
        m.insert("BFA", CountryCode::BFA);
        m.insert("BDI", CountryCode::BDI);
        m.insert("CPV", CountryCode::CPV);
        m.insert("KHM", CountryCode::KHM);
        m.insert("CMR", CountryCode::CMR);
        m.insert("CAN", CountryCode::CAN);
        m.insert("CYM", CountryCode::CYM);
        m.insert("CAF", CountryCode::CAF);
        m.insert("TCD", CountryCode::TCD);
        m.insert("CHL", CountryCode::CHL);
        m.insert("CHN", CountryCode::CHN);
        m.insert("CXR", CountryCode::CXR);
        m.insert("CCK", CountryCode::CCK);
        m.insert("COL", CountryCode::COL);
        m.insert("COM", CountryCode::COM);
        m.insert("COG", CountryCode::COG);
        m.insert("COD", CountryCode::COD);
        m.insert("COK", CountryCode::COK);
        m.insert("CRI", CountryCode::CRI);
        m.insert("CIV", CountryCode::CIV);
        m.insert("HRV", CountryCode::HRV);
        m.insert("CUB", CountryCode::CUB);
        m.insert("CUW", CountryCode::CUW);
        m.insert("CYP", CountryCode::CYP);
        m.insert("CZE", CountryCode::CZE);
        m.insert("DNK", CountryCode::DNK);
        m.insert("DJI", CountryCode::DJI);
        m.insert("DMA", CountryCode::DMA);
        m.insert("DOM", CountryCode::DOM);
        m.insert("TLS", CountryCode::TLS);
        m.insert("ECU", CountryCode::ECU);
        m.insert("EGY", CountryCode::EGY);
        m.insert("SLV", CountryCode::SLV);
        m.insert("GNQ", CountryCode::GNQ);
        m.insert("ERI", CountryCode::ERI);
        m.insert("EST", CountryCode::EST);
        m.insert("ETH", CountryCode::ETH);
        m.insert("FLK", CountryCode::FLK);
        m.insert("FRO", CountryCode::FRO);
        m.insert("FJI", CountryCode::FJI);
        m.insert("FIN", CountryCode::FIN);
        m.insert("FRA", CountryCode::FRA);
        m.insert("GUF", CountryCode::GUF);
        m.insert("PYF", CountryCode::PYF);
        m.insert("ATF", CountryCode::ATF);
        m.insert("GAB", CountryCode::GAB);
        m.insert("GMB", CountryCode::GMB);
        m.insert("GEO", CountryCode::GEO);
        m.insert("DEU", CountryCode::DEU);
        m.insert("GHA", CountryCode::GHA);
        m.insert("GIB", CountryCode::GIB);
        m.insert("GRC", CountryCode::GRC);
        m.insert("GRL", CountryCode::GRL);
        m.insert("GRD", CountryCode::GRD);
        m.insert("GLP", CountryCode::GLP);
        m.insert("GUM", CountryCode::GUM);
        m.insert("GTM", CountryCode::GTM);
        m.insert("GGY", CountryCode::GGY);
        m.insert("GIN", CountryCode::GIN);
        m.insert("GNB", CountryCode::GNB);
        m.insert("GUY", CountryCode::GUY);
        m.insert("HTI", CountryCode::HTI);
        m.insert("HMD", CountryCode::HMD);
        m.insert("VAT", CountryCode::VAT);
        m.insert("HND", CountryCode::HND);
        m.insert("HKG", CountryCode::HKG);
        m.insert("HUN", CountryCode::HUN);
        m.insert("ISL", CountryCode::ISL);
        m.insert("IND", CountryCode::IND);
        m.insert("IDN", CountryCode::IDN);
        m.insert("IRN", CountryCode::IRN);
        m.insert("IRQ", CountryCode::IRQ);
        m.insert("IRL", CountryCode::IRL);
        m.insert("IMN", CountryCode::IMN);
        m.insert("ISR", CountryCode::ISR);
        m.insert("ITA", CountryCode::ITA);
        m.insert("JAM", CountryCode::JAM);
        m.insert("JPN", CountryCode::JPN);
        m.insert("JEY", CountryCode::JEY);
        m.insert("JOR", CountryCode::JOR);
        m.insert("KAZ", CountryCode::KAZ);
        m.insert("KEN", CountryCode::KEN);
        m.insert("KIR", CountryCode::KIR);
        m.insert("PRK", CountryCode::PRK);
        m.insert("KWT", CountryCode::KWT);
        m.insert("KGZ", CountryCode::KGZ);
        m.insert("LAO", CountryCode::LAO);
        m.insert("LVA", CountryCode::LVA);
        m.insert("LBN", CountryCode::LBN);
        m.insert("LSO", CountryCode::LSO);
        m.insert("LBR", CountryCode::LBR);
        m.insert("LBY", CountryCode::LBY);
        m.insert("LIE", CountryCode::LIE);
        m.insert("LTU", CountryCode::LTU);
        m.insert("LUX", CountryCode::LUX);
        m.insert("MAC", CountryCode::MAC);
        m.insert("MKD", CountryCode::MKD);
        m.insert("MDG", CountryCode::MDG);
        m.insert("MWI", CountryCode::MWI);
        m.insert("MYS", CountryCode::MYS);
        m.insert("MDV", CountryCode::MDV);
        m.insert("MLI", CountryCode::MLI);
        m.insert("MLT", CountryCode::MLT);
        m.insert("MHL", CountryCode::MHL);
        m.insert("MTQ", CountryCode::MTQ);
        m.insert("MRT", CountryCode::MRT);
        m.insert("MUS", CountryCode::MUS);
        m.insert("MYT", CountryCode::MYT);
        m.insert("MEX", CountryCode::MEX);
        m.insert("FSM", CountryCode::FSM);
        m.insert("MDA", CountryCode::MDA);
        m.insert("MCO", CountryCode::MCO);
        m.insert("MNG", CountryCode::MNG);
        m.insert("MNE", CountryCode::MNE);
        m.insert("MSR", CountryCode::MSR);
        m.insert("MAR", CountryCode::MAR);
        m.insert("MOZ", CountryCode::MOZ);
        m.insert("MMR", CountryCode::MMR);
        m.insert("NAM", CountryCode::NAM);
        m.insert("NRU", CountryCode::NRU);
        m.insert("NPL", CountryCode::NPL);
        m.insert("NLD", CountryCode::NLD);
        m.insert("NCL", CountryCode::NCL);
        m.insert("NZL", CountryCode::NZL);
        m.insert("NIC", CountryCode::NIC);
        m.insert("NER", CountryCode::NER);
        m.insert("NGA", CountryCode::NGA);
        m.insert("NIU", CountryCode::NIU);
        m.insert("NFK", CountryCode::NFK);
        m.insert("MNP", CountryCode::MNP);
        m.insert("NOR", CountryCode::NOR);
        m.insert("OMN", CountryCode::OMN);
        m.insert("PAK", CountryCode::PAK);
        m.insert("PLW", CountryCode::PLW);
        m.insert("PSE", CountryCode::PSE);
        m.insert("PAN", CountryCode::PAN);
        m.insert("PNG", CountryCode::PNG);
        m.insert("PRY", CountryCode::PRY);
        m.insert("PER", CountryCode::PER);
        m.insert("PHL", CountryCode::PHL);
        m.insert("PCN", CountryCode::PCN);
        m.insert("POL", CountryCode::POL);
        m.insert("PRT", CountryCode::PRT);
        m.insert("PRI", CountryCode::PRI);
        m.insert("QAT", CountryCode::QAT);
        m.insert("REU", CountryCode::REU);
        m.insert("ROU", CountryCode::ROU);
        m.insert("RUS", CountryCode::RUS);
        m.insert("RWA", CountryCode::RWA);
        m.insert("BLM", CountryCode::BLM);
        m.insert("SHN", CountryCode::SHN);
        m.insert("KNA", CountryCode::KNA);
        m.insert("LCA", CountryCode::LCA);
        m.insert("MAF", CountryCode::MAF);
        m.insert("SPM", CountryCode::SPM);
        m.insert("VCT", CountryCode::VCT);
        m.insert("WSM", CountryCode::WSM);
        m.insert("SMR", CountryCode::SMR);
        m.insert("STP", CountryCode::STP);
        m.insert("SAU", CountryCode::SAU);
        m.insert("SEN", CountryCode::SEN);
        m.insert("SRB", CountryCode::SRB);
        m.insert("SYC", CountryCode::SYC);
        m.insert("SLE", CountryCode::SLE);
        m.insert("SGP", CountryCode::SGP);
        m.insert("SXM", CountryCode::SXM);
        m.insert("SVK", CountryCode::SVK);
        m.insert("SVN", CountryCode::SVN);
        m.insert("SLB", CountryCode::SLB);
        m.insert("SOM", CountryCode::SOM);
        m.insert("ZAF", CountryCode::ZAF);
        m.insert("SGS", CountryCode::SGS);
        m.insert("KOR", CountryCode::KOR);
        m.insert("SSD", CountryCode::SSD);
        m.insert("ESP", CountryCode::ESP);
        m.insert("LKA", CountryCode::LKA);
        m.insert("SDN", CountryCode::SDN);
        m.insert("SUR", CountryCode::SUR);
        m.insert("SJM", CountryCode::SJM);
        m.insert("SWZ", CountryCode::SWZ);
        m.insert("SWE", CountryCode::SWE);
        m.insert("CHE", CountryCode::CHE);
        m.insert("SYR", CountryCode::SYR);
        m.insert("TWN", CountryCode::TWN);
        m.insert("TJK", CountryCode::TJK);
        m.insert("TZA", CountryCode::TZA);
        m.insert("THA", CountryCode::THA);
        m.insert("TGO", CountryCode::TGO);
        m.insert("TKL", CountryCode::TKL);
        m.insert("TON", CountryCode::TON);
        m.insert("TTO", CountryCode::TTO);
        m.insert("TUN", CountryCode::TUN);
        m.insert("TUR", CountryCode::TUR);
        m.insert("TKM", CountryCode::TKM);
        m.insert("TCA", CountryCode::TCA);
        m.insert("TUV", CountryCode::TUV);
        m.insert("VIR", CountryCode::VIR);
        m.insert("UGA", CountryCode::UGA);
        m.insert("UKR", CountryCode::UKR);
        m.insert("ARE", CountryCode::ARE);
        m.insert("GBR", CountryCode::GBR);
        m.insert("USA", CountryCode::USA);
        m.insert("UMI", CountryCode::UMI);
        m.insert("URY", CountryCode::URY);
        m.insert("UZB", CountryCode::UZB);
        m.insert("VUT", CountryCode::VUT);
        m.insert("VEN", CountryCode::VEN);
        m.insert("VNM", CountryCode::VNM);
        m.insert("WLF", CountryCode::WLF);
        m.insert("ESH", CountryCode::ESH);
        m.insert("YEM", CountryCode::YEM);
        m.insert("ZMB", CountryCode::ZMB);
        m.insert("ZWE", CountryCode::ZWE);
        m.insert("XKX", CountryCode::XKX);


        m
    };
}

lazy_static::lazy_static! {
    pub static ref COUNTRIES_ISO_2_CODES: BTreeMap<&'static str, CountryCode> = {
        let mut m = BTreeMap::new();

        //Fill the map with the ISO 2 codes and CountryCode enums
        m.insert("AF", CountryCode::AFG);
        m.insert("AX", CountryCode::ALA);
        m.insert("AL", CountryCode::ALB);
        m.insert("DZ", CountryCode::DZA);
        m.insert("AS", CountryCode::ASM);
        m.insert("AD", CountryCode::AND);
        m.insert("AO", CountryCode::AGO);
        m.insert("AI", CountryCode::AIA);
        m.insert("AQ", CountryCode::ATA);
        m.insert("AG", CountryCode::ATG);
        m.insert("AR", CountryCode::ARG);
        m.insert("AM", CountryCode::ARM);
        m.insert("AW", CountryCode::ABW);
        m.insert("AU", CountryCode::AUS);
        m.insert("AT", CountryCode::AUT);
        m.insert("AZ", CountryCode::AZE);
        m.insert("BS", CountryCode::BHS);
        m.insert("BH", CountryCode::BHR);
        m.insert("BD", CountryCode::BGD);
        m.insert("BB", CountryCode::BRB);
        m.insert("BY", CountryCode::BLR);
        m.insert("BE", CountryCode::BEL);
        m.insert("BZ", CountryCode::BLZ);
        m.insert("BJ", CountryCode::BEN);
        m.insert("BM", CountryCode::BMU);
        m.insert("BT", CountryCode::BTN);
        m.insert("BO", CountryCode::BOL);
        m.insert("BQ", CountryCode::BES);
        m.insert("BA", CountryCode::BIH);
        m.insert("BW", CountryCode::BWA);
        m.insert("BV", CountryCode::BVT);
        m.insert("BR", CountryCode::BRA);
        m.insert("IO", CountryCode::IOT);
        m.insert("VG", CountryCode::VGB);
        m.insert("BN", CountryCode::BRN);
        m.insert("BG", CountryCode::BGR);
        m.insert("BF", CountryCode::BFA);
        m.insert("BI", CountryCode::BDI);
        m.insert("KH", CountryCode::KHM);
        m.insert("CM", CountryCode::CMR);
        m.insert("CA", CountryCode::CAN);
        m.insert("CV", CountryCode::CPV);
        m.insert("KY", CountryCode::CYM);
        m.insert("CF", CountryCode::CAF);
        m.insert("TD", CountryCode::TCD);
        m.insert("CL", CountryCode::CHL);
        m.insert("CN", CountryCode::CHN);
        m.insert("CX", CountryCode::CXR);
        m.insert("CC", CountryCode::CCK);
        m.insert("CO", CountryCode::COL);
        m.insert("KM", CountryCode::COM);
        m.insert("CG", CountryCode::COG);
        m.insert("CD", CountryCode::COD);
        m.insert("CK", CountryCode::COK);
        m.insert("CR", CountryCode::CRI);
        m.insert("CI", CountryCode::CIV);
        m.insert("HR", CountryCode::HRV);
        m.insert("CU", CountryCode::CUB);
        m.insert("CW", CountryCode::CUW);
        m.insert("CY", CountryCode::CYP);
        m.insert("CZ", CountryCode::CZE);
        m.insert("DK", CountryCode::DNK);
        m.insert("DJ", CountryCode::DJI);
        m.insert("DM", CountryCode::DMA);
        m.insert("DO", CountryCode::DOM);
        m.insert("TL", CountryCode::TLS);
        m.insert("EC", CountryCode::ECU);
        m.insert("EG", CountryCode::EGY);
        m.insert("SV", CountryCode::SLV);
        m.insert("GQ", CountryCode::GNQ);
        m.insert("ER", CountryCode::ERI);
        m.insert("EE", CountryCode::EST);
        m.insert("ET", CountryCode::ETH);
        m.insert("FK", CountryCode::FLK);
        m.insert("FO", CountryCode::FRO);
        m.insert("FJ", CountryCode::FJI);
        m.insert("FI", CountryCode::FIN);
        m.insert("FR", CountryCode::FRA);
        m.insert("GF", CountryCode::GUF);
        m.insert("PF", CountryCode::PYF);
        m.insert("TF", CountryCode::ATF);
        m.insert("GA", CountryCode::GAB);
        m.insert("GM", CountryCode::GMB);
        m.insert("GE", CountryCode::GEO);
        m.insert("DE", CountryCode::DEU);
        m.insert("GH", CountryCode::GHA);
        m.insert("GI", CountryCode::GIB);
        m.insert("GR", CountryCode::GRC);
        m.insert("GL", CountryCode::GRL);
        m.insert("GD", CountryCode::GRD);
        m.insert("GP", CountryCode::GLP);
        m.insert("GU", CountryCode::GUM);
        m.insert("GT", CountryCode::GTM);
        m.insert("GG", CountryCode::GGY);
        m.insert("GN", CountryCode::GIN);
        m.insert("GW", CountryCode::GNB);
        m.insert("GY", CountryCode::GUY);
        m.insert("HT", CountryCode::HTI);
        m.insert("HM", CountryCode::HMD);
        m.insert("VA", CountryCode::VAT);
        m.insert("HN", CountryCode::HND);
        m.insert("HK", CountryCode::HKG);
        m.insert("HU", CountryCode::HUN);
        m.insert("IS", CountryCode::ISL);
        m.insert("IN", CountryCode::IND);
        m.insert("ID", CountryCode::IDN);
        m.insert("IR", CountryCode::IRN);
        m.insert("IQ", CountryCode::IRQ);
        m.insert("IE", CountryCode::IRL);
        m.insert("IM", CountryCode::IMN);
        m.insert("IL", CountryCode::ISR);
        m.insert("IT", CountryCode::ITA);
        m.insert("JM", CountryCode::JAM);
        m.insert("JP", CountryCode::JPN);
        m.insert("JE", CountryCode::JEY);
        m.insert("JO", CountryCode::JOR);
        m.insert("KZ", CountryCode::KAZ);
        m.insert("KE", CountryCode::KEN);
        m.insert("KI", CountryCode::KIR);
        m.insert("KP", CountryCode::PRK);
        m.insert("KR", CountryCode::KOR);
        m.insert("KW", CountryCode::KWT);
        m.insert("KG", CountryCode::KGZ);
        m.insert("LA", CountryCode::LAO);
        m.insert("LV", CountryCode::LVA);
        m.insert("LB", CountryCode::LBN);
        m.insert("LS", CountryCode::LSO);
        m.insert("LR", CountryCode::LBR);
        m.insert("LY", CountryCode::LBY);
        m.insert("LI", CountryCode::LIE);
        m.insert("LT", CountryCode::LTU);
        m.insert("LU", CountryCode::LUX);
        m.insert("MO", CountryCode::MAC);
        m.insert("MK", CountryCode::MKD);
        m.insert("MG", CountryCode::MDG);
        m.insert("MW", CountryCode::MWI);
        m.insert("MY", CountryCode::MYS);
        m.insert("MV", CountryCode::MDV);
        m.insert("ML", CountryCode::MLI);
        m.insert("MT", CountryCode::MLT);
        m.insert("MH", CountryCode::MHL);
        m.insert("MQ", CountryCode::MTQ);
        m.insert("MR", CountryCode::MRT);
        m.insert("MU", CountryCode::MUS);
        m.insert("YT", CountryCode::MYT);
        m.insert("MX", CountryCode::MEX);
        m.insert("FM", CountryCode::FSM);
        m.insert("MD", CountryCode::MDA);
        m.insert("MC", CountryCode::MCO);
        m.insert("MN", CountryCode::MNG);
        m.insert("ME", CountryCode::MNE);
        m.insert("MS", CountryCode::MSR);
        m.insert("MA", CountryCode::MAR);
        m.insert("MZ", CountryCode::MOZ);
        m.insert("MM", CountryCode::MMR);
        m.insert("NA", CountryCode::NAM);
        m.insert("NR", CountryCode::NRU);
        m.insert("NP", CountryCode::NPL);
        m.insert("NL", CountryCode::NLD);
        m.insert("NC", CountryCode::NCL);
        m.insert("NZ", CountryCode::NZL);
        m.insert("NI", CountryCode::NIC);
        m.insert("NE", CountryCode::NER);
        m.insert("NG", CountryCode::NGA);
        m.insert("NU", CountryCode::NIU);
        m.insert("NF", CountryCode::NFK);
        m.insert("MP", CountryCode::MNP);
        m.insert("NO", CountryCode::NOR);
        m.insert("OM", CountryCode::OMN);
        m.insert("PK", CountryCode::PAK);
        m.insert("PW", CountryCode::PLW);
        m.insert("PS", CountryCode::PSE);
        m.insert("PA", CountryCode::PAN);
        m.insert("PG", CountryCode::PNG);
        m.insert("PY", CountryCode::PRY);
        m.insert("PE", CountryCode::PER);
        m.insert("PH", CountryCode::PHL);
        m.insert("PN", CountryCode::PCN);
        m.insert("PL", CountryCode::POL);
        m.insert("PT", CountryCode::PRT);
        m.insert("PR", CountryCode::PRI);
        m.insert("QA", CountryCode::QAT);
        m.insert("RE", CountryCode::REU);
        m.insert("RO", CountryCode::ROU);
        m.insert("RU", CountryCode::RUS);
        m.insert("RW", CountryCode::RWA);
        m.insert("BL", CountryCode::BLM);
        m.insert("SH", CountryCode::SHN);
        m.insert("KN", CountryCode::KNA);
        m.insert("LC", CountryCode::LCA);
        m.insert("MF", CountryCode::MAF);
        m.insert("PM", CountryCode::SPM);
        m.insert("VC", CountryCode::VCT);
        m.insert("WS", CountryCode::WSM);
        m.insert("SM", CountryCode::SMR);
        m.insert("ST", CountryCode::STP);
        m.insert("SA", CountryCode::SAU);
        m.insert("SN", CountryCode::SEN);
        m.insert("RS", CountryCode::SRB);
        m.insert("SC", CountryCode::SYC);
        m.insert("SL", CountryCode::SLE);
        m.insert("SG", CountryCode::SGP);
        m.insert("SX", CountryCode::SXM);
        m.insert("SK", CountryCode::SVK);
        m.insert("SI", CountryCode::SVN);
        m.insert("SB", CountryCode::SLB);
        m.insert("SO", CountryCode::SOM);
        m.insert("ZA", CountryCode::ZAF);
        m.insert("GS", CountryCode::SGS);
        m.insert("SS", CountryCode::SSD);
        m.insert("ES", CountryCode::ESP);
        m.insert("LK", CountryCode::LKA);
        m.insert("SD", CountryCode::SDN);
        m.insert("SR", CountryCode::SUR);
        m.insert("SJ", CountryCode::SJM);
        m.insert("SZ", CountryCode::SWZ);
        m.insert("SE", CountryCode::SWE);
        m.insert("CH", CountryCode::CHE);
        m.insert("SY", CountryCode::SYR);
        m.insert("TW", CountryCode::TWN);
        m.insert("TJ", CountryCode::TJK);
        m.insert("TZ", CountryCode::TZA);
        m.insert("TH", CountryCode::THA);
        m.insert("TG", CountryCode::TGO);
        m.insert("TK", CountryCode::TKL);
        m.insert("TO", CountryCode::TON);
        m.insert("TT", CountryCode::TTO);
        m.insert("TN", CountryCode::TUN);
        m.insert("TR", CountryCode::TUR);
        m.insert("TM", CountryCode::TKM);
        m.insert("TC", CountryCode::TCA);
        m.insert("TV", CountryCode::TUV);
        m.insert("VI", CountryCode::VIR);
        m.insert("UG", CountryCode::UGA);
        m.insert("UA", CountryCode::UKR);
        m.insert("AE", CountryCode::ARE);
        m.insert("GB", CountryCode::GBR);
        m.insert("US", CountryCode::USA);
        m.insert("UM", CountryCode::UMI);
        m.insert("UY", CountryCode::URY);
        m.insert("UZ", CountryCode::UZB);
        m.insert("VU", CountryCode::VUT);
        m.insert("VE", CountryCode::VEN);
        m.insert("VN", CountryCode::VNM);
        m.insert("WF", CountryCode::WLF);
        m.insert("EH", CountryCode::ESH);
        m.insert("YE", CountryCode::YEM);
        m.insert("ZM", CountryCode::ZMB);
        m.insert("ZW", CountryCode::ZWE);
        m.insert("XK", CountryCode::XKX);

        m
    };
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_derives() {
        let left = CountryCode::CHE;
        let right = CountryCode::CHE;
        assert!(left == right)
    }

    #[test]
    pub fn test_amounts() {
        assert!(COUNTRIES_ISO_3_CODES.len() == COUNTRIES_ISO_3_CODES.len())
    }

    #[test]
    pub fn check_if_we_have_whatever_in_iso3_in_iso2() {
        for code in COUNTRIES_ISO_3_CODES.values() {
            if !find_iso_2_code(code) {
                panic!("Can not find code {:?} in ISO2", code);
            }
        }
    }

    fn find_iso_2_code(code: &CountryCode) -> bool {
        for code_2 in COUNTRIES_ISO_2_CODES.values() {
            if code.equals_to(*code_2) {
                return true;
            }
        }

        false
    }
}
