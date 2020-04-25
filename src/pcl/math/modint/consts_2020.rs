use super::ModintInnerType;

pub trait ModintConst {
    const MOD: ModintInnerType;
}

/// `Modint` の定数 (`ModintConst` を実装する型) を簡単に定義するためのマクロ。
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "rust2016")]
/// # #[macro_use] extern crate procon_lib;
/// # #[cfg(feature = "rust2020")]
/// # use procon_lib::define_modint_const;
/// // use crate::define_modint_const;
/// define_modint_const! {
///     pub const MOD11 = 11;
/// }
///
/// # use procon_lib::pcl::math::Modint;
/// // use pcl::math::Modint;
/// type M11 = Modint<MOD11>;
/// #
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! define_modint_const {
    ($(#[doc = $doc:expr])* pub const $name:ident = $value:literal;) => {
        $(#[doc = $doc])*
        pub enum $name {}
        impl $crate::pcl::math::modint::consts::ModintConst for $name {
            const MOD: $crate::pcl::math::modint::ModintInnerType = $value;
        }
    };
}
