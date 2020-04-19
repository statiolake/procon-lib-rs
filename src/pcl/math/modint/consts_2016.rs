use super::ModintInnerType;

// polyfill: return value from function
pub trait ModintConst {
    fn get_modulus() -> ModintInnerType;
}

// polyfill: use function instead
macro_rules! define_modint_const {
    (pub const $name:ident = $value:expr;) => {
        pub enum $name {}
        impl $crate::pcl::math::modint::consts::ModintConst for $name {
            fn get_modulus() -> ModintInnerType {
                $value
            }
        }
    };
}
