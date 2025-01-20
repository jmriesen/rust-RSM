use super::{Number, carry_logic};

impl std::ops::Add for Number {
    type Output = Number;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        Number::match_padding(&mut [&mut self, &mut rhs]);
        let mut mantica: Vec<_> = self
            .mantica
            .iter()
            .zip(rhs.mantica.iter())
            .map(|(x, y)| x + y)
            .collect();
        // Handle carry over
        carry_logic(&mut mantica[..]);
        let mut exponent = self.exponent;
        mantica[0] %= 10;
        match mantica[0] {
            8 => {
                /*Negative add digit*/
                mantica.insert(0, 9);
                exponent += 1;
            }
            9 => { /* Negative */ }
            0 => { /*Positive*/ }
            1 => {
                /*Positive add digit*/
                mantica.insert(0, 0);
                exponent += 1;
            }
            _ => unreachable!(),
        }
        Number { mantica, exponent }
    }
}

#[cfg(test)]
mod tests {
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
    /*
    #[rstest]
    #[case::one_minus_one("1", "1", "0")]
    #[case::uint_of_different_length("22", "1", "21")]
    #[case::decimial_and_int("2.1", "1", "1.1")]
    #[case::carry("25", "7", "18")]
    #[case::carry_remove_order_of_maginitued("15", "7", "8")]
    #[case::carry_multiple_times("100.1", ".2", "99.9")]
    #[case::remove_trailing_zero("1.1", ".1", "1")]
    #[case::keep_int_traling_zero("11", "1", "10")]
    fn sub(#[case] a: Number, #[case] b: Number, #[case] difference: Number) {
        todo!();
        //assert_eq!(a - b, difference);
    }
    */
}
