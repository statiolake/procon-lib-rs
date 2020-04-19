use super::ModintInnerType;

// polyfill: return value from function
pub trait ModintConst {
    fn get_modulus() -> ModintInnerType;
}

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
