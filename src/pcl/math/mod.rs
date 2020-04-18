#[cfg(feature = "rust2020")]
pub mod modint;

#[cfg(feature = "rust2020")]
pub use self::modint::{Modint, Modint17};
