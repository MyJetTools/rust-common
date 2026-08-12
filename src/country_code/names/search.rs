use std::collections::BTreeMap;

use crate::country_code::CountryCode;

/// Folds a country name or a user query into a comparable form:
/// lower case, diacritics stripped to their ASCII base letter, apostrophes
/// removed and every other separator collapsed to a single space.
///
/// Both sides of a search go through this, which is why `Réunion` is found by
/// typing `reuni` and `Côte d'Ivoire` by typing `cote divoire` - no per-country
/// data is needed for accented names.
pub fn fold_for_search(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let mut pending_space = false;

    for c in src.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_space {
                result.push(' ');
                pending_space = false;
            }

            result.push(c.to_ascii_lowercase());
            continue;
        }

        for f in fold_char(c).chars() {
            if f == ' ' {
                pending_space = !result.is_empty();
                continue;
            }

            if pending_space {
                result.push(' ');
                pending_space = false;
            }

            result.push(f);
        }
    }

    result
}

fn fold_char(c: char) -> &'static str {
    match c {
        // dropped entirely so `d'Ivoire` and `dIvoire` fold the same
        '\'' | '\u{2019}' | '\u{02BC}' => "",
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => "a",
        'æ' | 'Æ' => "ae",
        'ç' | 'Ç' => "c",
        'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => "e",
        'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => "i",
        'ñ' | 'Ñ' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => "o",
        'œ' | 'Œ' => "oe",
        'ß' => "ss",
        'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => "u",
        'ý' | 'ÿ' | 'Ý' | 'Ÿ' => "y",
        'ð' | 'Ð' => "d",
        'þ' | 'Þ' => "th",
        'š' | 'Š' => "s",
        'ž' | 'Ž' => "z",
        'č' | 'Č' => "c",
        'ř' | 'Ř' => "r",
        'ğ' | 'Ğ' => "g",
        'ı' | 'İ' => "i",
        'ş' | 'Ş' => "s",
        'ł' | 'Ł' => "l",
        _ => " ",
    }
}

lazy_static::lazy_static! {
    /// Extra searchable names for countries whose common English name differs
    /// from the formal name in [`super::EN_NAMES`].
    ///
    /// Accented spellings do NOT belong here - [`fold_for_search`] already
    /// handles those. Only add an entry when the alternative name shares no
    /// searchable substring with the formal one.
    pub static ref SEARCH_ALIASES: BTreeMap<CountryCode, &'static [&'static str]> = {
        let mut result: BTreeMap<CountryCode, &'static [&'static str]> = BTreeMap::new();

        result.insert(CountryCode::CIV, &["Ivory Coast"]);
        result.insert(CountryCode::NLD, &["Holland", "The Netherlands"]);
        result.insert(CountryCode::MMR, &["Burma"]);
        result.insert(CountryCode::SWZ, &["Eswatini"]);
        result.insert(CountryCode::TUR, &["Turkiye", "Türkiye"]);
        result.insert(CountryCode::KOR, &["South Korea"]);
        result.insert(CountryCode::PRK, &["North Korea"]);
        result.insert(CountryCode::RUS, &["Russia"]);
        result.insert(
            CountryCode::GBR,
            &[
                "United Kingdom",
                "UK",
                "Great Britain",
                "Britain",
                "England",
                "Scotland",
                "Wales",
            ],
        );
        result.insert(CountryCode::ARE, &["UAE"]);
        result.insert(CountryCode::CZE, &["Czech Republic"]);
        result.insert(CountryCode::CPV, &["Cape Verde"]);
        result.insert(CountryCode::TLS, &["East Timor"]);
        result.insert(CountryCode::LAO, &["Laos"]);
        result.insert(CountryCode::VNM, &["Vietnam"]);
        result.insert(CountryCode::COD, &["DR Congo", "DRC", "Congo Kinshasa", "Zaire"]);
        result.insert(CountryCode::COG, &["Congo Brazzaville"]);
        result.insert(CountryCode::FLK, &["Falklands"]);
        result.insert(CountryCode::MAC, &["Macau"]);
        result.insert(CountryCode::KNA, &["St Kitts"]);
        result.insert(CountryCode::LCA, &["St Lucia"]);
        result.insert(CountryCode::VCT, &["St Vincent"]);
        result.insert(CountryCode::BIH, &["Bosnia"]);
        result.insert(CountryCode::PSE, &["Palestine"]);
        result.insert(CountryCode::XKX, &["Kosova"]);

        // territories whose formal name is not what anybody types
        result.insert(CountryCode::VAT, &["Vatican", "Vatican City"]);
        result.insert(CountryCode::BLM, &["St Barts", "St Barthelemy"]);
        result.insert(CountryCode::MAF, &["St Martin"]);
        result.insert(CountryCode::SXM, &["St Maarten"]);
        result.insert(CountryCode::BES, &["Caribbean Netherlands"]);
        result.insert(CountryCode::VIR, &["US Virgin Islands"]);
        result.insert(CountryCode::SHN, &["Ascension", "Tristan da Cunha"]);
        result.insert(CountryCode::SJM, &["Spitsbergen"]);

        result
    };
}

fn strip_spaces(src: &str) -> String {
    src.chars().filter(|c| *c != ' ').collect()
}

fn classify(key: &str, term: &str) -> Option<SearchMatch> {
    if key == term {
        return Some(SearchMatch::Exact);
    }

    if key.starts_with(term) {
        return Some(SearchMatch::Prefix);
    }

    if key.contains(term) {
        return Some(SearchMatch::Contains);
    }

    None
}

/// How closely a country matched a search term. Lower sorts first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SearchMatch {
    /// The term is one of the country's names or codes in full.
    Exact = 0,
    /// One of the country's names or codes starts with the term.
    Prefix = 1,
    /// The term appears somewhere inside one of the names.
    Contains = 2,
}

impl CountryCode {
    /// Every string this country can be found by: its English name, its ISO
    /// codes and any alias from [`SEARCH_ALIASES`], all already folded.
    pub fn search_keys(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(4);

        result.push(fold_for_search(self.as_iso2_str()));
        result.push(fold_for_search(self.as_iso3_str()));

        if let Some(name) = super::EN_NAMES.get(self) {
            result.push(fold_for_search(name));
        }

        if let Some(aliases) = SEARCH_ALIASES.get(self) {
            for alias in aliases.iter() {
                result.push(fold_for_search(alias));
            }
        }

        result
    }

    /// Returns how well this country matches `term`, or `None` if it does not.
    ///
    /// The term is folded the same way the names are, so `"reuni"`, `"Réuni"`
    /// and `"REUNI"` behave identically.
    pub fn match_search_term(&self, term: &str) -> Option<SearchMatch> {
        let term = fold_for_search(term);

        if term.is_empty() {
            return None;
        }

        let term_no_spaces = strip_spaces(&term);
        let mut best: Option<SearchMatch> = None;

        for key in self.search_keys() {
            // the second pass makes word breaks irrelevant, so a query typed as
            // `cote d ivoire` still reaches `Côte d'Ivoire`, which folds without
            // the break because the apostrophe is dropped
            let found = match classify(&key, &term) {
                Some(found) => found,
                None => match classify(&strip_spaces(&key), &term_no_spaces) {
                    Some(found) => found,
                    None => continue,
                },
            };

            match best {
                Some(current) if current <= found => {}
                _ => best = Some(found),
            }

            if best == Some(SearchMatch::Exact) {
                break;
            }
        }

        best
    }

    pub fn matches_search_term(&self, term: &str) -> bool {
        self.match_search_term(term).is_some()
    }
}

/// One row per country, folded once at first use.
///
/// [`CountryCode::search_keys`] rebuilds its keys on every call and resolves the
/// ISO codes through a linear scan of the code maps; a type-ahead calling that
/// for all 250 countries on each keystroke would redo tens of thousands of map
/// iterations. This table pays for it once.
struct SearchRow {
    country_code: CountryCode,
    /// sorted by name so ties come out alphabetically
    sort_name: &'static str,
    keys: Vec<String>,
    keys_no_spaces: Vec<String>,
}

lazy_static::lazy_static! {
    static ref SEARCH_INDEX: Vec<SearchRow> = {
        let mut result: Vec<SearchRow> = crate::country_code::COUNTRIES_ISO_3_CODES
            .values()
            .map(|country_code| {
                let keys = country_code.search_keys();
                let keys_no_spaces = keys
                    .iter()
                    .filter(|key| key.contains(' '))
                    .map(|key| strip_spaces(key))
                    .collect();

                SearchRow {
                    country_code: *country_code,
                    sort_name: country_code.as_country_name_en(),
                    keys,
                    keys_no_spaces,
                }
            })
            .collect();

        result.sort_by_key(|row| row.sort_name);

        result
    };
}

/// Finds every country matching `term`, best match first.
///
/// Ties are broken by English name so the order is stable across calls.
pub fn search_country_codes(term: &str) -> Vec<CountryCode> {
    let term = fold_for_search(term);

    if term.is_empty() {
        return Vec::new();
    }

    let term_no_spaces = strip_spaces(&term);
    let mut found: Vec<(SearchMatch, CountryCode)> = Vec::new();

    for row in SEARCH_INDEX.iter() {
        let mut best: Option<SearchMatch> = None;

        for key in row.keys.iter() {
            if let Some(matched) = classify(key, &term) {
                if best.map(|current| matched < current).unwrap_or(true) {
                    best = Some(matched);
                }
            }
        }

        if best != Some(SearchMatch::Exact) {
            for key in row.keys_no_spaces.iter() {
                if let Some(matched) = classify(key, &term_no_spaces) {
                    if best.map(|current| matched < current).unwrap_or(true) {
                        best = Some(matched);
                    }
                }
            }
        }

        if let Some(matched) = best {
            found.push((matched, row.country_code));
        }
    }

    // stable sort keeps the alphabetical order the index was built in
    found.sort_by_key(|itm| itm.0);

    found.into_iter().map(|itm| itm.1).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn accented_names_are_found_by_plain_ascii() {
        for term in ["reuni", "Reunion", "réunion", "REUNI"] {
            assert_eq!(
                search_country_codes(term).first().copied(),
                Some(CountryCode::REU),
                "term: {}",
                term
            );
        }

        for term in ["curacao", "Curaçao"] {
            assert_eq!(
                search_country_codes(term).first().copied(),
                Some(CountryCode::CUW),
                "term: {}",
                term
            );
        }

        for term in ["aland", "Åland"] {
            assert_eq!(
                search_country_codes(term).first().copied(),
                Some(CountryCode::ALA),
                "term: {}",
                term
            );
        }
    }

    #[test]
    fn cote_divoire_is_found_by_every_spelling() {
        for term in [
            "ivory coast",
            "Ivory Coast",
            "cote divoire",
            "Cote d'Ivoire",
            "Côte d'Ivoire",
            "cote d ivoire",
            "CIV",
        ] {
            assert!(
                search_country_codes(term).contains(&CountryCode::CIV),
                "term: {}",
                term
            );
        }
    }

    #[test]
    fn aliases_resolve_to_the_formal_name() {
        let cases = [
            ("Holland", CountryCode::NLD),
            ("Burma", CountryCode::MMR),
            ("South Korea", CountryCode::KOR),
            ("North Korea", CountryCode::PRK),
            ("Russia", CountryCode::RUS),
            ("Vietnam", CountryCode::VNM),
            ("Laos", CountryCode::LAO),
            ("Czech Republic", CountryCode::CZE),
            ("Cape Verde", CountryCode::CPV),
            ("East Timor", CountryCode::TLS),
            ("Eswatini", CountryCode::SWZ),
            ("Türkiye", CountryCode::TUR),
            ("UAE", CountryCode::ARE),
            ("Zaire", CountryCode::COD),
            ("Falklands", CountryCode::FLK),
            ("Macau", CountryCode::MAC),
        ];

        for (term, expected) in cases {
            assert!(
                search_country_codes(term).contains(&expected),
                "term: {} expected: {:?}",
                term,
                expected
            );
        }
    }

    #[test]
    fn iso_codes_match_exactly_and_sort_first() {
        assert_eq!(
            search_country_codes("RE").first().copied(),
            Some(CountryCode::REU)
        );
        assert_eq!(
            search_country_codes("lao").first().copied(),
            Some(CountryCode::LAO)
        );
        assert_eq!(
            search_country_codes("us").first().copied(),
            Some(CountryCode::USA)
        );
    }

    #[test]
    fn empty_and_junk_terms_return_nothing() {
        for term in ["", "   ", "-", "'", "zzzzzz"] {
            assert!(
                search_country_codes(term).is_empty(),
                "term: {:?} returned {:?}",
                term,
                search_country_codes(term)
            );
        }
    }

    #[test]
    fn every_country_is_reachable_by_its_own_codes() {
        for country_code in crate::country_code::COUNTRIES_ISO_3_CODES.values() {
            let iso3 = country_code.as_iso3_str();

            assert_eq!(
                country_code.match_search_term(iso3),
                Some(SearchMatch::Exact),
                "{:?} not found by its own iso3 {}",
                country_code,
                iso3
            );
        }
    }

    #[test]
    fn folding_is_stable() {
        assert_eq!(fold_for_search("Côte d'Ivoire"), "cote divoire");
        assert_eq!(fold_for_search("Réunion"), "reunion");
        assert_eq!(fold_for_search("Åland Islands"), "aland islands");
        assert_eq!(fold_for_search("Curaçao"), "curacao");
        assert_eq!(fold_for_search("  Timor-Leste  "), "timor leste");
        assert_eq!(fold_for_search("Iran (Islamic Republic of)"), "iran islamic republic of");
        assert_eq!(fold_for_search("Lao People's Democratic Republic"), "lao peoples democratic republic");
    }
}
