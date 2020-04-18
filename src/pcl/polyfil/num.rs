pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}
