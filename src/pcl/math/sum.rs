//! 区間の和を高速に計算する `CumSum`, `CumSum2D` を定義する。

use crate::pcl::traits::Group;
use std::cmp;
use std::ops::{Bound, RangeBounds};

/// ある数列の、指定された範囲の和を高速に計算する。
///
/// 実際は必ずしも通常の整数と和である必要はなく、群 (`Group`) であれば良い。
pub struct CumSum<T> {
    sum: Vec<T>,
}

impl<T: Group + Copy> CumSum<T> {
    /// 与えられた数列の累積和をとり、 `CumSum` を生成する。
    ///
    /// # 計算量
    ///
    /// O(n)
    pub fn from_array<A: AsRef<[T]>>(array: A) -> CumSum<T> {
        let array = array.as_ref();
        let mut sum = vec![T::zero(); array.len() + 1];
        for i in 1..=array.len() {
            unsafe {
                *sum.get_unchecked_mut(i) = *sum.get_unchecked(i - 1) + *array.get_unchecked(i - 1);
            }
        }

        CumSum { sum }
    }

    /// 指定された範囲内の総和を返す。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        // 最初の配列の長さ
        let orig_len = self.sum.len() - 1;

        let start = range_start(&range, 0);
        let end = range_end(&range, orig_len);

        if end <= start {
            return T::zero();
        }

        unsafe { *self.sum.get_unchecked(end) - *self.sum.get_unchecked(start) }
    }
}

/// ある二次元数列の、指定された範囲の和を高速に計算する。
///
/// 実際は必ずしも通常の整数と和である必要はなく、群 (`Group`) であれば良い。
pub struct CumSum2D<T> {
    sum: Vec<Vec<T>>,
}

impl<T: Group + Copy> CumSum2D<T> {
    /// 与えられた行列の累積和をとり、 `CumSum2D` を生成する。
    ///
    /// # 計算量
    ///
    /// n 行 m 列の行列に対し、 O(nm)
    pub fn from_matrix<M, A>(matrix: M) -> CumSum2D<T>
    where
        M: AsRef<[A]>,
        A: AsRef<[T]>,
    {
        let array = matrix.as_ref();
        let height = array.len();
        if height == 0 {
            return CumSum2D {
                sum: vec![vec![T::zero()]],
            };
        }

        let width = unsafe { array.get_unchecked(0) }.as_ref().len();
        let mut sum = vec![vec![T::zero(); width + 1]; height + 1];

        for i in 1..=height {
            for j in 1..=width {
                assert_eq!(
                    unsafe { array.get_unchecked(i - 1) }.as_ref().len(),
                    width,
                    "the array's length is differ line by line"
                );

                unsafe {
                    *sum.get_unchecked_mut(i).get_unchecked_mut(j) =
                        *sum.get_unchecked(i - 1).get_unchecked(j)
                            + *sum.get_unchecked(i).get_unchecked(j - 1)
                            - *sum.get_unchecked(i - 1).get_unchecked(j - 1)
                            + *array.get_unchecked(i - 1).as_ref().get_unchecked(j - 1)
                }
            }
        }

        CumSum2D { sum }
    }

    /// 指定された範囲内の総和を返す。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn sum<RY, RX>(&self, yrange: RY, xrange: RX) -> T
    where
        RY: RangeBounds<usize>,
        RX: RangeBounds<usize>,
    {
        // 最初の配列の長さ
        let orig_height = self.sum.len() - 1;
        // safety: self.sum は必ず要素を一つは含む (`vec![vec![0]]` が最小)
        let orig_width = unsafe { self.sum.get_unchecked(0).len() } - 1;

        let ystart = range_start(&yrange, 0);
        let yend = range_end(&yrange, orig_height);
        let xstart = range_start(&xrange, 0);
        let xend = range_end(&xrange, orig_width);

        if yend <= ystart || xend <= xstart {
            return T::zero();
        }

        unsafe {
            *self.sum.get_unchecked(yend).get_unchecked(xend)
                + *self.sum.get_unchecked(ystart).get_unchecked(xstart)
                - *self.sum.get_unchecked(ystart).get_unchecked(xend)
                - *self.sum.get_unchecked(yend).get_unchecked(xstart)
        }
    }
}

fn range_start<R: RangeBounds<usize>>(range: &R, min: usize) -> usize {
    let start = match range.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
        Bound::Unbounded => 0,
    };

    cmp::max(start, min)
}

fn range_end<R: RangeBounds<usize>>(range: &R, max: usize) -> usize {
    let end = match range.end_bound() {
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
        Bound::Unbounded => max,
    };

    cmp::min(end, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bounds() {
        assert_eq!(range_start(&(0..1), 0), 0);
        assert_eq!(range_start(&(0..=1), 0), 0);
        assert_eq!(range_start(&(..1), 0), 0);
        assert_eq!(range_start(&(0..), 0), 0);
        assert_eq!(range_start(&(..), 0), 0);
        assert_eq!(range_end(&(0..1), 1), 1);
        assert_eq!(range_end(&(0..=1), 1), 1);
        assert_eq!(range_end(&(..1), 1), 1);
        assert_eq!(range_end(&(0..), 1), 1);
        assert_eq!(range_end(&(..), 1), 1);
    }

    #[test]
    fn check_cumsum() {
        let cumsum = CumSum::from_array(&[5, 4, 1, 3, 2, 6]);
        assert_eq!(cumsum.sum(0..6), 21);
        assert_eq!(cumsum.sum(0..=5), 21);
        assert_eq!(cumsum.sum(..6), 21);
        assert_eq!(cumsum.sum(..=5), 21);
        assert_eq!(cumsum.sum(0..), 21);
        assert_eq!(cumsum.sum(..), 21);
        assert_eq!(cumsum.sum(1..2), 4);
        assert_eq!(cumsum.sum(1..5), 10);
        assert_eq!(cumsum.sum(1..=1), 4);
        assert_eq!(cumsum.sum(1..0), 0);
    }

    #[test]
    fn check_cumsum2d() {
        let cumsum2d = CumSum2D::from_matrix(vec![
            vec![4, 2, 3, 6, 1],
            vec![5, 5, 2, 1, 4],
            vec![1, 2, 3, 2, 2],
            vec![3, 2, 1, 3, 2],
        ]);
        assert_eq!(cumsum2d.sum(0..2, 3..4), 7);
        assert_eq!(cumsum2d.sum(.., ..), 54);
        assert_eq!(cumsum2d.sum(1..3, 2..4), 8);
        assert_eq!(cumsum2d.sum(3..2, 3..4), 0);
        assert_eq!(cumsum2d.sum(1..2, 4..3), 0);
    }
}
