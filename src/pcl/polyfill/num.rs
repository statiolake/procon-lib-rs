use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;
}

macro_rules! impl_for_primitive {
    ($ty:ty; $zero:expr, $one:expr) => {
        impl Zero for $ty {
            fn zero() -> Self {
                $zero
            }

            fn is_zero(&self) -> bool {
                *self == $zero
            }
        }

        impl One for $ty {
            fn one() -> Self {
                $one
            }
        }
    };
}

impl_for_primitive!(u8; 0, 1);
impl_for_primitive!(u16; 0, 1);
impl_for_primitive!(u32; 0, 1);
impl_for_primitive!(u64; 0, 1);
impl_for_primitive!(usize; 0, 1);
impl_for_primitive!(i8; 0, 1);
impl_for_primitive!(i16; 0, 1);
impl_for_primitive!(i32; 0, 1);
impl_for_primitive!(i64; 0, 1);
impl_for_primitive!(isize; 0, 1);
impl_for_primitive!(f32; 0.0, 1.0);
impl_for_primitive!(f64; 0.0, 1.0);

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

pub trait Num: Zero + One + NumOps + PartialEq<Self> {}
