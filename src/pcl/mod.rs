//! 実質のルート。
//!
//! 実際の提出用プロジェクトでは、このディレクトリをクレート直下に配置することを想定している。
#[cfg(feature = "rust2016")]
#[macro_use]
pub mod polyfill;

pub mod collections;
pub mod compat;
pub mod macros;
pub mod math;
pub mod prelude;
pub mod structure;
pub mod traits;
pub mod utils;
