extern crate num_traits;

pub mod polynomial_1;
pub mod fraction;

#[cfg(test)]
mod tests {
    use polynomial_1::Polynomial1;
    #[test]
    fn add() {
        let eq1 = Polynomial1::<i32>::new(vec![3, 1]); // 3x + 1
        let eq2 = Polynomial1::<i32>::new(vec![1, 2]); // x + 2
        assert_eq!(eq1.clone() + eq2.clone(), Polynomial1::<i32>::new(vec![4, 3])); // 4x + 3
    }

    #[test]
    fn sub() {
        let eq1 = Polynomial1::<i32>::new(vec![3, 1]); // 3x + 1
        let eq2 = Polynomial1::<i32>::new(vec![1, 2]); // x + 2
        assert_eq!(eq1.clone() - eq2.clone(), Polynomial1::<i32>::new(vec![2, -1])); // 2x - 1
    }

    #[test]
    fn mul() {
        let eq1 = Polynomial1::<i32>::new(vec![2, 1]); // 2x + 1
        let eq2 = Polynomial1::<i32>::new(vec![2, 1]); // 2x + 1
        assert_eq!(eq1.clone() * eq2.clone(), Polynomial1::<i32>::new(vec![4, 4, 1])); // 4x^2 + 4x + 1
    }

    #[test]
    fn neg() {
        let eq = Polynomial1::<i32>::new(vec![2, 1]); // 2x + 1
        assert_eq!(-eq.clone(), Polynomial1::<i32>::new(vec![-2, -1])); // -2x - 1
    }
}
