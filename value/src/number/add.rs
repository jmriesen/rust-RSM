use super::Number;

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
        for i in (1..mantica.len()).rev() {
            if mantica[i] > 9 {
                mantica[i] -= 10;
                mantica[i - 1] += 1;
            }
        }
        let mut exponent = self.exponent;
        // If we carried over to a new digit
        if mantica[0] == 1 {
            mantica.insert(0, 0);
            exponent += 1;
        }
        Number { mantica, exponent }
    }
}

/*
impl ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        let NormalizedDigits {
            lhs,
            rhs,
            int_len: decimal_position,
        } = Number::normalize_magnitieds(&self, &rhs);

        let mut difference: Vec<_> = lhs.iter().zip(rhs.iter()).map(|(x, y)| x - y).collect();
        // Handle carry over
        for i in (1..difference.len()).rev() {
            if difference[i] < 0 {
                difference[i] += 10;
                difference[i - 1] -= 1;
            }
        }
        // Handle carry over of most significant bit
        if difference[0] < 0 {
            dbg!(Number::from_normalized(difference, decimal_position));
            todo!("handle negitive number result");
        }

        Number::from_normalized(difference, decimal_position)
    }
}
*/

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
