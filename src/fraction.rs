use std::ops::{Add, Mul, Sub, Div};

pub struct Fraction<N, D> {
    num: N,
    denom: D
}

impl<N, D> Fraction<N, D> {
    pub fn new(num: N, denom: D) -> Self {
        Self { num, denom }
    }
}
