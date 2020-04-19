//! 各種アルゴリズムの数学的対象を定義する。
//!
//! 例えば、累積和は通常の (ℤ,+) でなくても群であればよい。そのために `Group` というトレイトを準備し
//! て群を定義する。

/// 群
///
/// M が群であるとは、M が次の条件を満たす集合であることをいう。
///
/// - 演算
///
///     演算 op(M, M) -> M が定義されている。
///
/// - 単位元の存在
///
///     M にある元 1 が存在して op(1, x) = op(x, 1) = x 。
///
/// - 逆元の存在
///
///     任意の M の元 x に対して inv(x) が存在して op(x, inv(x)) = x 。
///
/// - 結合律が成立
///
///     任意の M の元 x, y, z に対して op(op(x, y), z) = op(x, op(y, z)) 。
pub trait Group {
    /// 演算
    fn op(x: Self, y: Self) -> Self;

    /// 単位元
    fn id() -> Self;

    /// 逆元
    fn inv(x: Self) -> Self;
}

macro_rules! impl_group_for_primitives_using_add {
    ($($ty:ty)*) => {
        $(
        impl Group for $ty {
            fn op(x: Self, y: Self) -> Self {
                x + y
            }

            fn id() -> Self {
                <$ty as Zero>::zero()
            }

            fn inv(x: Self) -> Self {
                -x
            }
        }
        )*
    };
}

// マクロ内で #[cfg(feature = ...)] を使ってしまうとそれは除去することができない
// （マクロを展開する前の状態でパースするので）ため、バージョンで分けたい場合は
// このように外で use しておく必要がある。
#[cfg(feature = "rust2020")]
use num::Zero;
#[cfg(feature = "rust2016")]
use pcl::polyfill::num::Zero;

impl_group_for_primitives_using_add! { i8 i16 i32 i64 isize f32 f64 }

#[cfg(feature = "rust2020")]
impl_group_for_primitives_using_add! { i128 }
