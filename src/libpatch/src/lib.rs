extern crate regex;

/// Allow regex_macros on rust-stable
#[macro_export]
macro_rules! regex(
    ($s:expr) => (regex::Regex::new($s).unwrap());
);
