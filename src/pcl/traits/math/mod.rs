//! 各種アルゴリズムの数学的対象を定義する。
//!
//! 例えば、累積和は通常の (ℤ,+) でなくても群であればよい。そのために `Group` というトレイトを準備し
//! て群を定義する。

pub mod graph;
pub mod group;
pub mod monoid;

pub use self::graph::{Edge, Graph, ProvideAdjacencies, ReadonlyGraph, Undirected};
pub use self::group::Group;
pub use self::monoid::Monoid;
