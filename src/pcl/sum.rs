use crate::pcl::traits::Group;
use std::cmp;
use std::ops::{Bound, RangeBounds};

/// ある数列の、指定された範囲の和を高速に計算します。
pub struct Line<T> {
    sum: Vec<T>,
}

impl<T: Group + Copy> Line<T> {
    /// 与えられたスライスの累積和をとり、 `Line` を生成します。
    pub fn from_slice(slice: &[T]) -> Line<T> {
        let mut sum = vec![T::zero(); slice.len() + 1];
        for i in 1..=slice.len() {
            unsafe {
                *sum.get_unchecked_mut(i) = *sum.get_unchecked(i - 1) + *slice.get_unchecked(i - 1);
            }
        }

        Line { sum }
    }

    /// 指定された範囲内の総和を返します。
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
    fn check_line() {
        let line = Line::from_slice(&[5, 4, 1, 3, 2, 6]);
        assert_eq!(line.sum(0..6), 21);
        assert_eq!(line.sum(0..=5), 21);
        assert_eq!(line.sum(..6), 21);
        assert_eq!(line.sum(..=5), 21);
        assert_eq!(line.sum(0..), 21);
        assert_eq!(line.sum(..), 21);
        assert_eq!(line.sum(1..2), 4);
        assert_eq!(line.sum(1..5), 10);
        assert_eq!(line.sum(1..=1), 4);
        assert_eq!(line.sum(1..0), 0);
    }
}
