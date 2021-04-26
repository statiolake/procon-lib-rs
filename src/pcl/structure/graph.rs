//! 様々な表現のグラフやグラフアルゴリズムを定義する。
//!
//! # Examples
//!
//! グラフの表現には様々なものがあり、それぞれ得意不得意がある。最も基本となるのはおそらく隣接リスト
//! 形式のグラフだろう。いずれにせよ、すべてのグラフは `add_edge()` や `remove_edge()` 関数により辺
//! を追加したり削除したりできるようになっており、内部実装については気にしなくても扱えるように作られ
//! ている。
//!
//! 例えば、`n` 頂点のグラフで辺のリストが `edges` で与えられている場合、このグラフを保持する無向隣
//! 接グラフを生成するには次のようにかけば良い。
//!
//! ```rust
//! # use procon_lib::pcl::structure::graph::AdjacencyList;
//! # use procon_lib::pcl::traits::math::graph::{Edge, Graph};
//! // use crate::pcl::structure::graph::AdjacencyList;
//! // use crate::pcl::traits::math::graph::{Edge, Graph};
//! let n = 9;
//! let edges = vec![
//!     Edge::new(0, 2, 1),
//!     Edge::new(0, 3, 1),
//!     Edge::new(1, 4, 1),
//!     Edge::new(1, 5, 1),
//!     Edge::new(1, 6, 1),
//!     Edge::new(2, 7, 1),
//!     Edge::new(2, 8, 1)
//! ];
//! let mut graph = AdjacencyList::of_size(n);
//! graph.add_edges(edges);
//! ```

use crate::pcl::traits::math::graph::{Edge, Graph, ProvideAdjacencies, ReadonlyGraph, Undirected};
use crate::{member_name_of, type_name_of};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;
use std::iter;

/// 辺をリストとして所持するタイプのグラフ。
pub struct EdgeList<C> {
    size: usize,
    edges: Vec<Edge<C>>,
}

impl<C> fmt::Debug for EdgeList<C>
where
    C: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name_of!(EdgeList<C>))
            .field(member_name_of!(self.size), &self.size)
            .field(member_name_of!(self.edges), &self.edges)
            .finish()
    }
}

impl<C: Clone> Clone for EdgeList<C> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            edges: self.edges.clone(),
        }
    }
}

impl<C> ReadonlyGraph for EdgeList<C> {
    type Cost = C;

    fn size(&self) -> usize {
        self.size
    }
}

impl<C> Graph for EdgeList<C> {
    fn of_size(n: usize) -> Self {
        Self {
            size: n,
            edges: vec![],
        }
    }

    fn add_edge<E: Into<Edge<C>>>(&mut self, edge: E) {
        self.edges.push(edge.into());
    }

    fn remove_edge_exact<E: Into<Edge<C>>>(&mut self, edge: E)
    where
        C: Eq,
    {
        let edge = edge.into();
        self.edges.retain(|e| e != &edge);
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        self.edges.retain(|e| e.from != from || e.to != to);
    }
}

impl<C> EdgeList<C> {
    /// すべての辺のリストを得る。
    pub fn edges(&self) -> &[Edge<C>] {
        &self.edges
    }
}

/// 隣接リスト形式のグラフ。
pub struct AdjacencyList<C> {
    size: usize,
    adjacencies: Vec<Vec<Edge<C>>>,
}

impl<C> fmt::Debug for AdjacencyList<C>
where
    C: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name_of!(AdjacencyList<C>))
            .field(member_name_of!(self.size), &self.size)
            .field(member_name_of!(self.adjacencies), &self.adjacencies)
            .finish()
    }
}

impl<C: Clone> Clone for AdjacencyList<C> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            adjacencies: self.adjacencies.clone(),
        }
    }
}

impl<C> ProvideAdjacencies for AdjacencyList<C> {
    fn get_adjacencies(&self, idx: usize) -> Option<&[Edge<C>]> {
        self.adjacencies.get(idx).map(|x| &**x)
    }
}

impl<C> ReadonlyGraph for AdjacencyList<C> {
    type Cost = C;

    fn size(&self) -> usize {
        self.size
    }
}

impl<C> Graph for AdjacencyList<C> {
    fn of_size(n: usize) -> Self {
        AdjacencyList {
            size: n,
            adjacencies: iter::from_fn(|| Some(Vec::new())).take(n).collect(),
        }
    }

    fn add_edge<E: Into<Edge<C>>>(&mut self, edge: E) {
        let edge = edge.into();
        self.adjacencies[edge.from].push(edge);
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        self.adjacencies[from].retain(|e| e.to != to);
    }

    fn remove_edge_exact<E: Into<Edge<C>>>(&mut self, edge: E)
    where
        C: Eq,
    {
        let edge = edge.into();
        self.adjacencies[edge.from].retain(|e| e != &edge);
    }
}

impl<C> From<EdgeList<C>> for AdjacencyList<C> {
    fn from(edge_list: EdgeList<C>) -> AdjacencyList<C> {
        let mut graph = AdjacencyList::of_size(edge_list.size());
        graph.add_edges(edge_list.edges);
        graph
    }
}

/// 隣接リスト形式の無向グラフ。
pub struct UndirectedAdjacencyList<C>(AdjacencyList<C>);

impl<C: fmt::Debug> fmt::Debug for UndirectedAdjacencyList<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name_of!(UndirectedAdjacencyList<C>))
            .field(member_name_of!(AdjacencyList<C>::size), &self.0.size)
            .field(
                member_name_of!(AdjacencyList<C>::adjacencies),
                &self.0.adjacencies,
            )
            .finish()
    }
}

impl<C: Clone> Clone for UndirectedAdjacencyList<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C> ReadonlyGraph for UndirectedAdjacencyList<C> {
    type Cost = C;

    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<C> Graph for UndirectedAdjacencyList<C>
where
    C: Clone,
{
    fn of_size(n: usize) -> Self {
        Self(AdjacencyList::of_size(n))
    }

    fn add_edge<E: Into<Edge<C>>>(&mut self, edge: E) {
        let edge = edge.into();
        self.0.add_edge(edge.clone());
        self.0.add_edge(edge.reversed());
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        self.0.remove_edge(from, to);
        self.0.remove_edge(to, from);
    }

    fn remove_edge_exact<E: Into<Edge<C>>>(&mut self, edge: E)
    where
        C: Eq,
    {
        let edge = edge.into();
        self.0.remove_edge_exact(edge.clone());
        self.0.remove_edge_exact(edge.reversed());
    }
}

impl<C> Undirected for UndirectedAdjacencyList<C> {}

impl<C> ProvideAdjacencies for UndirectedAdjacencyList<C> {
    fn get_adjacencies(&self, idx: usize) -> Option<&[Edge<C>]> {
        self.0.get_adjacencies(idx)
    }
}

/// ツリー。ここでは無向グラフで連結かつサイクルを持たないものをいう。
///
/// ツリーは構造を保つかどうかをリアルタイムに判断することが難しいため、直接生成することはできない。
/// まずは [`UndirectedAdjacencyList`] でグラフを作り、それが木構造を持っていることを確かめた上で初
/// めて変換することができる。
///
/// ```rust
/// # use procon_lib::pcl::structure::graph::{UndirectedAdjacencyList, Tree};
/// # use procon_lib::pcl::traits::math::graph::Graph;
/// // use crate::pcl::structure::graph::{UndirectedAdjacencyList, Tree};
/// // use crate::pcl::traits::math::graph::Graph;
/// use std::convert::TryFrom;
/// let mut graph = UndirectedAdjacencyList::<i32>::of_size(4);
/// graph.add_edges(vec![
///     (0, 1),
///     (1, 2),
///     (1, 3),
/// ]);
/// let tree = Tree::try_from(graph); // ここで条件を満たしているか確認する。
/// assert!(tree.is_ok());
/// ```
pub struct Tree<C>(UndirectedAdjacencyList<C>);

impl<C: fmt::Debug> fmt::Debug for Tree<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self.0.0.size みたいなのはエラーになるようなので
        let inner = &self.0;
        f.debug_struct(type_name_of!(Tree<C>))
            .field(member_name_of!(AdjacencyList<C>::size), &inner.0.size)
            .field(
                member_name_of!(AdjacencyList<C>::adjacencies),
                &inner.0.adjacencies,
            )
            .finish()
    }
}

impl<C> ReadonlyGraph for Tree<C> {
    type Cost = C;

    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<C> Undirected for Tree<C> {}

impl<C> ProvideAdjacencies for Tree<C> {
    fn get_adjacencies(&self, idx: usize) -> Option<&[Edge<C>]> {
        self.0.get_adjacencies(idx)
    }
}

impl<C> Tree<C> {
    /// 隣接リスト形式の無向グラフから生成する。
    pub fn try_from_graph(graph: UndirectedAdjacencyList<C>) -> Result<Self, TreeTryFromError> {
        is_tree(&graph).map(|_| Self(graph))
    }

    /// 隣接リスト形式の無向グラフからチェックせずに生成する。
    ///
    /// # Safety
    ///
    /// 与えられたグラフは連結で閉路がない。`is_tree(&graph)` が `Ok(_)` である。
    pub unsafe fn from_graph_unchecked(graph: UndirectedAdjacencyList<C>) -> Self {
        Self(graph)
    }
}

#[cfg(feature = "rust-138")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// ツリーに変換できなかった理由を示す。
pub enum TreeTryFromError {
    /// 連結でない。
    NotConnected,

    /// 閉路を持つ。
    HasCycle,

    /// 連結でもなければ閉路も持つ。
    Both,
}

#[cfg(feature = "rust-138")]
impl<C> TryFrom<UndirectedAdjacencyList<C>> for Tree<C> {
    type Error = TreeTryFromError;

    fn try_from(graph: UndirectedAdjacencyList<C>) -> Result<Self, Self::Error> {
        Tree::try_from_graph(graph)
    }
}

/// 与えられた無向グラフが木かどうかを確認する。
///
/// すなわち次の2つの条件を満たすことを確かめる。
/// 1. グラフが連結
/// 2. サイクルがない
pub fn is_tree<G: Undirected + ProvideAdjacencies>(graph: &G) -> Result<(), TreeTryFromError> {
    match (is_connected(graph), has_cycle(graph)) {
        (true, false) => Ok(()),
        (false, false) => Err(TreeTryFromError::NotConnected),
        (true, true) => Err(TreeTryFromError::HasCycle),
        (false, true) => Err(TreeTryFromError::Both),
    }
}

/// 与えられた無向グラフにサイクルがないことを確認する。
pub fn has_cycle<G: Undirected + ProvideAdjacencies>(graph: &G) -> bool {
    // DFS してみつつ、ある頂点から 2 つ以上「訪れたことのある頂点」が見つからないことを確認すればよ
    // い。
    fn dfs<G: Undirected + ProvideAdjacencies>(
        graph: &G,
        current: usize,
        stepped: &mut HashSet<usize>,
    ) -> bool {
        assert!(stepped.insert(current));

        let num_visited = graph
            .get_adjacencies(current)
            .expect("vertex index out of bounds")
            .iter()
            .filter(|edge| stepped.contains(&edge.to))
            .count();
        if num_visited >= 2 {
            return true;
        }

        for edge in graph
            .get_adjacencies(current)
            .expect("vertex index out of bounds")
        {
            if stepped.contains(&edge.to) {
                continue;
            }

            if dfs(graph, edge.to, stepped) {
                return true;
            }
        }

        false
    }

    // グラフが連結でない可能性があるので全頂点を起点に試す必要がある。
    let mut visited = HashSet::new();
    for v in 0..graph.size() {
        if visited.contains(&v) {
            // 途中の DFS で訪れた頂点は調べる必要がない。
            continue;
        }

        if dfs(graph, v, &mut visited) {
            return true;
        }
    }

    false
}

/// 与えられた無向グラフが連結かどうかを確認する。
pub fn is_connected<G: Undirected + ProvideAdjacencies>(graph: &G) -> bool {
    // とりあえず雑に DFS してすべての頂点を訪れられるかどうかを調べれば良い。
    fn dfs<G: Undirected + ProvideAdjacencies>(
        graph: &G,
        current: usize,
        stepped: &mut HashSet<usize>,
    ) {
        assert!(stepped.insert(current));
        for edge in graph
            .get_adjacencies(current)
            .expect("vertex index out of bounds")
        {
            if stepped.contains(&edge.to) {
                continue;
            }
            dfs(graph, edge.to, stepped);
        }
    }

    let mut visited = HashSet::new();
    dfs(graph, 0, &mut visited);
    visited.len() == graph.size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected() {
        let mut graph = UndirectedAdjacencyList::<i32>::of_size(3);
        graph.add_edge((0, 2));
        assert!(!is_connected(&graph));
        graph.add_edge((0, 1));
        assert!(is_connected(&graph));

        let mut graph = UndirectedAdjacencyList::<i32>::of_size(9);
        let edges = [(0, 2), (0, 3), (1, 4), (1, 5), (1, 6), (2, 7), (2, 8)];
        graph.add_edges(edges.iter().copied());
        assert!(!is_connected(&graph));
    }

    #[test]
    fn test_has_cycle() {
        let mut graph = UndirectedAdjacencyList::<i32>::of_size(4);
        graph.add_edge((0, 2));
        graph.add_edge((0, 3));
        graph.add_edge((1, 2));
        assert!(!has_cycle(&graph));
        graph.add_edge((0, 1));
        assert!(has_cycle(&graph));

        let mut graph = UndirectedAdjacencyList::<i32>::of_size(9);
        let edges = [(0, 2), (0, 3), (1, 4), (1, 5), (1, 6), (2, 7), (2, 8)];
        graph.add_edges(edges.iter().copied());
        assert!(!has_cycle(&graph));
    }

    #[test]
    fn test_tree() {
        let mut graph = UndirectedAdjacencyList::<i32>::of_size(9);
        let edges = [(0, 2), (0, 3), (1, 4), (1, 5), (1, 6), (2, 7), (2, 8)];
        graph.add_edges(edges.iter().copied());
        assert!(matches!(
            Tree::try_from(graph.clone()),
            Err(TreeTryFromError::NotConnected),
        ));

        graph.add_edge((0, 1));
        let tree = match Tree::try_from(graph.clone()) {
            Ok(t) => t,
            Err(_) => panic!(),
        };
        assert!(is_tree(&tree).is_ok());

        graph.add_edge((1, 2));
        assert!(matches!(
            Tree::try_from(graph),
            Err(TreeTryFromError::HasCycle),
        ));
    }
}
