//! rust2016 と rust2020 の差を吸収するレイヤー。
//!
//! polyfill は rust2016 向けに機能を提供するモジュールで、ここは rust2016 と
//! rust2020 の間で主に `use` が煩雑にならないように共通化するレイヤー。

pub mod std {
    pub mod ops {
        #[cfg(feature = "rust2016")]
        pub use pcl::polyfill::std::ops::{Bound, RangeBounds};
        #[cfg(feature = "rust2020")]
        pub use std::ops::{Bound, RangeBounds};
    }
}

pub mod num {
    #[cfg(feature = "rust2020")]
    pub use num::{One, Zero};
    #[cfg(feature = "rust2016")]
    pub use pcl::polyfill::num::{One, Zero};
}
