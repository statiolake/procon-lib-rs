//! セグメント木 `SegmentTree` を定義する。
//!
//! セグメント木は、配列に対して次の操作が高速に行えるデータ構造である。
//!
//! - ある 1 要素の値の更新 (`update`) ― O(log n)
//! - ある区間の値にそのモノイドの演算を適用した結果を返す (`query`) ― O(log n)
//!     - 例えば Additive であれば区間の和、 Min であれば Range Minimum Query 。
//!
//! # Examples
//!
//! ```
//! # use procon_lib::pcl::structure::segment_tree::SegmentTree;
//! # use procon_lib::pcl::traits::math::monoid::Min;
//! // use crate::pcl::traits::math::monoid::Min;
//! let mut st = SegmentTree::from_array(vec![Min((1i64 << 31) - 1); 3]);
//! st.update(0, Min(1));
//! st.update(1, Min(2));
//! st.update(2, Min(3));
//! println!("{:?}", st);
//! assert_eq!(st.query(0..3).0, 1);
//! assert_eq!(st.query(1..3).0, 2);
//! ```

use crate::pcl::traits::math::Monoid;
use crate::pcl::utils::range;
use std::fmt;
use std::ops::RangeBounds;

/// セグメント木。
pub struct SegmentTree<T> {
    data: Vec<T>,
    lenexp2: usize,
    len: usize,
}

impl<T: fmt::Debug> fmt::Debug for SegmentTree<T> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        f.debug_struct("SegmentTree")
            .field("data", &self.data)
            .finish()
    }
}

impl<T> SegmentTree<T>
where
    T: Monoid + Copy,
{
    /// 初期値を持つ配列からセグメント木を生成する。
    pub fn from_array<A: AsRef<[T]>>(arr: A) -> SegmentTree<T> {
        let arr = arr.as_ref();
        let len = arr.len();
        let lenexp2 = calc_lenexp2(len);
        let data = {
            let mut v = vec![T::id(); lenexp2 * 2];
            v[lenexp2..(lenexp2 + len)].copy_from_slice(arr);
            v
        };

        SegmentTree { data, lenexp2, len }
    }

    /// あるインデックス `idx` の値を `value` に更新する。
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn update(&mut self, mut idx: usize, value: T) {
        assert!(idx <= self.len);
        idx += self.lenexp2;
        self.data[idx] = value;

        loop {
            idx >>= 1;
            if idx == 0 {
                break;
            }
            self.data[idx] = T::op(self.data[idx * 2], self.data[idx * 2 + 1]);
        }
    }

    /// ある区間 `range` の各要素に順に演算を適用して、結果を返す。
    ///
    /// たとえばモノイド `Min` であれば、ある区間の最小値を返す。 (Range Minimum
    /// Query)
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> T {
        let mut start = range::range_start(&range, 0);
        let mut end = range::range_end(&range, self.len);
        start += self.lenexp2;
        end += self.lenexp2;

        let mut res1 = T::id();
        let mut res2 = T::id();

        while start < end {
            if start & 1 != 0 {
                res1 = T::op(res1, self.data[start]);
                start += 1;
            }

            if end & 1 != 0 {
                end -= 1;
                res2 = T::op(self.data[end], res2);
            }

            start >>= 1;
            end >>= 1;
        }

        T::op(res1, res2)
    }
}

/// 2 の冪乗であって最初に len 以上になるような値を求める。
///
/// すなわち、 2^m >= len となるような最小の 2^m の値を求める。
fn calc_lenexp2(mut len: usize) -> usize {
    len -= 1;
    len |= len >> 1;
    len |= len >> 2;
    len |= len >> 4;
    len |= len >> 8;
    len |= len >> 16;
    len |= len >> 32;

    len + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcl::traits::math::monoid::Min;

    #[test]
    fn segment_tree_1() {
        let mut st = SegmentTree::from_array(vec![Min((1i64 << 31) - 1); 3]);
        st.update(0, Min(1));
        st.update(1, Min(2));
        st.update(2, Min(3));
        println!("{:?}", st);
        assert_eq!(st.query(0..3).0, 1);
        assert_eq!(st.query(0..=2).0, 1);
        assert_eq!(st.query(1..3).0, 2);
    }

    #[test]
    fn segment_tree_2() {
        let mut st = SegmentTree::from_array(vec![Min((1i64 << 31) - 1); 1]);
        assert_eq!(st.query(0..1).0, (1i64 << 31) - 1);
        st.update(0, Min(5));
        assert_eq!(st.query(0..1).0, 5);
    }
}
