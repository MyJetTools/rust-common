#[cfg(feature = "country-code")]
pub mod country_code;

mod markdown_applier;
#[cfg(feature = "time-zones")]
pub mod time_zones;
#[cfg(feature = "validators")]
pub mod validators;
pub use markdown_applier::*;

#[cfg(feature = "fuzzy_data")]
pub mod fuzzy_data;
pub mod markdown_segments;

#[cfg(feature = "placeholders")]
pub mod placeholders;

pub mod user_agent;
