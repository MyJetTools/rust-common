#[cfg(feature = "country-code")]
pub mod country_code;
#[cfg(feature = "markdown")]
mod markdown_applier;
#[cfg(feature = "time-zones")]
pub mod time_zones;
pub mod validators;
#[cfg(feature = "markdown")]
pub use markdown_applier::*;

#[cfg(feature = "fuzzy_data")]
pub mod fuzzy_data;

pub mod text_segments;

#[cfg(feature = "placeholders")]
pub mod placeholders;

#[cfg(feature = "user_agent")]
pub mod user_agent;

#[cfg(feature = "phone-code")]
pub mod phone_codes;

pub mod object_id;
