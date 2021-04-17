//! 群の定義といくつかの実装。

use super::monoid::Monoid;

/// 群
///
/// M が群であるとは、M が次の条件を満たす集合であることをいう。
///
/// - モノイドである
/// - 逆元の存在
///     任意の M の元 x に対して inv(x) が存在して op(x, inv(x)) = x 。
pub trait Group: Monoid {
    /// 逆元
    fn inv(x: Self) -> Self;
}

use crate::pcl::compat::num::Zero;

use std::ops::{Add, Neg};

/// 群の実装 : 加法群
///
/// 単位元を `Zero` 、演算を `Add` 、逆元を `Neg` によって提供するラッパー。
#[derive(Debug)]
pub struct Additive<T>(pub T);

impl<T: Clone> Clone for Additive<T> {
    fn clone(&self) -> Self {
        Additive(self.0.clone())
    }
}

impl<T: Copy> Copy for Additive<T> {}

impl<T> Monoid for Additive<T>
where
    T: Zero + Add<Output = T>,
{
    fn op(x: Self, y: Self) -> Self {
        Additive(x.0 + y.0)
    }

    fn id() -> Self {
        Additive(T::zero())
    }
}

impl<T> Group for Additive<T>
where
    T: Zero + Add<Output = T> + Neg<Output = T>,
{
    fn inv(x: Self) -> Self {
        Additive(-x.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Additive as A;
    use super::*;

    #[test]
    fn additive() {
        assert_eq!(A::<i32>::id().0, 0);
        assert_eq!(A::inv(A(2)).0, -2);
        assert_eq!(A::op(A(1), A(2)).0, 3);
    }
}
