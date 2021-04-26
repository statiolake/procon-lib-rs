//! グラフに関連するトレイトや共通部品を定義する。

use crate::pcl::compat::num::One;
use crate::{member_name_of, type_name_of};
use std::fmt;
use std::hash;

/// グラフの辺を表す。
pub struct Edge<C> {
    pub from: usize,
    pub to: usize,
    pub cost: C,
}

impl<C: fmt::Debug> fmt::Debug for Edge<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name_of!(Edge<C>))
            .field(member_name_of!(self.from), &self.from)
            .field(member_name_of!(self.to), &self.to)
            .field(member_name_of!(self.cost), &self.cost)
            .finish()
    }
}

impl<C: Clone> Clone for Edge<C> {
    fn clone(&self) -> Self {
        Edge {
            from: self.from,
            to: self.to,
            cost: self.cost.clone(),
        }
    }
}

impl<C: Copy> Copy for Edge<C> {}

impl<C: PartialEq> PartialEq for Edge<C> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.cost.eq(&other.cost)
    }
}

impl<C: Eq> Eq for Edge<C> {}

impl<C: hash::Hash> hash::Hash for Edge<C> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        self.cost.hash(state);
    }
}

impl<C> Edge<C> {
    /// 辺を生成する。
    pub fn new(from: usize, to: usize, cost: C) -> Self {
        Self { from, to, cost }
    }

    /// コストはそのままで、`from` と `to` が入れ替わった辺を返す。
    pub fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            cost: self.cost,
        }
    }
}

impl<C: One> Edge<C> {
    /// 重みが 1 であるような辺を生成する。
    pub fn one(from: usize, to: usize) -> Self {
        Edge::new(from, to, C::one())
    }
}

impl<C: One> From<(usize, usize)> for Edge<C> {
    fn from((from, to): (usize, usize)) -> Self {
        Edge::one(from, to)
    }
}

impl<C> From<(usize, usize, C)> for Edge<C> {
    fn from((from, to, cost): (usize, usize, C)) -> Self {
        Edge::new(from, to, cost)
    }
}

/// グラフの構造を持つデータであることを示す。
pub trait ReadonlyGraph {
    type Cost;

    /// 頂点数を取得する。
    fn size(&self) -> usize;
}

/// 生成したり辺を追加・削除したりできるグラフであることを示す。
pub trait Graph: ReadonlyGraph {
    /// 指定された頂点数で辺のないグラフを生成する。
    fn of_size(n: usize) -> Self;

    /// 辺を追加する。
    fn add_edge<E: Into<Edge<Self::Cost>>>(&mut self, edge: E);

    /// 指定された点を始点・終点とする辺をすべて削除する。
    fn remove_edge(&mut self, from: usize, to: usize);

    /// 指定された辺と全く同一の辺を削除する。
    fn remove_edge_exact<E: Into<Edge<Self::Cost>>>(&mut self, edge: E)
    where
        Self::Cost: Eq;

    /// 複数の辺を同時に追加する。
    fn add_edges<E, I>(&mut self, edges: I)
    where
        E: Into<Edge<Self::Cost>>,
        I: IntoIterator<Item = E>,
    {
        for edge in edges {
            self.add_edge(edge);
        }
    }
}

/// 無向グラフであることを示す。
pub trait Undirected: ReadonlyGraph {}

/// ある頂点から隣接する頂点のリストを取得できることを示す。
pub trait ProvideAdjacencies: ReadonlyGraph {
    fn get_adjacencies(&self, idx: usize) -> Option<&[Edge<Self::Cost>]>;
}
