use std::collections::HashSet;
use std::fs;

fn main() {
    // Read the examples file
    let content = fs::read_to_string("src/time_zones/examples_dst_north.txt").unwrap();

    // Parse timezones from examples
    let mut example_timezones = HashSet::new();
    for line in content.lines() {
        if let Some((tz, _)) = line.split_once(' ') {
            example_timezones.insert(tz.to_string());
        }
    }

    println!("Total timezones in examples: {}", example_timezones.len());

    // Our supported timezone categories
    let our_categories = vec![
        "Africa",
        "America",
        "Antarctica",
        "Arctic",
        "Asia",
        "Atlantic",
        "Australia",
        "Indian",
        "Pacific",
    ];

    // Check each category
    for category in our_categories {
        let mut category_count = 0;
        let mut supported_count = 0;

        for tz in &example_timezones {
            if tz.starts_with(&format!("{}/", category)) {
                category_count += 1;

                // Check if we support this specific timezone
                if is_supported_timezone(tz) {
                    supported_count += 1;
                } else {
                    println!("Missing: {}", tz);
                }
            }
        }

        if category_count > 0 {
            println!(
                "{}: {}/{} supported ({:.1}%)",
                category,
                supported_count,
                category_count,
                (supported_count as f64 / category_count as f64) * 100.0
            );
        }
    }

    // Check general timezone abbreviations
    let general_tz_count = example_timezones
        .iter()
        .filter(|tz| !tz.contains('/'))
        .count();
    println!("\nGeneral timezone abbreviations: {}", general_tz_count);

    // List unsupported general timezones
    println!("\nUnsupported general timezones:");
    for tz in &example_timezones {
        if !tz.contains('/') && !is_supported_general_timezone(tz) {
            println!("  {}", tz);
        }
    }
}

fn is_supported_timezone(tz: &str) -> bool {
    // This is a simplified check - in reality we'd need to check our actual enums
    // For now, let's check if it's in a supported category
    let supported_categories = vec![
        "Africa",
        "America",
        "Antarctica",
        "Arctic",
        "Asia",
        "Atlantic",
        "Australia",
        "Indian",
        "Pacific",
    ];

    for category in supported_categories {
        if tz.starts_with(&format!("{}/", category)) {
            return true; // We support the category, but need to check specific cities
        }
    }

    false
}

fn is_supported_general_timezone(tz: &str) -> bool {
    let supported_general = vec![
        "EEST",
        "EET",
        "CEST",
        "CET",
        "IDLW",
        "SST",
        "HST",
        "AKDT",
        "AKST",
        "PDT",
        "PST",
        "MDT",
        "MST",
        "CDT",
        "CST",
        "EDT",
        "EST",
        "ADT",
        "AST",
        "NDT",
        "NST",
        "ART",
        "FNT",
        "AZOST",
        "AZOT",
        "GMT",
        "WEST",
        "WET",
        "AFT",
        "GST",
        "IRST",
        "PKT",
        "IST",
        "NPT",
        "BST",
        "MMT",
        "ICT",
        "SGT",
        "JST",
        "ACDT",
        "ACST",
        "AEDT",
        "AEST",
        "SBT",
        "NZDT",
        "NZST",
        "TOT",
        "UTC",
        "CET",
        "CST6CDT",
        "EST5EDT",
        "MST7MDT",
        "PST8PDT",
        "HST",
        "Factory",
        "GB",
        "GB-Eire",
        "Hongkong",
        "Iceland",
        "Iran",
        "Israel",
        "Jamaica",
        "Japan",
        "Kwajalein",
        "Libya",
        "MET",
        "MST",
        "Navajo",
        "NZ",
        "NZ-CHAT",
        "Poland",
        "Portugal",
        "PRC",
        "ROC",
        "ROK",
        "Singapore",
        "Turkey",
        "UCT",
        "Universal",
        "US/Alaska",
        "US/Aleutian",
        "US/Arizona",
        "US/Central",
        "US/East-Indiana",
        "US/Eastern",
        "US/Hawaii",
        "US/Indiana-Starke",
        "US/Michigan",
        "US/Mountain",
        "US/Pacific",
        "US/Samoa",
        "W-SU",
        "Zulu",
    ];

    supported_general.contains(tz)
}


