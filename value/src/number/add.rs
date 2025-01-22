use super::{Number, carry_logic};

impl std::ops::Add for Number {
    type Output = Number;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        Number::match_padding(&mut [&mut self, &mut rhs]);
        let mantica = self
            .mantica
            .iter()
            .zip(rhs.mantica.iter())
            .map(|(x, y)| x + y)
            .collect();
        // Handle carry over
        carry_logic(mantica, self.exponent)
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, mut rhs: Self) -> Self::Output {
        rhs.negate();
        self + rhs
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use quickcheck_macros::quickcheck;
    use rstest::rstest;

    use crate::Number;

    #[rstest]
    #[case::one_pluse_one("1", "1", "2")]
    #[case::uint_of_different_length("10", "1", "11")]
    #[case::decimial_and_int(".1", "1", "1.1")]
    #[case::carry("18", "7", "25")]
    #[case::carry_to_new_order_of_magnituid("8", "7", "15")]
    #[case::carry_multiple_times("99.9", ".2", "100.1")]
    #[case::remove_trailing_zero(".19", ".01", ".2")]
    #[case::remove_trailing_dot(".9", ".1", "1")]
    #[case::keep_int_traling_zero("9", "1", "10")]
    #[case::add_negative("5", "-2", "3")]
    #[case::negative_result("2", "-5", "-3")]
    #[case::negative_carry_over("-9", "-1", "-10")]
    fn add(#[case] a: Number, #[case] b: Number, #[case] sum: Number) {
        assert_eq!(a.clone() + b.clone(), sum);
        assert_eq!(b + a, sum);
    }

    #[quickcheck]
    fn add_inverse_of_sub(a: i32, b: i32) {
        let sum: i64 = i64::from(a) + i64::from(b);
        let a = Number::from_str(&a.to_string()).unwrap();
        let b = Number::from_str(&b.to_string()).unwrap();
        let sum = Number::from_str(&sum.to_string()).unwrap();
        assert_eq!(a.clone() + b.clone(), sum.clone());
        assert_eq!(sum.clone() - b.clone(), a.clone());
        assert_eq!(b.clone() + a.clone(), sum.clone());
        assert_eq!(sum.clone() - a.clone(), b.clone());
    }
}
