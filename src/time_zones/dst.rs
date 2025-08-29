use std::collections::HashMap;

use crate::country_code::CountryCode;

lazy_static::lazy_static! {
    pub static ref DST: HashMap<CountryCode, bool> = {
        create_dst_map()
    };
}

// Function to create a HashMap of countries (ISO3 codes) and their DST status
fn create_dst_map() -> HashMap<CountryCode, bool> {
    let mut dst_map: HashMap<CountryCode, bool> = HashMap::new();

    // Countries observing DST (true)
    // Europe (most EU/EEA countries observe DST, typically March to October)
    dst_map.insert(CountryCode::AUT, true); // Austria
    dst_map.insert(CountryCode::BEL, true); // Belgium
    dst_map.insert(CountryCode::BGR, true); // Bulgaria
    dst_map.insert(CountryCode::HRV, true); // Croatia
    dst_map.insert(CountryCode::CYP, true); // Cyprus
    dst_map.insert(CountryCode::CZE, true); // Czech Republic
    dst_map.insert(CountryCode::DNK, true); // Denmark
    dst_map.insert(CountryCode::EST, true); // Estonia
    dst_map.insert(CountryCode::FIN, true); // Finland
    dst_map.insert(CountryCode::FRA, true); // France
    dst_map.insert(CountryCode::DEU, true); // Germany
    dst_map.insert(CountryCode::GRC, true); // Greece
    dst_map.insert(CountryCode::HUN, true); // Hungary
    dst_map.insert(CountryCode::IRL, true); // Ireland
    dst_map.insert(CountryCode::ITA, true); // Italy
    dst_map.insert(CountryCode::LVA, true); // Latvia
    dst_map.insert(CountryCode::LTU, true); // Lithuania
    dst_map.insert(CountryCode::LUX, true); // Luxembourg
    dst_map.insert(CountryCode::MLT, true); // Malta
    dst_map.insert(CountryCode::NLD, true); // Netherlands
    dst_map.insert(CountryCode::POL, true); // Poland
    dst_map.insert(CountryCode::PRT, true); // Portugal
    dst_map.insert(CountryCode::ROU, true); // Romania
    dst_map.insert(CountryCode::SVK, true); // Slovakia
    dst_map.insert(CountryCode::SVN, true); // Slovenia
    dst_map.insert(CountryCode::ESP, true); // Spain
    dst_map.insert(CountryCode::SWE, true); // Sweden
    dst_map.insert(CountryCode::GBR, true); // United Kingdom
    dst_map.insert(CountryCode::NOR, true); // Norway (EEA)
    dst_map.insert(CountryCode::CHE, true); // Switzerland (follows EU DST)
                                            // North America
    dst_map.insert(CountryCode::USA, true); // United States (except Hawaii, most of Arizona)
    dst_map.insert(CountryCode::CAN, true); // Canada (except Yukon, most of Saskatchewan)
    dst_map.insert(CountryCode::CUB, true); // Cuba
    dst_map.insert(CountryCode::HTI, true); // Haiti
    dst_map.insert(CountryCode::MEX, true); // Mexico (only certain border cities, e.g., Tijuana)
                                            // South America
    dst_map.insert(CountryCode::CHL, true); // Chile (except Aysén, Magallanes)
    dst_map.insert(CountryCode::PRY, true); // Paraguay
                                            // Oceania
    dst_map.insert(CountryCode::AUS, true); // Australia (NSW, VIC, SA, TAS, ACT only)
    dst_map.insert(CountryCode::NZL, true); // New Zealand
                                            // Middle East and Africa
    dst_map.insert(CountryCode::EGY, true); // Egypt (reintroduced 2023)
    dst_map.insert(CountryCode::ISR, true); // Israel
    dst_map.insert(CountryCode::LBN, true); // Lebanon
    dst_map.insert(CountryCode::MAR, true); // Morocco (effectively permanent DST, except Ramadan)
    dst_map.insert(CountryCode::SYR, true); // Syria
    dst_map.insert(CountryCode::GRL, true); // Greenland (follows EU DST)

    // Countries not observing DST (false)
    // Europe
    dst_map.insert(CountryCode::ARM, false); // Armenia
    dst_map.insert(CountryCode::AZE, false); // Azerbaijan
    dst_map.insert(CountryCode::BLR, false); // Belarus
    dst_map.insert(CountryCode::GEO, false); // Georgia
    dst_map.insert(CountryCode::ISL, false); // Iceland
    dst_map.insert(CountryCode::RUS, false); // Russia (abolished 2014)
    dst_map.insert(CountryCode::TUR, false); // Turkey (abolished 2016)
                                             // Asia
    dst_map.insert(CountryCode::AFG, false); // Afghanistan
    dst_map.insert(CountryCode::BHR, false); // Bahrain
    dst_map.insert(CountryCode::BGD, false); // Bangladesh
    dst_map.insert(CountryCode::BTN, false); // Bhutan
    dst_map.insert(CountryCode::BRN, false); // Brunei
    dst_map.insert(CountryCode::KHM, false); // Cambodia
    dst_map.insert(CountryCode::CHN, false); // China
    dst_map.insert(CountryCode::IND, false); // India
    dst_map.insert(CountryCode::IDN, false); // Indonesia
    dst_map.insert(CountryCode::IRN, false); // Iran (abolished 2022)
    dst_map.insert(CountryCode::IRQ, false); // Iraq
    dst_map.insert(CountryCode::JPN, false); // Japan
    dst_map.insert(CountryCode::JOR, false); // Jordan
    dst_map.insert(CountryCode::KAZ, false); // Kazakhstan
    dst_map.insert(CountryCode::KWT, false); // Kuwait
    dst_map.insert(CountryCode::KGZ, false); // Kyrgyzstan
    dst_map.insert(CountryCode::LAO, false); // Laos
    dst_map.insert(CountryCode::MYS, false); // Malaysia
    dst_map.insert(CountryCode::MDV, false); // Maldives
    dst_map.insert(CountryCode::MNG, false); // Mongolia
    dst_map.insert(CountryCode::MMR, false); // Myanmar
    dst_map.insert(CountryCode::NPL, false); // Nepal
    dst_map.insert(CountryCode::OMN, false); // Oman
    dst_map.insert(CountryCode::PAK, false); // Pakistan
    dst_map.insert(CountryCode::PHL, false); // Philippines
    dst_map.insert(CountryCode::QAT, false); // Qatar
    dst_map.insert(CountryCode::SAU, false); // Saudi Arabia
    dst_map.insert(CountryCode::SGP, false); // Singapore
    dst_map.insert(CountryCode::KOR, false); // South Korea
    dst_map.insert(CountryCode::LKA, false); // Sri Lanka
    dst_map.insert(CountryCode::TWN, false); // Taiwan
    dst_map.insert(CountryCode::THA, false); // Thailand
    dst_map.insert(CountryCode::TLS, false); // Timor-Leste
    dst_map.insert(CountryCode::ARE, false); // United Arab Emirates
    dst_map.insert(CountryCode::UZB, false); // Uzbekistan
    dst_map.insert(CountryCode::VNM, false); // Vietnam
                                             // Africa
    dst_map.insert(CountryCode::DZA, false); // Algeria
    dst_map.insert(CountryCode::AGO, false); // Angola
    dst_map.insert(CountryCode::BEN, false); // Benin
    dst_map.insert(CountryCode::BWA, false); // Botswana
    dst_map.insert(CountryCode::BFA, false); // Burkina Faso
    dst_map.insert(CountryCode::BDI, false); // Burundi
    dst_map.insert(CountryCode::CMR, false); // Cameroon
    dst_map.insert(CountryCode::CAF, false); // Central African Republic
    dst_map.insert(CountryCode::TCD, false); // Chad
    dst_map.insert(CountryCode::COD, false); // DR Congo
    dst_map.insert(CountryCode::COG, false); // Congo
    dst_map.insert(CountryCode::CIV, false); // Côte d'Ivoire
    dst_map.insert(CountryCode::DJI, false); // Djibouti
    dst_map.insert(CountryCode::ERI, false); // Eritrea
    dst_map.insert(CountryCode::ETH, false); // Ethiopia
    dst_map.insert(CountryCode::GAB, false); // Gabon
    dst_map.insert(CountryCode::GMB, false); // Gambia
    dst_map.insert(CountryCode::GHA, false); // Ghana
    dst_map.insert(CountryCode::GIN, false); // Guinea
    dst_map.insert(CountryCode::KEN, false); // Kenya
    dst_map.insert(CountryCode::LSO, false); // Lesotho
    dst_map.insert(CountryCode::LBR, false); // Liberia
    dst_map.insert(CountryCode::LBY, false); // Libya
    dst_map.insert(CountryCode::MDG, false); // Madagascar
    dst_map.insert(CountryCode::MWI, false); // Malawi
    dst_map.insert(CountryCode::MLI, false); // Mali
    dst_map.insert(CountryCode::MRT, false); // Mauritania
    dst_map.insert(CountryCode::MOZ, false); // Mozambique
    dst_map.insert(CountryCode::NAM, false); // Namibia (abolished 2017)
    dst_map.insert(CountryCode::NER, false); // Niger
    dst_map.insert(CountryCode::NGA, false); // Nigeria
    dst_map.insert(CountryCode::RWA, false); // Rwanda
    dst_map.insert(CountryCode::SEN, false); // Senegal
    dst_map.insert(CountryCode::SLE, false); // Sierra Leone
    dst_map.insert(CountryCode::SOM, false); // Somalia
    dst_map.insert(CountryCode::ZAF, false); // South Africa
    dst_map.insert(CountryCode::SSD, false); // South Sudan
    dst_map.insert(CountryCode::SDN, false); // Sudan
    dst_map.insert(CountryCode::TGO, false); // Togo
    dst_map.insert(CountryCode::TUN, false); // Tunisia
    dst_map.insert(CountryCode::UGA, false); // Uganda
    dst_map.insert(CountryCode::ZMB, false); // Zambia
    dst_map.insert(CountryCode::ZWE, false); // Zimbabwe
                                             // South America
    dst_map.insert(CountryCode::ARG, false); // Argentina
    dst_map.insert(CountryCode::BOL, false); // Bolivia
    dst_map.insert(CountryCode::BRA, false); // Brazil (abolished 2019)
    dst_map.insert(CountryCode::COL, false); // Colombia
    dst_map.insert(CountryCode::ECU, false); // Ecuador
    dst_map.insert(CountryCode::GUY, false); // Guyana
    dst_map.insert(CountryCode::PER, false); // Peru
    dst_map.insert(CountryCode::SUR, false); // Suriname
    dst_map.insert(CountryCode::URY, false); // Uruguay (abolished 2015)
    dst_map.insert(CountryCode::VEN, false); // Venezuela
                                             // Oceania
    dst_map.insert(CountryCode::FJI, false); // Fiji
    dst_map.insert(CountryCode::PNG, false); // Papua New Guinea
    dst_map.insert(CountryCode::WSM, false); // Samoa
    dst_map.insert(CountryCode::VUT, false); // Vanuatu
                                             // Caribbean and Other
    dst_map.insert(CountryCode::ATG, false); // Antigua and Barbuda
    dst_map.insert(CountryCode::BHS, false); // Bahamas
    dst_map.insert(CountryCode::BRB, false); // Barbados
    dst_map.insert(CountryCode::BLZ, false); // Belize
    dst_map.insert(CountryCode::DMA, false); // Dominica
    dst_map.insert(CountryCode::GRD, false); // Grenada
    dst_map.insert(CountryCode::GTM, false); // Guatemala
    dst_map.insert(CountryCode::HND, false); // Honduras
    dst_map.insert(CountryCode::JAM, false); // Jamaica
    dst_map.insert(CountryCode::NIC, false); // Nicaragua
    dst_map.insert(CountryCode::PAN, false); // Panama
    dst_map.insert(CountryCode::KNA, false); // Saint Kitts and Nevis
    dst_map.insert(CountryCode::LCA, false); // Saint Lucia
    dst_map.insert(CountryCode::VCT, false); // Saint Vincent and the Grenadines
    dst_map.insert(CountryCode::TTO, false); // Trinidad and Tobago

    dst_map
}
