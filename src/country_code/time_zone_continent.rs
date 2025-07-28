use rust_extensions::ShortString;

use crate::country_code::CountryCode;
#[cfg(feature = "time-zones")]
use crate::time_zones::TimeZoneOffset;

const EUROPE_PREFIX: &str = "Europe/";
const AFRICA_PREFIX: &str = "Africa/";
const ASIA_PREFIX: &str = "Asia/";
const AMERICA_PREFIX: &str = "America/";
const ANTARCTICA_PREFIX: &str = "Antarctica/";
const ARCTIC_PREFIX: &str = "Arctic/";
const ATLANTIC_PREFIX: &str = "Atlantic/";
const AUSTRALIA_PREFIX: &str = "Australia/";
const INDIAN_PREFIX: &str = "Indian/";
const PACIFIC_PREFIX: &str = "Pacific/";

pub enum IanaTimeZone<'s> {
    Europe(&'s str),
    Africa(&'s str),
    Asia(&'s str),
    America(&'s str),
    Antarctica(&'s str),
    Arctic(&'s str),
    Atlantic(&'s str),
    Australia(&'s str),
    Indian(&'s str),
    Pacific(&'s str),
}

impl<'s> IanaTimeZone<'s> {
    pub fn try_from_str(src: &'s str) -> Option<Self> {
        let slash_index = src.find('/')?;

        let (continent, country) = src.split_at(slash_index + 1);

        match continent {
            EUROPE_PREFIX => Some(IanaTimeZone::Europe(country)),
            AFRICA_PREFIX => Some(IanaTimeZone::Africa(country)),
            ASIA_PREFIX => Some(IanaTimeZone::Asia(country)),
            AMERICA_PREFIX => Some(IanaTimeZone::America(country)),
            ANTARCTICA_PREFIX => Some(IanaTimeZone::Antarctica(country)),
            ARCTIC_PREFIX => Some(IanaTimeZone::Arctic(country)),
            ATLANTIC_PREFIX => Some(IanaTimeZone::Atlantic(country)),
            AUSTRALIA_PREFIX => Some(IanaTimeZone::Australia(country)),
            INDIAN_PREFIX => Some(IanaTimeZone::Indian(country)),
            PACIFIC_PREFIX => Some(IanaTimeZone::Pacific(country)),
            _ => None,
        }
    }

    #[cfg(feature = "time-zones")]
    pub fn get_fallback_timezone(time_zone: TimeZoneOffset) -> Self {
        use crate::time_zones::*;
        let seconds = time_zone.as_seconds();
        match seconds {
            UTC_MINUS_12 => IanaTimeZone::Pacific("Baker"),
            UTC_MINUS_11 => IanaTimeZone::Pacific("Niue"),
            UTC_MINUS_10 => IanaTimeZone::Pacific("Honolulu"),
            UTC_MINUS_9 => IanaTimeZone::America("Adak"),
            UTC_MINUS_8 => IanaTimeZone::America("Pitcairn"),
            UTC_MINUS_7 => IanaTimeZone::America("Phoenix"),
            UTC_MINUS_6 => IanaTimeZone::America("Regina"),
            UTC_MINUS_5 => IanaTimeZone::America("Guayaquil"),
            UTC_MINUS_4 => IanaTimeZone::America("La_Paz"),
            UTC_MINUS_3 => IanaTimeZone::America("Argentina/Buenos_Aires"),
            UTC_MINUS_2 => IanaTimeZone::America("South_Georgia"),
            UTC_MINUS_1 => IanaTimeZone::Atlantic("Azores"),
            UTC_0 => IanaTimeZone::Atlantic("Reykjavik"),
            UTC_1 => IanaTimeZone::Europe("Algiers"),
            UTC_2 => IanaTimeZone::Europe("Johannesburg"),
            UTC_3 => IanaTimeZone::Africa("Moscow"),
            UTC_4 => IanaTimeZone::Asia("Dubai"),
            UTC_5 => IanaTimeZone::Asia("Karachi"),
            UTC_5_30 => IanaTimeZone::Asia("Kolkata"),
            UTC_6 => IanaTimeZone::Asia("Dhaka"),
            UTC_7 => IanaTimeZone::Asia("Jakarta"),
            UTC_8 => IanaTimeZone::Asia("Shanghai"),
            UTC_8_45 => IanaTimeZone::Australia("Ecula"),
            UTC_9 => IanaTimeZone::Asia("Tokyo"),
            UTC_10 => IanaTimeZone::Australia("Brisbane"),
            UTC_11 => IanaTimeZone::Pacific("Noumea"),
            UTC_12 => IanaTimeZone::Pacific("Fiji"),
            UTC_13 => IanaTimeZone::Pacific("Tongatapu"),
            UTC_14 => IanaTimeZone::Pacific("Kiritimati"),
            _ => IanaTimeZone::Europe("Dubai"),
        }
    }

    #[cfg(feature = "time-zones")]
    pub fn from_country_code(
        country_code: CountryCode,
        time_zone: TimeZoneOffset,
        is_day_light_saving: bool,
    ) -> Option<Self> {
        match country_code {
            CountryCode::AFG => Self::Asia("Kabul").into(), // Afghanistan
            CountryCode::ALA => Self::Europe("Mariehamn").into(), // Åland Islands
            CountryCode::ALB => Self::Europe("Tirane").into(), // Albania
            CountryCode::DZA => Self::Africa("Algiers").into(), // Algeria
            CountryCode::ASM => Self::Pacific("Pago_Pago").into(), // American Samoa
            CountryCode::AND => Self::Europe("Andorra").into(), // Andorra
            CountryCode::AGO => Self::Africa("Luanda").into(), // Angola
            CountryCode::AIA => Self::America("Anguilla").into(), // Anguilla
            CountryCode::ATA => Self::Antarctica("McMurdo").into(), // Antarctica (primary station)
            CountryCode::ATG => Self::America("Antigua").into(), // Antigua and Barbuda
            CountryCode::ARG => Self::America("Argentina/Buenos_Aires").into(), // Argentina
            CountryCode::ARM => Self::Asia("Yerevan").into(), // Armenia
            CountryCode::ABW => Self::America("Aruba").into(), // Aruba
            CountryCode::AUS => get_australia_time_zone(time_zone, is_day_light_saving), // Australia
            CountryCode::AUT => Self::Europe("Vienna").into(),                           // Austria
            CountryCode::AZE => Self::Asia("Baku").into(), // Azerbaijan
            CountryCode::BHS => Self::America("Nassau").into(), // Bahamas
            CountryCode::BHR => Self::Asia("Bahrain").into(), // Bahrain
            CountryCode::BGD => Self::Asia("Dhaka").into(), // Bangladesh
            CountryCode::BRB => Self::America("Barbados").into(), // Barbados
            CountryCode::BLR => Self::Europe("Minsk").into(), // Belarus
            CountryCode::BEL => Self::Europe("Brussels").into(), // Belgium
            CountryCode::BLZ => Self::America("Belize").into(), // Belize
            CountryCode::BEN => Self::Africa("Porto-Novo").into(), // Benin
            CountryCode::BMU => Self::Atlantic("Bermuda").into(), // Bermuda
            CountryCode::BTN => Self::Asia("Thimphu").into(), // Bhutan
            CountryCode::BOL => Self::America("La_Paz").into(), // Bolivia
            CountryCode::BES => Self::America("Kralendijk").into(), // Bonaire, Sint Eustatius, Saba
            CountryCode::BIH => Self::Europe("Sarajevo").into(), // Bosnia and Herzegovina
            CountryCode::BWA => Self::Africa("Gaborone").into(), // Botswana
            CountryCode::BVT => Self::Atlantic("Reykjavik").into(), // Bouvet Island (uninhabited, uses UTC)
            CountryCode::BRA => get_brazil_time_zone(time_zone),    // Brazil
            CountryCode::IOT => Self::Indian("Chagos").into(), // British Indian Ocean Territory
            CountryCode::VGB => Self::America("Tortola").into(), // British Virgin Islands
            CountryCode::BRN => Self::Asia("Brunei").into(),   // Brunei
            CountryCode::BGR => Self::Europe("Sofia").into(),  // Bulgaria
            CountryCode::BFA => Self::Africa("Ouagadougou").into(), // Burkina Faso
            CountryCode::BDI => Self::Africa("Bujumbura").into(), // Burundi
            CountryCode::KHM => Self::Asia("Phnom_Penh").into(), // Cambodia
            CountryCode::CMR => Self::Africa("Douala").into(), // Cameroon
            CountryCode::CAN => get_canada_time_zone(time_zone, is_day_light_saving), // Canada
            CountryCode::CPV => Self::Atlantic("Cape_Verde").into(), // Cape Verde
            CountryCode::CYM => Self::America("Cayman").into(), // Cayman Islands
            CountryCode::CAF => Self::Africa("Bangui").into(), // Central African Republic
            CountryCode::TCD => Self::Africa("Ndjamena").into(), // Chad
            CountryCode::CHL => get_chile_time_zone(time_zone, is_day_light_saving), // Chile
            CountryCode::CHN => get_china_time_zone(time_zone), // China
            CountryCode::CXR => Self::Indian("Christmas").into(), // Christmas Island
            CountryCode::CCK => Self::Indian("Cocos").into(),  // Cocos Islands
            CountryCode::COL => Self::America("Bogota").into(), // Colombia
            CountryCode::COM => Self::Indian("Comoro").into(), // Comoros
            CountryCode::COK => Self::Pacific("Rarotonga").into(), // Cook Islands
            CountryCode::CRI => Self::America("Costa_Rica").into(), // Costa Rica
            CountryCode::HRV => Self::Europe("Zagreb").into(), // Croatia
            CountryCode::CUB => Self::America("Havana").into(), // Cuba
            CountryCode::CUW => Self::America("Curacao").into(), // Curaçao
            CountryCode::CYP => Self::Asia("Nicosia").into(),  // Cyprus
            CountryCode::CZE => Self::Europe("Prague").into(), // Czech Republic
            CountryCode::COD => Self::Africa("Kinshasa").into(), // Democratic Republic of the Congo
            CountryCode::DNK => Self::Europe("Copenhagen").into(), // Denmark
            CountryCode::DJI => Self::Africa("Djibouti").into(), // Djibouti
            CountryCode::DMA => Self::America("Dominica").into(), // Dominica
            CountryCode::DOM => Self::America("Santo_Domingo").into(), // Dominican Republic
            CountryCode::TLS => Self::Asia("Dili").into(),     // Timor-Leste
            CountryCode::ECU => Self::America("Guayaquil").into(), // Ecuador
            CountryCode::EGY => Self::Africa("Cairo").into(),  // Egypt
            CountryCode::SLV => Self::America("El_Salvador").into(), // El Salvador
            CountryCode::GNQ => Self::Africa("Malabo").into(), // Equatorial Guinea
            CountryCode::ERI => Self::Africa("Asmara").into(), // Eritrea
            CountryCode::EST => Self::Europe("Tallinn").into(), // Estonia
            CountryCode::ETH => Self::Africa("Addis_Ababa").into(), // Ethiopia
            CountryCode::FLK => Self::Atlantic("Stanley").into(), // Falkland Islands
            CountryCode::FRO => Self::Atlantic("Faeroe").into(), // Faroe Islands
            CountryCode::FJI => Self::Pacific("Fiji").into(),  // Fiji
            CountryCode::FIN => Self::Europe("Helsinki").into(), // Finland
            CountryCode::FRA => Self::Europe("Paris").into(),  // France
            CountryCode::GUF => Self::America("Cayenne").into(), // French Guiana
            CountryCode::PYF => Self::Pacific("Tahiti").into(), // French Polynesia
            CountryCode::ATF => Self::Indian("Kerguelen").into(), // French Southern Territories
            CountryCode::GAB => Self::Africa("Libreville").into(), // Gabon
            CountryCode::GMB => Self::Africa("Banjul").into(), // Gambia
            CountryCode::GEO => Self::Asia("Tbilisi").into(),  // Georgia
            CountryCode::DEU => Self::Europe("Berlin").into(), // Germany
            CountryCode::GHA => Self::Africa("Accra").into(),  // Ghana
            CountryCode::GIB => Self::Europe("Gibraltar").into(), // Gibraltar
            CountryCode::GRC => Self::Europe("Athens").into(), // Greece
            CountryCode::GRL => Self::America("Nuuk").into(),  // Greenland
            CountryCode::GRD => Self::America("Grenada").into(), // Grenada
            CountryCode::GLP => Self::America("Guadeloupe").into(), // Guadeloupe
            CountryCode::GUM => Self::Pacific("Guam").into(),  // Guam
            CountryCode::GTM => Self::America("Guatemala").into(), // Guatemala
            CountryCode::GGY => Self::Europe("Guernsey").into(), // Guernsey
            CountryCode::GIN => Self::Africa("Conakry").into(), // Guinea
            CountryCode::GNB => Self::Africa("Bissau").into(), // Guinea-Bissau
            CountryCode::GUY => Self::America("Guyana").into(), // Guyana
            CountryCode::HTI => Self::America("Port-au-Prince").into(), // Haiti
            CountryCode::HMD => Self::Indian("Kerguelen").into(), // Heard Island and McDonald Islands (uses ATF)
            CountryCode::HND => Self::America("Tegucigalpa").into(), // Honduras
            CountryCode::HKG => Self::Asia("Hong_Kong").into(),   // Hong Kong
            CountryCode::HUN => Self::Europe("Budapest").into(),  // Hungary
            CountryCode::ISL => Self::Atlantic("Reykjavik").into(), // Iceland
            CountryCode::IND => Self::Asia("Kolkata").into(),     // India
            CountryCode::IDN => get_indonesia_time_zone(time_zone), // Indonesia
            CountryCode::IRN => Self::Asia("Tehran").into(),      // Iran
            CountryCode::IRQ => Self::Asia("Baghdad").into(),     // Iraq
            CountryCode::IRL => Self::Europe("Dublin").into(),    // Ireland
            CountryCode::IMN => Self::Europe("Isle_of_Man").into(), // Isle of Man
            CountryCode::ISR => Self::Asia("Jerusalem").into(),   // Israel
            CountryCode::ITA => Self::Europe("Rome").into(),      // Italy
            CountryCode::CIV => Self::Africa("Abidjan").into(),   // Côte d'Ivoire
            CountryCode::JAM => Self::America("Jamaica").into(),  // Jamaica
            CountryCode::JPN => Self::Asia("Tokyo").into(),       // Japan
            CountryCode::JEY => Self::Europe("Jersey").into(),    // Jersey
            CountryCode::JOR => Self::Asia("Amman").into(),       // Jordan
            CountryCode::KAZ => Self::Asia("Almaty").into(),      // Kazakhstan
            CountryCode::KEN => Self::Africa("Nairobi").into(),   // Kenya
            CountryCode::KIR => Self::Pacific("Tarawa").into(),   // Kiribati
            CountryCode::KWT => Self::Asia("Kuwait").into(),      // Kuwait
            CountryCode::KGZ => Self::Asia("Bishkek").into(),     // Kyrgyzstan
            CountryCode::LAO => Self::Asia("Vientiane").into(),   // Laos
            CountryCode::LVA => Self::Europe("Riga").into(),      // Latvia
            CountryCode::LBN => Self::Asia("Beirut").into(),      // Lebanon
            CountryCode::LSO => Self::Africa("Maseru").into(),    // Lesotho
            CountryCode::LBR => Self::Africa("Monrovia").into(),  // Liberia
            CountryCode::LBY => Self::Africa("Tripoli").into(),   // Libya
            CountryCode::LIE => Self::Europe("Vaduz").into(),     // Liechtenstein
            CountryCode::LTU => Self::Europe("Vilnius").into(),   // Lithuania
            CountryCode::LUX => Self::Europe("Luxembourg").into(), // Luxembourg
            CountryCode::MAC => Self::Asia("Macau").into(),       // Macau
            CountryCode::MKD => Self::Europe("Skopje").into(),    // North Macedonia
            CountryCode::MDG => Self::Indian("Antananarivo").into(), // Madagascar
            CountryCode::MWI => Self::Africa("Blantyre").into(),  // Malawi
            CountryCode::MYS => Self::Asia("Kuala_Lumpur").into(), // Malaysia
            CountryCode::MDV => Self::Indian("Maldives").into(),  // Maldives
            CountryCode::MLI => Self::Africa("Bamako").into(),    // Mali
            CountryCode::MLT => Self::Europe("Malta").into(),     // Malta
            CountryCode::MHL => Self::Pacific("Majuro").into(),   // Marshall Islands
            CountryCode::MTQ => Self::America("Martinique").into(), // Martinique
            CountryCode::MRT => Self::Africa("Nouakchott").into(), // Mauritania
            CountryCode::MUS => Self::Indian("Mauritius").into(), // Mauritius
            CountryCode::MYT => Self::Indian("Mayotte").into(),   // Mayotte
            CountryCode::MEX => get_mexico_time_zone(time_zone, is_day_light_saving),
            CountryCode::FSM => Self::Pacific("Pohnpei").into(), // Micronesia
            CountryCode::MDA => Self::Europe("Chisinau").into(), // Moldova
            CountryCode::MCO => Self::Europe("Monaco").into(),   // Monaco
            CountryCode::MNG => get_mongolia_time_zone(time_zone), // Mongolia
            CountryCode::MNE => Self::Europe("Podgorica").into(), // Montenegro
            CountryCode::MSR => Self::America("Montserrat").into(), // Montserrat
            CountryCode::MAR => Self::Africa("Casablanca").into(), // Morocco
            CountryCode::MOZ => Self::Africa("Maputo").into(),   // Mozambique
            CountryCode::MMR => Self::Asia("Yangon").into(),     // Myanmar
            CountryCode::NAM => Self::Africa("Windhoek").into(), // Namibia
            CountryCode::NRU => Self::Pacific("Nauru").into(),   // Nauru
            CountryCode::NPL => Self::Asia("Kathmandu").into(),  // Nepal
            CountryCode::NLD => Self::Europe("Amsterdam").into(), // Netherlands
            CountryCode::NCL => Self::Pacific("Noumea").into(),  // New Caledonia
            CountryCode::NZL => Self::Pacific("Auckland").into(), // New Zealand
            CountryCode::NIC => Self::America("Managua").into(), // Nicaragua
            CountryCode::NER => Self::Africa("Niamey").into(),   // Niger
            CountryCode::NGA => Self::Africa("Lagos").into(),    // Nigeria
            CountryCode::NIU => Self::Pacific("Niue").into(),    // Niue
            CountryCode::NFK => Self::Pacific("Norfolk").into(), // Norfolk Island
            CountryCode::PRK => Self::Asia("Pyongyang").into(),  // North Korea
            CountryCode::MNP => Self::Pacific("Saipan").into(),  // Northern Mariana Islands
            CountryCode::NOR => Self::Europe("Oslo").into(),     // Norway
            CountryCode::OMN => Self::Asia("Muscat").into(),     // Oman
            CountryCode::PAK => Self::Asia("Karachi").into(),    // Pakistan
            CountryCode::PLW => Self::Pacific("Palau").into(),   // Palau
            CountryCode::PSE => Self::Asia("Gaza").into(),       // Palestine
            CountryCode::PAN => Self::America("Panama").into(),  // Panama
            CountryCode::PNG => Self::Pacific("Port_Moresby").into(), // Papua New Guinea
            CountryCode::PRY => Self::America("Asuncion").into(), // Paraguay
            CountryCode::PER => Self::America("Lima").into(),    // Peru
            CountryCode::PHL => Self::Asia("Manila").into(),     // Philippines
            CountryCode::PCN => Self::Pacific("Pitcairn").into(), // Pitcairn Islands
            CountryCode::POL => Self::Europe("Warsaw").into(),   // Poland
            CountryCode::PRT => Self::Europe("Lisbon").into(),   // Portugal
            CountryCode::PRI => Self::America("Puerto_Rico").into(), // Puerto Rico
            CountryCode::QAT => Self::Asia("Qatar").into(),      // Qatar
            CountryCode::COG => Self::Africa("Brazzaville").into(), // Republic of the Congo
            CountryCode::REU => Self::Indian("Reunion").into(),  // Réunion
            CountryCode::ROU => Self::Europe("Bucharest").into(), // Romania
            CountryCode::RUS => get_rus_time_zone(time_zone).into(), // Russia
            CountryCode::RWA => Self::Africa("Kigali").into(),   // Rwanda
            CountryCode::BLM => Self::America("St_Barthelemy").into(), // Saint Barthélemy
            CountryCode::SHN => Self::Atlantic("St_Helena").into(), // Saint Helena
            CountryCode::KNA => Self::America("St_Kitts").into(), // Saint Kitts and Nevis
            CountryCode::LCA => Self::America("St_Lucia").into(), // Saint Lucia
            CountryCode::MAF => Self::America("Marigot").into(), // Saint Martin
            CountryCode::SPM => Self::America("Miquelon").into(), // Saint Pierre and Miquelon
            CountryCode::VCT => Self::America("St_Vincent").into(), // Saint Vincent and the Grenadines
            CountryCode::WSM => Self::Pacific("Apia").into(),       // Samoa
            CountryCode::SMR => Self::Europe("San_Marino").into(),  // San Marino
            CountryCode::STP => Self::Africa("Sao_Tome").into(),    // São Tomé and Príncipe
            CountryCode::SAU => Self::Asia("Riyadh").into(),        // Saudi Arabia
            CountryCode::SEN => Self::Africa("Dakar").into(),       // Senegal
            CountryCode::SRB => Self::Europe("Belgrade").into(),    // Serbia
            CountryCode::SYC => Self::Indian("Mahe").into(),        // Seychelles
            CountryCode::SLE => Self::Africa("Freetown").into(),    // Sierra Leone
            CountryCode::SGP => Self::Asia("Singapore").into(),     // Singapore
            CountryCode::SXM => Self::America("Lower_Princes").into(), // Sint Maarten
            CountryCode::SVK => Self::Europe("Bratislava").into(),  // Slovakia
            CountryCode::SVN => Self::Europe("Ljubljana").into(),   // Slovenia
            CountryCode::SLB => Self::Pacific("Guadalcanal").into(), // Solomon Islands
            CountryCode::SOM => Self::Africa("Mogadishu").into(),   // Somalia
            CountryCode::ZAF => Self::Africa("Johannesburg").into(), // South Africa
            CountryCode::SGS => Self::Atlantic("South_Georgia").into(), // South Georgia
            CountryCode::KOR => Self::Asia("Seoul").into(),         // South Korea
            CountryCode::SSD => Self::Africa("Juba").into(),        // South Sudan
            CountryCode::ESP => Self::Europe("Madrid").into(),      // Spain
            CountryCode::LKA => Self::Asia("Colombo").into(),       // Sri Lanka
            CountryCode::SDN => Self::Africa("Khartoum").into(),    // Sudan
            CountryCode::SUR => Self::America("Paramaribo").into(), // Suriname
            CountryCode::SJM => Self::Arctic("Longyearbyen").into(), // Svalbard and Jan Mayen
            CountryCode::SWZ => Self::Africa("Mbabane").into(),     // Eswatini
            CountryCode::SWE => Self::Europe("Stockholm").into(),   // Sweden
            CountryCode::CHE => Self::Europe("Zurich").into(),      // Switzerland
            CountryCode::SYR => Self::Asia("Damascus").into(),      // Syria
            CountryCode::TWN => Self::Asia("Taipei").into(),        // Taiwan
            CountryCode::TJK => Self::Asia("Dushanbe").into(),      // Tajikistan
            CountryCode::TZA => Self::Africa("Dar_es_Salaam").into(), // Tanzania
            CountryCode::THA => Self::Asia("Bangkok").into(),       // Thailand
            CountryCode::TGO => Self::Africa("Lome").into(),        // Togo
            CountryCode::TKL => Self::Pacific("Fakaofo").into(),    // Tokelau
            CountryCode::TON => Self::Pacific("Tongatapu").into(),  // Tonga
            CountryCode::TTO => Self::America("Port_of_Spain").into(), // Trinidad and Tobago
            CountryCode::TUN => Self::Africa("Tunis").into(),       // Tunisia
            CountryCode::TUR => Self::Europe("Istanbul").into(),    // Turkey
            CountryCode::TKM => Self::Asia("Ashgabat").into(),      // Turkmenistan
            CountryCode::TCA => Self::America("Grand_Turk").into(), // Turks and Caicos Islands
            CountryCode::TUV => Self::Pacific("Funafuti").into(),   // Tuvalu
            CountryCode::VIR => Self::America("St_Thomas").into(),  // U.S. Virgin Islands
            CountryCode::UGA => Self::Africa("Kampala").into(),     // Uganda
            CountryCode::UKR => Self::Europe("Kiev").into(),        // Ukraine
            CountryCode::ARE => Self::Asia("Dubai").into(),         // United Arab Emirates
            CountryCode::GBR => Self::Europe("London").into(),      // United Kingdom
            CountryCode::USA => get_us_time_zone(time_zone, is_day_light_saving), // United States
            CountryCode::UMI => Self::Pacific("Midway").into(), // United States Minor Outlying Islands
            CountryCode::URY => Self::America("Montevideo").into(), // Uruguay
            CountryCode::UZB => Self::Asia("Tashkent").into(),  // Uzbekistan
            CountryCode::VUT => Self::Pacific("Efate").into(),  // Vanuatu
            CountryCode::VAT => Self::Europe("Vatican").into(), // Vatican City
            CountryCode::VEN => Self::America("Caracas").into(), // Venezuela
            CountryCode::VNM => Self::Asia("Ho_Chi_Minh").into(), // Vietnam
            CountryCode::WLF => Self::Pacific("Wallis").into(), // Wallis and Futuna
            CountryCode::ESH => Self::Africa("El_Aaiun").into(), // Western Sahara
            CountryCode::YEM => Self::Asia("Aden").into(),      // Yemen
            CountryCode::ZMB => Self::Africa("Lusaka").into(),  // Zambia
            CountryCode::ZWE => Self::Africa("Harare").into(),  // Zimbabwe
        }
    }

    pub fn as_str(&self) -> ShortString {
        match self {
            IanaTimeZone::Europe(country) => {
                let mut result = ShortString::from_str(EUROPE_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Africa(country) => {
                let mut result = ShortString::from_str(AFRICA_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Asia(country) => {
                let mut result = ShortString::from_str(ASIA_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::America(country) => {
                let mut result = ShortString::from_str(AMERICA_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Antarctica(country) => {
                let mut result = ShortString::from_str(ANTARCTICA_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Arctic(country) => {
                let mut result = ShortString::from_str(ARCTIC_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Atlantic(country) => {
                let mut result = ShortString::from_str(ATLANTIC_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Australia(country) => {
                let mut result = ShortString::from_str(AUSTRALIA_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Indian(country) => {
                let mut result = ShortString::from_str(INDIAN_PREFIX).unwrap();
                result.push_str(country);
                result
            }
            IanaTimeZone::Pacific(country) => {
                let mut result = ShortString::from_str(PACIFIC_PREFIX).unwrap();
                result.push_str(country);
                result
            }
        }
    }
}

#[cfg(feature = "time-zones")]
fn get_rus_time_zone(time_zone: TimeZoneOffset) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_2 => IanaTimeZone::Europe("Kaliningrad").into(), // Kaliningrad
        UTC_3 => IanaTimeZone::Europe("Moscow").into(),      // Moscow
        UTC_4 => IanaTimeZone::Europe("Samara").into(),      // Samara
        UTC_5 => IanaTimeZone::Asia("Yekaterinburg").into(), // Yekaterinburg
        UTC_6 => IanaTimeZone::Asia("Omsk").into(),          // Omsk
        UTC_7 => IanaTimeZone::Asia("Krasnoyarsk").into(),   // Krasnoyarsk
        UTC_8 => IanaTimeZone::Asia("Irkutsk").into(),       // Irkutsk
        UTC_9 => IanaTimeZone::Asia("Yakutsk").into(),       // Yakutsk
        UTC_10 => IanaTimeZone::Asia("Vladivostok").into(),  // Vladivostok
        UTC_11 => IanaTimeZone::Asia("Magadan").into(),      // Magadan
        UTC_12 => IanaTimeZone::Asia("Kamchatka").into(),    // Kamchatka
        _ => None,
    }
}

fn get_us_time_zone(
    time_zone: TimeZoneOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_4 => IanaTimeZone::America("New_York").into(), // Central Time
            UTC_MINUS_5 => IanaTimeZone::America("Chicago").into(),  // Central Time
            UTC_MINUS_6 => IanaTimeZone::America("Denver").into(),   // Mountain Time
            UTC_MINUS_7 => IanaTimeZone::America("Los_Angeles").into(), // Pacific Time
            UTC_MINUS_8 => IanaTimeZone::America("Anchorage").into(), // Alaska Time
            UTC_MINUS_10 => IanaTimeZone::America("Honolulu").into(), // Hawaii-Aleutian Time
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_5 => IanaTimeZone::America("New_York").into(), // Central Time
            UTC_MINUS_6 => IanaTimeZone::America("Chicago").into(),  // Central Standard Time
            UTC_MINUS_7 => IanaTimeZone::America("Denver").into(),   // Mountain Standard Time
            UTC_MINUS_8 => IanaTimeZone::America("Los_Angeles").into(), // Pacific Standard Time
            UTC_MINUS_9 => IanaTimeZone::America("Anchorage").into(), // Alaska Standard Time
            UTC_MINUS_10 => IanaTimeZone::America("Honolulu").into(), // Hawaii Standard Time
            _ => None,
        }
    }
}

fn get_canada_time_zone(
    time_zone: TimeZoneOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_2_30 => IanaTimeZone::America("St_Johns").into(), // Newfoundland Daylight Time (UTC-2:30 DST)
            UTC_MINUS_3 => IanaTimeZone::America("Halifax").into(), // Atlantic Daylight Time (UTC-3 DST)
            UTC_MINUS_4 => IanaTimeZone::America("Toronto").into(), // Atlantic Daylight Time (UTC-3 DST)
            UTC_MINUS_5 => IanaTimeZone::America("Winnipeg").into(), // Central Daylight Time (UTC-5 DST)
            UTC_MINUS_6 => IanaTimeZone::America("Edmonton").into(), // Mountain Daylight Time (UTC-6 DST)
            UTC_MINUS_7 => IanaTimeZone::America("Vancouver").into(), // Pacific Daylight Time (UTC-7 DST)
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_3_30 => IanaTimeZone::America("St_Johns").into(), // Newfoundland Standard Time
            UTC_MINUS_4 => IanaTimeZone::America("Halifax").into(),     // Atlantic Standard Time
            UTC_MINUS_5 => IanaTimeZone::America("Toronto").into(),     // Atlantic Daylight Time
            UTC_MINUS_6 => IanaTimeZone::America("Winnipeg").into(),    // Central Standard Time
            UTC_MINUS_7 => IanaTimeZone::America("Edmonton").into(),    // Mountain Standard Time
            UTC_MINUS_8 => IanaTimeZone::America("Vancouver").into(),   // Pacific Standard Time
            _ => None,
        }
    }
}

fn get_australia_time_zone(
    time_zone: TimeZoneOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_8 => Some(IanaTimeZone::Australia("Perth")), // No DST
            UTC_8_45 => Some(IanaTimeZone::Australia("Ecula")), // No DST
            UTC_10_30 => Some(IanaTimeZone::Australia("Adelaide")),
            UTC_11 => Some(IanaTimeZone::Australia("Sydney")), //Lord_Howe as well
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_8 => Some(IanaTimeZone::Australia("Perth")), // No DST
            UTC_8_45 => Some(IanaTimeZone::Australia("Ecula")), // No DST
            UTC_9_30 => Some(IanaTimeZone::Australia("Adelaide")),
            UTC_10 => Some(IanaTimeZone::Australia("Sydney")), // AEST (UTC+10)
            UTC_10_30 => Some(IanaTimeZone::Australia("Lord_Howe")), // LHST (UTC+10:30)
            _ => None,
        }
    }
}

fn get_brazil_time_zone(time_zone: TimeZoneOffset) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_MINUS_2 => Some(IanaTimeZone::America("Noronha")), // Fernando de Noronha
        UTC_MINUS_3 => Some(IanaTimeZone::America("Sao_Paulo")), // São Paulo
        UTC_MINUS_4 => Some(IanaTimeZone::America("Manaus")),  // Manaus
        UTC_MINUS_5 => Some(IanaTimeZone::America("Rio_Branco")), // Rio Branco
        _ => None,
    }
}

fn get_indonesia_time_zone(time_zone: TimeZoneOffset) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_7 => Some(IanaTimeZone::Asia("Jakarta")), // Western Indonesia Time
        UTC_8 => Some(IanaTimeZone::Asia("Makassar")), // Central Indonesia Time
        UTC_9 => Some(IanaTimeZone::Asia("Jayapura")), // Eastern Indonesia Time
        _ => None,
    }
}

fn get_china_time_zone(time_zone: TimeZoneOffset) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_8 => Some(IanaTimeZone::Asia("Shanghai")), // China Standard Time
        UTC_6 => Some(IanaTimeZone::Asia("Urumqi")),   // Xinjiang Time
        _ => None,
    }
}

fn get_mexico_time_zone(
    time_zone: TimeZoneOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;

    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_5 => Some(IanaTimeZone::America("Cancun")), // No DST
            UTC_MINUS_6 => Some(IanaTimeZone::America("Mexico_City")), // No DST
            UTC_MINUS_7 => Some(IanaTimeZone::America("Tijuana")),
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_5 => Some(IanaTimeZone::America("Cancun")), // No DST
            UTC_MINUS_6 => Some(IanaTimeZone::America("Mexico_City")), // No DST
            UTC_MINUS_7 => Some(IanaTimeZone::America("Mazatlan")),
            UTC_MINUS_8 => Some(IanaTimeZone::America("Tijuana")),
            _ => None,
        }
    }
}

fn get_chile_time_zone(
    time_zone: TimeZoneOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_3 => Some(IanaTimeZone::America("Punta_Arenas")), // No DST
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_3 => Some(IanaTimeZone::America("Punta_Arenas")), // No DST
            UTC_MINUS_4 => Some(IanaTimeZone::America("Santiago")),     // Chile Standard Time
            _ => None,
        }
    }
}

fn get_mongolia_time_zone(time_zone: TimeZoneOffset) -> Option<IanaTimeZone<'static>> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_7 => Some(IanaTimeZone::Asia("Hovd")),
        UTC_8 => Some(IanaTimeZone::Asia("Ulaanbaatar")), // Ulaanbaatar Time
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_from_str() {
        use super::*;

        let time_zone = IanaTimeZone::try_from_str("Asia/Dubai").unwrap();
        match time_zone {
            IanaTimeZone::Asia(country) => assert_eq!(country, "Dubai"),
            _ => panic!("Expected Asia time zone"),
        }

        assert_eq!(time_zone.as_str().as_str(), "Asia/Dubai");
    }
}
