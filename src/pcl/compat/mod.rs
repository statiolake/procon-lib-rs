//! バージョン違いの差異を吸収するレイヤー。

pub mod std {
    pub mod ops {
        #[cfg(not(feature = "rust-131"))]
        pub use pcl::polyfill::std::ops::{Bound, RangeBounds};
        #[cfg(feature = "rust-131")]
        pub use std::ops::{Bound, RangeBounds};
    }
}

pub mod num {
    #[cfg(feature = "crates-atc-2020")]
    pub use num::{One, Zero};
    #[cfg(not(feature = "crates-atc-2020"))]
    pub use pcl::polyfill::num::{One, Zero};
}
