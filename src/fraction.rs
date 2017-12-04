use std::fmt;
use std::ops::{Add, Mul, Sub, Div};
use std::ops::{Neg};
use num_traits::{Zero, One};

#[derive(Eq,PartialEq,Clone,Debug)]
pub struct Fraction<N, D> {
    num: N,
    denom: D
}

impl<N: Zero + PartialEq, D: One> Zero for Fraction<N, D>
    where Fraction<N, D>: Add<Fraction<N, D>, Output=Fraction<N, D>>
{
    fn zero() -> Self {
        Self { num: <N as Zero>::zero(), denom: <D as One>::one() }
    }

    fn is_zero(&self) -> bool {
        self.num == <N as Zero>::zero()
    }
}

impl<N: One, D: One> One for Fraction<N, D> {
    fn one() -> Self {
        Self { num: <N as One>::one(), denom: <D as One>::one() }
    }
}

impl<N: One> From<N> for Fraction<N, N> {
    fn from(num: N) -> Self {
        Self { num, denom: N::one() }
    }
}

impl<N, D> Fraction<N, D> {
    pub fn new(num: N, denom: D) -> Self {
        Self { num, denom }
    }
}

impl<LHSN: Clone, LHSD: Clone, RHSN: Clone, RHSD: Clone> Add<Fraction<RHSN, RHSD>> for Fraction<LHSN, LHSD>
    where LHSN: Mul<RHSD>,
          RHSN: Mul<LHSD>,
          <LHSN as Mul<RHSD>>::Output: Add<<RHSN as Mul<LHSD>>::Output>,
          LHSD: Mul<RHSD>,
{
    type Output = Fraction<<<LHSN as Mul<RHSD>>::Output as Add<<RHSN as Mul<LHSD>>::Output>>::Output, <LHSD as Mul<RHSD>>::Output>;
    fn add(self, rhs: Fraction<RHSN, RHSD>) -> Self::Output {
        let num = self.num.clone() * rhs.denom.clone() + rhs.num.clone() * self.denom.clone();
        let denom = self.denom.clone() * rhs.denom.clone();
        Self::Output::new(num, denom)
    }
}

impl<N: Neg, D> Neg for Fraction<N, D> {
    type Output = Fraction<<N as Neg>::Output, D>;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.num, self.denom)
    }
}

impl<LHSN, LHSD, RHSN: Neg, RHSD> Sub<Fraction<RHSN, RHSD>> for Fraction<LHSN, LHSD>
    where Fraction<RHSN, RHSD>: Neg,
          Fraction<LHSN, LHSD>: Add<<Fraction<RHSN, RHSD> as Neg>::Output>
{
    type Output = <Fraction<LHSN, LHSD> as Add<<Fraction<RHSN, RHSD> as Neg>::Output>>::Output;
    fn sub(self, rhs: Fraction<RHSN, RHSD>) -> Self::Output {
        self + (-rhs)
    }
}

impl<LHSN, LHSD, RHSN, RHSD> Mul<Fraction<RHSN, RHSD>> for Fraction<LHSN, LHSD>
    where LHSN: Mul<RHSN>,
          LHSD: Mul<RHSD>
{
    type Output = Fraction<<LHSN as Mul<RHSN>>::Output, <LHSD as Mul<RHSD>>::Output>;
    fn mul(self, rhs: Fraction<RHSN, RHSD>) -> Self::Output {
        Self::Output::new(self.num * rhs.num, self.denom * rhs.denom)
    }
}

impl<LHSN, LHSD, RHSN, RHSD> Div<Fraction<RHSN, RHSD>> for Fraction<LHSN, LHSD>
    where Fraction<LHSN, LHSD>: Mul<Fraction<RHSD, RHSN>>
{
    type Output = <Fraction<LHSN, LHSD> as Mul<Fraction<RHSD, RHSN>>>::Output;
    fn div(self, rhs: Fraction<RHSN, RHSD>) -> Self::Output {
        let divider = Fraction::new(rhs.denom, rhs.num);
        self * divider
    }
}

impl<N: fmt::Display, D: fmt::Display + PartialEq + One> fmt::Display for Fraction<N, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.denom == D::one() {
            write!(f, "{}", self.num)?;
        } else {
            write!(f, "({})/({})", self.num, self.denom)?;
        }
        Ok(())
    }
}
