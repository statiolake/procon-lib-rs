#[cfg(feature = "rust2016")]
pub mod polyfill;
#[cfg(feature = "rust2016")]
pub mod stdin;

#[cfg(feature = "rust2020")]
pub mod sum;
#[cfg(feature = "rust2020")]
pub mod traits;

pub mod macros;
pub mod math;
pub mod structure;
