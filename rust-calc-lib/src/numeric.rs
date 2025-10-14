use num_traits::{Num, Pow};

pub trait NumericValue: Num + Clone + Pow<Self, Output = Self> {}
impl<T: Num + Clone + Pow<Self, Output = Self>> NumericValue for T {}

pub trait BuiltinFn<N: NumericValue> {
    fn call(&self, name: &str, arg: N) -> Option<N>;
}
