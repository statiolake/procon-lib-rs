//! ライブラリ自体の作成を簡単にするマクロを定義する。
//!
//! `crate::macros` とは異なり、実際の競プロで利用するよりはライブラリ作成時に利用するものが中心で、
//! よりメタなマクロを集めたものと言える。

/// 型の名前を文字列化するマクロ。
///
/// 基本的には `stringify!` と同様だが、その型が実在することを確かめる。`debug_struct(...)` などで型
/// 名が必要なことがあるが、将来のリファクタリングで型名が変更になってもコンパイルエラーとして気付け
/// るようにすることが目的。
///
/// ```rust
/// # use procon_lib::type_name_of;
/// struct SomeType<T>(T);
/// assert_eq!(type_name_of!(SomeType<T>), "SomeType");
/// ```
///
/// 次のように存在しない名前を与えるとコンパイルエラーとなる。
///
/// ```compile_fail
/// # use procon_lib::type_name_of;
/// assert_eq!(type_name_of!(UnknownType), "UnknownType");
/// ```
#[macro_export]
macro_rules! type_name_of {
    ($ty:ident) => {{
        let _: $ty;
        stringify!($ty)
    }};
    ($ty:ident<$($typevar:ident),+>) => {{
        fn _foo<$($typevar),*>(_: $ty<$($typevar),*>) {
            let _: $ty<$($typevar),*>;
        }
        stringify!($ty)
    }};
}

/// メンバ変数の名前を文字列化するマクロ。
///
/// 基本的には `stringify!` と同様だが、その変数が実在することを確かめる。`debug_struct(...)
/// .field(...)` などでメンバ名が必要なことがあるが、将来のリファクタリングでメンバ名が変更になって
/// もコンパイルエラーとして気付けるようにすることが目的。
///
/// ```rust
/// # use procon_lib::member_name_of;
/// struct SomeType<T> {
///     member: T,
/// };
/// assert_eq!(member_name_of!(SomeType<T>::member), "member");
/// ```
///
/// 次のように存在しない名前を与えるとコンパイルエラーとなる。
///
/// ```compile_fail
/// # use procon_lib::member_name_of;
/// struct SomeType<T> {
///     member: T,
/// };
/// assert_eq!(member_name_of!(SomeType<T>::unknown_member), "unknown_member");
/// ```
///
/// ここまでの例のようにまだインスタンスが存在しないような状況でも使えるように用意されているが、すで
/// にインスタンスが存在する場合はそれを利用して簡単に書くこともできる。
///
/// ```rust
/// # use procon_lib::member_name_of;
/// struct SomeType<T> {
///     member: T,
/// }
/// let val = SomeType { member: 3 };
/// assert_eq!(member_name_of!(val.member), "member");
/// ```
#[macro_export]
macro_rules! member_name_of {
    ($ty:ident::$member:ident) => {{
        fn _foo($member: $ty) {
            let _ = $member.$member;
        }
        stringify!($member)
    }};
    ($ty:ident<$($typevar:ident),+>::$member:ident) => {{
        fn _foo<$($typevar),*>($member: $ty<$($typevar),*>) {
            let _ = $member.$member;
        }
        stringify!($member)
    }};
    ($self:ident.$member:ident) => {{
        let _ = $self.$member;
        stringify!($member)
    }}
}
