#[macro_use]
extern crate rust_i18n;

pub mod types;
pub mod services;

i18n!("locales", fallback = "en");