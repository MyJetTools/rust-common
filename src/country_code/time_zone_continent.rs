use rust_extensions::ShortString;

use crate::country_code::CountryCode;

pub enum TimeZoneByCountry {
    Europe(&'static str),
    Africa(&'static str),
    Asia(&'static str),
    America(&'static str),
    Antarctica(&'static str),
    Arctic(&'static str),
    Atlantic(&'static str),
    Australia(&'static str),
    Indian(&'static str),
    Pacific(&'static str),
}

impl TimeZoneByCountry {
    pub fn from_country_code(country_code: CountryCode) -> Self {
        match country_code {
            CountryCode::AFG => Self::Asia("Kabul"),       // Afghanistan
            CountryCode::ALA => Self::Europe("Mariehamn"), // Åland Islands
            CountryCode::ALB => Self::Europe("Tirane"),    // Albania
            CountryCode::DZA => Self::Africa("Algiers"),   // Algeria
            CountryCode::ASM => Self::Pacific("Pago_Pago"), // American Samoa
            CountryCode::AND => Self::Europe("Andorra"),   // Andorra
            CountryCode::AGO => Self::Africa("Luanda"),    // Angola
            CountryCode::AIA => Self::America("Anguilla"), // Anguilla
            CountryCode::ATA => Self::Antarctica("McMurdo"), // Antarctica (primary station)
            CountryCode::ATG => Self::America("Antigua"),  // Antigua and Barbuda
            CountryCode::ARG => Self::America("Argentina/Buenos_Aires"), // Argentina
            CountryCode::ARM => Self::Asia("Yerevan"),     // Armenia
            CountryCode::ABW => Self::America("Aruba"),    // Aruba
            CountryCode::AUS => Self::Australia("Sydney"), // Australia
            CountryCode::AUT => Self::Europe("Vienna"),    // Austria
            CountryCode::AZE => Self::Asia("Baku"),        // Azerbaijan
            CountryCode::BHS => Self::America("Nassau"),   // Bahamas
            CountryCode::BHR => Self::Asia("Bahrain"),     // Bahrain
            CountryCode::BGD => Self::Asia("Dhaka"),       // Bangladesh
            CountryCode::BRB => Self::America("Barbados"), // Barbados
            CountryCode::BLR => Self::Europe("Minsk"),     // Belarus
            CountryCode::BEL => Self::Europe("Brussels"),  // Belgium
            CountryCode::BLZ => Self::America("Belize"),   // Belize
            CountryCode::BEN => Self::Africa("Porto-Novo"), // Benin
            CountryCode::BMU => Self::Atlantic("Bermuda"), // Bermuda
            CountryCode::BTN => Self::Asia("Thimphu"),     // Bhutan
            CountryCode::BOL => Self::America("La_Paz"),   // Bolivia
            CountryCode::BES => Self::America("Kralendijk"), // Bonaire, Sint Eustatius, Saba
            CountryCode::BIH => Self::Europe("Sarajevo"),  // Bosnia and Herzegovina
            CountryCode::BWA => Self::Africa("Gaborone"),  // Botswana
            CountryCode::BVT => Self::Atlantic("Reykjavik"), // Bouvet Island (uninhabited, uses UTC)
            CountryCode::BRA => Self::America("Sao_Paulo"),  // Brazil
            CountryCode::IOT => Self::Indian("Chagos"),      // British Indian Ocean Territory
            CountryCode::VGB => Self::America("Tortola"),    // British Virgin Islands
            CountryCode::BRN => Self::Asia("Brunei"),        // Brunei
            CountryCode::BGR => Self::Europe("Sofia"),       // Bulgaria
            CountryCode::BFA => Self::Africa("Ouagadougou"), // Burkina Faso
            CountryCode::BDI => Self::Africa("Bujumbura"),   // Burundi
            CountryCode::KHM => Self::Asia("Phnom_Penh"),    // Cambodia
            CountryCode::CMR => Self::Africa("Douala"),      // Cameroon
            CountryCode::CAN => Self::America("Toronto"),    // Canada
            CountryCode::CPV => Self::Atlantic("Cape_Verde"), // Cape Verde
            CountryCode::CYM => Self::America("Cayman"),     // Cayman Islands
            CountryCode::CAF => Self::Africa("Bangui"),      // Central African Republic
            CountryCode::TCD => Self::Africa("Ndjamena"),    // Chad
            CountryCode::CHL => Self::America("Santiago"),   // Chile
            CountryCode::CHN => Self::Asia("Shanghai"),      // China
            CountryCode::CXR => Self::Indian("Christmas"),   // Christmas Island
            CountryCode::CCK => Self::Indian("Cocos"),       // Cocos Islands
            CountryCode::COL => Self::America("Bogota"),     // Colombia
            CountryCode::COM => Self::Indian("Comoro"),      // Comoros
            CountryCode::COK => Self::Pacific("Rarotonga"),  // Cook Islands
            CountryCode::CRI => Self::America("Costa_Rica"), // Costa Rica
            CountryCode::HRV => Self::Europe("Zagreb"),      // Croatia
            CountryCode::CUB => Self::America("Havana"),     // Cuba
            CountryCode::CUW => Self::America("Curacao"),    // Curaçao
            CountryCode::CYP => Self::Asia("Nicosia"),       // Cyprus
            CountryCode::CZE => Self::Europe("Prague"),      // Czech Republic
            CountryCode::COD => Self::Africa("Kinshasa"),    // Democratic Republic of the Congo
            CountryCode::DNK => Self::Europe("Copenhagen"),  // Denmark
            CountryCode::DJI => Self::Africa("Djibouti"),    // Djibouti
            CountryCode::DMA => Self::America("Dominica"),   // Dominica
            CountryCode::DOM => Self::America("Santo_Domingo"), // Dominican Republic
            CountryCode::TLS => Self::Asia("Dili"),          // Timor-Leste
            CountryCode::ECU => Self::America("Guayaquil"),  // Ecuador
            CountryCode::EGY => Self::Africa("Cairo"),       // Egypt
            CountryCode::SLV => Self::America("El_Salvador"), // El Salvador
            CountryCode::GNQ => Self::Africa("Malabo"),      // Equatorial Guinea
            CountryCode::ERI => Self::Africa("Asmara"),      // Eritrea
            CountryCode::EST => Self::Europe("Tallinn"),     // Estonia
            CountryCode::ETH => Self::Africa("Addis_Ababa"), // Ethiopia
            CountryCode::FLK => Self::Atlantic("Stanley"),   // Falkland Islands
            CountryCode::FRO => Self::Atlantic("Faeroe"),    // Faroe Islands
            CountryCode::FJI => Self::Pacific("Fiji"),       // Fiji
            CountryCode::FIN => Self::Europe("Helsinki"),    // Finland
            CountryCode::FRA => Self::Europe("Paris"),       // France
            CountryCode::GUF => Self::America("Cayenne"),    // French Guiana
            CountryCode::PYF => Self::Pacific("Tahiti"),     // French Polynesia
            CountryCode::ATF => Self::Indian("Kerguelen"),   // French Southern Territories
            CountryCode::GAB => Self::Africa("Libreville"),  // Gabon
            CountryCode::GMB => Self::Africa("Banjul"),      // Gambia
            CountryCode::GEO => Self::Asia("Tbilisi"),       // Georgia
            CountryCode::DEU => Self::Europe("Berlin"),      // Germany
            CountryCode::GHA => Self::Africa("Accra"),       // Ghana
            CountryCode::GIB => Self::Europe("Gibraltar"),   // Gibraltar
            CountryCode::GRC => Self::Europe("Athens"),      // Greece
            CountryCode::GRL => Self::America("Nuuk"),       // Greenland
            CountryCode::GRD => Self::America("Grenada"),    // Grenada
            CountryCode::GLP => Self::America("Guadeloupe"), // Guadeloupe
            CountryCode::GUM => Self::Pacific("Guam"),       // Guam
            CountryCode::GTM => Self::America("Guatemala"),  // Guatemala
            CountryCode::GGY => Self::Europe("Guernsey"),    // Guernsey
            CountryCode::GIN => Self::Africa("Conakry"),     // Guinea
            CountryCode::GNB => Self::Africa("Bissau"),      // Guinea-Bissau
            CountryCode::GUY => Self::America("Guyana"),     // Guyana
            CountryCode::HTI => Self::America("Port-au-Prince"), // Haiti
            CountryCode::HMD => Self::Indian("Kerguelen"), // Heard Island and McDonald Islands (uses ATF)
            CountryCode::HND => Self::America("Tegucigalpa"), // Honduras
            CountryCode::HKG => Self::Asia("Hong_Kong"),   // Hong Kong
            CountryCode::HUN => Self::Europe("Budapest"),  // Hungary
            CountryCode::ISL => Self::Atlantic("Reykjavik"), // Iceland
            CountryCode::IND => Self::Asia("Kolkata"),     // India
            CountryCode::IDN => Self::Asia("Jakarta"),     // Indonesia
            CountryCode::IRN => Self::Asia("Tehran"),      // Iran
            CountryCode::IRQ => Self::Asia("Baghdad"),     // Iraq
            CountryCode::IRL => Self::Europe("Dublin"),    // Ireland
            CountryCode::IMN => Self::Europe("Isle_of_Man"), // Isle of Man
            CountryCode::ISR => Self::Asia("Jerusalem"),   // Israel
            CountryCode::ITA => Self::Europe("Rome"),      // Italy
            CountryCode::CIV => Self::Africa("Abidjan"),   // Côte d'Ivoire
            CountryCode::JAM => Self::America("Jamaica"),  // Jamaica
            CountryCode::JPN => Self::Asia("Tokyo"),       // Japan
            CountryCode::JEY => Self::Europe("Jersey"),    // Jersey
            CountryCode::JOR => Self::Asia("Amman"),       // Jordan
            CountryCode::KAZ => Self::Asia("Almaty"),      // Kazakhstan
            CountryCode::KEN => Self::Africa("Nairobi"),   // Kenya
            CountryCode::KIR => Self::Pacific("Tarawa"),   // Kiribati
            CountryCode::KWT => Self::Asia("Kuwait"),      // Kuwait
            CountryCode::KGZ => Self::Asia("Bishkek"),     // Kyrgyzstan
            CountryCode::LAO => Self::Asia("Vientiane"),   // Laos
            CountryCode::LVA => Self::Europe("Riga"),      // Latvia
            CountryCode::LBN => Self::Asia("Beirut"),      // Lebanon
            CountryCode::LSO => Self::Africa("Maseru"),    // Lesotho
            CountryCode::LBR => Self::Africa("Monrovia"),  // Liberia
            CountryCode::LBY => Self::Africa("Tripoli"),   // Libya
            CountryCode::LIE => Self::Europe("Vaduz"),     // Liechtenstein
            CountryCode::LTU => Self::Europe("Vilnius"),   // Lithuania
            CountryCode::LUX => Self::Europe("Luxembourg"), // Luxembourg
            CountryCode::MAC => Self::Asia("Macau"),       // Macau
            CountryCode::MKD => Self::Europe("Skopje"),    // North Macedonia
            CountryCode::MDG => Self::Indian("Antananarivo"), // Madagascar
            CountryCode::MWI => Self::Africa("Blantyre"),  // Malawi
            CountryCode::MYS => Self::Asia("Kuala_Lumpur"), // Malaysia
            CountryCode::MDV => Self::Indian("Maldives"),  // Maldives
            CountryCode::MLI => Self::Africa("Bamako"),    // Mali
            CountryCode::MLT => Self::Europe("Malta"),     // Malta
            CountryCode::MHL => Self::Pacific("Majuro"),   // Marshall Islands
            CountryCode::MTQ => Self::America("Martinique"), // Martinique
            CountryCode::MRT => Self::Africa("Nouakchott"), // Mauritania
            CountryCode::MUS => Self::Indian("Mauritius"), // Mauritius
            CountryCode::MYT => Self::Indian("Mayotte"),   // Mayotte
            CountryCode::MEX => Self::America("Mexico_City"), // Mexico
            CountryCode::FSM => Self::Pacific("Pohnpei"),  // Micronesia
            CountryCode::MDA => Self::Europe("Chisinau"),  // Moldova
            CountryCode::MCO => Self::Europe("Monaco"),    // Monaco
            CountryCode::MNG => Self::Asia("Ulaanbaatar"), // Mongolia
            CountryCode::MNE => Self::Europe("Podgorica"), // Montenegro
            CountryCode::MSR => Self::America("Montserrat"), // Montserrat
            CountryCode::MAR => Self::Africa("Casablanca"), // Morocco
            CountryCode::MOZ => Self::Africa("Maputo"),    // Mozambique
            CountryCode::MMR => Self::Asia("Yangon"),      // Myanmar
            CountryCode::NAM => Self::Africa("Windhoek"),  // Namibia
            CountryCode::NRU => Self::Pacific("Nauru"),    // Nauru
            CountryCode::NPL => Self::Asia("Kathmandu"),   // Nepal
            CountryCode::NLD => Self::Europe("Amsterdam"), // Netherlands
            CountryCode::NCL => Self::Pacific("Noumea"),   // New Caledonia
            CountryCode::NZL => Self::Pacific("Auckland"), // New Zealand
            CountryCode::NIC => Self::America("Managua"),  // Nicaragua
            CountryCode::NER => Self::Africa("Niamey"),    // Niger
            CountryCode::NGA => Self::Africa("Lagos"),     // Nigeria
            CountryCode::NIU => Self::Pacific("Niue"),     // Niue
            CountryCode::NFK => Self::Pacific("Norfolk"),  // Norfolk Island
            CountryCode::PRK => Self::Asia("Pyongyang"),   // North Korea
            CountryCode::MNP => Self::Pacific("Saipan"),   // Northern Mariana Islands
            CountryCode::NOR => Self::Europe("Oslo"),      // Norway
            CountryCode::OMN => Self::Asia("Muscat"),      // Oman
            CountryCode::PAK => Self::Asia("Karachi"),     // Pakistan
            CountryCode::PLW => Self::Pacific("Palau"),    // Palau
            CountryCode::PSE => Self::Asia("Gaza"),        // Palestine
            CountryCode::PAN => Self::America("Panama"),   // Panama
            CountryCode::PNG => Self::Pacific("Port_Moresby"), // Papua New Guinea
            CountryCode::PRY => Self::America("Asuncion"), // Paraguay
            CountryCode::PER => Self::America("Lima"),     // Peru
            CountryCode::PHL => Self::Asia("Manila"),      // Philippines
            CountryCode::PCN => Self::Pacific("Pitcairn"), // Pitcairn Islands
            CountryCode::POL => Self::Europe("Warsaw"),    // Poland
            CountryCode::PRT => Self::Europe("Lisbon"),    // Portugal
            CountryCode::PRI => Self::America("Puerto_Rico"), // Puerto Rico
            CountryCode::QAT => Self::Asia("Qatar"),       // Qatar
            CountryCode::COG => Self::Africa("Brazzaville"), // Republic of the Congo
            CountryCode::REU => Self::Indian("Reunion"),   // Réunion
            CountryCode::ROU => Self::Europe("Bucharest"), // Romania
            CountryCode::RUS => Self::Europe("Moscow"),    // Russia
            CountryCode::RWA => Self::Africa("Kigali"),    // Rwanda
            CountryCode::BLM => Self::America("St_Barthelemy"), // Saint Barthélemy
            CountryCode::SHN => Self::Atlantic("St_Helena"), // Saint Helena
            CountryCode::KNA => Self::America("St_Kitts"), // Saint Kitts and Nevis
            CountryCode::LCA => Self::America("St_Lucia"), // Saint Lucia
            CountryCode::MAF => Self::America("Marigot"),  // Saint Martin
            CountryCode::SPM => Self::America("Miquelon"), // Saint Pierre and Miquelon
            CountryCode::VCT => Self::America("St_Vincent"), // Saint Vincent and the Grenadines
            CountryCode::WSM => Self::Pacific("Apia"),     // Samoa
            CountryCode::SMR => Self::Europe("San_Marino"), // San Marino
            CountryCode::STP => Self::Africa("Sao_Tome"),  // São Tomé and Príncipe
            CountryCode::SAU => Self::Asia("Riyadh"),      // Saudi Arabia
            CountryCode::SEN => Self::Africa("Dakar"),     // Senegal
            CountryCode::SRB => Self::Europe("Belgrade"),  // Serbia
            CountryCode::SYC => Self::Indian("Mahe"),      // Seychelles
            CountryCode::SLE => Self::Africa("Freetown"),  // Sierra Leone
            CountryCode::SGP => Self::Asia("Singapore"),   // Singapore
            CountryCode::SXM => Self::America("Lower_Princes"), // Sint Maarten
            CountryCode::SVK => Self::Europe("Bratislava"), // Slovakia
            CountryCode::SVN => Self::Europe("Ljubljana"), // Slovenia
            CountryCode::SLB => Self::Pacific("Guadalcanal"), // Solomon Islands
            CountryCode::SOM => Self::Africa("Mogadishu"), // Somalia
            CountryCode::ZAF => Self::Africa("Johannesburg"), // South Africa
            CountryCode::SGS => Self::Atlantic("South_Georgia"), // South Georgia
            CountryCode::KOR => Self::Asia("Seoul"),       // South Korea
            CountryCode::SSD => Self::Africa("Juba"),      // South Sudan
            CountryCode::ESP => Self::Europe("Madrid"),    // Spain
            CountryCode::LKA => Self::Asia("Colombo"),     // Sri Lanka
            CountryCode::SDN => Self::Africa("Khartoum"),  // Sudan
            CountryCode::SUR => Self::America("Paramaribo"), // Suriname
            CountryCode::SJM => Self::Arctic("Longyearbyen"), // Svalbard and Jan Mayen
            CountryCode::SWZ => Self::Africa("Mbabane"),   // Eswatini
            CountryCode::SWE => Self::Europe("Stockholm"), // Sweden
            CountryCode::CHE => Self::Europe("Zurich"),    // Switzerland
            CountryCode::SYR => Self::Asia("Damascus"),    // Syria
            CountryCode::TWN => Self::Asia("Taipei"),      // Taiwan
            CountryCode::TJK => Self::Asia("Dushanbe"),    // Tajikistan
            CountryCode::TZA => Self::Africa("Dar_es_Salaam"), // Tanzania
            CountryCode::THA => Self::Asia("Bangkok"),     // Thailand
            CountryCode::TGO => Self::Africa("Lome"),      // Togo
            CountryCode::TKL => Self::Pacific("Fakaofo"),  // Tokelau
            CountryCode::TON => Self::Pacific("Tongatapu"), // Tonga
            CountryCode::TTO => Self::America("Port_of_Spain"), // Trinidad and Tobago
            CountryCode::TUN => Self::Africa("Tunis"),     // Tunisia
            CountryCode::TUR => Self::Europe("Istanbul"),  // Turkey
            CountryCode::TKM => Self::Asia("Ashgabat"),    // Turkmenistan
            CountryCode::TCA => Self::America("Grand_Turk"), // Turks and Caicos Islands
            CountryCode::TUV => Self::Pacific("Funafuti"), // Tuvalu
            CountryCode::VIR => Self::America("St_Thomas"), // U.S. Virgin Islands
            CountryCode::UGA => Self::Africa("Kampala"),   // Uganda
            CountryCode::UKR => Self::Europe("Kiev"),      // Ukraine
            CountryCode::ARE => Self::Asia("Dubai"),       // United Arab Emirates
            CountryCode::GBR => Self::Europe("London"),    // United Kingdom
            CountryCode::USA => Self::America("New_York"), // United States
            CountryCode::UMI => Self::Pacific("Midway"),   // United States Minor Outlying Islands
            CountryCode::URY => Self::America("Montevideo"), // Uruguay
            CountryCode::UZB => Self::Asia("Tashkent"),    // Uzbekistan
            CountryCode::VUT => Self::Pacific("Efate"),    // Vanuatu
            CountryCode::VAT => Self::Europe("Vatican"),   // Vatican City
            CountryCode::VEN => Self::America("Caracas"),  // Venezuela
            CountryCode::VNM => Self::Asia("Ho_Chi_Minh"), // Vietnam
            CountryCode::WLF => Self::Pacific("Wallis"),   // Wallis and Futuna
            CountryCode::ESH => Self::Africa("El_Aaiun"),  // Western Sahara
            CountryCode::YEM => Self::Asia("Aden"),        // Yemen
            CountryCode::ZMB => Self::Africa("Lusaka"),    // Zambia
            CountryCode::ZWE => Self::Africa("Harare"),    // Zimbabwe
        }
    }

    pub fn as_str(&self) -> ShortString {
        match self {
            TimeZoneByCountry::Europe(country) => {
                let mut result = ShortString::from_str("Europe/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Africa(country) => {
                let mut result = ShortString::from_str("Africa/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Asia(country) => {
                let mut result = ShortString::from_str("Asia/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::America(country) => {
                let mut result = ShortString::from_str("America/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Antarctica(country) => {
                let mut result = ShortString::from_str("Antarctica/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Arctic(country) => {
                let mut result = ShortString::from_str("Arctic/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Atlantic(country) => {
                let mut result = ShortString::from_str("Atlantic/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Australia(country) => {
                let mut result = ShortString::from_str("Australia/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Indian(country) => {
                let mut result = ShortString::from_str("Indian/").unwrap();
                result.push_str(country);
                result
            }
            TimeZoneByCountry::Pacific(country) => {
                let mut result = ShortString::from_str("Pacific/").unwrap();
                result.push_str(country);
                result
            }
        }
    }
}
