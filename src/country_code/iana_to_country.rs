use std::collections::HashMap;

use crate::country_code::CountryCode;

lazy_static::lazy_static! {
    pub static ref IANA_TO_COUNTRY_CODE: HashMap<&'static str, CountryCode> = {
        create_iana_to_country_map()
    };
}

// Function to create a HashMap mapping IANA time zone names to Country ISO3 codes
fn create_iana_to_country_map() -> HashMap<&'static str, CountryCode> {
    let mut iana_map: HashMap<&'static str, CountryCode> = HashMap::new();

    // Africa (54 countries/territories with time zones)
    iana_map.insert("Africa/Abidjan", CountryCode::CIV); // Côte d'Ivoire
    iana_map.insert("Africa/Accra", CountryCode::GHA); // Ghana
    iana_map.insert("Africa/Addis_Ababa", CountryCode::ETH); // Ethiopia
    iana_map.insert("Africa/Algiers", CountryCode::DZA); // Algeria
    iana_map.insert("Africa/Asmara", CountryCode::ERI); // Eritrea
    iana_map.insert("Africa/Bamako", CountryCode::MLI); // Mali
    iana_map.insert("Africa/Bangui", CountryCode::CAF); // Central African Republic
    iana_map.insert("Africa/Banjul", CountryCode::GMB); // Gambia
    iana_map.insert("Africa/Bissau", CountryCode::GNB); // Guinea-Bissau
    iana_map.insert("Africa/Blantyre", CountryCode::MWI); // Malawi
    iana_map.insert("Africa/Brazzaville", CountryCode::COG); // Congo
    iana_map.insert("Africa/Bujumbura", CountryCode::BDI); // Burundi
    iana_map.insert("Africa/Cairo", CountryCode::EGY); // Egypt
    iana_map.insert("Africa/Casablanca", CountryCode::MAR); // Morocco
    iana_map.insert("Africa/Ceuta", CountryCode::ESP); // Spain (Ceuta)
    iana_map.insert("Africa/Conakry", CountryCode::GIN); // Guinea
    iana_map.insert("Africa/Dakar", CountryCode::SEN); // Senegal
    iana_map.insert("Africa/Dar_es_Salaam", CountryCode::TZA); // Tanzania
    iana_map.insert("Africa/Djibouti", CountryCode::DJI); // Djibouti
    iana_map.insert("Africa/Douala", CountryCode::CMR); // Cameroon
    iana_map.insert("Africa/El_Aaiun", CountryCode::ESH); // Western Sahara
    iana_map.insert("Africa/Freetown", CountryCode::SLE); // Sierra Leone
    iana_map.insert("Africa/Gaborone", CountryCode::BWA); // Botswana
    iana_map.insert("Africa/Harare", CountryCode::ZWE); // Zimbabwe
    iana_map.insert("Africa/Johannesburg", CountryCode::ZAF); // South Africa
    iana_map.insert("Africa/Juba", CountryCode::SSD); // South Sudan
    iana_map.insert("Africa/Kampala", CountryCode::UGA); // Uganda
    iana_map.insert("Africa/Khartoum", CountryCode::SDN); // Sudan
    iana_map.insert("Africa/Kigali", CountryCode::RWA); // Rwanda
    iana_map.insert("Africa/Kinshasa", CountryCode::COD); // DR Congo
    iana_map.insert("Africa/Lagos", CountryCode::NGA); // Nigeria
    iana_map.insert("Africa/Libreville", CountryCode::GAB); // Gabon
    iana_map.insert("Africa/Lome", CountryCode::TGO); // Togo
    iana_map.insert("Africa/Luanda", CountryCode::AGO); // Angola
    iana_map.insert("Africa/Lubumbashi", CountryCode::COD); // DR Congo
    iana_map.insert("Africa/Lusaka", CountryCode::ZMB); // Zambia
    iana_map.insert("Africa/Malabo", CountryCode::GNQ); // Equatorial Guinea
    iana_map.insert("Africa/Maputo", CountryCode::MOZ); // Mozambique
    iana_map.insert("Africa/Maseru", CountryCode::LSO); // Lesotho
    iana_map.insert("Africa/Mbabane", CountryCode::SWZ); // Eswatini
    iana_map.insert("Africa/Mogadishu", CountryCode::SOM); // Somalia
    iana_map.insert("Africa/Monrovia", CountryCode::LBR); // Liberia
    iana_map.insert("Africa/Nairobi", CountryCode::KEN); // Kenya
    iana_map.insert("Africa/Ndjamena", CountryCode::TCD); // Chad
    iana_map.insert("Africa/Niamey", CountryCode::NER); // Niger
    iana_map.insert("Africa/Nouakchott", CountryCode::MRT); // Mauritania
    iana_map.insert("Africa/Ouagadougou", CountryCode::BFA); // Burkina Faso
    iana_map.insert("Africa/Porto-Novo", CountryCode::BEN); // Benin
    iana_map.insert("Africa/Sao_Tome", CountryCode::STP); // São Tomé and Príncipe
    iana_map.insert("Africa/Tripoli", CountryCode::LBY); // Libya
    iana_map.insert("Africa/Tunis", CountryCode::TUN); // Tunisia
    iana_map.insert("Africa/Windhoek", CountryCode::NAM); // Namibia

    // America (35 countries/territories with time zones)
    iana_map.insert("America/Adak", CountryCode::USA); // United States
    iana_map.insert("America/Anchorage", CountryCode::USA); // United States
    iana_map.insert("America/Anguilla", CountryCode::AIA); // Anguilla
    iana_map.insert("America/Antigua", CountryCode::ATG); // Antigua and Barbuda
    iana_map.insert("America/Araguaina", CountryCode::BRA); // Brazil
    iana_map.insert("America/Argentina/Buenos_Aires", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Catamarca", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Cordoba", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Jujuy", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/La_Rioja", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Mendoza", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Rio_Gallegos", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Salta", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/San_Juan", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/San_Luis", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Tucuman", CountryCode::ARG); // Argentina
    iana_map.insert("America/Argentina/Ushuaia", CountryCode::ARG); // Argentina
    iana_map.insert("America/Aruba", CountryCode::ABW); // Aruba
    iana_map.insert("America/Asuncion", CountryCode::PRY); // Paraguay
    iana_map.insert("America/Atikokan", CountryCode::CAN); // Canada
    iana_map.insert("America/Bahia", CountryCode::BRA); // Brazil
    iana_map.insert("America/Bahia_Banderas", CountryCode::MEX); // Mexico
    iana_map.insert("America/Barbados", CountryCode::BRB); // Barbados
    iana_map.insert("America/Belem", CountryCode::BRA); // Brazil
    iana_map.insert("America/Belize", CountryCode::BLZ); // Belize
    iana_map.insert("America/Blanc-Sablon", CountryCode::CAN); // Canada
    iana_map.insert("America/Boa_Vista", CountryCode::BRA); // Brazil
    iana_map.insert("America/Bogota", CountryCode::COL); // Colombia
    iana_map.insert("America/Boise", CountryCode::USA); // United States
    iana_map.insert("America/Cambridge_Bay", CountryCode::CAN); // Canada
    iana_map.insert("America/Campo_Grande", CountryCode::BRA); // Brazil
    iana_map.insert("America/Cancun", CountryCode::MEX); // Mexico
    iana_map.insert("America/Caracas", CountryCode::VEN); // Venezuela
    iana_map.insert("America/Cayenne", CountryCode::GUF); // French Guiana
    iana_map.insert("America/Cayman", CountryCode::CYM); // Cayman Islands
    iana_map.insert("America/Chicago", CountryCode::USA); // United States
    iana_map.insert("America/Chihuahua", CountryCode::MEX); // Mexico
    iana_map.insert("America/Ciudad_Juarez", CountryCode::MEX); // Mexico
    iana_map.insert("America/Costa_Rica", CountryCode::CRI); // Costa Rica
    iana_map.insert("America/Creston", CountryCode::CAN); // Canada
    iana_map.insert("America/Cuiaba", CountryCode::BRA); // Brazil
    iana_map.insert("America/Curacao", CountryCode::CUW); // Curaçao
    iana_map.insert("America/Danmarkshavn", CountryCode::GRL); // Greenland
    iana_map.insert("America/Dawson", CountryCode::CAN); // Canada
    iana_map.insert("America/Dawson_Creek", CountryCode::CAN); // Canada
    iana_map.insert("America/Denver", CountryCode::USA); // United States
    iana_map.insert("America/Detroit", CountryCode::USA); // United States
    iana_map.insert("America/Dominica", CountryCode::DMA); // Dominica
    iana_map.insert("America/Edmonton", CountryCode::CAN); // Canada
    iana_map.insert("America/Eirunepe", CountryCode::BRA); // Brazil
    iana_map.insert("America/El_Salvador", CountryCode::SLV); // El Salvador
    iana_map.insert("America/Fort_Nelson", CountryCode::CAN); // Canada
    iana_map.insert("America/Fortaleza", CountryCode::BRA); // Brazil
    iana_map.insert("America/Glace_Bay", CountryCode::CAN); // Canada
    iana_map.insert("America/Goose_Bay", CountryCode::CAN); // Canada
    iana_map.insert("America/Grand_Turk", CountryCode::TCA); // Turks and Caicos Islands
    iana_map.insert("America/Grenada", CountryCode::GRD); // Grenada
    iana_map.insert("America/Guadeloupe", CountryCode::GLP); // Guadeloupe
    iana_map.insert("America/Guatemala", CountryCode::GTM); // Guatemala
    iana_map.insert("America/Guayaquil", CountryCode::ECU); // Ecuador
    iana_map.insert("America/Guyana", CountryCode::GUY); // Guyana
    iana_map.insert("America/Halifax", CountryCode::CAN); // Canada
    iana_map.insert("America/Havana", CountryCode::CUB); // Cuba
    iana_map.insert("America/Hermosillo", CountryCode::MEX); // Mexico
    iana_map.insert("America/Indiana/Indianapolis", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Knox", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Marengo", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Petersburg", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Tell_City", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Vevay", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Vincennes", CountryCode::USA); // United States
    iana_map.insert("America/Indiana/Winamac", CountryCode::USA); // United States
    iana_map.insert("America/Inuvik", CountryCode::CAN); // Canada
    iana_map.insert("America/Iqaluit", CountryCode::CAN); // Canada
    iana_map.insert("America/Jamaica", CountryCode::JAM); // Jamaica
    iana_map.insert("America/Juneau", CountryCode::USA); // United States
    iana_map.insert("America/Kentucky/Louisville", CountryCode::USA); // United States
    iana_map.insert("America/Kentucky/Monticello", CountryCode::USA); // United States
    iana_map.insert("America/Kralendijk", CountryCode::BES); // Bonaire, Sint Eustatius and Saba
    iana_map.insert("America/La_Paz", CountryCode::BOL); // Bolivia
    iana_map.insert("America/Lima", CountryCode::PER); // Peru
    iana_map.insert("America/Los_Angeles", CountryCode::USA); // United States
    iana_map.insert("America/Maceio", CountryCode::BRA); // Brazil
    iana_map.insert("America/Managua", CountryCode::NIC); // Nicaragua
    iana_map.insert("America/Manaus", CountryCode::BRA); // Brazil
    iana_map.insert("America/Marigot", CountryCode::MAF); // Saint Martin
    iana_map.insert("America/Martinique", CountryCode::MTQ); // Martinique
    iana_map.insert("America/Matamoros", CountryCode::MEX); // Mexico
    iana_map.insert("America/Mazatlan", CountryCode::MEX); // Mexico
    iana_map.insert("America/Menominee", CountryCode::USA); // United States
    iana_map.insert("America/Merida", CountryCode::MEX); // Mexico
    iana_map.insert("America/Metlakatla", CountryCode::USA); // United States
    iana_map.insert("America/Mexico_City", CountryCode::MEX); // Mexico
    iana_map.insert("America/Miquelon", CountryCode::SPM); // Saint Pierre and Miquelon
    iana_map.insert("America/Moncton", CountryCode::CAN); // Canada
    iana_map.insert("America/Monterrey", CountryCode::MEX); // Mexico
    iana_map.insert("America/Montevideo", CountryCode::URY); // Uruguay
    iana_map.insert("America/Montserrat", CountryCode::MSR); // Montserrat
    iana_map.insert("America/Nassau", CountryCode::BHS); // Bahamas
    iana_map.insert("America/New_York", CountryCode::USA); // United States
    iana_map.insert("America/Nome", CountryCode::USA); // United States
    iana_map.insert("America/Noronha", CountryCode::BRA); // Brazil
    iana_map.insert("America/North_Dakota/Beulah", CountryCode::USA); // United States
    iana_map.insert("America/North_Dakota/Center", CountryCode::USA); // United States
    iana_map.insert("America/North_Dakota/New_Salem", CountryCode::USA); // United States
    iana_map.insert("America/Nuuk", CountryCode::GRL); // Greenland
    iana_map.insert("America/Ojinaga", CountryCode::MEX); // Mexico
    iana_map.insert("America/Panama", CountryCode::PAN); // Panama
    iana_map.insert("America/Paramaribo", CountryCode::SUR); // Suriname
    iana_map.insert("America/Phoenix", CountryCode::USA); // United States
    iana_map.insert("America/Port-au-Prince", CountryCode::HTI); // Haiti
    iana_map.insert("America/Port_of_Spain", CountryCode::TTO); // Trinidad and Tobago
    iana_map.insert("America/Porto_Velho", CountryCode::BRA); // Brazil
    iana_map.insert("America/Puerto_Rico", CountryCode::PRI); // Puerto Rico
    iana_map.insert("America/Punta_Arenas", CountryCode::CHL); // Chile
    iana_map.insert("America/Rainy_River", CountryCode::CAN); // Canada
    iana_map.insert("America/Rankin_Inlet", CountryCode::CAN); // Canada
    iana_map.insert("America/Recife", CountryCode::BRA); // Brazil
    iana_map.insert("America/Regina", CountryCode::CAN); // Canada
    iana_map.insert("America/Resolute", CountryCode::CAN); // Canada
    iana_map.insert("America/Rio_Branco", CountryCode::BRA); // Brazil
    iana_map.insert("America/Santarem", CountryCode::BRA); // Brazil
    iana_map.insert("America/Santiago", CountryCode::CHL); // Chile
    iana_map.insert("America/Santo_Domingo", CountryCode::DOM); // Dominican Republic
    iana_map.insert("America/Sao_Paulo", CountryCode::BRA); // Brazil
    iana_map.insert("America/Scoresbysund", CountryCode::GRL); // Greenland
    iana_map.insert("America/Sitka", CountryCode::USA); // United States
    iana_map.insert("America/St_Barthelemy", CountryCode::BLM); // Saint Barthélemy
    iana_map.insert("America/St_Johns", CountryCode::CAN); // Canada
    iana_map.insert("America/St_Kitts", CountryCode::KNA); // Saint Kitts and Nevis
    iana_map.insert("America/St_Lucia", CountryCode::LCA); // Saint Lucia
    iana_map.insert("America/St_Thomas", CountryCode::VIR); // U.S. Virgin Islands
    iana_map.insert("America/St_Vincent", CountryCode::VCT); // Saint Vincent and the Grenadines
    iana_map.insert("America/Swift_Current", CountryCode::CAN); // Canada
    iana_map.insert("America/Tegucigalpa", CountryCode::HND); // Honduras
    iana_map.insert("America/Thule", CountryCode::GRL); // Greenland
    iana_map.insert("America/Tijuana", CountryCode::MEX); // Mexico
    iana_map.insert("America/Toronto", CountryCode::CAN); // Canada
    iana_map.insert("America/Tortola", CountryCode::VGB); // British Virgin Islands
    iana_map.insert("America/Vancouver", CountryCode::CAN); // Canada
    iana_map.insert("America/Whitehorse", CountryCode::CAN); // Canada
    iana_map.insert("America/Winnipeg", CountryCode::CAN); // Canada
    iana_map.insert("America/Yakutat", CountryCode::USA); // United States
    iana_map.insert("America/Yellowknife", CountryCode::CAN); // Canada

    // Antarctica (12 zones, mapped to administering countries)
    iana_map.insert("Antarctica/Casey", CountryCode::AUS); // Australia
    iana_map.insert("Antarctica/Davis", CountryCode::AUS); // Australia
    iana_map.insert("Antarctica/DumontDUrville", CountryCode::AUS); // Australia
    iana_map.insert("Antarctica/Macquarie", CountryCode::AUS); // Australia
    iana_map.insert("Antarctica/Mawson", CountryCode::AUS); // Australia
    iana_map.insert("Antarctica/McMurdo", CountryCode::NZL); // New Zealand
    iana_map.insert("Antarctica/Palmer", CountryCode::CHL); // Chile
    iana_map.insert("Antarctica/Rothera", CountryCode::GBR); // United Kingdom
    iana_map.insert("Antarctica/Syowa", CountryCode::JPN); // Japan
    iana_map.insert("Antarctica/Troll", CountryCode::NOR); // Norway
    iana_map.insert("Antarctica/Vostok", CountryCode::RUS); // Russia

    // Asia (39 countries/territories with time zones)
    iana_map.insert("Asia/Aden", CountryCode::YEM); // Yemen
    iana_map.insert("Asia/Almaty", CountryCode::KAZ); // Kazakhstan
    iana_map.insert("Asia/Amman", CountryCode::JOR); // Jordan
    iana_map.insert("Asia/Anadyr", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Aqtau", CountryCode::KAZ); // Kazakhstan
    iana_map.insert("Asia/Aqtobe", CountryCode::KAZ); // Kazakhstan
    iana_map.insert("Asia/Ashgabat", CountryCode::TKM); // Turkmenistan
    iana_map.insert("Asia/Atyrau", CountryCode::KAZ); // Kazakhstan
    iana_map.insert("Asia/Baghdad", CountryCode::IRQ); // Iraq
    iana_map.insert("Asia/Bahrain", CountryCode::BHR); // Bahrain
    iana_map.insert("Asia/Baku", CountryCode::AZE); // Azerbaijan
    iana_map.insert("Asia/Bangkok", CountryCode::THA); // Thailand
    iana_map.insert("Asia/Barnaul", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Beirut", CountryCode::LBN); // Lebanon
    iana_map.insert("Asia/Bishkek", CountryCode::KGZ); // Kyrgyzstan
    iana_map.insert("Asia/Brunei", CountryCode::BRN); // Brunei
    iana_map.insert("Asia/Chita", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Choibalsan", CountryCode::MNG); // Mongolia
    iana_map.insert("Asia/Colombo", CountryCode::LKA); // Sri Lanka
    iana_map.insert("Asia/Damascus", CountryCode::SYR); // Syria
    iana_map.insert("Asia/Dhaka", CountryCode::BGD); // Bangladesh
    iana_map.insert("Asia/Dili", CountryCode::TLS); // Timor-Leste
    iana_map.insert("Asia/Dubai", CountryCode::ARE); // United Arab Emirates
    iana_map.insert("Asia/Dushanbe", CountryCode::TJK); // Tajikistan
    iana_map.insert("Asia/Famagusta", CountryCode::CYP); // Cyprus
    iana_map.insert("Asia/Gaza", CountryCode::PSE); // Palestine
    iana_map.insert("Asia/Hebron", CountryCode::PSE); // Palestine
    iana_map.insert("Asia/Ho_Chi_Minh", CountryCode::VNM); // Vietnam
    iana_map.insert("Asia/Hong_Kong", CountryCode::HKG); // Hong Kong
    iana_map.insert("Asia/Hovd", CountryCode::MNG); // Mongolia
    iana_map.insert("Asia/Irkutsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Jakarta", CountryCode::IDN); // Indonesia
    iana_map.insert("Asia/Jayapura", CountryCode::IDN); // Indonesia
    iana_map.insert("Asia/Jerusalem", CountryCode::ISR); // Israel
    iana_map.insert("Asia/Kabul", CountryCode::AFG); // Afghanistan
    iana_map.insert("Asia/Kamchatka", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Karachi", CountryCode::PAK); // Pakistan
    iana_map.insert("Asia/Kathmandu", CountryCode::NPL); // Nepal
    iana_map.insert("Asia/Khandyga", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Kolkata", CountryCode::IND); // India
    iana_map.insert("Asia/Krasnoyarsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Kuala_Lumpur", CountryCode::MYS); // Malaysia
    iana_map.insert("Asia/Kuching", CountryCode::MYS); // Malaysia
    iana_map.insert("Asia/Kuwait", CountryCode::KWT); // Kuwait
    iana_map.insert("Asia/Macau", CountryCode::MAC); // Macau
    iana_map.insert("Asia/Magadan", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Makassar", CountryCode::IDN); // Indonesia
    iana_map.insert("Asia/Manila", CountryCode::PHL); // Philippines
    iana_map.insert("Asia/Muscat", CountryCode::OMN); // Oman
    iana_map.insert("Asia/Nicosia", CountryCode::CYP); // Cyprus
    iana_map.insert("Asia/Novokuznetsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Novosibirsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Omsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Oral", CountryCode::KAZ); // Kazakhstan
    iana_map.insert("Asia/Phnom_Penh", CountryCode::KHM); // Cambodia
    iana_map.insert("Asia/Pontianak", CountryCode::IDN); // Indonesia
    iana_map.insert("Asia/Pyongyang", CountryCode::PRK); // North Korea
    iana_map.insert("Asia/Qatar", CountryCode::QAT); // Qatar
    iana_map.insert("Asia/Qostanay", CountryCode::KAZ); // Kazakhstan
    iana_map.insert("Asia/Riyadh", CountryCode::SAU); // Saudi Arabia
    iana_map.insert("Asia/Sakhalin", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Samarkand", CountryCode::UZB); // Uzbekistan
    iana_map.insert("Asia/Seoul", CountryCode::KOR); // South Korea
    iana_map.insert("Asia/Shanghai", CountryCode::CHN); // China
    iana_map.insert("Asia/Singapore", CountryCode::SGP); // Singapore
    iana_map.insert("Asia/Srednekolymsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Taipei", CountryCode::TWN); // Taiwan
    iana_map.insert("Asia/Tashkent", CountryCode::UZB); // Uzbekistan
    iana_map.insert("Asia/Tbilisi", CountryCode::GEO); // Georgia
    iana_map.insert("Asia/Tehran", CountryCode::IRN); // Iran
    iana_map.insert("Asia/Thimphu", CountryCode::BTN); // Bhutan
    iana_map.insert("Asia/Tokyo", CountryCode::JPN); // Japan
    iana_map.insert("Asia/Tomsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Ulaanbaatar", CountryCode::MNG); // Mongolia
    iana_map.insert("Asia/Urumqi", CountryCode::CHN); // China
    iana_map.insert("Asia/Ust-Nera", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Vientiane", CountryCode::LAO); // Laos
    iana_map.insert("Asia/Vladivostok", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Yakutsk", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Yangon", CountryCode::MMR); // Myanmar
    iana_map.insert("Asia/Yekaterinburg", CountryCode::RUS); // Russia
    iana_map.insert("Asia/Yerevan", CountryCode::ARM); // Armenia

    // Atlantic (8 countries/territories with time zones)
    iana_map.insert("Atlantic/Azores", CountryCode::PRT); // Portugal
    iana_map.insert("Atlantic/Bermuda", CountryCode::BMU); // Bermuda
    iana_map.insert("Atlantic/Canary", CountryCode::ESP); // Spain
    iana_map.insert("Atlantic/Cape_Verde", CountryCode::CPV); // Cape Verde
    iana_map.insert("Atlantic/Faroe", CountryCode::FRO); // Faroe Islands
    iana_map.insert("Atlantic/Madeira", CountryCode::PRT); // Portugal
    iana_map.insert("Atlantic/Reykjavik", CountryCode::ISL); // Iceland
    iana_map.insert("Atlantic/South_Georgia", CountryCode::SGS); // South Georgia and the South Sandwich Islands
    iana_map.insert("Atlantic/St_Helena", CountryCode::SHN); // Saint Helena
    iana_map.insert("Atlantic/Stanley", CountryCode::FLK); // Falkland Islands

    // Australia (9 zones, all mapped to AUS)
    iana_map.insert("Australia/Adelaide", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Brisbane", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Broken_Hill", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Darwin", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Eucla", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Hobart", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Lindeman", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Melbourne", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Perth", CountryCode::AUS); // Australia
    iana_map.insert("Australia/Sydney", CountryCode::AUS); // Australia

    // Europe (37 countries/territories with time zones)
    iana_map.insert("Europe/Amsterdam", CountryCode::NLD); // Netherlands
    iana_map.insert("Europe/Andorra", CountryCode::AND); // Andorra
    iana_map.insert("Europe/Astrakhan", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Athens", CountryCode::GRC); // Greece
    iana_map.insert("Europe/Belgrade", CountryCode::SRB); // Serbia
    iana_map.insert("Europe/Berlin", CountryCode::DEU); // Germany
    iana_map.insert("Europe/Bratislava", CountryCode::SVK); // Slovakia
    iana_map.insert("Europe/Brussels", CountryCode::BEL); // Belgium
    iana_map.insert("Europe/Bucharest", CountryCode::ROU); // Romania
    iana_map.insert("Europe/Budapest", CountryCode::HUN); // Hungary
    iana_map.insert("Europe/Busingen", CountryCode::DEU); // Germany
    iana_map.insert("Europe/Chisinau", CountryCode::MDA); // Moldova
    iana_map.insert("Europe/Copenhagen", CountryCode::DNK); // Denmark
    iana_map.insert("Europe/Dublin", CountryCode::IRL); // Ireland
    iana_map.insert("Europe/Gibraltar", CountryCode::GIB); // Gibraltar
    iana_map.insert("Europe/Guernsey", CountryCode::GGY); // Guernsey
    iana_map.insert("Europe/Helsinki", CountryCode::FIN); // Finland
    iana_map.insert("Europe/Isle_of_Man", CountryCode::IMN); // Isle of Man
    iana_map.insert("Europe/Istanbul", CountryCode::TUR); // Turkey
    iana_map.insert("Europe/Jersey", CountryCode::JEY); // Jersey
    iana_map.insert("Europe/Kaliningrad", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Kirov", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Kyiv", CountryCode::UKR); // Ukraine
    iana_map.insert("Europe/Lisbon", CountryCode::PRT); // Portugal
    iana_map.insert("Europe/Ljubljana", CountryCode::SVN); // Slovenia
    iana_map.insert("Europe/London", CountryCode::GBR); // United Kingdom
    iana_map.insert("Europe/Luxembourg", CountryCode::LUX); // Luxembourg
    iana_map.insert("Europe/Madrid", CountryCode::ESP); // Spain
    iana_map.insert("Europe/Malta", CountryCode::MLT); // Malta
    iana_map.insert("Europe/Mariehamn", CountryCode::ALA); // Åland Islands
    iana_map.insert("Europe/Minsk", CountryCode::BLR); // Belarus
    iana_map.insert("Europe/Monaco", CountryCode::MCO); // Monaco
    iana_map.insert("Europe/Moscow", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Oslo", CountryCode::NOR); // Norway
    iana_map.insert("Europe/Paris", CountryCode::FRA); // France
    iana_map.insert("Europe/Podgorica", CountryCode::MNE); // Montenegro
    iana_map.insert("Europe/Prague", CountryCode::CZE); // Czech Republic
    iana_map.insert("Europe/Riga", CountryCode::LVA); // Latvia
    iana_map.insert("Europe/Rome", CountryCode::ITA); // Italy
    iana_map.insert("Europe/Samara", CountryCode::RUS); // Russia
    iana_map.insert("Europe/San_Marino", CountryCode::SMR); // San Marino
    iana_map.insert("Europe/Sarajevo", CountryCode::BIH); // Bosnia and Herzegovina
    iana_map.insert("Europe/Saratov", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Simferopol", CountryCode::UKR); // Ukraine (disputed, occupied by RUS)
    iana_map.insert("Europe/Skopje", CountryCode::MKD); // North Macedonia
    iana_map.insert("Europe/Sofia", CountryCode::BGR); // Bulgaria
    iana_map.insert("Europe/Stockholm", CountryCode::SWE); // Sweden
    iana_map.insert("Europe/Tallinn", CountryCode::EST); // Estonia
    iana_map.insert("Europe/Tirane", CountryCode::ALB); // Albania
    iana_map.insert("Europe/Ulyanovsk", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Vaduz", CountryCode::LIE); // Liechtenstein
    iana_map.insert("Europe/Vatican", CountryCode::VAT); // Vatican City
    iana_map.insert("Europe/Vienna", CountryCode::AUT); // Austria
    iana_map.insert("Europe/Vilnius", CountryCode::LTU); // Lithuania
    iana_map.insert("Europe/Volgograd", CountryCode::RUS); // Russia
    iana_map.insert("Europe/Warsaw", CountryCode::POL); // Poland
    iana_map.insert("Europe/Zagreb", CountryCode::HRV); // Croatia
    iana_map.insert("Europe/Zurich", CountryCode::CHE); // Switzerland

    // Indian (7 countries/territories with time zones)
    iana_map.insert("Indian/Antananarivo", CountryCode::MDG); // Madagascar
    iana_map.insert("Indian/Chagos", CountryCode::IOT); // British Indian Ocean Territory
    iana_map.insert("Indian/Christmas", CountryCode::CXR); // Christmas Island
    iana_map.insert("Indian/Cocos", CountryCode::CCK); // Cocos (Keeling) Islands
    iana_map.insert("Indian/Comoro", CountryCode::COM); // Comoros
    iana_map.insert("Indian/Kerguelen", CountryCode::ATF); // French Southern Territories
    iana_map.insert("Indian/Mahe", CountryCode::SYC); // Seychelles
    iana_map.insert("Indian/Maldives", CountryCode::MDV); // Maldives
    iana_map.insert("Indian/Mauritius", CountryCode::MUS); // Mauritius
    iana_map.insert("Indian/Mayotte", CountryCode::MYT); // Mayotte
    iana_map.insert("Indian/Reunion", CountryCode::REU); // Réunion

    // Pacific (16 countries/territories with time zones)
    iana_map.insert("Pacific/Apia", CountryCode::WSM); // Samoa
    iana_map.insert("Pacific/Auckland", CountryCode::NZL); // New Zealand
    iana_map.insert("Pacific/Bougainville", CountryCode::PNG); // Papua New Guinea
    iana_map.insert("Pacific/Chatham", CountryCode::NZL); // New Zealand
    iana_map.insert("Pacific/Chuuk", CountryCode::FSM); // Micronesia
    iana_map.insert("Pacific/Easter", CountryCode::CHL); // Chile
    iana_map.insert("Pacific/Efate", CountryCode::VUT); // Vanuatu
    iana_map.insert("Pacific/Fakaofo", CountryCode::TKL); // Tokelau
    iana_map.insert("Pacific/Fiji", CountryCode::FJI); // Fiji
    iana_map.insert("Pacific/Funafuti", CountryCode::TUV); // Tuvalu
    iana_map.insert("Pacific/Galapagos", CountryCode::ECU); // Ecuador
    iana_map.insert("Pacific/Gambier", CountryCode::PYF); // French Polynesia
    iana_map.insert("Pacific/Guadalcanal", CountryCode::SLB); // Solomon Islands
    iana_map.insert("Pacific/Guam", CountryCode::GUM); // Guam
    iana_map.insert("Pacific/Honolulu", CountryCode::USA); // United States
    iana_map.insert("Pacific/Kanton", CountryCode::KIR); // Kiribati
    iana_map.insert("Pacific/Kiritimati", CountryCode::KIR); // Kiribati
    iana_map.insert("Pacific/Kosrae", CountryCode::FSM); // Micronesia
    iana_map.insert("Pacific/Kwajalein", CountryCode::MHL); // Marshall Islands
    iana_map.insert("Pacific/Majuro", CountryCode::MHL); // Marshall Islands
    iana_map.insert("Pacific/Marquesas", CountryCode::PYF); // French Polynesia
    iana_map.insert("Pacific/Midway", CountryCode::UMI); // United States Minor Outlying Islands
    iana_map.insert("Pacific/Nauru", CountryCode::NRU); // Nauru
    iana_map.insert("Pacific/Niue", CountryCode::NIU); // Niue
    iana_map.insert("Pacific/Norfolk", CountryCode::NFK); // Norfolk Island
    iana_map.insert("Pacific/Noumea", CountryCode::NCL); // New Caledonia
    iana_map.insert("Pacific/Pago_Pago", CountryCode::ASM); // American Samoa
    iana_map.insert("Pacific/Palau", CountryCode::PLW); // Palau
    iana_map.insert("Pacific/Pitcairn", CountryCode::PCN); // Pitcairn
    iana_map.insert("Pacific/Pohnpei", CountryCode::FSM); // Micronesia
    iana_map.insert("Pacific/Port_Moresby", CountryCode::PNG); // Papua New Guinea
    iana_map.insert("Pacific/Rarotonga", CountryCode::COK); // Cook Islands
    iana_map.insert("Pacific/Saipan", CountryCode::MNP); // Northern Mariana Islands
    iana_map.insert("Pacific/Tahiti", CountryCode::PYF); // French Polynesia
    iana_map.insert("Pacific/Tarawa", CountryCode::KIR); // Kiribati
    iana_map.insert("Pacific/Tongatapu", CountryCode::TON); // Tonga
    iana_map.insert("Pacific/Wake", CountryCode::UMI); // United States Minor Outlying Islands
    iana_map.insert("Pacific/Wallis", CountryCode::WLF); // Wallis and Futuna

    iana_map
}
