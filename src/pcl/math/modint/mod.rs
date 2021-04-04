//! 常にある素数で割ったあまりになる整数型 `Modint` を定義する。
//!
//! 中身の整数型は `ModintInnerType` で定められており、変更することを前
//! 提としていない。ジェネリクスにしないのは、諸々の必要な条件を満たすのは結局ほぼプリミティブ整数し
//! かなく、さらに実際 1e9+7 などの特有の法が使われることが多いために、ジェネリクスとして定義するほ
//! どの意味がそもそもなくなっているからである。
//!
//! 実際に法を指定するときは、特別なトレイトを実装した型 (「定数」) を用意してジェネリクスとして
//! `Modint` に与える。例えば定数 1e9+7 を表す型は `MOD17` である。それを使った`Modint<MOD17>` はよ
//! く使われると考えられるので `Modint17` のエイリアスを用意している。
//!
//! 任意の定数を法に指定する方法を含めて、使い方は次の通りである。
//!
//! # Example
//!
//! ```
//! # use procon_lib::define_modint_const;
//! # use procon_lib::pcl::math::modint::Modint;
//! #
//! // use crate::define_modint_const;
//! define_modint_const! {
//!     pub const MOD5 = 5;
//! }
//!
//! type M5 = Modint<MOD5>;
//!
//! assert_eq!(M5::new(10), M5::new(0));
//! assert_eq!(M5::new(3) + M5::new(4), M5::new(2));
//! assert_eq!(M5::new(4) / M5::new(2), M5::new(2));
//! assert_eq!(M5::new(3) / M5::new(2), M5::new(4));
//! ```

/// `Modint` の法になる定数を定めるマクロを提供する。
#[macro_use]
pub mod consts;

#[cfg(feature = "crates-atc-2020")]
use num::Num;

use self::consts::ModintConst;
use super::super::compat::num::{One, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::mem;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// `Modint` が扱う内部型。
pub type ModintInnerType = i64;

define_modint_const! {
    #[doc = "1e9+7 を表す定数。"]
    pub const MOD17 = 1_000_000_007;
}

/// 1e9+7 で割ったあまりを利用する `Modint` 。
pub type Modint17 = Modint<MOD17>;

/// 常にある法 `C` で割ったあまりを計算する整数型。
pub struct Modint<C> {
    value: ModintInnerType,
    marker: PhantomData<C>,
}

impl<C> Modint<C> {
    /// チェックしないで新しい `Modint` を作成する。
    ///
    /// # Safety
    ///
    /// - `0 <= value < C` を満たすこと。
    pub unsafe fn new_unchecked(value: ModintInnerType) -> Modint<C> {
        Modint {
            value,
            marker: PhantomData,
        }
    }

    /// 中身の値を取り出す。
    pub fn inner(self) -> ModintInnerType {
        self.value
    }
}

impl<C: ModintConst> Modint<C> {
    /// 新しい `Modint` を作成する。値は最初に丸められる。
    pub fn new(mut value: ModintInnerType) -> Modint<C> {
        assert_ne!(C::MOD, 0, "MOD is 0");
        if value < 0 {
            let m = (-value) / C::MOD;
            value += (m + 1) * C::MOD;
        }

        unsafe { Modint::new_unchecked(value % C::MOD) }
    }

    /// 逆元を求める。
    pub fn inv(self) -> Modint<C> {
        let mut modulus = C::MOD;
        let mut a = self.value;
        let mut u = 1;
        let mut v = 0;
        while modulus > 0 {
            let t = a / modulus;
            a -= t * modulus;
            u -= t * v;
            mem::swap(&mut a, &mut modulus);
            mem::swap(&mut u, &mut v);
        }

        Modint::new(u)
    }
}

impl<C: ModintConst> PartialEq for Modint<C> {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl<C: ModintConst> PartialOrd for Modint<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner().partial_cmp(&other.inner())
    }
}

impl<C: ModintConst> Eq for Modint<C> {}

impl<C: ModintConst> Ord for Modint<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner().cmp(&other.inner())
    }
}

impl<C: ModintConst> Hash for Modint<C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state);
    }
}

impl<C> fmt::Debug for Modint<C> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<C> Clone for Modint<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> Copy for Modint<C> {}

impl<C: ModintConst> AddAssign for Modint<C> {
    fn add_assign(&mut self, rhs: Modint<C>) {
        self.value += rhs.value;
        if self.value >= C::MOD {
            self.value -= C::MOD;
        }
    }
}

impl<C: ModintConst> SubAssign for Modint<C> {
    fn sub_assign(&mut self, rhs: Modint<C>) {
        self.value -= rhs.value;
        if self.value < 0 {
            self.value += C::MOD;
        }
    }
}

impl<C: ModintConst> MulAssign for Modint<C> {
    fn mul_assign(&mut self, rhs: Modint<C>) {
        self.value *= rhs.value;
        self.value %= C::MOD;
    }
}

impl<C: ModintConst> DivAssign for Modint<C> {
    fn div_assign(&mut self, rhs: Modint<C>) {
        if rhs.value == 0 {
            panic!("attempted to divide by zero");
        }

        *self *= rhs.inv();
    }
}

// Num の条件を満たすため仕方なく
impl<C: ModintConst> RemAssign for Modint<C> {
    fn rem_assign(&mut self, rhs: Modint<C>) {
        if rhs.value == 0 {
            panic!("attempted to divide by zero.")
        }

        self.value %= rhs.value;
    }
}

impl<C: ModintConst> Neg for Modint<C> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::zero() - self
    }
}

macro_rules! impl_arith_by_assign {
    (impl $traitname:ident::$fnname:ident { use $op:tt; }) => {
        impl<C: ModintConst> $traitname for Modint<C> {
            type Output = Modint<C>;
            fn $fnname(mut self, rhs: Modint<C>) -> Modint<C> {
                self $op rhs;
                self
            }
        }
    };
}

impl_arith_by_assign!(impl Add::add { use +=; });
impl_arith_by_assign!(impl Sub::sub { use -=; });
impl_arith_by_assign!(impl Mul::mul { use *=; });
impl_arith_by_assign!(impl Div::div { use /=; });
impl_arith_by_assign!(impl Rem::rem { use %=; });

impl<C: ModintConst> One for Modint<C> {
    fn one() -> Modint<C> {
        assert_ne!(C::MOD, 1, "one() is called for Modint with MOD = 1");
        unsafe { Modint::new_unchecked(1) }
    }
}

impl<C: ModintConst> Sum for Modint<C> {
    fn sum<I: Iterator<Item = Modint<C>>>(iter: I) -> Modint<C> {
        iter.fold(Modint::zero(), Add::add)
    }
}

impl<C: ModintConst> Product for Modint<C> {
    fn product<I: Iterator<Item = Modint<C>>>(iter: I) -> Self {
        iter.fold(Modint::one(), Mul::mul)
    }
}

impl<C: ModintConst> Zero for Modint<C> {
    fn zero() -> Modint<C> {
        unsafe { Modint::new_unchecked(0) }
    }

    fn is_zero(&self) -> bool {
        self.inner() == 0
    }
}

impl<C> fmt::Display for Modint<C> {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "{}", self.inner())
    }
}

#[cfg(feature = "crates-atc-2020")]
impl<C: ModintConst> Num for Modint<C> {
    type FromStrRadixErr = <ModintInnerType as Num>::FromStrRadixErr;
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        ModintInnerType::from_str_radix(src, radix).map(Modint::new)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::traits::math::group::Additive as A;
    use super::super::CumSum;
    use super::*;

    define_modint_const! {
        pub const MOD5 = 5;
    }

    type M = Modint<MOD5>;

    #[test]
    fn modint() {
        let mut a = M::new(2);
        let b = M::new(3);

        assert_eq!(a + b, M::new(0));
        assert_eq!(a - b, M::new(4));
        assert_eq!(a * b, M::new(1));
        assert_eq!(a.inv(), M::new(3));
        assert_eq!(b.inv(), M::new(2));
        assert_eq!(a / b, M::new(4));
        assert_eq!(b % a, M::new(1));

        a *= b;
        assert_eq!(a, M::new(1));
        a -= b;
        assert_eq!(a, M::new(3));
        a += b;
        assert_eq!(a, M::new(1));
        a /= b;
        assert_eq!(a, M::new(2));

        assert_eq!(
            [M::new(1), M::new(2), M::new(3), M::new(4)]
                .iter()
                .cloned()
                .sum::<M>(),
            M::new(0)
        );

        assert_eq!(
            [M::new(1), M::new(2), M::new(3), M::new(4)]
                .iter()
                .cloned()
                .product::<M>(),
            M::new(4)
        );

        #[cfg(feature = "crates-atc-2020")]
        assert_eq!(num::pow(a, 10), M::new(4));

        let cs = CumSum::from_array(vec![A(M::new(3)), A(M::new(4)), A(M::new(2))]);
        assert_eq!(cs.sum(1..).0, M::new(1));
        assert_eq!(cs.sum(..2).0, M::new(2));
    }
}
