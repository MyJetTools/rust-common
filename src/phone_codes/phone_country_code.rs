use crate::country_code::CountryCode;

pub fn get_country_phone_prefix(cc: CountryCode) -> &'static str {
    match cc {
        // Africa
        CountryCode::DZA => "+213", // Algeria
        CountryCode::AGO => "+244", // Angola
        CountryCode::BEN => "+229", // Benin
        CountryCode::BWA => "+267", // Botswana
        CountryCode::BFA => "+226", // Burkina Faso
        CountryCode::BDI => "+257", // Burundi
        CountryCode::CMR => "+237", // Cameroon
        CountryCode::CPV => "+238", // Cape Verde
        CountryCode::CAF => "+236", // Central African Republic
        CountryCode::TCD => "+235", // Chad
        CountryCode::COM => "+269", // Comoros
        CountryCode::COG => "+242", // Republic of the Congo
        CountryCode::COD => "+243", // Democratic Republic of the Congo
        CountryCode::CIV => "+225", // Côte d'Ivoire
        CountryCode::DJI => "+253", // Djibouti
        CountryCode::EGY => "+20",  // Egypt
        CountryCode::GNQ => "+240", // Equatorial Guinea
        CountryCode::ERI => "+291", // Eritrea
        CountryCode::ETH => "+251", // Ethiopia
        CountryCode::GAB => "+241", // Gabon
        CountryCode::GMB => "+220", // Gambia
        CountryCode::GHA => "+233", // Ghana
        CountryCode::GIN => "+224", // Guinea
        CountryCode::GNB => "+245", // Guinea-Bissau
        CountryCode::KEN => "+254", // Kenya
        CountryCode::LSO => "+266", // Lesotho
        CountryCode::LBR => "+231", // Liberia
        CountryCode::LBY => "+218", // Libya
        CountryCode::MDG => "+261", // Madagascar
        CountryCode::MWI => "+265", // Malawi
        CountryCode::MLI => "+223", // Mali
        CountryCode::MRT => "+222", // Mauritania
        CountryCode::MUS => "+230", // Mauritius
        CountryCode::MAR => "+212", // Morocco
        CountryCode::MOZ => "+258", // Mozambique
        CountryCode::NAM => "+264", // Namibia
        CountryCode::NER => "+227", // Niger
        CountryCode::NGA => "+234", // Nigeria
        CountryCode::RWA => "+250", // Rwanda
        CountryCode::STP => "+239", // São Tomé and Príncipe
        CountryCode::SEN => "+221", // Senegal
        CountryCode::SYC => "+248", // Seychelles
        CountryCode::SLE => "+232", // Sierra Leone
        CountryCode::SOM => "+252", // Somalia
        CountryCode::ZAF => "+27",  // South Africa
        CountryCode::SSD => "+211", // South Sudan
        CountryCode::SDN => "+249", // Sudan
        CountryCode::SWZ => "+268", // Eswatini
        CountryCode::TZA => "+255", // Tanzania
        CountryCode::TGO => "+228", // Togo
        CountryCode::TUN => "+216", // Tunisia
        CountryCode::UGA => "+256", // Uganda
        CountryCode::ZMB => "+260", // Zambia
        CountryCode::ZWE => "+263", // Zimbabwe

        // Asia
        CountryCode::AFG => "+93",  // Afghanistan
        CountryCode::ARM => "+374", // Armenia
        CountryCode::AZE => "+994", // Azerbaijan
        CountryCode::BHR => "+973", // Bahrain
        CountryCode::BGD => "+880", // Bangladesh
        CountryCode::BTN => "+975", // Bhutan
        CountryCode::BRN => "+673", // Brunei
        CountryCode::KHM => "+855", // Cambodia
        CountryCode::CHN => "+86",  // China
        CountryCode::CYP => "+357", // Cyprus
        CountryCode::GEO => "+995", // Georgia
        CountryCode::HKG => "+852", // Hong Kong
        CountryCode::IND => "+91",  // India
        CountryCode::IDN => "+62",  // Indonesia
        CountryCode::IRN => "+98",  // Iran
        CountryCode::IRQ => "+964", // Iraq
        CountryCode::ISR => "+972", // Israel
        CountryCode::JPN => "+81",  // Japan
        CountryCode::JOR => "+962", // Jordan
        CountryCode::KAZ => "+7",   // Kazakhstan
        CountryCode::KWT => "+965", // Kuwait
        CountryCode::KGZ => "+996", // Kyrgyzstan
        CountryCode::LAO => "+856", // Laos
        CountryCode::LBN => "+961", // Lebanon
        CountryCode::MAC => "+853", // Macau
        CountryCode::MYS => "+60",  // Malaysia
        CountryCode::MDV => "+960", // Maldives
        CountryCode::MNG => "+976", // Mongolia
        CountryCode::MMR => "+95",  // Myanmar
        CountryCode::NPL => "+977", // Nepal
        CountryCode::PRK => "+850", // North Korea
        CountryCode::OMN => "+968", // Oman
        CountryCode::PAK => "+92",  // Pakistan
        CountryCode::PSE => "+970", // Palestine
        CountryCode::PHL => "+63",  // Philippines
        CountryCode::QAT => "+974", // Qatar
        CountryCode::SAU => "+966", // Saudi Arabia
        CountryCode::SGP => "+65",  // Singapore
        CountryCode::KOR => "+82",  // South Korea
        CountryCode::LKA => "+94",  // Sri Lanka
        CountryCode::SYR => "+963", // Syria
        CountryCode::TWN => "+886", // Taiwan
        CountryCode::TJK => "+992", // Tajikistan
        CountryCode::THA => "+66",  // Thailand
        CountryCode::TKM => "+993", // Turkmenistan
        CountryCode::ARE => "+971", // United Arab Emirates
        CountryCode::UZB => "+998", // Uzbekistan
        CountryCode::VNM => "+84",  // Vietnam
        CountryCode::YEM => "+967", // Yemen

        // Europe
        CountryCode::ALB => "+355", // Albania
        CountryCode::AND => "+376", // Andorra
        CountryCode::AUT => "+43",  // Austria
        CountryCode::BLR => "+375", // Belarus
        CountryCode::BEL => "+32",  // Belgium
        CountryCode::BIH => "+387", // Bosnia and Herzegovina
        CountryCode::BGR => "+359", // Bulgaria
        CountryCode::HRV => "+385", // Croatia
        CountryCode::CZE => "+420", // Czech Republic
        CountryCode::DNK => "+45",  // Denmark
        CountryCode::EST => "+372", // Estonia
        CountryCode::FIN => "+358", // Finland
        CountryCode::FRA => "+33",  // France
        CountryCode::DEU => "+49",  // Germany
        CountryCode::GRC => "+30",  // Greece
        CountryCode::HUN => "+36",  // Hungary
        CountryCode::ISL => "+354", // Iceland
        CountryCode::IRL => "+353", // Ireland
        CountryCode::ITA => "+39",  // Italy
        CountryCode::LVA => "+371", // Latvia
        CountryCode::LIE => "+423", // Liechtenstein
        CountryCode::LTU => "+370", // Lithuania
        CountryCode::LUX => "+352", // Luxembourg
        CountryCode::MKD => "+389", // North Macedonia
        CountryCode::MLT => "+356", // Malta
        CountryCode::MDA => "+373", // Moldova
        CountryCode::MCO => "+377", // Monaco
        CountryCode::MNE => "+382", // Montenegro
        CountryCode::NLD => "+31",  // Netherlands
        CountryCode::NOR => "+47",  // Norway
        CountryCode::POL => "+48",  // Poland
        CountryCode::PRT => "+351", // Portugal
        CountryCode::ROU => "+40",  // Romania
        CountryCode::RUS => "+7",   // Russia
        CountryCode::SMR => "+378", // San Marino
        CountryCode::SRB => "+381", // Serbia
        CountryCode::SVK => "+421", // Slovakia
        CountryCode::SVN => "+386", // Slovenia
        CountryCode::ESP => "+34",  // Spain
        CountryCode::SWE => "+46",  // Sweden
        CountryCode::CHE => "+41",  // Switzerland
        CountryCode::TUR => "+90",  // Turkey
        CountryCode::UKR => "+380", // Ukraine
        CountryCode::GBR => "+44",  // United Kingdom
        CountryCode::VAT => "+379", // Vatican City

        // North America
        CountryCode::CAN => "+1",  // Canada
        CountryCode::MEX => "+52", // Mexico
        CountryCode::USA => "+1",  // United States

        // Central America & Caribbean
        CountryCode::ATG => "+1",   // Antigua and Barbuda
        CountryCode::BHS => "+1",   // Bahamas
        CountryCode::BRB => "+1",   // Barbados
        CountryCode::BLZ => "+501", // Belize
        CountryCode::CRI => "+506", // Costa Rica
        CountryCode::CUB => "+53",  // Cuba
        CountryCode::DMA => "+1",   // Dominica
        CountryCode::DOM => "+1",   // Dominican Republic
        CountryCode::SLV => "+503", // El Salvador
        CountryCode::GRD => "+1",   // Grenada
        CountryCode::GTM => "+502", // Guatemala
        CountryCode::HTI => "+509", // Haiti
        CountryCode::HND => "+504", // Honduras
        CountryCode::JAM => "+1",   // Jamaica
        CountryCode::KNA => "+1",   // Saint Kitts and Nevis
        CountryCode::LCA => "+1",   // Saint Lucia
        CountryCode::VCT => "+1",   // Saint Vincent and the Grenadines
        CountryCode::TTO => "+1",   // Trinidad and Tobago

        // South America
        CountryCode::ARG => "+54",  // Argentina
        CountryCode::BOL => "+591", // Bolivia
        CountryCode::BRA => "+55",  // Brazil
        CountryCode::CHL => "+56",  // Chile
        CountryCode::COL => "+57",  // Colombia
        CountryCode::ECU => "+593", // Ecuador
        CountryCode::GUY => "+592", // Guyana
        CountryCode::PRY => "+595", // Paraguay
        CountryCode::PER => "+51",  // Peru
        CountryCode::SUR => "+597", // Suriname
        CountryCode::URY => "+598", // Uruguay
        CountryCode::VEN => "+58",  // Venezuela

        // Oceania
        CountryCode::AUS => "+61",  // Australia
        CountryCode::FJI => "+679", // Fiji
        CountryCode::KIR => "+686", // Kiribati
        CountryCode::MHL => "+692", // Marshall Islands
        CountryCode::FSM => "+691", // Micronesia
        CountryCode::NRU => "+674", // Nauru
        CountryCode::NZL => "+64",  // New Zealand
        CountryCode::PLW => "+680", // Palau
        CountryCode::PNG => "+675", // Papua New Guinea
        CountryCode::WSM => "+685", // Samoa
        CountryCode::SLB => "+677", // Solomon Islands
        CountryCode::TON => "+676", // Tonga
        CountryCode::TUV => "+688", // Tuvalu
        CountryCode::VUT => "+678", // Vanuatu

        // Special cases and territories
        CountryCode::ALA => "+358", // Åland Islands (Finland)
        CountryCode::ASM => "+1",   // American Samoa
        CountryCode::ABW => "+297", // Aruba
        CountryCode::BMU => "+1",   // Bermuda
        CountryCode::BES => "+599", // Caribbean Netherlands
        CountryCode::BVT => "+47",  // Bouvet Island (Norway)
        CountryCode::IOT => "+246", // British Indian Ocean Territory
        CountryCode::VGB => "+1",   // British Virgin Islands
        CountryCode::CYM => "+1",   // Cayman Islands
        CountryCode::COK => "+682", // Cook Islands
        CountryCode::CUW => "+599", // Curaçao
        CountryCode::CXR => "+61",  // Christmas Island (Australia)
        CountryCode::CCK => "+61",  // Cocos Islands (Australia)
        CountryCode::FLK => "+500", // Falkland Islands
        CountryCode::FRO => "+298", // Faroe Islands
        CountryCode::GUF => "+594", // French Guiana
        CountryCode::PYF => "+689", // French Polynesia
        CountryCode::ATF => "+262", // French Southern Territories
        CountryCode::GIB => "+350", // Gibraltar
        CountryCode::GRL => "+299", // Greenland
        CountryCode::GLP => "+590", // Guadeloupe
        CountryCode::GUM => "+1",   // Guam
        CountryCode::GGY => "+44",  // Guernsey
        CountryCode::HMD => "+672", // Heard Island and McDonald Islands
        CountryCode::IMN => "+44",  // Isle of Man
        CountryCode::JEY => "+44",  // Jersey
        CountryCode::MTQ => "+596", // Martinique
        CountryCode::MYT => "+262", // Mayotte
        CountryCode::MSR => "+1",   // Montserrat
        CountryCode::NCL => "+687", // New Caledonia
        CountryCode::NIU => "+683", // Niue
        CountryCode::NFK => "+672", // Norfolk Island
        CountryCode::MNP => "+1",   // Northern Mariana Islands
        CountryCode::PCN => "+64",  // Pitcairn Islands
        CountryCode::PRI => "+1",   // Puerto Rico
        CountryCode::REU => "+262", // Réunion
        CountryCode::BLM => "+590", // Saint Barthélemy
        CountryCode::SHN => "+290", // Saint Helena
        CountryCode::MAF => "+590", // Saint Martin
        CountryCode::SPM => "+508", // Saint Pierre and Miquelon
        CountryCode::SXM => "+1",   // Sint Maarten
        CountryCode::SGS => "+500", // South Georgia and the South Sandwich Islands
        CountryCode::TKL => "+690", // Tokelau
        CountryCode::TCA => "+1",   // Turks and Caicos Islands
        CountryCode::VIR => "+1",   // U.S. Virgin Islands
        CountryCode::WLF => "+681", // Wallis and Futuna
        CountryCode::ESH => "+212", // Western Sahara
        CountryCode::XKX => "+383", // Kosovo
        CountryCode::UMI => "+1",   // United States Minor Outlying Islands
        CountryCode::ATA => "+672", // Antarctica

        // Missing country codes
        CountryCode::AIA => "+1",   // Anguilla
        CountryCode::TLS => "+670", // East Timor (Timor-Leste)
        CountryCode::NIC => "+505", // Nicaragua
        CountryCode::PAN => "+507", // Panama
        CountryCode::SJM => "+47",  // Svalbard and Jan Mayen
    }
}

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

    pub fn get_phone_prefix(&self) -> &'static str {
        match self {
            PhoneCountryCode::CountryCode(cc) => get_country_phone_prefix(*cc),
            PhoneCountryCode::UniversalInternationalFreephone => "+800",
            PhoneCountryCode::InternationalSharedCost => "+808",
            PhoneCountryCode::GlobalMobileSatelliteSystem => "+881",
            PhoneCountryCode::InternationalNetworks => "+882",
            PhoneCountryCode::InmarsatSNAC => "+870",
        }
    }
}

impl Into<PhoneCountryCode> for CountryCode {
    fn into(self) -> PhoneCountryCode {
        PhoneCountryCode::CountryCode(self)
    }
}
