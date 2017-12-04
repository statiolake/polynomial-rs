use std::fmt;
use std::cmp;
use num_traits::{Zero, One};
use std::ops::{Add, Mul, Sub, Div};
use std::ops::{AddAssign, Neg};
use fraction::Fraction;

#[derive(Eq,PartialEq,Clone,Debug)]
pub struct Polynomial1<T> {
    coeffs: Vec<T>
}

impl<T: Zero> Polynomial1<T> {
    fn correct(mut self) -> Self {
        if self.coeffs.is_empty() {
            self.coeffs.push(T::zero());
        }
        loop {
            if self.coeffs.len() > 1 && self.coeffs.last().unwrap().is_zero() {
                self.coeffs.pop();
            } else {
                break;
            }
        }
        self
    }
}

impl<T: Zero> Polynomial1<T> {
    pub fn new(mut coeffs: Vec<T>) -> Self {
        coeffs.reverse();
        Polynomial1::<T>::new_impl(coeffs)
    }

    fn new_impl(raw_coeffs: Vec<T>) -> Self {
        Self { coeffs: raw_coeffs }.correct()
    }
}

impl<T: Zero + Eq> Zero for Polynomial1<T>
    where Polynomial1<T>: Add<Polynomial1<T>, Output = Polynomial1<T>>
{
    fn zero() -> Self {
        Polynomial1::<T>::new(vec![T::zero()])
    }

    fn is_zero(&self) -> bool {
        self.coeffs.len() == 1 && self.coeffs[0] == T::zero()
    }
}

impl<T: Zero + One + Clone + AddAssign> One for Polynomial1<T> {
    fn one() -> Self {
        Polynomial1::<T>::new(vec![T::one()])
    }
}

impl<LHS: Clone, RHS: Clone> Mul<Polynomial1<RHS>> for Polynomial1<LHS>
    where LHS: Mul<RHS>,
          <LHS as Mul<RHS>>::Output: AddAssign + Zero + Clone,
          // <<LHS as Mul<RHS>>::Output as AddAssign>::Output: AddAssign<<LHS as Mul<RHS>>::Output> + Zero + Clone
{
    type Output = Polynomial1<<LHS as Mul<RHS>>::Output>;
    fn mul(self, rhs: Polynomial1<RHS>) -> Self::Output {
        let mut result = vec![<LHS as Mul<RHS>>::Output::zero(); self.coeffs.len() + rhs.coeffs.len() - 1];
        let l = self.coeffs;
        let r = rhs.coeffs;
        for i in 0..l.len() {
            for j in 0..r.len() {
                result[i + j] += l[i].clone() * r[j].clone();
            }
        }
        Self::Output::new_impl(result)
    }
}

impl<LHS: Zero + Clone, RHS: Zero + Clone> Add<Polynomial1<RHS>> for Polynomial1<LHS>
    where LHS: Add<RHS>,
          <LHS as Add<RHS>>::Output: Zero + Clone
{
    type Output = Polynomial1<<LHS as Add<RHS>>::Output>;
    fn add(self, rhs: Polynomial1<RHS>) -> Self::Output {
        let maxlen = cmp::max(self.coeffs.len(), rhs.coeffs.len());
        let mut result = vec![<LHS as Add<RHS>>::Output::zero(); maxlen];
        let l = self.coeffs;
        let r = rhs.coeffs;
        for i in 0..maxlen {
            result[i] = match (i < l.len(), i < r.len()) {
                (true, true) => l[i].clone() + r[i].clone(),
                (true, false) => l[i].clone() + RHS::zero(),
                (false, true) => LHS::zero() + r[i].clone(),
                (false, false) => unreachable!(""),
            }
        }
        Self::Output::new_impl(result)
    }
}

impl<T: Neg> Neg for Polynomial1<T>
    where <T as Neg>::Output: Zero
{
    type Output = Polynomial1<<T as Neg>::Output>;
    fn neg(self) -> Self::Output {
        let mut result = vec![];
        for i in self.coeffs {
            result.push(-i);
        }
        Self::Output::new_impl(result)
    }
}

impl<LHS: Zero + Clone, RHS: Zero + Clone + Neg> Sub<Polynomial1<RHS>> for Polynomial1<LHS>
    where LHS: Add<<RHS as Neg>::Output>,
          <RHS as Neg>::Output: Zero + Clone,
          <LHS as Add<<RHS as Neg>::Output>>::Output: Zero + Clone
{
    type Output = Polynomial1<<LHS as Add<<RHS as Neg>::Output>>::Output>;
    fn sub(self, rhs: Polynomial1<RHS>) -> Self::Output {
        self + (-rhs)
    }
}

impl <LHS, RHS> Div<Polynomial1<RHS>> for Polynomial1<LHS> {
    type Output = Fraction<Polynomial1<LHS>, Polynomial1<RHS>>;
    fn div(self, rhs: Polynomial1<RHS>) -> Self::Output {
        Fraction::new(self, rhs)
    }
}

impl<T> fmt::Display for Polynomial1<T>
    where T: Neg + Zero + One + fmt::Display + PartialOrd + Clone,
          <T as Neg>::Output: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let reversed = { let mut t = self.coeffs.clone(); t.reverse(); t };
        let mut first = true;
        let n = self.coeffs.len() - 1;
        for (j, c) in reversed.into_iter().enumerate() {
            let i = n - j;
            if first {
                first = false;
                if c < T::zero() {
                    write!(f, "-")?;
                }
            } else {
                if c < T::zero() {
                    write!(f, " - ")?;
                } else {
                    write!(f, " + ")?;
                }
            }
            if c < T::zero() {
                write!(f, "{}", -c)?;
            } else if i != 0 && c == T::one() {
            } else {
                write!(f, "{}", c)?;
            }
            if i == 1 {
                write!(f, "x")?;
            } else if i >= 2 {
                write!(f, "x^{}", i)?;
            }
        }
        Ok(())
    }
}
