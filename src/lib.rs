#![cfg_attr(not(feature = "std"), no_std)]
//! Russian language analysis for Pizza search engine.
//!
//! Provides a full-featured Russian analyzer with ё→е normalization,
//! light stemming, and stop words.
//!
//! # Components
//!
//! - [`RussianYoFilter`] — Normalizes ё/Ё to е/Е
//! - [`RussianLightStemFilter`] — Light suffix-stripping stemmer
//! - [`RussianStopFilter`] — Russian stop words filter
extern crate alloc;
mod normalization;
mod stem;
mod stop;

pub mod register;

pub use normalization::RussianYoFilter;
pub use register::register_all;
pub use stem::RussianLightStemFilter;
pub use stop::RussianStopFilter;
