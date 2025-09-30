#![allow(dead_code)]

use num_traits::Num;

pub trait Numeric: Num + Clone {}
impl<T: Num + Clone> Numeric for T {}

pub mod evaluator;
pub mod lexer;
pub mod parser;
