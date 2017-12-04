use std::cmp;
use num_traits::{Zero, One};
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Eq,PartialEq,Clone,Debug)]
pub struct Polynomial1<T> {
    coeffs: Vec<T>
}

impl<T: Zero> Polynomial1<T> {
    pub fn new(mut coeffs: Vec<T>) -> Self {
        if coeffs.is_empty() {
            coeffs.push(T::zero());
        }
        Self { coeffs }
    }
}

impl<T: Zero + Eq> Zero for Polynomial1<T>
    where Polynomial1<T>: Add<Polynomial1<T>, Output = Polynomial1<T>>
{
    fn zero() -> Self {
        Polynomial1::<T>::new(vec![])
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
        Self::Output::new(result)
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
        Self::Output::new(result)
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
        Self::Output::new(result)
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
