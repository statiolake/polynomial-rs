extern crate num_traits;

pub mod polynomial_1;
#[cfg(test)]
mod tests {
    use polynomial_1::Polynomial1;
    #[test]
    fn add() {
        let eq1 = Polynomial1::<i32>::new(vec![1, 3]); // 3x + 1
        let eq2 = Polynomial1::<i32>::new(vec![2, 1]); // x + 2
        assert_eq!(eq1.clone() + eq2.clone(), Polynomial1::<i32>::new(vec![3, 4])); // 4x + 3
    }

    #[test]
    fn sub() {
        let eq1 = Polynomial1::<i32>::new(vec![1, 3]); // 3x + 1
        let eq2 = Polynomial1::<i32>::new(vec![2, 1]); // x + 2
        assert_eq!(eq1.clone() - eq2.clone(), Polynomial1::<i32>::new(vec![-1, 2])); // 4x + 3
    }

    #[test]
    fn mul() {
        let eq1 = Polynomial1::<i32>::new(vec![1, 2]); // 2x + 1
        let eq2 = Polynomial1::<i32>::new(vec![1, 2]); // 2x + 1
        assert_eq!(eq1.clone() * eq2.clone(), Polynomial1::<i32>::new(vec![1, 4, 4])); // 4x^2 + 4x + 1
    }

    #[test]
    fn neg() {
        let eq = Polynomial1::<i32>::new(vec![1, 2]); // 2x + 1
        assert_eq!(-eq.clone(), Polynomial1::<i32>::new(vec![-1, -2])); // -2x - 1
    }
}
