use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

pub trait Numeric:
    Clone
    + FromStr
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialEq
{
}

impl<T> Numeric for T where
    T: Clone
        + FromStr
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + PartialEq
{
}
