//! 与えられた配列の中に、同じ要素が何個ずつあるかをカウントする。
//!
//! 内部的には単に HashMap を利用している。
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "rust2016")]
//! # extern crate procon_lib;
//! # use procon_lib::pcl::collections::counter::Counter;
//! #
//! # fn main() {
//! let slice = &[1, 2, 5, 3, 2, 1];
//! let counter = Counter::from_slice(slice);
//! assert_eq!(counter[&1], 2);
//! assert_eq!(counter[&2], 2);
//! assert_eq!(counter[&3], 1);
//! assert_eq!(counter[&5], 1);
//! assert_eq!(counter[&9], 0);
//! # }
//! ```

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, Index};

/// 与えられた配列の中に特定の要素が何個あるかをカウントするコレクション。
pub struct Counter<T> {
    inner: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Deref,
    <T as Deref>::Target: Eq + Hash,
{
    /// スライスから Counter を生成する。得られる Counter は deref されたものになる。これは
    /// &[&String] から &str のカウンタを作るときなどに便利。
    pub fn from_slice_deref(slice: &[T]) -> Counter<&<T as Deref>::Target> {
        slice.iter().map(|x| &**x).collect()
    }
}

impl<T: Eq + Hash> Counter<T> {
    /// スライスから Counter を生成する。
    pub fn from_slice(slice: &[T]) -> Counter<&T> {
        slice.iter().collect()
    }

    /// 特定の要素の個数を取得する。
    pub fn get<Q: ?Sized>(&self, index: &Q) -> &usize
    where
        Q: Eq + Hash,
        T: Borrow<Q>,
    {
        static ZERO: usize = 0;
        self.inner.get(index).unwrap_or(&ZERO)
    }
}

impl<T: Eq + Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Counter<T> {
        let mut inner = HashMap::new();
        for item in iter {
            *inner.entry(item).or_insert(0) += 1;
        }

        #[allow(unknown_lints, renamed_and_removed_lints, redundant_field_names)]
        Counter { inner: inner }
    }
}

impl<'a, T: Eq + Hash, Q: ?Sized> Index<&'a Q> for Counter<T>
where
    Q: Eq + Hash,
    T: Borrow<Q>,
{
    type Output = usize;

    fn index(&self, index: &Q) -> &usize {
        self.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_slice() {
        let v = vec![1, 2, 3, 3, 6, 4, 5, 2];
        let c = Counter::from_slice(&v);

        assert_eq!(c.get(&1), &1);
        assert_eq!(c.get(&2), &2);
        assert_eq!(c[&3], 2);
        assert_eq!(c[&4], 1);
        assert_eq!(c[&5], 1);
        assert_eq!(c[&6], 1);
        assert_eq!(c[&7], 0);
        assert_eq!(c.get(&8), &0);
        assert_eq!(c.get(&9), &0);
    }

    #[test]
    fn counter_iter() {
        let v = vec![1, 2, 3, 3, 6, 4, 5, 2];
        let c = Counter::from_iter(v);

        assert_eq!(c.get(&1), &1);
        assert_eq!(c.get(&2), &2);
        assert_eq!(c[&3], 2);
        assert_eq!(c[&4], 1);
        assert_eq!(c[&5], 1);
        assert_eq!(c[&6], 1);
        assert_eq!(c[&7], 0);
        assert_eq!(c.get(&8), &0);
        assert_eq!(c.get(&9), &0);
    }

    #[test]
    fn non_integer_slice_deref() {
        let v = vec![
            "hello".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        let c = Counter::from_slice_deref(&v);

        assert_eq!(c.get("hello"), &2);
        assert_eq!(c["hello"], 2);
        assert_eq!(c[&*"world".to_string()], 1);
        assert_eq!(c[&*"rust".to_string()], 0);
    }

    #[test]
    fn non_integer_iter() {
        let v = vec![
            "hello".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        let c = Counter::from_iter(v);

        assert_eq!(c.get("hello"), &2);
        assert_eq!(c["hello"], 2);
        assert_eq!(c[&"world".to_string()], 1);
        assert_eq!(c[&"rust".to_string()], 0);
    }
}
