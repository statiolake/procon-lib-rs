//! バージョン違いの差異を吸収するレイヤー。

pub mod num {
    #[cfg(feature = "crates-atc-2020")]
    pub use num::{One, Zero};
    #[cfg(not(feature = "crates-atc-2020"))]
    pub use pcl::polyfill::num::{One, Zero};
}
