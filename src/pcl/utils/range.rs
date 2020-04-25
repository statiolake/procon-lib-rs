//! 範囲に関する処理を共通化する。
//!
//! rust2016 では `std::ops::RangeBounds` は利用できないので同様の
//! `pcl::polyfill::std::ops::RangeBounds` を用意したが、それでも `use` のたびに `#[cfg(feature)]`を
//! 付ける必要があって面倒くさい。ここでは `RangeBounds` と `Bound` をバージョンに合わせて re-export
//! しているので、通常 `RangeBounds` を利用したい場合は `pcl::utils::range::RangeBounds` を利用すれ
//! ばよい。また、様々な RangeBounds から実際の始点・終点を取るのはいちいち inclusive / exclusive /
//! bounded / unbounded などの観点で分岐する必要があって面倒くさいので、実際の半開区間の始点と終点を
//! 求めるヘルパー関数 `range_start` と `range_end`を追加する。

use std::cmp;

#[cfg(feature = "rust2016")]
pub use pcl::polyfill::std::ops::{Bound, RangeBounds};
#[cfg(feature = "rust2020")]
pub use std::ops::{Bound, RangeBounds};

/// 範囲から始点を得る関数。範囲はこのインデックスを "含む" (半開区間) 。
///
/// `..` や `..b` のように始点が不明な範囲の場合は `min` を返す。もし始点が `min` より小さいようであ
/// れば `min` を返すので、たとえば `min = 0` とすればその後の配列の境界チェックは不要である。
pub fn range_start<R: RangeBounds<usize>>(range: &R, min: usize) -> usize {
    let start = match range.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
        Bound::Unbounded => 0,
    };

    cmp::max(start, min)
}

/// 範囲から終点を得る関数。範囲はこのインデックスを "含まない" (半開区間) 。
///
/// `..` や `a..` のように終点が不明な範囲の場合は `max` を返す。もし終点が `max` より大きいようであ
/// れば `max` を返すので、たとえば `max = len` とすればその後の配列の境界チェックは不要である。当然
/// 、 `0..=usize::MAX` のような範囲についてこれを呼び出すと、半開区間にするために 1 を足す段階で終
/// 点がオーバーフローしてしまうので注意すること。実際には要素数などが usize::MAX になることはないの
/// で大丈夫だと思われるが...。
pub fn range_end<R: RangeBounds<usize>>(range: &R, max: usize) -> usize {
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
        #[cfg(feature = "rust2020")]
        assert_eq!(range_start(&(0..=1), 0), 0);
        assert_eq!(range_start(&(..1), 0), 0);
        assert_eq!(range_start(&(0..), 0), 0);
        assert_eq!(range_start(&(..), 0), 0);
        assert_eq!(range_end(&(0..1), 1), 1);
        #[cfg(feature = "rust2020")]
        assert_eq!(range_end(&(0..=1), 1), 1);
        assert_eq!(range_end(&(..1), 1), 1);
        assert_eq!(range_end(&(0..), 1), 1);
        assert_eq!(range_end(&(..), 1), 1);
    }
}
