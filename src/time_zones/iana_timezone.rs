use rust_extensions::ShortString;

#[cfg(feature = "time-zones")]
use crate::time_zones::TimeZoneGmtOffset;

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

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum IanaTimeZone {
    Europe(IanaEurope),
    Africa(IanaAfrica),
    Asia(IanaAsia),
    America(IanaAmerica),
    Antarctica(IanaAntarctica),
    Arctic(IanaArctic),
    Atlantic(IanaAtlantic),
    Australia(IanaAustralia),
    Indian(IanaIndian),
    Pacific(IanaPacific),
}

impl IanaTimeZone {
    pub fn try_from_str(src: &str) -> Option<Self> {
        let slash_index = src.find('/')?;

        let (continent, country) = src.split_at(slash_index + 1);

        match continent {
            EUROPE_PREFIX => Some(IanaTimeZone::Europe(IanaEurope::try_from_str(country)?)),
            AFRICA_PREFIX => Some(IanaTimeZone::Africa(IanaAfrica::try_from_str(country)?)),
            ASIA_PREFIX => Some(IanaTimeZone::Asia(IanaAsia::try_from_str(country)?)),
            AMERICA_PREFIX => Some(IanaTimeZone::America(IanaAmerica::try_from_str(country)?)),
            ANTARCTICA_PREFIX => Some(IanaTimeZone::Antarctica(IanaAntarctica::try_from_str(
                country,
            )?)),
            ARCTIC_PREFIX => Some(IanaTimeZone::Arctic(IanaArctic::try_from_str(country)?)),
            ATLANTIC_PREFIX => Some(IanaTimeZone::Atlantic(IanaAtlantic::try_from_str(country)?)),
            AUSTRALIA_PREFIX => Some(IanaTimeZone::Australia(IanaAustralia::try_from_str(
                country,
            )?)),
            INDIAN_PREFIX => Some(IanaTimeZone::Indian(IanaIndian::try_from_str(country)?)),
            PACIFIC_PREFIX => Some(IanaTimeZone::Pacific(IanaPacific::try_from_str(country)?)),
            _ => None,
        }
    }

    #[cfg(feature = "time-zones")]
    pub fn get_fallback_timezone(time_zone: TimeZoneGmtOffset) -> Self {
        use crate::time_zones::*;
        let seconds = time_zone.as_seconds();
        match seconds {
            UTC_MINUS_12 => IanaTimeZone::Pacific(IanaPacific::Baker),
            UTC_MINUS_11 => IanaTimeZone::Pacific(IanaPacific::Niue),
            UTC_MINUS_10 => IanaTimeZone::Pacific(IanaPacific::Honolulu),
            UTC_MINUS_9 => IanaTimeZone::America(IanaAmerica::Adak),
            UTC_MINUS_8 => IanaTimeZone::America(IanaAmerica::Pitcairn),
            UTC_MINUS_7 => IanaTimeZone::America(IanaAmerica::Phoenix),
            UTC_MINUS_6 => IanaTimeZone::America(IanaAmerica::Regina),
            UTC_MINUS_5 => IanaTimeZone::America(IanaAmerica::Guayaquil),
            UTC_MINUS_4 => IanaTimeZone::America(IanaAmerica::LaPaz),
            UTC_MINUS_3 => {
                IanaTimeZone::America(IanaAmerica::Argentina(IanaArgentina::BuenosAires))
            }
            UTC_MINUS_2 => IanaTimeZone::Atlantic(IanaAtlantic::SouthGeorgia),
            UTC_MINUS_1 => IanaTimeZone::Atlantic(IanaAtlantic::Azores),
            UTC_0 => IanaTimeZone::Atlantic(IanaAtlantic::Reykjavik),
            UTC_1 => IanaTimeZone::Africa(IanaAfrica::Algiers),
            UTC_2 => IanaTimeZone::Africa(IanaAfrica::Johannesburg),
            UTC_3 => IanaTimeZone::Europe(IanaEurope::Moscow),
            UTC_4 => IanaTimeZone::Asia(IanaAsia::Dubai),
            UTC_5 => IanaTimeZone::Asia(IanaAsia::Karachi),
            UTC_5_30 => IanaTimeZone::Asia(IanaAsia::Kolkata),
            UTC_6 => IanaTimeZone::Asia(IanaAsia::Dhaka),
            UTC_7 => IanaTimeZone::Asia(IanaAsia::Jakarta),
            UTC_8 => IanaTimeZone::Asia(IanaAsia::Shanghai),
            UTC_8_45 => IanaTimeZone::Australia(IanaAustralia::Ecula),
            UTC_9 => IanaTimeZone::Asia(IanaAsia::Tokyo),
            UTC_10 => IanaTimeZone::Australia(IanaAustralia::Brisbane),
            UTC_11 => IanaTimeZone::Pacific(IanaPacific::Noumea),
            UTC_12 => IanaTimeZone::Pacific(IanaPacific::Fiji),
            UTC_13 => IanaTimeZone::Pacific(IanaPacific::Tongatapu),
            UTC_14 => IanaTimeZone::Pacific(IanaPacific::Kiritimati),
            _ => IanaTimeZone::Asia(IanaAsia::Dubai),
        }
    }

    #[cfg(feature = "time-zones")]
    pub fn from_country_code(
        country_code: crate::country_code::CountryCode,
        time_zone: TimeZoneGmtOffset,
        is_day_light_saving: bool,
    ) -> Option<Self> {
        use crate::country_code::CountryCode;
        match country_code {
            CountryCode::AFG => Some(Self::Asia(IanaAsia::Kabul)), // Afghanistan
            CountryCode::ALA => Some(Self::Europe(IanaEurope::Mariehamn)), // Åland Islands
            CountryCode::ALB => Some(Self::Europe(IanaEurope::Tirane)), // Albania
            CountryCode::DZA => Some(Self::Africa(IanaAfrica::Algiers)),
            CountryCode::ASM => Some(Self::Pacific(IanaPacific::PagoPago)), // American Samoa
            CountryCode::AND => Some(Self::Europe(IanaEurope::Andorra)),    // Andorra
            CountryCode::AGO => Some(Self::Africa(IanaAfrica::Luanda)),     // Angola
            CountryCode::AIA => Some(Self::America(IanaAmerica::Anguilla)), // Anguilla
            CountryCode::ATA => Some(Self::Antarctica(IanaAntarctica::McMurdo)), // Antarctica (primary station)
            CountryCode::ATG => Some(Self::America(IanaAmerica::Antigua)), // Antigua and Barbuda
            CountryCode::ARG => Some(Self::America(IanaAmerica::Argentina(
                IanaArgentina::BuenosAires,
            ))), // Argentina
            CountryCode::ARM => Some(Self::Asia(IanaAsia::Yerevan)),       // Armenia
            CountryCode::ABW => Some(Self::America(IanaAmerica::Aruba)),   // Aruba
            CountryCode::AUS => get_australia_time_zone(time_zone, is_day_light_saving), // Australia
            CountryCode::AUT => Some(Self::Europe(IanaEurope::Vienna)),                  // Austria
            CountryCode::AZE => Some(Self::Asia(IanaAsia::Baku)), // Azerbaijan
            CountryCode::BHS => Some(Self::America(IanaAmerica::Nassau)), // Bahamas
            CountryCode::BHR => Some(Self::Asia(IanaAsia::Bahrain)), // Bahrain
            CountryCode::BGD => Some(Self::Asia(IanaAsia::Dhaka)), // Bangladesh
            CountryCode::BRB => Some(Self::America(IanaAmerica::Barbados)), // Barbados
            CountryCode::BLR => Some(Self::Europe(IanaEurope::Minsk)), // Belarus
            CountryCode::BEL => Some(Self::Europe(IanaEurope::Brussels)), // Belgium
            CountryCode::BLZ => Some(Self::America(IanaAmerica::Belize)), // Belize
            CountryCode::BEN => Some(Self::Africa(IanaAfrica::PortoNovo)), // Benin
            CountryCode::BMU => Some(Self::Atlantic(IanaAtlantic::Bermuda)), // Bermuda
            CountryCode::BTN => Some(Self::Asia(IanaAsia::Thimphu)), // Bhutan
            CountryCode::BOL => Some(Self::America(IanaAmerica::LaPaz)), // Bolivia
            CountryCode::BES => Some(Self::America(IanaAmerica::Kralendijk)), // Bonaire, Sint Eustatius, Saba
            CountryCode::BIH => Some(Self::Europe(IanaEurope::Sarajevo)), // Bosnia and Herzegovina
            CountryCode::BWA => Some(Self::Africa(IanaAfrica::Gaborone)), // Botswana
            CountryCode::BVT => Some(Self::Atlantic(IanaAtlantic::Reykjavik)), // Bouvet Island (uninhabited, uses UTC)
            CountryCode::BRA => get_brazil_time_zone(time_zone),               // Brazil
            CountryCode::IOT => Some(Self::Indian(IanaIndian::Chagos)), // British Indian Ocean Territory
            CountryCode::VGB => Some(Self::America(IanaAmerica::Tortola)), // British Virgin Islands
            CountryCode::BRN => Some(Self::Asia(IanaAsia::Brunei)),     // Brunei
            CountryCode::BGR => Some(Self::Europe(IanaEurope::Sofia)),  // Bulgaria
            CountryCode::BFA => Some(Self::Africa(IanaAfrica::Ouagadougou)), // Burkina Faso
            CountryCode::BDI => Some(Self::Africa(IanaAfrica::Bujumbura)), // Burundi
            CountryCode::KHM => Some(Self::Asia(IanaAsia::PhnomPenh)),  // Cambodia
            CountryCode::CMR => Some(Self::Africa(IanaAfrica::Douala)), // Cameroon
            CountryCode::CAN => get_canada_time_zone(time_zone, is_day_light_saving), // Canada
            CountryCode::CPV => Some(Self::Atlantic(IanaAtlantic::CapeVerde)), // Cape Verde
            CountryCode::CYM => Some(Self::America(IanaAmerica::Cayman)), // Cayman Islands
            CountryCode::CAF => Some(Self::Africa(IanaAfrica::Bangui)), // Central African Republic
            CountryCode::TCD => Some(Self::Africa(IanaAfrica::Ndjamena)), // Chad
            CountryCode::CHL => get_chile_time_zone(time_zone, is_day_light_saving), // Chile
            CountryCode::CHN => get_china_time_zone(time_zone),         // China
            CountryCode::CXR => Some(Self::Indian(IanaIndian::Christmas)), // Christmas Island
            CountryCode::CCK => Some(Self::Indian(IanaIndian::Cocos)),  // Cocos Islands
            CountryCode::COL => Some(Self::America(IanaAmerica::Bogota)), // Colombia
            CountryCode::COM => Some(Self::Indian(IanaIndian::Comoro)), // Comoros
            CountryCode::COK => Some(Self::Pacific(IanaPacific::Rarotonga)), // Cook Islands
            CountryCode::CRI => Some(Self::America(IanaAmerica::CostaRica)), // Costa Rica
            CountryCode::HRV => Some(Self::Europe(IanaEurope::Zagreb)), // Croatia
            CountryCode::CUB => Some(Self::America(IanaAmerica::Havana)), // Cuba
            CountryCode::CUW => Some(Self::America(IanaAmerica::Curacao)), // Curaçao
            CountryCode::CYP => Some(Self::Europe(IanaEurope::Nicosia)), // Cyprus
            CountryCode::CZE => Some(Self::Europe(IanaEurope::Prague)), // Czech Republic
            CountryCode::COD => Some(Self::Africa(IanaAfrica::Kinshasa)), // Democratic Republic of the Congo
            CountryCode::DNK => Some(Self::Europe(IanaEurope::Copenhagen)), // Denmark
            CountryCode::DJI => Some(Self::Africa(IanaAfrica::Djibouti)),
            CountryCode::DMA => Some(Self::America(IanaAmerica::Dominica)), // Dominica
            CountryCode::DOM => Some(Self::America(IanaAmerica::SantoDomingo)), // Dominican Republic
            CountryCode::TLS => Some(Self::Asia(IanaAsia::Dili)),               // Timor-Leste
            CountryCode::ECU => Some(Self::America(IanaAmerica::Guayaquil)),    // Ecuador
            CountryCode::EGY => Some(Self::Africa(IanaAfrica::Cairo)),
            CountryCode::SLV => Some(Self::America(IanaAmerica::ElSalvador)), // El Salvador
            CountryCode::GNQ => Some(Self::Africa(IanaAfrica::Malabo)),
            CountryCode::ERI => Some(Self::Africa(IanaAfrica::Asmara)),
            CountryCode::EST => Some(Self::Europe(IanaEurope::Tallinn)), // Estonia
            CountryCode::ETH => Some(Self::Africa(IanaAfrica::AddisAbaba)),
            CountryCode::FLK => Some(Self::Atlantic(IanaAtlantic::Stanley)), // Falkland Islands
            CountryCode::FRO => Some(Self::Atlantic(IanaAtlantic::Faeroe)),  // Faroe Islands
            CountryCode::FJI => Some(Self::Pacific(IanaPacific::Fiji)),      // Fiji
            CountryCode::FIN => Some(Self::Europe(IanaEurope::Helsinki)),    // Finland
            CountryCode::FRA => Some(Self::Europe(IanaEurope::Paris)),       // France
            CountryCode::GUF => Some(Self::America(IanaAmerica::Cayenne)),   // French Guiana
            CountryCode::PYF => Some(Self::Pacific(IanaPacific::Tahiti)),    // French Polynesia
            CountryCode::ATF => Some(Self::Indian(IanaIndian::Kerguelen)), // French Southern Territories
            CountryCode::GAB => Some(Self::Africa(IanaAfrica::Libreville)),
            CountryCode::GMB => Some(Self::Africa(IanaAfrica::Banjul)),
            CountryCode::GEO => Some(Self::Asia(IanaAsia::Tbilisi)), // Georgia
            CountryCode::DEU => Some(Self::Europe(IanaEurope::Berlin)), // Germany
            CountryCode::GHA => Some(Self::Africa(IanaAfrica::Accra)),
            CountryCode::GIB => Some(Self::Europe(IanaEurope::Gibraltar)), // Gibraltar
            CountryCode::GRC => Some(Self::Europe(IanaEurope::Athens)),    // Greece
            CountryCode::GRL => Some(Self::America(IanaAmerica::Nuuk)),    // Greenland
            CountryCode::GRD => Some(Self::America(IanaAmerica::Grenada)), // Grenada
            CountryCode::GLP => Some(Self::America(IanaAmerica::Guadeloupe)), // Guadeloupe
            CountryCode::GUM => Some(Self::Pacific(IanaPacific::Guam)),    // Guam
            CountryCode::GTM => Some(Self::America(IanaAmerica::Guatemala)), // Guatemala
            CountryCode::GGY => Some(Self::Europe(IanaEurope::Guernsey)),  // Guernsey
            CountryCode::GIN => Some(Self::Africa(IanaAfrica::Conakry)),
            CountryCode::GNB => Some(Self::Africa(IanaAfrica::Bissau)),
            CountryCode::GUY => Some(Self::America(IanaAmerica::Guyana)), // Guyana
            CountryCode::HTI => Some(Self::America(IanaAmerica::PortAuPrince)), // Haiti
            CountryCode::HMD => Some(Self::Indian(IanaIndian::Kerguelen)), // Heard Island and McDonald Islands (uses ATF)
            CountryCode::HND => Some(Self::America(IanaAmerica::Tegucigalpa)), // Honduras
            CountryCode::HKG => Some(Self::Asia(IanaAsia::HongKong)),      // Hong Kong
            CountryCode::HUN => Some(Self::Europe(IanaEurope::Budapest)),  // Hungary
            CountryCode::ISL => Some(Self::Atlantic(IanaAtlantic::Reykjavik)), // Iceland
            CountryCode::IND => Some(Self::Asia(IanaAsia::Kolkata)),       // India
            CountryCode::IDN => get_indonesia_time_zone(time_zone),        // Indonesia
            CountryCode::IRN => Some(Self::Asia(IanaAsia::Tehran)),        // Iran
            CountryCode::IRQ => Some(Self::Asia(IanaAsia::Baghdad)),       // Iraq
            CountryCode::IRL => Some(Self::Europe(IanaEurope::Dublin)),    // Ireland
            CountryCode::IMN => Some(Self::Europe(IanaEurope::IsleOfMan)), // Isle of Man
            CountryCode::ISR => Some(Self::Asia(IanaAsia::Jerusalem)),     // Israel
            CountryCode::ITA => Some(Self::Europe(IanaEurope::Rome)),      // Italy
            CountryCode::CIV => Some(Self::Africa(IanaAfrica::Abidjan)),
            CountryCode::JAM => Some(Self::America(IanaAmerica::Jamaica)), // Jamaica
            CountryCode::JPN => Some(Self::Asia(IanaAsia::Tokyo)),         // Japan
            CountryCode::JEY => Some(Self::Europe(IanaEurope::Jersey)),    // Jersey
            CountryCode::JOR => Some(Self::Asia(IanaAsia::Amman)),         // Jordan
            CountryCode::KAZ => Some(Self::Asia(IanaAsia::Almaty)),        // Kazakhstan
            CountryCode::KEN => Some(Self::Africa(IanaAfrica::Nairobi)),   // Kenya
            CountryCode::KIR => Some(Self::Pacific(IanaPacific::Tarawa)),  // Kiribati
            CountryCode::KWT => Some(Self::Asia(IanaAsia::Kuwait)),        // Kuwait
            CountryCode::KGZ => Some(Self::Asia(IanaAsia::Bishkek)),       // Kyrgyzstan
            CountryCode::LAO => Some(Self::Asia(IanaAsia::Vientiane)),     // Laos
            CountryCode::LVA => Some(Self::Europe(IanaEurope::Riga)),      // Latvia
            CountryCode::LBN => Some(Self::Asia(IanaAsia::Beirut)),        // Lebanon
            CountryCode::LSO => Some(Self::Africa(IanaAfrica::Maseru)),
            CountryCode::LBR => Some(Self::Africa(IanaAfrica::Monrovia)),
            CountryCode::LBY => Some(Self::Africa(IanaAfrica::Tripoli)),
            CountryCode::LIE => Some(Self::Europe(IanaEurope::Vaduz)), // Liechtenstein
            CountryCode::LTU => Some(Self::Europe(IanaEurope::Vilnius)), // Lithuania
            CountryCode::LUX => Some(Self::Europe(IanaEurope::Luxembourg)), // Luxembourg
            CountryCode::MAC => Some(Self::Asia(IanaAsia::Macao)),     // Macau
            CountryCode::MKD => Some(Self::Europe(IanaEurope::Skopje)), // North Macedonia
            CountryCode::MDG => Some(Self::Africa(IanaAfrica::Antananarivo)), // Madagascar
            CountryCode::MWI => Some(Self::Africa(IanaAfrica::Blantyre)),
            CountryCode::MYS => Some(Self::Asia(IanaAsia::KualaLumpur)), // Malaysia
            CountryCode::MDV => Some(Self::Indian(IanaIndian::Maldives)), // Maldives
            CountryCode::MLI => Some(Self::Africa(IanaAfrica::Bamako)),
            CountryCode::MLT => Some(Self::Europe(IanaEurope::Malta)), // Malta
            CountryCode::MHL => Some(Self::Pacific(IanaPacific::Majuro)), // Marshall Islands
            CountryCode::MTQ => Some(Self::America(IanaAmerica::Martinique)), // Martinique
            CountryCode::MRT => Some(Self::Africa(IanaAfrica::Nouakchott)),
            CountryCode::MUS => Some(Self::Africa(IanaAfrica::PortLouis)),
            CountryCode::MYT => Some(Self::Africa(IanaAfrica::Mamoudzou)),
            CountryCode::MEX => get_mexico_time_zone(time_zone, is_day_light_saving),
            CountryCode::FSM => Some(Self::Pacific(IanaPacific::Pohnpei)), // Micronesia
            CountryCode::MDA => Some(Self::Europe(IanaEurope::Chisinau)),  // Moldova
            CountryCode::MCO => Some(Self::Europe(IanaEurope::Monaco)),    // Monaco
            CountryCode::MNG => get_mongolia_time_zone(time_zone),         // Mongolia
            CountryCode::MNE => Some(Self::Europe(IanaEurope::Podgorica)), // Montenegro
            CountryCode::MSR => Some(Self::America(IanaAmerica::Montserrat)), // Montserrat
            CountryCode::MAR => Some(Self::Africa(IanaAfrica::Casablanca)),
            CountryCode::MOZ => Some(Self::Africa(IanaAfrica::Maputo)),
            CountryCode::MMR => Some(Self::Asia(IanaAsia::Yangon)), // Myanmar
            CountryCode::NAM => Some(Self::Africa(IanaAfrica::Windhoek)),
            CountryCode::NRU => Some(Self::Pacific(IanaPacific::Nauru)), // Nauru
            CountryCode::NPL => Some(Self::Asia(IanaAsia::Kathmandu)),   // Nepal
            CountryCode::NLD => Some(Self::Europe(IanaEurope::Amsterdam)), // Netherlands
            CountryCode::NCL => Some(Self::Pacific(IanaPacific::Noumea)), // New Caledonia
            CountryCode::NZL => Some(Self::Pacific(IanaPacific::Auckland)), // New Zealand
            CountryCode::NIC => Some(Self::America(IanaAmerica::Managua)), // Nicaragua
            CountryCode::NER => Some(Self::Africa(IanaAfrica::Niamey)),
            CountryCode::NGA => Some(Self::Africa(IanaAfrica::Lagos)),
            CountryCode::NIU => Some(Self::Pacific(IanaPacific::Niue)), // Niue
            CountryCode::NFK => Some(Self::Pacific(IanaPacific::Norfolk)), // Norfolk Island
            CountryCode::PRK => Some(Self::Asia(IanaAsia::Pyongyang)),  // North Korea
            CountryCode::MNP => Some(Self::Pacific(IanaPacific::Saipan)), // Northern Mariana Islands
            CountryCode::NOR => Some(Self::Europe(IanaEurope::Oslo)),     // Norway
            CountryCode::OMN => Some(Self::Asia(IanaAsia::Muscat)),       // Oman
            CountryCode::PAK => Some(Self::Asia(IanaAsia::Karachi)),      // Pakistan
            CountryCode::PLW => Some(Self::Pacific(IanaPacific::Palau)),  // Palau
            CountryCode::PSE => Some(Self::Asia(IanaAsia::Gaza)),         // Palestine
            CountryCode::PAN => Some(Self::America(IanaAmerica::Panama)), // Panama
            CountryCode::PNG => Some(Self::Pacific(IanaPacific::PortMoresby)), // Papua New Guinea
            CountryCode::PRY => Some(Self::America(IanaAmerica::Asuncion)), // Paraguay
            CountryCode::PER => Some(Self::America(IanaAmerica::Lima)),   // Peru
            CountryCode::PHL => Some(Self::Asia(IanaAsia::Manila)),       // Philippines
            CountryCode::PCN => Some(Self::Pacific(IanaPacific::Pitcairn)), // Pitcairn Islands
            CountryCode::POL => Some(Self::Europe(IanaEurope::Warsaw)),   // Poland
            CountryCode::PRT => Some(Self::Europe(IanaEurope::Lisbon)),   // Portugal
            CountryCode::PRI => Some(Self::America(IanaAmerica::PuertoRico)), // Puerto Rico
            CountryCode::QAT => Some(Self::Asia(IanaAsia::Qatar)),        // Qatar
            CountryCode::COG => Some(Self::Africa(IanaAfrica::Brazzaville)), // Republic of the Congo
            CountryCode::REU => Some(Self::Indian(IanaIndian::Reunion)),     // Réunion
            CountryCode::ROU => Some(Self::Europe(IanaEurope::Bucharest)),   // Romania
            CountryCode::RUS => get_rus_time_zone(time_zone).into(),         // Russia
            CountryCode::RWA => Some(Self::Africa(IanaAfrica::Kigali)),
            CountryCode::BLM => Some(Self::America(IanaAmerica::StBarthelemy)), // Saint Barthélemy
            CountryCode::SHN => Some(Self::Africa(IanaAfrica::Jamestown)),
            CountryCode::KNA => Some(Self::America(IanaAmerica::StKitts)), // Saint Kitts and Nevis
            CountryCode::LCA => Some(Self::America(IanaAmerica::StLucia)), // Saint Lucia
            CountryCode::MAF => Some(Self::America(IanaAmerica::Marigot)), // Saint Martin
            CountryCode::SPM => Some(Self::America(IanaAmerica::Miquelon)), // Saint Pierre and Miquelon
            CountryCode::VCT => Some(Self::America(IanaAmerica::StVincent)), // Saint Vincent and the Grenadines
            CountryCode::WSM => Some(Self::Pacific(IanaPacific::Apia)),      // Samoa
            CountryCode::SMR => Some(Self::Europe(IanaEurope::SanMarino)),   // San Marino
            CountryCode::STP => Some(Self::Africa(IanaAfrica::SaoTome)),
            CountryCode::SAU => Some(Self::Asia(IanaAsia::Riyadh)), // Saudi Arabia
            CountryCode::SEN => Some(Self::Africa(IanaAfrica::Dakar)),
            CountryCode::SRB => Some(Self::Europe(IanaEurope::Belgrade)), // Serbia
            CountryCode::XKX => Some(Self::Europe(IanaEurope::Belgrade)), // Kosovo
            CountryCode::SYC => Some(Self::Indian(IanaIndian::Mahe)),     // Seychelles
            CountryCode::SLE => Some(Self::Africa(IanaAfrica::Freetown)), // Sierra Leone
            CountryCode::SGP => Some(Self::Asia(IanaAsia::Singapore)),    // Singapore
            CountryCode::SXM => Some(Self::America(IanaAmerica::LowerPrinces)), // Sint Maarten
            CountryCode::SVK => Some(Self::Europe(IanaEurope::Bratislava)), // Slovakia
            CountryCode::SVN => Some(Self::Europe(IanaEurope::Ljubljana)), // Slovenia
            CountryCode::SLB => Some(Self::Pacific(IanaPacific::Guadalcanal)), // Solomon Islands
            CountryCode::SOM => Some(Self::Africa(IanaAfrica::Mogadishu)), // Somalia
            CountryCode::ZAF => Some(Self::Africa(IanaAfrica::Johannesburg)), // South Africa
            CountryCode::SGS => Some(Self::Atlantic(IanaAtlantic::SouthGeorgia)), // South Georgia
            CountryCode::KOR => Some(Self::Asia(IanaAsia::Seoul)),        // South Korea
            CountryCode::SSD => Some(Self::Africa(IanaAfrica::Juba)),
            CountryCode::ESP => Some(Self::Europe(IanaEurope::Madrid)), // Spain
            CountryCode::LKA => Some(Self::Asia(IanaAsia::Colombo)),    // Sri Lanka
            CountryCode::SDN => Some(Self::Africa(IanaAfrica::Khartoum)), // Sudan
            CountryCode::SUR => Some(Self::America(IanaAmerica::Paramaribo)), // Suriname
            CountryCode::SJM => Some(Self::Arctic(IanaArctic::Longyearbyen)), // Svalbard and Jan Mayen
            CountryCode::SWZ => Some(Self::Africa(IanaAfrica::Mbabane)),
            CountryCode::SWE => Some(Self::Europe(IanaEurope::Stockholm)), // Sweden
            CountryCode::CHE => Some(Self::Europe(IanaEurope::Zurich)),    // Switzerland
            CountryCode::SYR => Some(Self::Asia(IanaAsia::Damascus)),      // Syria
            CountryCode::TWN => Some(Self::Asia(IanaAsia::Taipei)),        // Taiwan
            CountryCode::TJK => Some(Self::Asia(IanaAsia::Dushanbe)),      // Tajikistan
            CountryCode::TZA => Some(Self::Africa(IanaAfrica::DarEsSalaam)), // Tanzania
            CountryCode::THA => Some(Self::Asia(IanaAsia::Bangkok)),       // Thailand
            CountryCode::TGO => Some(Self::Africa(IanaAfrica::Lome)),
            CountryCode::TKL => Some(Self::Pacific(IanaPacific::Fakaofo)), // Tokelau
            CountryCode::TON => Some(Self::Pacific(IanaPacific::Tongatapu)), // Tonga
            CountryCode::TTO => Some(Self::America(IanaAmerica::PortOfSpain)), // Trinidad and Tobago
            CountryCode::TUN => Some(Self::Africa(IanaAfrica::Tunis)),
            CountryCode::TUR => Some(Self::Europe(IanaEurope::Istanbul)),
            CountryCode::TKM => Some(Self::Asia(IanaAsia::Ashgabat)),
            CountryCode::TCA => Some(Self::America(IanaAmerica::GrandTurk)), // Turks and Caicos Islands
            CountryCode::TUV => Some(Self::Pacific(IanaPacific::Funafuti)),
            CountryCode::VIR => Some(Self::America(IanaAmerica::StThomas)), // U.S. Virgin Islands
            CountryCode::UGA => Some(Self::Africa(IanaAfrica::Kampala)),
            CountryCode::UKR => Some(Self::Europe(IanaEurope::Kyiv)),
            CountryCode::ARE => Some(Self::Asia(IanaAsia::Dubai)),
            CountryCode::GBR => Some(Self::Europe(IanaEurope::London)),
            CountryCode::USA => get_us_time_zone(time_zone, is_day_light_saving), // United States
            CountryCode::UMI => Some(Self::Pacific(IanaPacific::Midway)), // United States Minor Outlying Islands
            CountryCode::URY => Some(Self::America(IanaAmerica::Montevideo)), // Uruguay
            CountryCode::UZB => Some(Self::Asia(IanaAsia::Tashkent)),     // Uzbekistan
            CountryCode::VUT => Some(Self::Pacific(IanaPacific::Efate)),  // Vanuatu
            CountryCode::VAT => Some(Self::Europe(IanaEurope::Vatican)),
            CountryCode::VEN => Some(Self::America(IanaAmerica::Caracas)),
            CountryCode::VNM => Some(Self::Asia(IanaAsia::HoChiMinh)),
            CountryCode::WLF => Some(Self::Pacific(IanaPacific::Wallis)),
            CountryCode::ESH => Some(Self::Africa(IanaAfrica::ElAaiun)),
            CountryCode::YEM => Some(Self::Asia(IanaAsia::Aden)),
            CountryCode::ZMB => Some(Self::Africa(IanaAfrica::Lusaka)),
            CountryCode::ZWE => Some(Self::Africa(IanaAfrica::Harare)),
        }
    }

    pub fn as_str(&self) -> ShortString {
        match self {
            IanaTimeZone::Europe(country) => {
                let mut result = ShortString::from_str(EUROPE_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Africa(country) => {
                let mut result = ShortString::from_str(AFRICA_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Asia(country) => {
                let mut result = ShortString::from_str(ASIA_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::America(country) => {
                let mut result = ShortString::from_str(AMERICA_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Antarctica(country) => {
                let mut result = ShortString::from_str(ANTARCTICA_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Arctic(country) => {
                let mut result = ShortString::from_str(ARCTIC_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Atlantic(country) => {
                let mut result = ShortString::from_str(ATLANTIC_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Australia(country) => {
                let mut result = ShortString::from_str(AUSTRALIA_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Indian(country) => {
                let mut result = ShortString::from_str(INDIAN_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
            IanaTimeZone::Pacific(country) => {
                let mut result = ShortString::from_str(PACIFIC_PREFIX).unwrap();
                result.push_str(country.as_str());
                result
            }
        }
    }
}

#[cfg(feature = "time-zones")]
fn get_rus_time_zone(time_zone: TimeZoneGmtOffset) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_2 => Some(IanaTimeZone::Europe(IanaEurope::Kaliningrad)), // Kaliningrad
        UTC_3 => Some(IanaTimeZone::Europe(IanaEurope::Moscow)),      // Moscow
        UTC_4 => Some(IanaTimeZone::Europe(IanaEurope::Samara)),      // Samara
        UTC_5 => Some(IanaTimeZone::Asia(IanaAsia::Yekaterinburg)),   // Yekaterinburg
        UTC_6 => Some(IanaTimeZone::Asia(IanaAsia::Omsk)),            // Omsk
        UTC_7 => Some(IanaTimeZone::Asia(IanaAsia::Krasnoyarsk)),     // Krasnoyarsk
        UTC_8 => Some(IanaTimeZone::Asia(IanaAsia::Irkutsk)),         // Irkutsk
        UTC_9 => Some(IanaTimeZone::Asia(IanaAsia::Yakutsk)),         // Yakutsk
        UTC_10 => Some(IanaTimeZone::Asia(IanaAsia::Vladivostok)),    // Vladivostok
        UTC_11 => Some(IanaTimeZone::Asia(IanaAsia::Magadan)),        // Magadan
        UTC_12 => Some(IanaTimeZone::Asia(IanaAsia::Kamchatka)),      // Kamchatka
        _ => None,
    }
}
#[cfg(feature = "time-zones")]
fn get_us_time_zone(
    time_zone: TimeZoneGmtOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_4 => IanaTimeZone::America(IanaAmerica::NewYork).into(), // Central Time
            UTC_MINUS_5 => IanaTimeZone::America(IanaAmerica::Chicago).into(), // Central Time
            UTC_MINUS_6 => IanaTimeZone::America(IanaAmerica::Denver).into(),  // Mountain Time
            UTC_MINUS_7 => IanaTimeZone::America(IanaAmerica::LosAngeles).into(), // Pacific Time
            UTC_MINUS_8 => IanaTimeZone::America(IanaAmerica::Anchorage).into(), // Alaska Time
            UTC_MINUS_10 => IanaTimeZone::America(IanaAmerica::Honolulu).into(), // Hawaii-Aleutian Time
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_5 => IanaTimeZone::America(IanaAmerica::NewYork).into(), // Central Time
            UTC_MINUS_6 => IanaTimeZone::America(IanaAmerica::Chicago).into(), // Central Standard Time
            UTC_MINUS_7 => IanaTimeZone::America(IanaAmerica::Denver).into(), // Mountain Standard Time
            UTC_MINUS_8 => IanaTimeZone::America(IanaAmerica::LosAngeles).into(), // Pacific Standard Time
            UTC_MINUS_9 => IanaTimeZone::America(IanaAmerica::Anchorage).into(), // Alaska Standard Time
            UTC_MINUS_10 => IanaTimeZone::America(IanaAmerica::Honolulu).into(), // Hawaii Standard Time
            _ => None,
        }
    }
}
#[cfg(feature = "time-zones")]
fn get_canada_time_zone(
    time_zone: TimeZoneGmtOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_2_30 => IanaTimeZone::America(IanaAmerica::StJohns).into(), // Newfoundland Daylight Time (UTC-2:30 DST)
            UTC_MINUS_3 => IanaTimeZone::America(IanaAmerica::Halifax).into(), // Atlantic Daylight Time (UTC-3 DST)
            UTC_MINUS_4 => IanaTimeZone::America(IanaAmerica::Toronto).into(), // Atlantic Daylight Time (UTC-3 DST)
            UTC_MINUS_5 => IanaTimeZone::America(IanaAmerica::Winnipeg).into(), // Central Daylight Time (UTC-5 DST)
            UTC_MINUS_6 => IanaTimeZone::America(IanaAmerica::Edmonton).into(), // Mountain Daylight Time (UTC-6 DST)
            UTC_MINUS_7 => IanaTimeZone::America(IanaAmerica::Vancouver).into(), // Pacific Daylight Time (UTC-7 DST)
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_3_30 => IanaTimeZone::America(IanaAmerica::StJohns).into(), // Newfoundland Standard Time
            UTC_MINUS_4 => IanaTimeZone::America(IanaAmerica::Halifax).into(), // Atlantic Standard Time
            UTC_MINUS_5 => IanaTimeZone::America(IanaAmerica::Toronto).into(), // Atlantic Daylight Time
            UTC_MINUS_6 => IanaTimeZone::America(IanaAmerica::Winnipeg).into(), // Central Standard Time
            UTC_MINUS_7 => IanaTimeZone::America(IanaAmerica::Edmonton).into(), // Mountain Standard Time
            UTC_MINUS_8 => IanaTimeZone::America(IanaAmerica::Vancouver).into(), // Pacific Standard Time
            _ => None,
        }
    }
}
#[cfg(feature = "time-zones")]
fn get_australia_time_zone(
    time_zone: TimeZoneGmtOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_8 => Some(IanaTimeZone::Australia(IanaAustralia::Perth)), // No DST
            UTC_8_45 => Some(IanaTimeZone::Australia(IanaAustralia::Ecula)), // No DST
            UTC_10_30 => Some(IanaTimeZone::Australia(IanaAustralia::Adelaide)),
            UTC_11 => Some(IanaTimeZone::Australia(IanaAustralia::Sydney)), //Lord_Howe as well
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_8 => Some(IanaTimeZone::Australia(IanaAustralia::Perth)), // No DST
            UTC_8_45 => Some(IanaTimeZone::Australia(IanaAustralia::Ecula)), // No DST
            UTC_9_30 => Some(IanaTimeZone::Australia(IanaAustralia::Adelaide)),
            UTC_10 => Some(IanaTimeZone::Australia(IanaAustralia::Sydney)), // AEST (UTC+10)
            UTC_10_30 => Some(IanaTimeZone::Australia(IanaAustralia::LordHowe)), // LHST (UTC+10:30)
            _ => None,
        }
    }
}
#[cfg(feature = "time-zones")]
fn get_brazil_time_zone(time_zone: TimeZoneGmtOffset) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_MINUS_2 => Some(IanaTimeZone::America(IanaAmerica::Noronha)), // Fernando de Noronha
        UTC_MINUS_3 => Some(IanaTimeZone::America(IanaAmerica::SaoPaulo)), // São Paulo
        UTC_MINUS_4 => Some(IanaTimeZone::America(IanaAmerica::Manaus)),  // Manaus
        UTC_MINUS_5 => Some(IanaTimeZone::America(IanaAmerica::RioBranco)), // Rio Branco
        _ => None,
    }
}
#[cfg(feature = "time-zones")]
fn get_indonesia_time_zone(time_zone: TimeZoneGmtOffset) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_7 => Some(IanaTimeZone::Asia(IanaAsia::Jakarta)), // Western Indonesia Time
        UTC_8 => Some(IanaTimeZone::Asia(IanaAsia::Makassar)), // Central Indonesia Time
        UTC_9 => Some(IanaTimeZone::Asia(IanaAsia::Jayapura)), // Eastern Indonesia Time
        _ => None,
    }
}
#[cfg(feature = "time-zones")]
fn get_china_time_zone(time_zone: TimeZoneGmtOffset) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_8 => Some(IanaTimeZone::Asia(IanaAsia::Shanghai)), // China Standard Time
        UTC_6 => Some(IanaTimeZone::Asia(IanaAsia::Urumqi)),   // Xinjiang Time
        _ => None,
    }
}
#[cfg(feature = "time-zones")]
fn get_mexico_time_zone(
    time_zone: TimeZoneGmtOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone> {
    use crate::time_zones::*;

    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_5 => Some(IanaTimeZone::America(IanaAmerica::Cancun)), // No DST
            UTC_MINUS_6 => Some(IanaTimeZone::America(IanaAmerica::MexicoCity)), // No DST
            UTC_MINUS_7 => Some(IanaTimeZone::America(IanaAmerica::Tijuana)),
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_5 => Some(IanaTimeZone::America(IanaAmerica::Cancun)), // No DST
            UTC_MINUS_6 => Some(IanaTimeZone::America(IanaAmerica::MexicoCity)), // No DST
            UTC_MINUS_7 => Some(IanaTimeZone::America(IanaAmerica::Mazatlan)),
            UTC_MINUS_8 => Some(IanaTimeZone::America(IanaAmerica::Tijuana)),
            _ => None,
        }
    }
}
#[cfg(feature = "time-zones")]
fn get_chile_time_zone(
    time_zone: TimeZoneGmtOffset,
    is_day_light_saving: bool,
) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    if is_day_light_saving {
        match time_zone.as_seconds() {
            UTC_MINUS_3 => Some(IanaTimeZone::America(IanaAmerica::PuntaArenas)), // No DST
            _ => None,
        }
    } else {
        match time_zone.as_seconds() {
            UTC_MINUS_3 => Some(IanaTimeZone::America(IanaAmerica::PuntaArenas)), // No DST
            UTC_MINUS_4 => Some(IanaTimeZone::America(IanaAmerica::Santiago)), // Chile Standard Time
            _ => None,
        }
    }
}
#[cfg(feature = "time-zones")]
fn get_mongolia_time_zone(time_zone: TimeZoneGmtOffset) -> Option<IanaTimeZone> {
    use crate::time_zones::*;
    match time_zone.as_seconds() {
        UTC_7 => Some(IanaTimeZone::Asia(IanaAsia::Hovd)),
        UTC_8 => Some(IanaTimeZone::Asia(IanaAsia::Ulaanbaatar)), // Ulaanbaatar Time
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
            IanaTimeZone::Asia(country) => assert_eq!(country.as_str(), "Dubai"),
            _ => panic!("Expected Asia time zone"),
        }

        assert_eq!(time_zone.as_str().as_str(), "Asia/Dubai");
    }
}
