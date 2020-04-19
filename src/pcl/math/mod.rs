//! 各種の数学的なアルゴリズムを定義する。

pub mod modint;
#[cfg(feature = "rust2020")]
pub mod sum;

pub use self::modint::{Modint, Modint17};
#[cfg(feature = "rust2020")]
pub use self::sum::{CumSum, CumSum2D};
