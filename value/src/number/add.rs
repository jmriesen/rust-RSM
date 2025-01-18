use crate::Value;

use super::Number;
use std::{
    iter::{self},
    ops, usize,
};

impl ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let NormalizedDigits {
            lhs,
            rhs,
            mut int_len,
        } = Number::normalize_magnitieds(&self, &rhs);

        let mut sum: Vec<_> = lhs
            .iter()
            .zip(rhs.iter())
            .map(|(x, y)| x + y - b'0')
            .collect();
        // Handle carry over
        for i in (1..sum.len()).rev() {
            if sum[i] > b'9' {
                sum[i] -= 10;
                sum[i - 1] += 1;
            }
        }
        // Handle carry over of most significant bit
        if sum[0] > b'9' {
            sum[0] -= 10;
            sum.insert(0, b'1');
            int_len += 1;
        }

        Number::from_normalized(sum, int_len)
    }
}
impl ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        let NormalizedDigits {
            lhs,
            rhs,
            int_len: decimal_position,
        } = Number::normalize_magnitieds(&self, &rhs);

        let mut difference: Vec<_> = lhs
            .iter()
            .zip(rhs.iter())
            .map(|(x, y)| b'0' + x - y)
            .collect();
        // Handle carry over
        for i in (1..difference.len()).rev() {
            if difference[i] < b'0' {
                difference[i] += 10;
                difference[i - 1] -= 1;
            }
        }
        // Handle carry over of most significant bit
        if difference[0] < b'0' {
            dbg!(Value(difference));
            todo!("handle negitive number result");
        }

        Number::from_normalized(difference, decimal_position)
    }
}
/// Stores digits in a normalized format.
/// See `normalize_magnitieds`
#[derive(Debug)]
struct NormalizedDigits {
    lhs: Vec<u8>,
    rhs: Vec<u8>,
    int_len: usize,
}

impl Number {
    /// Takes two numbers and converses them to a vector of digit's.
    ///
    /// The resulting vectors have the same length and have there place values lined up.
    /// For example 10 and .01 would result in
    /// 1000
    /// 0001
    /// Some(2)
    fn normalize_magnitieds(lhs: &Number, rhs: &Number) -> NormalizedDigits {
        let add_whitespace = |number: &Number, int_len: usize, dec_len: usize| {
            let leading_zeros = (number.int_part().len()..int_len).count();
            let trailing_zeros = (number.dec_part().len()..dec_len).count();
            iter::repeat(b'0')
                .take(leading_zeros)
                .chain(number.int_part().iter().cloned())
                .chain(number.dec_part().iter().cloned())
                .chain(iter::repeat(b'0').take(trailing_zeros))
                .collect()
        };
        let int_len = *[lhs, rhs].map(|x| x.int_part().len()).iter().max().unwrap();
        let dec_len = *[lhs, rhs].map(|x| x.dec_part().len()).iter().max().unwrap();
        let lhs = add_whitespace(lhs, int_len, dec_len);
        let rhs = add_whitespace(rhs, int_len, dec_len);

        NormalizedDigits { lhs, rhs, int_len }
    }
    fn from_normalized(mut digits: Vec<u8>, int_len: usize) -> Number {
        if digits.len() > int_len {
            digits.insert(int_len, b'.');
            // Striping trailing zeros
            while digits.last() == Some(&b'0') {
                digits.pop();
            }
            // Striping trailing dot
            if digits.last() == Some(&b'.') {
                digits.pop();
            }
        }
        // Striping leading zeros
        while digits.first() == Some(&b'0') && digits.len() > 1 {
            digits.remove(0);
        }
        Number(Value(digits))
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
    fn add(#[case] a: Number, #[case] b: Number, #[case] sum: Number) {
        assert_eq!(a.clone() + b.clone(), sum);
        assert_eq!(b + a, sum);
    }

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
        assert_eq!(a - b, difference);
    }
}
