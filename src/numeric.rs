use num_traits::Num;

pub trait NumericValue: Num + Clone {}
impl<T: Num + Clone> NumericValue for T {}

pub trait BuiltinFn<N: NumericValue> {
    fn call(&self, name: &str, arg: N) -> Option<N>;
}
