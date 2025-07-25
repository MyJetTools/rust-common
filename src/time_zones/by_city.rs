lazy_static::lazy_static! {
    pub static ref COUNTRIES_ISO_3_CODES: Vec<(&'static str, &'static str, &'static str)> = {
        get_timezones_with_utc_offset_and_country()
    };
}

fn get_timezones_with_utc_offset_and_country() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Africa/Abidjan", "UTC+00:00", "CI"),       // Côte d'Ivoire
        ("Africa/Accra", "UTC+00:00", "GH"),         // Ghana
        ("Africa/Addis_Ababa", "UTC+03:00", "ET"),   // Ethiopia
        ("Africa/Algiers", "UTC+01:00", "DZ"),       // Algeria
        ("Africa/Asmara", "UTC+03:00", "ER"),        // Eritrea
        ("Africa/Asmera", "UTC+03:00", "ER"),        // Alias for Asmara
        ("Africa/Bamako", "UTC+00:00", "ML"),        // Mali
        ("Africa/Bangui", "UTC+01:00", "CF"),        // Central African Republic
        ("Africa/Banjul", "UTC+00:00", "GM"),        // Gambia
        ("Africa/Bissau", "UTC+00:00", "GW"),        // Guinea-Bissau
        ("Africa/Blantyre", "UTC+02:00", "MW"),      // Malawi
        ("Africa/Brazzaville", "UTC+01:00", "CG"),   // Republic of the Congo
        ("Africa/Bujumbura", "UTC+02:00", "BI"),     // Burundi
        ("Africa/Cairo", "UTC+03:00", "EG"),         // Egypt (DST)
        ("Africa/Casablanca", "UTC+01:00", "MA"),    // Morocco (permanent DST)
        ("Africa/Ceuta", "UTC+02:00", "ES"),         // Spain (Ceuta, DST)
        ("Africa/Conakry", "UTC+00:00", "GN"),       // Guinea
        ("Africa/Dakar", "UTC+00:00", "SN"),         // Senegal
        ("Africa/Dar_es_Salaam", "UTC+03:00", "TZ"), // Tanzania
        ("Africa/Djibouti", "UTC+03:00", "DJ"),      // Djibouti
        ("Africa/Douala", "UTC+01:00", "CM"),        // Cameroon
        ("Africa/El_Aaiun", "UTC+01:00", "EH"),      // Western Sahara (permanent DST)
        ("Africa/Freetown", "UTC+00:00", "SL"),      // Sierra Leone
        ("Africa/Gaborone", "UTC+02:00", "BW"),      // Botswana
        ("Africa/Harare", "UTC+02:00", "ZW"),        // Zimbabwe
        ("Africa/Johannesburg", "UTC+02:00", "ZA"),  // South Africa
        ("Africa/Juba", "UTC+02:00", "SS"),          // South Sudan
        ("Africa/Kampala", "UTC+03:00", "UG"),       // Uganda
        ("Africa/Khartoum", "UTC+02:00", "SD"),      // Sudan
        ("Africa/Kigali", "UTC+02:00", "RW"),        // Rwanda
        ("Africa/Kinshasa", "UTC+01:00", "CD"),      // Democratic Republic of the Congo (west)
        ("Africa/Lagos", "UTC+01:00", "NG"),         // Nigeria
        ("Africa/Libreville", "UTC+01:00", "GA"),    // Gabon
        ("Africa/Lome", "UTC+00:00", "TG"),          // Togo
        ("Africa/Luanda", "UTC+01:00", "AO"),        // Angola
        ("Africa/Lubumbashi", "UTC+02:00", "CD"),    // Democratic Republic of the Congo (east)
        ("Africa/Lusaka", "UTC+02:00", "ZM"),        // Zambia
        ("Africa/Malabo", "UTC+01:00", "GQ"),        // Equatorial Guinea
        ("Africa/Maputo", "UTC+02:00", "MZ"),        // Mozambique
        ("Africa/Maseru", "UTC+02:00", "LS"),        // Lesotho
        ("Africa/Mbabane", "UTC+02:00", "SZ"),       // Eswatini
        ("Africa/Mogadishu", "UTC+03:00", "SO"),     // Somalia
        ("Africa/Monrovia", "UTC+00:00", "LR"),      // Liberia
        ("Africa/Nairobi", "UTC+03:00", "KE"),       // Kenya
        ("Africa/Ndjamena", "UTC+01:00", "TD"),      // Chad
        ("Africa/Niamey", "UTC+01:00", "NE"),        // Niger
        ("Africa/Nouakchott", "UTC+00:00", "MR"),    // Mauritania
        ("Africa/Ouagadougou", "UTC+00:00", "BF"),   // Burkina Faso
        ("Africa/Porto-Novo", "UTC+01:00", "BJ"),    // Benin
        ("Africa/Sao_Tome", "UTC+00:00", "ST"),      // São Tomé and Príncipe
        ("Africa/Timbuktu", "UTC+00:00", "ML"),      // Alias for Bamako
        ("Africa/Tripoli", "UTC+02:00", "LY"),       // Libya
        ("Africa/Tunis", "UTC+01:00", "TN"),         // Tunisia
        ("Africa/Windhoek", "UTC+02:00", "NA"),      // Namibia
        ("America/Adak", "UTC-09:00", "US"),         // United States (Alaska, DST)
        ("America/Anchorage", "UTC-08:00", "US"),    // United States (Alaska, DST)
        ("America/Anguilla", "UTC-04:00", "AI"),     // Anguilla
        ("America/Antigua", "UTC-04:00", "AG"),      // Antigua and Barbuda
        ("America/Araguaina", "UTC-03:00", "BR"),    // Brazil
        ("America/Argentina/Buenos_Aires", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Catamarca", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/ComodRivadavia", "UTC-03:00", "AR"), // Alias for Catamarca
        ("America/Argentina/Cordoba", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Jujuy", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/La_Rioja", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Mendoza", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Rio_Gallegos", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Salta", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/San_Juan", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/San_Luis", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Tucuman", "UTC-03:00", "AR"), // Argentina
        ("America/Argentina/Ushuaia", "UTC-03:00", "AR"), // Argentina
        ("America/Aruba", "UTC-04:00", "AW"),        // Aruba
        ("America/Asuncion", "UTC-04:00", "PY"),     // Paraguay (no DST in July)
        ("America/Atikokan", "UTC-05:00", "CA"),     // Canada
        ("America/Atka", "UTC-09:00", "US"),         // Alias for Adak (DST)
        ("America/Bahia", "UTC-03:00", "BR"),        // Brazil
        ("America/Bahia_Banderas", "UTC-06:00", "MX"), // Mexico (no DST in July)
        ("America/Barbados", "UTC-04:00", "BB"),     // Barbados
        ("America/Belem", "UTC-03:00", "BR"),        // Brazil
        ("America/Belize", "UTC-06:00", "BZ"),       // Belize
        ("America/Blanc-Sablon", "UTC-04:00", "CA"), // Canada
        ("America/Boa_Vista", "UTC-04:00", "BR"),    // Brazil
        ("America/Bogota", "UTC-05:00", "CO"),       // Colombia
        ("America/Boise", "UTC-06:00", "US"),        // United States (DST)
        ("America/Buenos_Aires", "UTC-03:00", "AR"), // Alias for Argentina/Buenos_Aires
        ("America/Cambridge_Bay", "UTC-06:00", "CA"), // Canada (DST)
        ("America/Campo_Grande", "UTC-04:00", "BR"), // Brazil
        ("America/Cancun", "UTC-05:00", "MX"),       // Mexico
        ("America/Caracas", "UTC-04:00", "VE"),      // Venezuela
        ("America/Catamarca", "UTC-03:00", "AR"),    // Alias for Argentina/Catamarca
        ("America/Cayenne", "UTC-03:00", "GF"),      // French Guiana
        ("America/Cayman", "UTC-05:00", "KY"),       // Cayman Islands
        ("America/Chicago", "UTC-05:00", "US"),      // United States (DST)
        ("America/Chihuahua", "UTC-06:00", "MX"),    // Mexico
        ("America/Ciudad_Juarez", "UTC-06:00", "MX"), // Mexico (DST)
        ("America/Coral_Harbour", "UTC-05:00", "CA"), // Alias for Atikokan
        ("America/Cordoba", "UTC-03:00", "AR"),      // Alias for Argentina/Cordoba
        ("America/Costa_Rica", "UTC-06:00", "CR"),   // Costa Rica
        ("America/Creston", "UTC-07:00", "CA"),      // Canada
        ("America/Cuiaba", "UTC-04:00", "BR"),       // Brazil
        ("America/Curacao", "UTC-04:00", "CW"),      // Curaçao
        ("America/Danmarkshavn", "UTC+00:00", "GL"), // Greenland
        ("America/Dawson", "UTC-07:00", "CA"),       // Canada
        ("America/Dawson_Creek", "UTC-07:00", "CA"), // Canada
        ("America/Denver", "UTC-06:00", "US"),       // United States (DST)
        ("America/Detroit", "UTC-04:00", "US"),      // United States (DST)
        ("America/Dominica", "UTC-04:00", "DM"),     // Dominica
        ("America/Edmonton", "UTC-06:00", "CA"),     // Canada (DST)
        ("America/Eirunepe", "UTC-05:00", "BR"),     // Brazil
        ("America/El_Salvador", "UTC-06:00", "SV"),  // El Salvador
        ("America/Fort_Nelson", "UTC-07:00", "CA"),  // Canada
        ("America/Fort_Wayne", "UTC-04:00", "US"),   // Alias for America/Indiana/Indianapolis
        ("America/Fortaleza", "UTC-03:00", "BR"),    // Brazil
        ("America/Glace_Bay", "UTC-03:00", "CA"),    // Canada (DST)
        ("America/Godthab", "UTC-02:00", "GL"),      // Greenland (DST)
        ("America/Goose_Bay", "UTC-03:00", "CA"),    // Canada (DST)
        ("America/Grand_Turk", "UTC-04:00", "TC"),   // Turks and Caicos (DST)
        ("America/Grenada", "UTC-04:00", "GD"),      // Grenada
        ("America/Guadeloupe", "UTC-04:00", "GP"),   // Guadeloupe
        ("America/Guatemala", "UTC-06:00", "GT"),    // Guatemala
        ("America/Guayaquil", "UTC-05:00", "EC"),    // Ecuador
        ("America/Guyana", "UTC-04:00", "GY"),       // Guyana
        ("America/Halifax", "UTC-03:00", "CA"),      // Canada (DST)
        ("America/Havana", "UTC-04:00", "CU"),       // Cuba (DST)
        ("America/Hermosillo", "UTC-07:00", "MX"),   // Mexico
        ("America/Indiana/Indianapolis", "UTC-04:00", "US"), // United States (DST)
        ("America/Indiana/Knox", "UTC-05:00", "US"), // United States (DST)
        ("America/Indiana/Marengo", "UTC-04:00", "US"), // United States (DST)
        ("America/Indiana/Petersburg", "UTC-04:00", "US"), // United States (DST)
        ("America/Indiana/Tell_City", "UTC-05:00", "US"), // United States (DST)
        ("America/Indiana/Vevay", "UTC-04:00", "US"), // United States (DST)
        ("America/Indiana/Vincennes", "UTC-04:00", "US"), // United States (DST)
        ("America/Indiana/Winamac", "UTC-04:00", "US"), // United States (DST)
        ("America/Indianapolis", "UTC-04:00", "US"), // Alias for America/Indiana/Indianapolis
        ("America/Inuvik", "UTC-06:00", "CA"),       // Canada (DST)
        ("America/Iqaluit", "UTC-04:00", "CA"),      // Canada (DST)
        ("America/Jamaica", "UTC-05:00", "JM"),      // Jamaica
        ("America/Juneau", "UTC-08:00", "US"),       // United States (DST)
        ("America/Kentucky/Louisville", "UTC-04:00", "US"), // United States (DST)
        ("America/Kentucky/Monticello", "UTC-04:00", "US"), // United States (DST)
        ("America/Kralendijk", "UTC-04:00", "BQ"),   // Bonaire
        ("America/La_Paz", "UTC-04:00", "BO"),       // Bolivia
        ("America/Lima", "UTC-05:00", "PE"),         // Peru
        ("America/Los_Angeles", "UTC-07:00", "US"),  // United States (DST)
        ("America/Louisville", "UTC-04:00", "US"),   // Alias for America/Kentucky/Louisville
        ("America/Lower_Princes", "UTC-04:00", "SX"), // Sint Maarten
        ("America/Maceio", "UTC-03:00", "BR"),       // Brazil
        ("America/Managua", "UTC-06:00", "NI"),      // Nicaragua
        ("America/Manaus", "UTC-04:00", "BR"),       // Brazil
        ("America/Marigot", "UTC-04:00", "MF"),      // Saint Martin
        ("America/Martinique", "UTC-04:00", "MQ"),   // Martinique
        ("America/Matamoros", "UTC-05:00", "MX"),    // Mexico (DST)
        ("America/Mazatlan", "UTC-07:00", "MX"),     // Mexico
        ("America/Menominee", "UTC-05:00", "US"),    // United States (DST)
        ("America/Merida", "UTC-06:00", "MX"),       // Mexico
        ("America/Metlakatla", "UTC-08:00", "US"),   // United States (DST)
        ("America/Mexico_City", "UTC-06:00", "MX"),  // Mexico
        ("America/Miquelon", "UTC-02:00", "PM"),     // Saint Pierre and Miquelon (DST)
        ("America/Moncton", "UTC-03:00", "CA"),      // Canada (DST)
        ("America/Monterrey", "UTC-06:00", "MX"),    // Mexico
        ("America/Montevideo", "UTC-03:00", "UY"),   // Uruguay
        ("America/Montreal", "UTC-04:00", "CA"),     // Canada (DST)
        ("America/Montserrat", "UTC-04:00", "MS"),   // Montserrat
        ("America/Nassau", "UTC-04:00", "BS"),       // Bahamas (DST)
        ("America/New_York", "UTC-04:00", "US"),     // United States (DST)
        ("America/Nipigon", "UTC-04:00", "CA"),      // Canada (DST)
        ("America/Nome", "UTC-08:00", "US"),         // United States (DST)
        ("America/Noronha", "UTC-02:00", "BR"),      // Brazil
        ("America/North_Dakota/Beulah", "UTC-05:00", "US"), // United States (DST)
        ("America/North_Dakota/Center", "UTC-05:00", "US"), // United States (DST)
        ("America/North_Dakota/New_Salem", "UTC-05:00", "US"), // United States (DST)
        ("America/Nuuk", "UTC-02:00", "GL"),         // Greenland (DST)
        ("America/Ojinaga", "UTC-05:00", "MX"),      // Mexico (DST)
        ("America/Panama", "UTC-05:00", "PA"),       // Panama
        ("America/Pangnirtung", "UTC-04:00", "CA"),  // Canada (DST)
        ("America/Paramaribo", "UTC-03:00", "SR"),   // Suriname
        ("America/Phoenix", "UTC-07:00", "US"),      // United States (no DST)
        ("America/Port_of_Spain", "UTC-04:00", "TT"), // Trinidad and Tobago
        ("America/Port-au-Prince", "UTC-04:00", "HT"), // Haiti (DST)
        ("America/Porto_Acre", "UTC-05:00", "BR"),   // Brazil
        ("America/Porto_Velho", "UTC-04:00", "BR"),  // Brazil
        ("America/Puerto_Rico", "UTC-04:00", "PR"),  // Puerto Rico
        ("America/Punta_Arenas", "UTC-03:00", "CL"), // Chile
        ("America/Rainy_River", "UTC-05:00", "CA"),  // Canada (DST)
        ("America/Rankin_Inlet", "UTC-05:00", "CA"), // Canada (DST)
        ("America/Recife", "UTC-03:00", "BR"),       // Brazil
        ("America/Regina", "UTC-06:00", "CA"),       // Canada (no DST)
        ("America/Resolute", "UTC-05:00", "CA"),     // Canada (DST)
        ("America/Rio_Branco", "UTC-05:00", "BR"),   // Brazil
        ("America/Rosario", "UTC-03:00", "AR"),      // Alias for Argentina/Cordoba
        ("America/Santa_Isabel", "UTC-07:00", "MX"), // Mexico
        ("America/Santarem", "UTC-03:00", "BR"),     // Brazil
        ("America/Santiago", "UTC-04:00", "CL"),     // Chile (no DST in July)
        ("America/Santo_Domingo", "UTC-04:00", "DO"), // Dominican Republic
        ("America/Sao_Paulo", "UTC-03:00", "BR"),    // Brazil
        ("America/Scoresbysund", "UTC+00:00", "GL"), // Greenland (DST)
        ("America/Shiprock", "UTC-06:00", "US"),     // Alias for America/Denver
        ("America/Sitka", "UTC-08:00", "US"),        // United States (DST)
        ("America/St_Barthelemy", "UTC-04:00", "BL"), // Saint Barthélemy
        ("America/St_Johns", "UTC-02:30", "CA"),     // Canada (DST)
        ("America/St_Kitts", "UTC-04:00", "KN"),     // Saint Kitts and Nevis
        ("America/St_Lucia", "UTC-04:00", "LC"),     // Saint Lucia
        ("America/St_Thomas", "UTC-04:00", "VI"),    // U.S. Virgin Islands
        ("America/St_Vincent", "UTC-04:00", "VC"),   // Saint Vincent and the Grenadines
        ("America/Swift_Current", "UTC-06:00", "CA"), // Canada (no DST)
        ("America/Tegucigalpa", "UTC-06:00", "HN"),  // Honduras
        ("America/Thule", "UTC-03:00", "GL"),        // Greenland (DST)
        ("America/Thunder_Bay", "UTC-04:00", "CA"),  // Canada (DST)
        ("America/Tijuana", "UTC-07:00", "MX"),      // Mexico (DST)
        ("America/Toronto", "UTC-04:00", "CA"),      // Canada (DST)
        ("America/Tortola", "UTC-04:00", "VG"),      // British Virgin Islands
        ("America/Vancouver", "UTC-07:00", "CA"),    // Canada (DST)
        ("America/Virgin", "UTC-04:00", "VI"),       // Alias for America/St_Thomas
        ("America/Whitehorse", "UTC-07:00", "CA"),   // Canada
        ("America/Winnipeg", "UTC-05:00", "CA"),     // Canada (DST)
        ("America/Yakutat", "UTC-08:00", "US"),      // United States (DST)
        ("America/Yellowknife", "UTC-06:00", "CA"),  // Canada (DST)
        ("Antarctica/Casey", "UTC+11:00", "AQ"),     // Antarctica
        ("Antarctica/Davis", "UTC+07:00", "AQ"),     // Antarctica
        ("Antarctica/DumontDUrville", "UTC+10:00", "AQ"), // Antarctica
        ("Antarctica/Mawson", "UTC+05:00", "AQ"),    // Antarctica
        ("Antarctica/McMurdo", "UTC+12:00", "AQ"),   // Antarctica (NZ time, DST)
        ("Antarctica/Palmer", "UTC-03:00", "AQ"),    // Antarctica
        ("Antarctica/Rothera", "UTC-03:00", "AQ"),   // Antarctica
        ("Antarctica/South_Pole", "UTC+12:00", "AQ"), // Alias for McMurdo
        ("Antarctica/Syowa", "UTC+03:00", "AQ"),     // Antarctica
        ("Antarctica/Troll", "UTC+02:00", "AQ"),     // Antarctica (DST)
        ("Antarctica/Vostok", "UTC+06:00", "AQ"),    // Antarctica
        ("Arctic/Longyearbyen", "UTC+02:00", "SJ"),  // Svalbard and Jan Mayen (DST)
        ("Asia/Aden", "UTC+03:00", "YE"),            // Yemen
        ("Asia/Almaty", "UTC+05:00", "KZ"),          // Kazakhstan
        ("Asia/Amman", "UTC+03:00", "JO"),           // Jordan (DST)
        ("Asia/Anadyr", "UTC+12:00", "RU"),          // Russia
        ("Asia/Aqtau", "UTC+05:00", "KZ"),           // Kazakhstan
        ("Asia/Aqtobe", "UTC+05:00", "KZ"),          // Kazakhstan
        ("Asia/Ashgabat", "UTC+05:00", "TM"),        // Turkmenistan
        ("Asia/Ashkhabad", "UTC+05:00", "TM"),       // Alias for Ashgabat
        ("Asia/Atyrau", "UTC+05:00", "KZ"),          // Kazakhstan
        ("Asia/Baghdad", "UTC+03:00", "IQ"),         // Iraq
        ("Asia/Bahrain", "UTC+03:00", "BH"),         // Bahrain
        ("Asia/Baku", "UTC+04:00", "AZ"),            // Azerbaijan
        ("Asia/Bangkok", "UTC+07:00", "TH"),         // Thailand
        ("Asia/Barnaul", "UTC+07:00", "RU"),         // Russia
        ("Asia/Beirut", "UTC+03:00", "LB"),          // Lebanon (DST)
        ("Asia/Bishkek", "UTC+06:00", "KG"),         // Kyrgyzstan
        ("Asia/Brunei", "UTC+08:00", "BN"),          // Brunei
        ("Asia/Calcutta", "UTC+05:30", "IN"),        // India
        ("Asia/Chita", "UTC+09:00", "RU"),           // Russia
        ("Asia/Choibalsan", "UTC+08:00", "MN"),      // Mongolia
        ("Asia/Chongqing", "UTC+08:00", "CN"),       // China
        ("Asia/Chungking", "UTC+08:00", "CN"),       // Alias for Chongqing
        ("Asia/Colombo", "UTC+05:30", "LK"),         // Sri Lanka
        ("Asia/Dacca", "UTC+06:00", "BD"),           // Alias for Dhaka
        ("Asia/Damascus", "UTC+03:00", "SY"),        // Syria
        ("Asia/Dhaka", "UTC+06:00", "BD"),           // Bangladesh
        ("Asia/Dili", "UTC+09:00", "TL"),            // Timor-Leste
        ("Asia/Dubai", "UTC+04:00", "AE"),           // United Arab Emirates
        ("Asia/Dushanbe", "UTC+05:00", "TJ"),        // Tajikistan
        ("Asia/Famagusta", "UTC+03:00", "CY"),       // Cyprus (DST)
        ("Asia/Gaza", "UTC+03:00", "PS"),            // Palestine (DST)
        ("Asia/Harbin", "UTC+08:00", "CN"),          // China
        ("Asia/Hebron", "UTC+03:00", "PS"),          // Palestine (DST)
        ("Asia/Ho_Chi_Minh", "UTC+07:00", "VN"),     // Vietnam
        ("Asia/Hong_Kong", "UTC+08:00", "HK"),       // Hong Kong
        ("Asia/Hovd", "UTC+07:00", "MN"),            // Mongolia
        ("Asia/Irkutsk", "UTC+08:00", "RU"),         // Russia
        ("Asia/Istanbul", "UTC+03:00", "TR"),        // Turkey
        ("Asia/Jakarta", "UTC+07:00", "ID"),         // Indonesia
        ("Asia/Jayapura", "UTC+09:00", "ID"),        // Indonesia
        ("Asia/Jerusalem", "UTC+03:00", "IL"),       // Israel (DST)
        ("Asia/Kabul", "UTC+04:30", "AF"),           // Afghanistan
        ("Asia/Kamchatka", "UTC+12:00", "RU"),       // Russia
        ("Asia/Karachi", "UTC+05:00", "PK"),         // Pakistan
        ("Asia/Kashgar", "UTC+06:00", "CN"),         // China
        ("Asia/Kathmandu", "UTC+05:45", "NP"),       // Nepal
        ("Asia/Katmandu", "UTC+05:45", "NP"),        // Alias for Kathmandu
        ("Asia/Khandyga", "UTC+09:00", "RU"),        // Russia
        ("Asia/Kolkata", "UTC+05:30", "IN"),         // India
        ("Asia/Krasnoyarsk", "UTC+07:00", "RU"),     // Russia
        ("Asia/Kuala_Lumpur", "UTC+08:00", "MY"),    // Malaysia
        ("Asia/Kuching", "UTC+08:00", "MY"),         // Malaysia
        ("Asia/Kuwait", "UTC+03:00", "KW"),          // Kuwait
        ("Asia/Macao", "UTC+08:00", "MO"),           // Macau
        ("Asia/Macau", "UTC+08:00", "MO"),           // Alias for Macao
        ("Asia/Magadan", "UTC+11:00", "RU"),         // Russia
        ("Asia/Makassar", "UTC+08:00", "ID"),        // Indonesia
        ("Asia/Manila", "UTC+08:00", "PH"),          // Philippines
        ("Asia/Muscat", "UTC+04:00", "OM"),          // Oman
        ("Asia/Nicosia", "UTC+03:00", "CY"),         // Cyprus (DST)
        ("Asia/Novokuznetsk", "UTC+07:00", "RU"),    // Russia
        ("Asia/Novosibirsk", "UTC+07:00", "RU"),     // Russia
        ("Asia/Omsk", "UTC+06:00", "RU"),            // Russia
        ("Asia/Oral", "UTC+05:00", "KZ"),            // Kazakhstan
        ("Asia/Phnom_Penh", "UTC+07:00", "KH"),      // Cambodia
        ("Asia/Pontianak", "UTC+07:00", "ID"),       // Indonesia
        ("Asia/Pyongyang", "UTC+09:00", "KP"),       // North Korea
        ("Asia/Qatar", "UTC+03:00", "QA"),           // Qatar
        ("Asia/Qostanay", "UTC+05:00", "KZ"),        // Kazakhstan
        ("Asia/Qyzylorda", "UTC+05:00", "KZ"),       // Kazakhstan
        ("Asia/Rangoon", "UTC+06:30", "MM"),         // Myanmar
        ("Asia/Riyadh", "UTC+03:00", "SA"),          // Saudi Arabia
        ("Asia/Saigon", "UTC+07:00", "VN"),          // Alias for Ho_Chi_Minh
        ("Asia/Sakhalin", "UTC+11:00", "RU"),        // Russia
        ("Asia/Samarkand", "UTC+05:00", "UZ"),       // Uzbekistan
        ("Asia/Seoul", "UTC+09:00", "KR"),           // South Korea
        ("Asia/Shanghai", "UTC+08:00", "CN"),        // China
        ("Asia/Singapore", "UTC+08:00", "SG"),       // Singapore
        ("Asia/Srednekolymsk", "UTC+11:00", "RU"),   // Russia
        ("Asia/Taipei", "UTC+08:00", "TW"),          // Taiwan
        ("Asia/Tashkent", "UTC+05:00", "UZ"),        // Uzbekistan
        ("Asia/Tbilisi", "UTC+04:00", "GE"),         // Georgia
        ("Asia/Tehran", "UTC+03:30", "IR"),          // Iran
        ("Asia/Tel_Aviv", "UTC+03:00", "IL"),        // Alias for Jerusalem
        ("Asia/Thimbu", "UTC+06:00", "BT"),          // Bhutan
        ("Asia/Thimphu", "UTC+06:00", "BT"),         // Alias for Thimbu
        ("Asia/Tokyo", "UTC+09:00", "JP"),           // Japan
        ("Asia/Tomsk", "UTC+07:00", "RU"),           // Russia
        ("Asia/Ujung_Pandang", "UTC+08:00", "ID"),   // Alias for Makassar
        ("Asia/Ulaanbaatar", "UTC+08:00", "MN"),     // Mongolia
        ("Asia/Ulan_Bator", "UTC+08:00", "MN"),      // Alias for Ulaanbaatar
        ("Asia/Urumqi", "UTC+06:00", "CN"),          // China
        ("Asia/Ust-Nera", "UTC+10:00", "RU"),        // Russia
        ("Asia/Vientiane", "UTC+07:00", "LA"),       // Laos
        ("Asia/Vladivostok", "UTC+10:00", "RU"),     // Russia
        ("Asia/Yakutsk", "UTC+09:00", "RU"),         // Russia
        ("Asia/Yangon", "UTC+06:30", "MM"),          // Alias for Rangoon
        ("Asia/Yekaterinburg", "UTC+05:00", "RU"),   // Russia
        ("Asia/Yerevan", "UTC+04:00", "AM"),         // Armenia
        ("Atlantic/Azores", "UTC+00:00", "PT"),      // Portugal (DST)
        ("Atlantic/Bermuda", "UTC-03:00", "BM"),     // Bermuda (DST)
        ("Atlantic/Canary", "UTC+01:00", "ES"),      // Spain (DST)
        ("Atlantic/Cape_Verde", "UTC-01:00", "CV"),  // Cape Verde
        ("Atlantic/Faeroe", "UTC+01:00", "FO"),      // Faroe Islands (DST)
        ("Atlantic/Faroe", "UTC+01:00", "FO"),       // Alias for Faeroe
        ("Atlantic/Jan_Mayen", "UTC+02:00", "SJ"),   // Alias for Arctic/Longyearbyen
        ("Atlantic/Madeira", "UTC+01:00", "PT"),     // Portugal (DST)
        ("Atlantic/Reykjavik", "UTC+00:00", "IS"),   // Iceland
        ("Atlantic/South_Georgia", "UTC-02:00", "GS"), // South Georgia
        ("Atlantic/St_Helena", "UTC+00:00", "SH"),   // Saint Helena
        ("Atlantic/Stanley", "UTC-03:00", "FK"),     // Falkland Islands
        ("Australia/ACT", "UTC+10:00", "AU"),        // Alias for Australia/Sydney
        ("Australia/Adelaide", "UTC+09:30", "AU"),   // Australia
        ("Australia/Brisbane", "UTC+10:00", "AU"),   // Australia
        ("Australia/Broken_Hill", "UTC+09:30", "AU"), // Australia
        ("Australia/Canberra", "UTC+10:00", "AU"),   // Alias for Australia/Sydney
        ("Australia/Currie", "UTC+10:00", "AU"),     // Australia
        ("Australia/Darwin", "UTC+09:30", "AU"),     // Australia (no DST)
        ("Australia/Eucla", "UTC+08:45", "AU"),      // Australia
        ("Australia/Hobart", "UTC+10:00", "AU"),     // Australia
        ("Australia/Lindeman", "UTC+10:00", "AU"),   // Australia
        ("Australia/Lord_Howe", "UTC+10:30", "AU"),  // Australia
        ("Australia/Melbourne", "UTC+10:00", "AU"),  // Australia
        ("Australia/North", "UTC+09:30", "AU"),      // Alias for Australia/Darwin
        ("Australia/Perth", "UTC+08:00", "AU"),      // Australia (no DST)
        ("Australia/Queensland", "UTC+10:00", "AU"), // Alias for Australia/Brisbane
        ("Australia/South", "UTC+09:30", "AU"),      // Alias for Australia/Adelaide
        ("Australia/Sydney", "UTC+10:00", "AU"),     // Australia
        ("Australia/Tasmania", "UTC+10:00", "AU"),   // Alias for Australia/Hobart
        ("Australia/Victoria", "UTC+10:00", "AU"),   // Alias for Australia/Melbourne
        ("Australia/West", "UTC+08:00", "AU"),       // Alias for Australia/Perth
        ("Australia/Yancowinna", "UTC+09:30", "AU"), // Alias for Australia/Broken_Hill
        ("Europe/Amsterdam", "UTC+02:00", "NL"),     // Netherlands (DST)
        ("Europe/Andorra", "UTC+02:00", "AD"),       // Andorra (DST)
        ("Europe/Astrakhan", "UTC+04:00", "RU"),     // Russia
        ("Europe/Athens", "UTC+03:00", "GR"),        // Greece (DST)
        ("Europe/Belfast", "UTC+01:00", "GB"),       // United Kingdom (DST)
        ("Europe/Belgrade", "UTC+02:00", "RS"),      // Serbia (DST)
        ("Europe/Berlin", "UTC+02:00", "DE"),        // Germany (DST)
        ("Europe/Bratislava", "UTC+02:00", "SK"),    // Slovakia (DST)
        ("Europe/Brussels", "UTC+02:00", "BE"),      // Belgium (DST)
        ("Europe/Bucharest", "UTC+03:00", "RO"),     // Romania (DST)
        ("Europe/Budapest", "UTC+02:00", "HU"),      // Hungary (DST)
        ("Europe/Busingen", "UTC+02:00", "DE"),      // Germany (DST)
        ("Europe/Chisinau", "UTC+03:00", "MD"),      // Moldova (DST)
        ("Europe/Copenhagen", "UTC+02:00", "DK"),    // Denmark (DST)
        ("Europe/Dublin", "UTC+01:00", "IE"),        // Ireland (DST)
        ("Europe/Gibraltar", "UTC+02:00", "GI"),     // Gibraltar (DST)
        ("Europe/Guernsey", "UTC+01:00", "GG"),      // Guernsey (DST)
        ("Europe/Helsinki", "UTC+03:00", "FI"),      // Finland (DST)
        ("Europe/Isle_of_Man", "UTC+01:00", "IM"),   // Isle of Man (DST)
        ("Europe/Istanbul", "UTC+03:00", "TR"),      // Turkey
        ("Europe/Jersey", "UTC+01:00", "JE"),        // Jersey (DST)
        ("Europe/Kaliningrad", "UTC+02:00", "RU"),   // Russia
        ("Europe/Kiev", "UTC+03:00", "UA"),          // Ukraine (DST)
        ("Europe/Kirov", "UTC+03:00", "RU"),         // Russia
        ("Europe/Lisbon", "UTC+01:00", "PT"),        // Portugal (DST)
        ("Europe/Ljubljana", "UTC+02:00", "SI"),     // Slovenia (DST)
        ("Europe/London", "UTC+01:00", "GB"),        // United Kingdom (DST)
        ("Europe/Luxembourg", "UTC+02:00", "LU"),    // Luxembourg (DST)
        ("Europe/Madrid", "UTC+02:00", "ES"),        // Spain (DST)
        ("Europe/Malta", "UTC+02:00", "MT"),         // Malta (DST)
        ("Europe/Mariehamn", "UTC+03:00", "AX"),     // Åland Islands (DST)
        ("Europe/Minsk", "UTC+03:00", "BY"),         // Belarus
        ("Europe/Monaco", "UTC+02:00", "MC"),        // Monaco (DST)
        ("Europe/Moscow", "UTC+03:00", "RU"),        // Russia
        ("Europe/Nicosia", "UTC+03:00", "CY"),       // Alias for Asia/Nicosia
        ("Europe/Oslo", "UTC+02:00", "NO"),          // Norway (DST)
        ("Europe/Paris", "UTC+02:00", "FR"),         // France (DST)
        ("Europe/Podgorica", "UTC+02:00", "ME"),     // Montenegro (DST)
        ("Europe/Prague", "UTC+02:00", "CZ"),        // Czech Republic (DST)
        ("Europe/Riga", "UTC+03:00", "LV"),          // Latvia (DST)
        ("Europe/Rome", "UTC+02:00", "IT"),          // Italy (DST)
        ("Europe/Samara", "UTC+04:00", "RU"),        // Russia
        ("Europe/San_Marino", "UTC+02:00", "SM"),    // San Marino (DST)
        ("Europe/Sarajevo", "UTC+02:00", "BA"),      // Bosnia and Herzegovina (DST)
        ("Europe/Saratov", "UTC+04:00", "RU"),       // Russia
        ("Europe/Simferopol", "UTC+03:00", "UA"),    // Ukraine (disputed, used in Crimea)
        ("Europe/Skopje", "UTC+02:00", "MK"),        // North Macedonia (DST)
        ("Europe/Sofia", "UTC+03:00", "BG"),         // Bulgaria (DST)
        ("Europe/Stockholm", "UTC+02:00", "SE"),     // Sweden (DST)
        ("Europe/Tallinn", "UTC+03:00", "EE"),       // Estonia (DST)
        ("Europe/Tirane", "UTC+02:00", "AL"),        // Albania (DST)
        ("Europe/Tiraspol", "UTC+03:00", "MD"),      // Alias for Chisinau
        ("Europe/Ulyanovsk", "UTC+04:00", "RU"),     // Russia
        ("Europe/Uzhgorod", "UTC+03:00", "UA"),      // Ukraine (DST)
        ("Europe/Vaduz", "UTC+02:00", "LI"),         // Liechtenstein (DST)
        ("Europe/Vatican", "UTC+02:00", "VA"),       // Vatican City (DST)
        ("Europe/Vienna", "UTC+02:00", "AT"),        // Austria (DST)
        ("Europe/Vilnius", "UTC+03:00", "LT"),       // Lithuania (DST)
        ("Europe/Volgograd", "UTC+03:00", "RU"),     // Russia
        ("Europe/Warsaw", "UTC+02:00", "PL"),        // Poland (DST)
        ("Europe/Zagreb", "UTC+02:00", "HR"),        // Croatia (DST)
        ("Europe/Zaporozhye", "UTC+03:00", "UA"),    // Ukraine (DST)
        ("Europe/Zurich", "UTC+02:00", "CH"),        // Switzerland (DST)
        ("Indian/Antananarivo", "UTC+03:00", "MG"),  // Madagascar
        ("Indian/Chagos", "UTC+06:00", "IO"),        // British Indian Ocean Territory
        ("Indian/Christmas", "UTC+07:00", "CX"),     // Christmas Island
        ("Indian/Cocos", "UTC+06:30", "CC"),         // Cocos Islands
        ("Indian/Comoro", "UTC+03:00", "KM"),        // Comoros
        ("Indian/Kerguelen", "UTC+05:00", "TF"),     // French Southern Territories
        ("Indian/Mahe", "UTC+04:00", "SC"),          // Seychelles
        ("Indian/Maldives", "UTC+05:00", "MV"),      // Maldives
        ("Indian/Mauritius", "UTC+04:00", "MU"),     // Mauritius
        ("Indian/Mayotte", "UTC+03:00", "YT"),       // Mayotte
        ("Indian/Reunion", "UTC+04:00", "RE"),       // Réunion
        ("Pacific/Apia", "UTC+13:00", "WS"),         // Samoa
        ("Pacific/Auckland", "UTC+12:00", "NZ"),     // New Zealand
        ("Pacific/Bougainville", "UTC+11:00", "PG"), // Papua New Guinea
        ("Pacific/Chatham", "UTC+12:45", "NZ"),      // New Zealand
        ("Pacific/Chuuk", "UTC+10:00", "FM"),        // Micronesia
        ("Pacific/Easter", "UTC-06:00", "CL"),       // Chile
        ("Pacific/Efate", "UTC+11:00", "VU"),        // Vanuatu
        ("Pacific/Enderbury", "UTC+13:00", "KI"),    // Kiribati
        ("Pacific/Fakaofo", "UTC+13:00", "TK"),      // Tokelau
        ("Pacific/Fiji", "UTC+12:00", "FJ"),         // Fiji
        ("Pacific/Funafuti", "UTC+12:00", "TV"),     // Tuvalu
        ("Pacific/Galapagos", "UTC-06:00", "EC"),    // Ecuador
        ("Pacific/Gambier", "UTC-09:00", "PF"),      // French Polynesia
        ("Pacific/Guadalcanal", "UTC+11:00", "SB"),  // Solomon Islands
        ("Pacific/Guam", "UTC+10:00", "GU"),         // Guam
        ("Pacific/Honolulu", "UTC-10:00", "US"),     // United States (no DST)
        ("Pacific/Johnston", "UTC-10:00", "UM"),     // United States Minor Outlying Islands
        ("Pacific/Kiritimati", "UTC+14:00", "KI"),   // Kiribati
        ("Pacific/Kosrae", "UTC+11:00", "FM"),       // Micronesia
        ("Pacific/Kwajalein", "UTC+12:00", "MH"),    // Marshall Islands
        ("Pacific/Majuro", "UTC+12:00", "MH"),       // Marshall Islands
        ("Pacific/Marquesas", "UTC-09:30", "PF"),    // French Polynesia
        ("Pacific/Midway", "UTC-11:00", "UM"),       // United States Minor Outlying Islands
        ("Pacific/Nauru", "UTC+12:00", "NR"),        // Nauru
        ("Pacific/Niue", "UTC-11:00", "NU"),         // Niue
        ("Pacific/Norfolk", "UTC+11:00", "NF"),      // Norfolk Island
        ("Pacific/Noumea", "UTC+11:00", "NC"),       // New Caledonia
        ("Pacific/Pago_Pago", "UTC-11:00", "AS"),    // American Samoa
        ("Pacific/Palau", "UTC+09:00", "PW"),        // Palau
        ("Pacific/Pitcairn", "UTC-08:00", "PN"),     // Pitcairn Islands
        ("Pacific/Pohnpei", "UTC+11:00", "FM"),      // Micronesia
        ("Pacific/Ponape", "UTC+11:00", "FM"),       // Alias for Pohnpei
        ("Pacific/Port_Moresby", "UTC+10:00", "PG"), // Papua New Guinea
        ("Pacific/Rarotonga", "UTC-10:00", "CK"),    // Cook Islands
        ("Pacific/Saipan", "UTC+10:00", "MP"),       // Northern Mariana Islands
        ("Pacific/Samoa", "UTC-11:00", "AS"),        // Alias for Pago_Pago
        ("Pacific/Tahiti", "UTC-10:00", "PF"),       // French Polynesia
        ("Pacific/Tarawa", "UTC+12:00", "KI"),       // Kiribati
        ("Pacific/Tongatapu", "UTC+13:00", "TO"),    // Tonga
        ("Pacific/Truk", "UTC+10:00", "FM"),         // Alias for Chuuk
        ("Pacific/Wake", "UTC+12:00", "UM"),         // United States Minor Outlying Islands
        ("Pacific/Wallis", "UTC+12:00", "WF"),       // Wallis and Futuna
        ("Pacific/Yap", "UTC+10:00", "FM"),          // Alias for Chuuk
    ]
}
