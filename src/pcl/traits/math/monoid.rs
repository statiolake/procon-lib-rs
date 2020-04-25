/// モノイド
///
/// M がモノイドであるとは、M が次の条件を満たす集合であることをいう。
///
/// - 演算  
///     演算 op(M, M) -> M が定義されている。
/// - 単位元の存在  
///     M にある元 1 が存在して op(1, x) = op(x, 1) = x 。
/// - 結合律が成立  
///     任意の M の元 x, y, z に対して op(op(x, y), z) = op(x, op(y, z)) 。
pub trait Monoid {
    /// 演算
    fn op(x: Self, y: Self) -> Self;

    /// 単位元
    fn id() -> Self;
}
