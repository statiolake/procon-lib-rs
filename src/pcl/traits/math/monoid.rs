//! モノイドの定義といくつかの実装。

/// モノイド
///
/// M がモノイドであるとは、M が次の条件を満たす集合であることをいう。
///
/// - 演算  
///     演算 op(M, M) -> M が定義されている。
/// - 単位元の存在  
///     M にある元 1 が存在して op(1, x) = op(x, 1) = x 。
/// - 結合律が成立  
///     任意の M の元 x, y, z に対して op(op(x, y), z) = op(x, op(y, z)) 。
pub trait Monoid {
    /// 演算
    fn op(x: Self, y: Self) -> Self;

    /// 単位元
    fn id() -> Self;
}

use crate::pcl::traits::utils::num::{MaxValue, MinValue};
use std::cmp::Ord;
use std::cmp::{max, min};
use std::fmt;

/// モノイドの実装: 最小値を取る演算
///
/// 単位元は T::MAX でよい。 Range Minimum Query などの問題で Segment Tree と一
/// 緒に使う。
pub struct Min<T>(pub T);

impl<T: fmt::Debug> fmt::Debug for Min<T> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        f.debug_tuple("Min").field(&self.0).finish()
    }
}

impl<T: Clone> Clone for Min<T> {
    fn clone(&self) -> Self {
        Min(self.0.clone())
    }
}

impl<T: Copy> Copy for Min<T> {}

impl<T: Ord + MaxValue> Monoid for Min<T> {
    fn op(x: Self, y: Self) -> Self {
        Min(min(x.0, y.0))
    }

    fn id() -> Self {
        Min(T::max_value())
    }
}

/// モノイドの実装: 最大値を取る演算
///
/// 単位元は T::MIN でよい。 Range Maximum Query などの問題で Segment Tree と一
/// 緒に使う。
pub struct Max<T>(pub T);

impl<T: fmt::Debug> fmt::Debug for Max<T> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        f.debug_tuple("Max").field(&self.0).finish()
    }
}

impl<T: Clone> Clone for Max<T> {
    fn clone(&self) -> Self {
        Max(self.0.clone())
    }
}

impl<T: Copy> Copy for Max<T> {}

impl<T: Ord + MinValue> Monoid for Max<T> {
    fn op(x: Self, y: Self) -> Self {
        Max(max(x.0, y.0))
    }

    fn id() -> Self {
        Max(T::min_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rminq() {
        assert_eq!(Min::op(Min(1), Min(2)).0, 1);
        assert_eq!(Min::<i32>::id().0, ::std::i32::MAX);
        assert_eq!(Min::op(Min(1), Min::id()).0, 1);
    }

    #[test]
    fn rmaxq() {
        assert_eq!(Max::op(Max(1), Max(2)).0, 2);
        assert_eq!(Max::<i32>::id().0, ::std::i32::MIN);
        assert_eq!(Max::op(Max(1), Max::id()).0, 1);
    }
}
