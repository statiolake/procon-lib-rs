//! 一部の面倒を解消するマクロ類を定義する。
//!
//! Rust の安全志向な言語仕様には競プロに向かない設計もある。それらに多少の答えを与えるために用意さ
//! れたマクロ類を集める。
//!
//! 実際のドキュメントはクレートのルートに配置されている。
//!
//! - [`rtl!`](../../macro.rtl.html) ― 複合代入演算子を右辺から評価するマクロ。

/// 複合代入演算子を右辺から評価するマクロ。
///
/// Rust では一部のケースで複合代入演算子がプリミティブ型とユーザー定義型との間で異なる挙動をするケ
/// ースがある。
///
/// 例えば `Vec<i32>` について、次のような複合代入は可能である。
/// ```
/// let mut v = vec![1, 2];
/// v[0] += v[1];
/// assert_eq!(v[0], 3);
/// ```
///
/// 一方、これが `i32` ではなくユーザー定義型 (AddAssign が定義されている型) になると、挙動が異なる。
/// ```compile_fail
/// use std::num::Wrapping;
/// let mut v = vec![Wrapping(1), Wrapping(2)];
/// v[0] += v[1]; // ERROR: cannot borrow `v` as immutable because it is also borrowed as mutable
/// assert_eq!(v[0], Wrapping(3));
/// ```
///
/// これはプリミティブ型 `i32` に対する `+=` は想像通り「先に右辺を評価してからその値を左辺の変数に
/// 足す」という挙動をするのに対し、 `Wrapping<i32>` に対する `+=` は演算子オーバーロードの解決によ
/// って `AddAssign::add_assign(&mut v[0], &v[1]);` へと変換されるため、通常の借用のルールによってエ
/// ラーになる。
///
/// このマクロは、この状況を解消するため、プリミティブではない型であっても複合代入演算子を右から先に
/// 評価するようにするものである。
/// ```
/// # #[macro_use]
/// # extern crate procon_lib;
/// # use procon_lib::rtl;
/// # fn main() {
/// use std::num::Wrapping;
/// let mut v = vec![Wrapping(1), Wrapping(2)];
/// rtl!(v[0] += v[1]);
/// assert_eq!(v[0], Wrapping(3));
/// # }
/// ```
#[macro_export]
macro_rules! rtl {
    (@lhs ($($lhs:tt)*) @rest += $($rest:tt)*) => {
        rtl!(@op += @lhs ($($lhs)*) @rhs () @rest $($rest)*)
    };
    (@lhs ($($lhs:tt)*) @rest -= $($rest:tt)*) => {
        rtl!(@op -= @lhs ($($lhs)*) @rhs () @rest $($rest)*)
    };
    (@lhs ($($lhs:tt)*) @rest *= $($rest:tt)*) => {
        rtl!(@op *= @lhs ($($lhs)*) @rhs () @rest $($rest)*)
    };
    (@lhs ($($lhs:tt)*) @rest /= $($rest:tt)*) => {
        rtl!(@op /= @lhs ($($lhs)*) @rhs () @rest $($rest)*)
    };
    (@lhs ($($lhs:tt)*) @rest %= $($rest:tt)*) => {
        rtl!(@op %= @lhs ($($lhs)*) @rhs () @rest $($rest)*)
    };
    (@lhs ($($lhs:tt)*) @rest $head:tt $($rest:tt)*) => {
        rtl!(@lhs ($($lhs)* $head) @rest $($rest)*);
    };
    (@op $op:tt @lhs ($($lhs:tt)*) @rhs ($($rhs:tt)*) @rest) => {
        let __rhs = $($rhs)*;
        $($lhs)* $op __rhs;
    };
    (@op $op:tt @lhs ($($lhs:tt)*) @rhs ($($rhs:tt)*) @rest $head:tt $($rest:tt)*) => {
        rtl!(@op $op @lhs ($($lhs)*) @rhs ($($rhs)* $head) @rest $($rest)*);
    };
    ($($rest:tt)*) => {
        rtl!(@lhs () @rest $($rest)*)
    };
}
