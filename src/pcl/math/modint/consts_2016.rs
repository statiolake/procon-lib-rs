use super::ModintInnerType;

// polyfill: return value from function
pub trait ModintConst {
    fn get_modulus() -> ModintInnerType;
}

/// `Modint` の定数 (`ModintConst` を実装する型) を簡単に定義するためのマクロ。
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "rust-131"))]
/// # #[macro_use] extern crate procon_lib;
/// # #[cfg(feature = "rust-131")]
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
// polyfill: use function instead
#[macro_export]
macro_rules! define_modint_const {
    ($(#[doc = $doc:expr])* pub const $name:ident = $value:expr;) => {
        $(#[doc = $doc])*
        pub enum $name {}
        impl $crate::pcl::math::modint::consts::ModintConst for $name {
            fn get_modulus() -> $crate::pcl::math::modint::ModintInnerType {
                $value
            }
        }
    };
}
