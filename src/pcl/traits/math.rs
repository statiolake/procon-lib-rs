use num::Zero;
use std::ops::{Add, Neg, Sub};

pub trait Group: Add<Output = Self> + Sub<Output = Self> + Neg + Zero {}

impl<T> Group for T where T: Add<Output = Self> + Sub<Output = Self> + Neg + Zero {}
