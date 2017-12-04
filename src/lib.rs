extern crate num_traits;

pub mod polynomial_1;
pub mod fraction;

pub use polynomial_1::Polynomial1;
pub use fraction::Fraction;

#[cfg(test)]
mod tests_polynomial_1 {
    use polynomial_1::Polynomial1;
    use fraction::Fraction;

    #[test]
    fn add() {
        let eq1 = Polynomial1::<i32>::new(vec![3, 1]); // 3x + 1
        let eq2 = Polynomial1::<i32>::new(vec![1, 2]); // x + 2
        println!("{} + {}", eq1, eq2);
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
    fn div() {
        let eq1 = Polynomial1::<i32>::new(vec![2, 1]); // 2x + 1
        let eq2 = Polynomial1::<i32>::new(vec![2, 1]); // 2x + 1
        assert_eq!(eq1.clone() / eq2.clone(), Fraction::new(eq1, eq2)); // (2x+1)/(2x+1)
    }

    #[test]
    fn neg() {
        let eq = Polynomial1::<i32>::new(vec![2, 1]); // 2x + 1
        assert_eq!(-eq.clone(), Polynomial1::<i32>::new(vec![-2, -1])); // -2x - 1
    }
}

#[cfg(test)]
mod tests_fraction {
    use polynomial_1::Polynomial1;
    use fraction::Fraction;

    #[test]
    fn add() {
        let eq1 = Fraction::new(Polynomial1::new(vec![3, 1]), Polynomial1::new(vec![1, 1])); // (3x+1)/(x+1)
        let eq2 = Fraction::new(Polynomial1::new(vec![1, 2]), Polynomial1::new(vec![4, 1])); // (x+2)/(4x+1)
        println!("{} + {} = {}", eq1, eq2, eq1.clone() + eq2.clone());
        assert_eq!(eq1 + eq2, Fraction::new(Polynomial1::new(vec![13,10,3]), Polynomial1::new(vec![4,5,1])));
    }

    #[test]
    fn neg() {
        let eq = Fraction::new(Polynomial1::new(vec![3, 1]), Polynomial1::new(vec![1, 1])); // (3x+1)/(x+1)
        assert_eq!(-eq, Fraction::new(Polynomial1::new(vec![-3, -1]), Polynomial1::new(vec![1, 1])));
    }

    #[test]
    fn sub() {
        let eq1 = Fraction::new(Polynomial1::new(vec![3, 1]), Polynomial1::new(vec![1])); // (3x+1)/(1)
        let eq2 = Fraction::new(Polynomial1::new(vec![1, 2]), Polynomial1::new(vec![1])); // (x+2)/(1)
        assert_eq!(eq1 - eq2, Fraction::new(Polynomial1::new(vec![2, -1]), Polynomial1::new(vec![1])));
    }

    #[test]
    fn mul() {
        let eq1 = Fraction::new(Polynomial1::new(vec![1, 1]), Polynomial1::new(vec![1, 2])); // (x+1)/(x+2)
        let eq2 = Fraction::new(Polynomial1::new(vec![1, 3]), Polynomial1::new(vec![1, 4])); // (x+3)/(x+4)
        assert_eq!(eq1 * eq2, Fraction::new(Polynomial1::new(vec![1,4,3]), Polynomial1::new(vec![1,6,8])));
    }

    #[test]
    fn div() {
        let eq1 = Fraction::new(Polynomial1::new(vec![1, 1]), Polynomial1::new(vec![1, 2])); // (x+1)/(x+2)
        let eq2 = Fraction::new(Polynomial1::new(vec![1, 4]), Polynomial1::new(vec![1, 3])); // (x+4)/(x+3)
        assert_eq!(eq1 / eq2, Fraction::new(Polynomial1::new(vec![1,4,3]), Polynomial1::new(vec![1,6,8])));
    }
}
