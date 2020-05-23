pub use std::cmp::*;
pub use std::collections::*;
pub use std::f64::consts::*;
pub use std::{f32, f64, i16, i32, i64, i8, isize, u16, u32, u64, u8, usize};

#[cfg(feature = "rust2016")]
#[path = "prelude_2016.rs"]
mod prelude_ext;

#[cfg(feature = "rust2020")]
#[path = "prelude_2020.rs"]
mod prelude_ext;

pub use self::prelude_ext::*;
