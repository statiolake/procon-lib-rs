//! 区間の和を高速に計算する `CumSum`, `CumSum2D` を定義する。
//!
//! # `CumSum`
//!
//! 一次元の区間の和を高速に計算する。生成時に前処理として累積和をとる。
//!
//! ```
//! # use procon_lib::pcl::math::CumSum;
//! # use procon_lib::pcl::traits::math::group::Additive as A;
//! // use crate::pcl::math::group::Additive as A;
//! let cumsum = CumSum::from_array(&[A(5), A(4), A(1), A(3), A(2), A(6)]);
//! assert_eq!(cumsum.sum(0..6).0, 21);
//! # #[cfg(feature = "rust2020")]
//! assert_eq!(cumsum.sum(0..=5).0, 21);
//! assert_eq!(cumsum.sum(..6).0, 21);
//! # #[cfg(feature = "rust2020")]
//! assert_eq!(cumsum.sum(..=5).0, 21);
//! assert_eq!(cumsum.sum(0..).0, 21);
//! assert_eq!(cumsum.sum(..).0, 21);
//! assert_eq!(cumsum.sum(1..2).0, 4);
//! assert_eq!(cumsum.sum(1..5).0, 10);
//! # #[cfg(feature = "rust2020")]
//! assert_eq!(cumsum.sum(1..=1).0, 4);
//! assert_eq!(cumsum.sum(1..0).0, 0);
//! ```
//!
//!
//! # `CumSum2D`
//!
//! 二次元の範囲の和を高速に計算する。生成時に前処理として累積和をとる。
//!
//! ```
//! # use procon_lib::pcl::math::CumSum2D;
//! # use procon_lib::pcl::traits::math::group::Additive as A;
//! // use crate::pcl::math::group::Additive as A;
//! let cumsum2d = CumSum2D::from_matrix(vec![
//!     vec![A(4), A(2), A(3), A(6), A(1)],
//!     vec![A(5), A(5), A(2), A(1), A(4)],
//!     vec![A(1), A(2), A(3), A(2), A(2)],
//!     vec![A(3), A(2), A(1), A(3), A(2)],
//! ]);
//! assert_eq!(cumsum2d.sum(0..2, 3..4).0, 7);
//! assert_eq!(cumsum2d.sum(.., ..).0, 54);
//! assert_eq!(cumsum2d.sum(1..3, 2..4).0, 8);
//! assert_eq!(cumsum2d.sum(3..2, 3..4).0, 0);
//! assert_eq!(cumsum2d.sum(1..2, 4..3).0, 0);
//! ```

use super::super::traits::Group;
use super::super::utils::range;
use super::super::utils::range::RangeBounds;

/// ある数列の、指定された範囲の和を高速に計算する。
///
/// 実際は必ずしも通常の整数と和である必要はなく、群 (`Group`) であれば良い。
pub struct CumSum<T> {
    psum: Vec<T>,
}

#[allow(unknown_lints, renamed_and_removed_lints, len_without_is_empty)]
impl<T: Group + Copy> CumSum<T> {
    /// 与えられた数列の累積和をとり、 `CumSum` を生成する。
    ///
    /// # 計算量
    ///
    /// O(n)
    pub fn from_array<A: AsRef<[T]>>(array: A) -> CumSum<T> {
        let array = array.as_ref();
        let mut psum = vec![T::id(); array.len() + 1];
        for i in 0..array.len() {
            // to support rust2016
            let i = i + 1;
            psum[i] = T::op(psum[i - 1], array[i - 1]);
        }

        #[allow(unknown_lints, renamed_and_removed_lints, redundant_field_names)]
        CumSum { psum: psum }
    }

    /// 指定された範囲内の総和を返す。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        // 最初の配列の長さ
        let orig_len = self.psum.len() - 1;

        let start = range::range_start(&range, 0);
        let end = range::range_end(&range, orig_len);

        if end <= start {
            return T::id();
        }

        T::op(self.psum[end], T::inv(self.psum[start]))
    }

    /// もとの配列の長さを取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn len(&self) -> usize {
        self.psum.len() - 1
    }
}

/// ある二次元数列の、指定された範囲の和を高速に計算する。
///
/// 実際は必ずしも通常の整数と和である必要はなく、群 (`Group`) であれば良い。
pub struct CumSum2D<T> {
    psum: Vec<Vec<T>>,
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
                psum: vec![vec![T::id()]],
            };
        }

        let width = array[0].as_ref().len();
        let mut psum = vec![vec![T::id(); width + 1]; height + 1];

        for i in 0..height {
            // to support rust2016
            let i = i + 1;
            for j in 0..width {
                let j = j + 1;
                assert_eq!(
                    array[i - 1].as_ref().len(),
                    width,
                    "the array's length is differ line by line"
                );

                psum[i][j] = T::op(
                    T::op(
                        T::op(psum[i - 1][j], psum[i][j - 1]),
                        T::inv(psum[i - 1][j - 1]),
                    ),
                    array[i - 1].as_ref()[j - 1],
                )
            }
        }

        #[allow(unknown_lints, renamed_and_removed_lints, redundant_field_names)]
        CumSum2D { psum: psum }
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
        let orig_height = self.psum.len() - 1;
        // safety: self.psum は必ず要素を一つは含む (`vec![vec![0]]` が最小)
        let orig_width = unsafe { self.psum.get_unchecked(0).len() } - 1;

        let ystart = range::range_start(&yrange, 0);
        let yend = range::range_end(&yrange, orig_height);
        let xstart = range::range_start(&xrange, 0);
        let xend = range::range_end(&xrange, orig_width);

        if yend <= ystart || xend <= xstart {
            return T::id();
        }

        T::op(
            T::op(
                T::op(self.psum[yend][xend], self.psum[ystart][xstart]),
                T::inv(self.psum[ystart][xend]),
            ),
            T::inv(self.psum[yend][xstart]),
        )
    }

    /// もとの行列の長さを取得する。
    ///
    /// 戻り値は (高さ, 幅)
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn size(&self) -> (usize, usize) {
        (self.psum.len() - 1, self.psum[0].len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::traits::math::group::Additive as A;
    use super::*;

    #[test]
    fn check_cumsum() {
        let cumsum = CumSum::from_array(&[A(5), A(4), A(1), A(3), A(2), A(6)]);
        assert_eq!(cumsum.sum(0..6).0, 21);
        #[cfg(feature = "rust2020")]
        assert_eq!(cumsum.sum(0..=5).0, 21);
        assert_eq!(cumsum.sum(..6).0, 21);
        #[cfg(feature = "rust2020")]
        assert_eq!(cumsum.sum(..=5).0, 21);
        assert_eq!(cumsum.sum(0..).0, 21);
        assert_eq!(cumsum.sum(..).0, 21);
        assert_eq!(cumsum.sum(1..2).0, 4);
        assert_eq!(cumsum.sum(1..5).0, 10);
        #[cfg(feature = "rust2020")]
        assert_eq!(cumsum.sum(1..=1).0, 4);
        assert_eq!(cumsum.sum(1..0).0, 0);

        assert_eq!(cumsum.len(), 6);
    }

    #[test]
    fn check_cumsum2d() {
        let cumsum2d = CumSum2D::from_matrix(vec![
            vec![A(4), A(2), A(3), A(6), A(1)],
            vec![A(5), A(5), A(2), A(1), A(4)],
            vec![A(1), A(2), A(3), A(2), A(2)],
            vec![A(3), A(2), A(1), A(3), A(2)],
        ]);
        assert_eq!(cumsum2d.sum(0..2, 3..4).0, 7);
        assert_eq!(cumsum2d.sum(.., ..).0, 54);
        assert_eq!(cumsum2d.sum(1..3, 2..4).0, 8);
        assert_eq!(cumsum2d.sum(3..2, 3..4).0, 0);
        assert_eq!(cumsum2d.sum(1..2, 4..3).0, 0);

        assert_eq!(cumsum2d.size(), (4, 5));
    }
}
