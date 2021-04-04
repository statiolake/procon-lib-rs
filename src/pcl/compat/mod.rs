//! バージョン違いの差異を吸収するレイヤー。

pub mod num {
    #[cfg(not(feature = "crates-atc-2020"))]
    pub use crate::pcl::polyfill::num::{One, Zero};
    #[cfg(feature = "crates-atc-2020")]
    pub use num::{One, Zero};
}
