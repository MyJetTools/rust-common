use std::collections::HashMap;

pub fn get_city_name_map() -> HashMap<&'static str, &'static str> {
    let mut city_map: HashMap<&'static str, &'static str> = HashMap::new();

    // New York City, USA
    city_map.insert("nyc", "new york city");
    city_map.insert("new york", "new york city");
    city_map.insert("the big apple", "new york city");
    city_map.insert("new york city", "new york city");
    city_map.insert("ny", "new york city");

    // London, UK
    city_map.insert("london", "london");
    city_map.insert("the smoke", "london");
    city_map.insert("ldn", "london");
    city_map.insert("big smoke", "london");

    // Tokyo, Japan
    city_map.insert("tokyo", "tokyo");
    city_map.insert("edo", "tokyo");
    city_map.insert("tky", "tokyo");

    // Paris, France
    city_map.insert("paris", "paris");
    city_map.insert("city of light", "paris");
    city_map.insert("paname", "paris");

    // Sydney, Australia
    city_map.insert("sydney", "sydney");
    city_map.insert("syd", "sydney");
    city_map.insert("harbour city", "sydney");

    // Dubai, UAE
    city_map.insert("dubai", "dubai");
    city_map.insert("dxb", "dubai");
    city_map.insert("city of gold", "dubai");

    // Singapore
    city_map.insert("singapore", "singapore");
    city_map.insert("sg", "singapore");
    city_map.insert("lion city", "singapore");

    // Barcelona, Spain
    city_map.insert("barcelona", "barcelona");
    city_map.insert("bcn", "barcelona");
    city_map.insert("barca", "barcelona");

    // Cape Town, South Africa
    city_map.insert("cape town", "cape town");
    city_map.insert("mother city", "cape town");
    city_map.insert("cpt", "cape town");

    // Vancouver, Canada
    city_map.insert("vancouver", "vancouver");
    city_map.insert("vancity", "vancouver");
    city_map.insert("van", "vancouver");

    // Amsterdam, Netherlands
    city_map.insert("amsterdam", "amsterdam");
    city_map.insert("ams", "amsterdam");
    city_map.insert("mokum", "amsterdam");

    // Rio de Janeiro, Brazil
    city_map.insert("rio de janeiro", "rio de janeiro");
    city_map.insert("rio", "rio de janeiro");
    city_map.insert("cidade maravilhosa", "rio de janeiro");

    // Hong Kong
    city_map.insert("hong kong", "hong kong");
    city_map.insert("hk", "hong kong");
    city_map.insert("fragrant harbour", "hong kong");

    // Lisbon, Portugal
    city_map.insert("lisbon", "lisbon");
    city_map.insert("lisboa", "lisbon");
    city_map.insert("city of seven hills", "lisbon");

    // Auckland, New Zealand
    city_map.insert("auckland", "auckland");
    city_map.insert("akl", "auckland");
    city_map.insert("city of sails", "auckland");

    // Doha, Qatar
    city_map.insert("doha", "doha");
    city_map.insert("ad-dawhah", "doha");
    city_map.insert("doh", "doha");

    // Ras Al Khaimah, UAE
    city_map.insert("ras al khaimah", "ras al khaimah");
    city_map.insert("rak", "ras al khaimah");

    // Benahavis, Spain
    city_map.insert("benahavis", "benahavis");
    city_map.insert("benahavís", "benahavis");

    // Ealing, England, UK
    city_map.insert("ealing", "ealing");
    city_map.insert("queen of the suburbs", "ealing");

    // Muscat, Oman
    city_map.insert("muscat", "muscat");
    city_map.insert("masqat", "muscat");
    city_map.insert("mct", "muscat");

    // European Capitals
    // Tirana, Albania
    city_map.insert("tirana", "tirana");
    city_map.insert("tiranë", "tirana");

    // Andorra la Vella, Andorra
    city_map.insert("andorra la vella", "andorra la vella");
    city_map.insert("andorra", "andorra la vella");

    // Yerevan, Armenia
    city_map.insert("yerevan", "yerevan");
    city_map.insert("erevan", "yerevan");

    // Vienna, Austria
    city_map.insert("vienna", "vienna");
    city_map.insert("wien", "vienna");
    city_map.insert("vna", "vienna");

    // Baku, Azerbaijan
    city_map.insert("baku", "baku");
    city_map.insert("baki", "baku");

    // Minsk, Belarus
    city_map.insert("minsk", "minsk");
    city_map.insert("miensk", "minsk");

    // Brussels, Belgium
    city_map.insert("brussels", "brussels");
    city_map.insert("bruxelles", "brussels");
    city_map.insert("brussel", "brussels");

    // Sarajevo, Bosnia and Herzegovina
    city_map.insert("sarajevo", "sarajevo");
    city_map.insert("sara", "sarajevo");

    // Sofia, Bulgaria
    city_map.insert("sofia", "sofia");
    city_map.insert("sofiya", "sofia");

    // Zagreb, Croatia
    city_map.insert("zagreb", "zagreb");
    city_map.insert("zgb", "zagreb");

    // Nicosia, Cyprus
    city_map.insert("nicosia", "nicosia");
    city_map.insert("lefkosia", "nicosia");

    // Prague, Czech Republic
    city_map.insert("prague", "prague");
    city_map.insert("praha", "prague");
    city_map.insert("prg", "prague");

    // Copenhagen, Denmark
    city_map.insert("copenhagen", "copenhagen");
    city_map.insert("københavn", "copenhagen");
    city_map.insert("cph", "copenhagen");

    // Tallinn, Estonia
    city_map.insert("tallinn", "tallinn");
    city_map.insert("tln", "tallinn");
    city_map.insert("medieval gem", "tallinn");

    // Helsinki, Finland
    city_map.insert("helsinki", "helsinki");
    city_map.insert("hki", "helsinki");

    // Tbilisi, Georgia
    city_map.insert("tbilisi", "tbilisi");
    city_map.insert("tbs", "tbilisi");

    // Berlin, Germany
    city_map.insert("berlin", "berlin");
    city_map.insert("ber", "berlin");

    // Athens, Greece
    city_map.insert("athens", "athens");
    city_map.insert("athina", "athens");

    // Budapest, Hungary
    city_map.insert("budapest", "budapest");
    city_map.insert("buda", "budapest");
    city_map.insert("pest", "budapest");

    // Reykjavik, Iceland
    city_map.insert("reykjavik", "reykjavik");
    city_map.insert("rvk", "reykjavik");

    // Dublin, Ireland
    city_map.insert("dublin", "dublin");
    city_map.insert("dub", "dublin");
    city_map.insert("baile átha cliath", "dublin");

    // Rome, Italy
    city_map.insert("rome", "rome");
    city_map.insert("roma", "rome");
    city_map.insert("eternal city", "rome");

    // Astana, Kazakhstan
    city_map.insert("astana", "astana");
    city_map.insert("nur-sultan", "astana");

    // Riga, Latvia
    city_map.insert("riga", "riga");
    city_map.insert("rīga", "riga");

    // Vaduz, Liechtenstein
    city_map.insert("vaduz", "vaduz");

    // Vilnius, Lithuania
    city_map.insert("vilnius", "vilnius");
    city_map.insert("vno", "vilnius");

    // Luxembourg, Luxembourg
    city_map.insert("luxembourg", "luxembourg");
    city_map.insert("lux", "luxembourg");

    // Valletta, Malta
    city_map.insert("valletta", "valletta");
    city_map.insert("vlt", "valletta");

    // Chișinău, Moldova
    city_map.insert("chișinău", "chișinău");
    city_map.insert("kishinev", "chișinău");

    // Monaco, Monaco
    city_map.insert("monaco", "monaco");
    city_map.insert("monte carlo", "monaco");

    // Podgorica, Montenegro
    city_map.insert("podgorica", "podgorica");
    city_map.insert("pg", "podgorica");

    // Oslo, Norway
    city_map.insert("oslo", "oslo");
    city_map.insert("osl", "oslo");

    // Warsaw, Poland
    city_map.insert("warsaw", "warsaw");
    city_map.insert("warszawa", "warsaw");

    // Bucharest, Romania
    city_map.insert("bucharest", "bucharest");
    city_map.insert("bucurești", "bucharest");

    // Moscow, Russia
    city_map.insert("moscow", "moscow");
    city_map.insert("moskva", "moscow");

    // San Marino, San Marino
    city_map.insert("san marino", "san marino");

    // Belgrade, Serbia
    city_map.insert("belgrade", "belgrade");
    city_map.insert("beograd", "belgrade");

    // Bratislava, Slovakia
    city_map.insert("bratislava", "bratislava");
    city_map.insert("bts", "bratislava");

    // Ljubljana, Slovenia
    city_map.insert("ljubljana", "ljubljana");
    city_map.insert("lj", "ljubljana");

    // Madrid, Spain
    city_map.insert("madrid", "madrid");
    city_map.insert("mad", "madrid");

    // Stockholm, Sweden
    city_map.insert("stockholm", "stockholm");
    city_map.insert("sto", "stockholm");

    // Bern, Switzerland
    city_map.insert("bern", "bern");
    city_map.insert("berne", "bern");

    // Ankara, Turkey
    city_map.insert("ankara", "ankara");
    city_map.insert("ank", "ankara");

    // Kyiv, Ukraine
    city_map.insert("kyiv", "kyiv");
    city_map.insert("kiev", "kyiv");

    // Vatican City, Vatican City
    city_map.insert("vatican city", "vatican city");
    city_map.insert("vatican", "vatican city");

    // Resort Towns and Cities
    // Cancun, Mexico

    city_map.insert("cancun", "cancun");
    city_map.insert("cancún", "cancun");

    // Phuket, Thailand
    city_map.insert("phuket", "phuket");
    city_map.insert("phu", "phuket");

    // Bali, Indonesia
    city_map.insert("bali", "bali");
    city_map.insert("island of gods", "bali");

    // Nice, France
    city_map.insert("nice", "nice");
    city_map.insert("nizza", "nice");

    // Santorini, Greece
    city_map.insert("santorini", "santorini");
    city_map.insert("thira", "santorini");

    // Ibiza, Spain
    city_map.insert("ibiza", "ibiza");
    city_map.insert("eivissa", "ibiza");

    // Aspen, USA
    city_map.insert("aspen", "aspen");
    city_map.insert("asp", "aspen");

    // Whistler, Canada
    city_map.insert("whistler", "whistler");
    city_map.insert("whis", "whistler");

    // St. Moritz, Switzerland
    city_map.insert("st. moritz", "st. moritz");
    city_map.insert("saint moritz", "st. moritz");

    // Maldives
    city_map.insert("malé", "malé");
    city_map.insert("maldives", "malé");

    // Bora Bora, French Polynesia
    city_map.insert("bora bora", "bora bora");
    city_map.insert("bora", "bora bora");

    // Amalfi, Italy
    city_map.insert("amalfi", "amalfi");
    city_map.insert("amalfi coast", "amalfi");

    // Antalya, Turkey
    city_map.insert("antalya", "antalya");
    city_map.insert("ant", "antalya");

    // Sochi, Russia
    city_map.insert("sochi", "sochi");
    city_map.insert("soci", "sochi");

    // Hurghada, Egypt
    city_map.insert("hurghada", "hurghada");
    city_map.insert("hrg", "hurghada");

    // Marbella, Spain (including Costa del Sol)
    city_map.insert("marbella", "marbella");
    city_map.insert("marbs", "marbella");
    city_map.insert("costa del sol", "marbella");

    // Mykonos, Greece
    city_map.insert("mykonos", "mykonos");
    city_map.insert("myk", "mykonos");

    // Algarve, Portugal (Faro as main city)
    city_map.insert("faro", "faro");
    city_map.insert("algarve", "faro");

    // Pattaya, Thailand
    city_map.insert("pattaya", "pattaya");
    city_map.insert("pty", "pattaya");

    // Gold Coast, Australia
    city_map.insert("gold coast", "gold coast");
    city_map.insert("gc", "gold coast");

    // Sharm El Sheikh, Egypt
    city_map.insert("sharm el sheikh", "sharm el sheikh");
    city_map.insert("sharm", "sharm el sheikh");

    // Chamonix, France
    city_map.insert("chamonix", "chamonix");
    city_map.insert("cham", "chamonix");

    // Courchevel, France
    city_map.insert("courchevel", "courchevel");
    city_map.insert("courch", "courchevel");

    // Queenstown, New Zealand
    city_map.insert("queenstown", "queenstown");
    city_map.insert("qtn", "queenstown");

    // Langkawi, Malaysia
    city_map.insert("langkawi", "langkawi");
    city_map.insert("lgk", "langkawi");

    // Portofino, Italy
    city_map.insert("portofino", "portofino");
    city_map.insert("porto", "portofino");

    // St. Tropez, France
    city_map.insert("st. tropez", "st. tropez");
    city_map.insert("saint tropez", "st. tropez");

    // Hvar, Croatia
    city_map.insert("hvar", "hvar");
    city_map.insert("hvr", "hvar");

    // Kitzbühel, Austria
    city_map.insert("kitzbühel", "kitzbühel");
    city_map.insert("kitz", "kitzbühel");

    // Biarritz, France
    city_map.insert("biarritz", "biarritz");
    city_map.insert("bia", "biarritz");

    // Zermatt, Switzerland
    city_map.insert("zermatt", "zermatt");
    city_map.insert("zmt", "zermatt");

    // European Tourist Destinations
    // Florence, Italy
    city_map.insert("florence", "florence");
    city_map.insert("firenze", "florence");
    city_map.insert("city of lilies", "florence");

    // Venice, Italy
    city_map.insert("venice", "venice");
    city_map.insert("venezia", "venice");
    city_map.insert("la serenissima", "venice");

    // Salzburg, Austria
    city_map.insert("salzburg", "salzburg");
    city_map.insert("szg", "salzburg");

    // Bruges, Belgium
    city_map.insert("bruges", "bruges");
    city_map.insert("brugge", "bruges");
    city_map.insert("venice of the north", "bruges");

    // Dubrovnik, Croatia
    city_map.insert("dubrovnik", "dubrovnik");
    city_map.insert("dbv", "dubrovnik");
    city_map.insert("pearl of the adriatic", "dubrovnik");

    // Edinburgh, Scotland, UK
    city_map.insert("edinburgh", "edinburgh");
    city_map.insert("edi", "edinburgh");
    city_map.insert("auld reekie", "edinburgh");

    // Granada, Spain
    city_map.insert("granada", "granada");
    city_map.insert("grx", "granada");

    // Krakow, Poland
    city_map.insert("krakow", "krakow");
    city_map.insert("kraków", "krakow");
    city_map.insert("krk", "krakow");

    // Seville, Spain
    city_map.insert("seville", "seville");
    city_map.insert("sevilla", "seville");
    city_map.insert("svq", "seville");

    // Porto, Portugal
    city_map.insert("porto", "porto");
    city_map.insert("oporto", "porto");
    city_map.insert("opo", "porto");

    // Siena, Italy
    city_map.insert("siena", "siena");
    city_map.insert("say", "siena");

    // Bath, England, UK
    city_map.insert("bath", "bath");
    city_map.insert("bth", "bath");

    // Heidelberg, Germany
    city_map.insert("heidelberg", "heidelberg");
    city_map.insert("hdb", "heidelberg");

    // Rothenburg ob der Tauber, Germany
    city_map.insert("rothenburg ob der tauber", "rothenburg ob der tauber");
    city_map.insert("rothenburg", "rothenburg ob der tauber");

    // Cesky Krumlov, Czech Republic
    city_map.insert("cesky krumlov", "cesky krumlov");
    city_map.insert("český krumlov", "cesky krumlov");

    // Lviv, Ukraine
    city_map.insert("lviv", "lviv");
    city_map.insert("l'viv", "lviv");
    city_map.insert("little paris of ukraine", "lviv");

    // Bergen, Norway
    city_map.insert("bergen", "bergen");
    city_map.insert("bgo", "bergen");
    city_map.insert("gateway to the fjords", "bergen");

    // Interlaken, Switzerland
    city_map.insert("interlaken", "interlaken");
    city_map.insert("itk", "interlaken");

    // Toledo, Spain
    city_map.insert("toledo", "toledo");
    city_map.insert("city of three cultures", "toledo");

    city_map
}
