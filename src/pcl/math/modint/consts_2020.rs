use super::ModintInnerType;

pub trait ModintConst {
    const MOD: ModintInnerType;
}

#[macro_export]
macro_rules! define_modint_const {
    (pub const $name:ident = $value:literal;) => {
        pub enum $name {}
        impl crate::pcl::math::modint::consts::ModintConst for $name {
            const MOD: crate::pcl::math::modint::ModintInnerType = $value;
        }
    };
}
