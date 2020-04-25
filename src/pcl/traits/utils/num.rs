//! 数に関するユーティリティトレイトを追加する。

/// その型で扱える最大値を返す関数を定義する。
///
/// この型を実装しているとモノイド `Min` になれる (モノイドの単位元が最小値になるため) 。 Segment
/// Tree と合わせると Range Minimum Query を解ける。
pub trait MaxValue {
    fn max_value() -> Self;
}

/// その型で扱える最小値を返す関数を定義する。
///
/// この型を実装しているとモノイド `Max` になれる (モノイドの単位元が最大値になるため) 。 Segment
/// Tree と合わせると Range Maximum Query を解ける。
pub trait MinValue {
    fn min_value() -> Self;
}

macro_rules! impl_minmax_value_for_primitives {
    ($($ty:tt)*) => {
        $(
        impl MaxValue for $ty {
            fn max_value() -> $ty {
                ::std::$ty::MAX
            }
        }

        impl MinValue for $ty {
            fn min_value() -> $ty {
                ::std::$ty::MIN
            }
        }
        )*
    };
}

impl_minmax_value_for_primitives! {
    u8 u16 u32 u64 usize
    i8 i16 i32 i64 isize
}
