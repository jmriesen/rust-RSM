mod add;
use crate::convertions::CreationError;

use super::Value;
use std::iter;
// Stores the number in scientific notation/9's complement form
#[derive(Debug, Clone)]
pub struct Number {
    exponent: usize,
    ///Note due to 9's complement
    /// positive numbers start with a leading 0
    /// negative numbers start with a leading 9
    mantissa: Vec<i8>,
}
fn extract_normalized_representation(number: &Number) -> (&[i8], usize) {
    let sign_char = number.mantissa[0];
    let start = number
        .mantissa
        .iter()
        .position(|x| *x != sign_char)
        // Include the leading sign bit
        .map(|x| x - 1);
    let end = number
        .mantissa
        .iter()
        .rev()
        .position(|x| *x != sign_char)
        .map(|x| number.mantissa.len() - x);

    //If non-zero
    if let (Some(start), Some(end)) = (start, end) {
        let mantissa = &number.mantissa[start..end];
        let order_of_magnituide = number.exponent - start;
        (mantissa, order_of_magnituide)
    } else {
        (&[0], 0)
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        extract_normalized_representation(&self) == extract_normalized_representation(other)
    }
}

// Handles propagating the carry bit.
// This should be called after any operation that could result in a carry.
// Note this is specifically NOT a method on Number since since it is invalid
// to have a "number" that contains digits other then 0-9
fn carry_logic(mut mantissa: Vec<i8>, mut exponent: usize) -> Number {
    for i in (1..mantissa.len()).rev() {
        if mantissa[i] > 9 {
            mantissa[i] -= 10;
            mantissa[i - 1] += 1;
        }
    }
    mantissa[0] %= 10;
    match mantissa[0] {
        8 => {
            /*Negative add digit*/
            mantissa.insert(0, 9);
            exponent += 1;
        }
        9 => { /* Negative */ }
        0 => { /*Positive*/ }
        1 => {
            /*Positive add digit*/
            mantissa.insert(0, 0);
            exponent += 1;
        }
        _ => unreachable!(),
    }
    Number { exponent, mantissa }
}

impl Number {
    /// Adds padding leading and trailing zeros until all numbers are the same level of precision
    /// and order of magnitude
    fn match_padding(numbers: &mut [&mut Number]) {
        let get_padding_digit = |x: &Number| if x.is_negative() { 9 } else { 0 };
        if !numbers.is_empty() {
            //Match order of magnitude
            let new_exponent = numbers
                .iter()
                .map(|x| x.exponent)
                .max()
                .expect("non-empty slice");
            for number in &mut *numbers {
                let padding_digit = get_padding_digit(number);
                for _ in number.exponent..new_exponent {
                    number.mantissa.insert(0, padding_digit);
                    number.exponent += 1;
                }
            }
            // Match precision
            let new_mantica_len = numbers
                .iter()
                .map(|x| x.mantissa.len())
                .max()
                .expect("non-empty slice");
            for number in &mut *numbers {
                let padding_digit = get_padding_digit(number);
                for _ in number.mantissa.len()..new_mantica_len {
                    number.mantissa.push(padding_digit);
                }
            }
        }
    }

    fn is_negative(&self) -> bool {
        self.mantissa[0] == 9
    }

    fn negate(&mut self) {
        self.mantissa = self.mantissa.iter().map(|x| 9 - x).collect();
        *self
            .mantissa
            .last_mut()
            .expect("mantica is always non Empty") += 1;
        *self = carry_logic(std::mem::take(&mut self.mantissa), self.exponent);
    }
}

impl From<Value> for Number {
    fn from(value: Value) -> Self {
        let is_sign_character = |x: &&u8| **x == b'-' || **x == b'+';
        let value = &value.0[..];

        let sign = value.iter().take_while(is_sign_character);
        let tail = value.iter().skip_while(is_sign_character);

        //Pulling of integer part
        let integer = tail.clone().take_while(|x| x.is_ascii_digit());
        let mut tail = tail.skip_while(|x| x.is_ascii_digit());
        let exponent = integer.clone().count();

        //Pulling out decimal part.
        let has_decimal_point = tail.next() == Some(&b'.');
        let decimal = tail.take_while(|x| x.is_ascii_digit() && has_decimal_point);

        let digits = integer.chain(decimal).map(|x| (x - b'0') as i8);

        let mut num = Number {
            mantissa: iter::once(0).chain(digits).collect(),
            exponent,
        };

        let is_negative = sign.filter(|x| **x == b'-').count() % 2 == 1;
        if is_negative {
            num.negate();
        };
        num
    }
}

impl From<Number> for Value {
    fn from(mut value: Number) -> Self {
        let is_negative = value.is_negative();
        if is_negative {
            value.negate();
        }

        let Number {
            mut mantissa,
            mut exponent,
        } = value;

        //Removing the sign bit
        mantissa.remove(0);

        //Strip trailing zeros
        while mantissa.len() > exponent && mantissa.last() == Some(&0) {
            mantissa.pop();
        }
        //Strip leading zeros
        while exponent > 0 && mantissa.first() == Some(&0) {
            mantissa.remove(0);
            exponent -= 1;
        }
        let mut digits: Vec<_> = mantissa.iter().map(|x| (x + b'0' as i8) as u8).collect();

        if digits.len() > exponent {
            digits.insert(exponent, b'.');
        }
        if digits.is_empty() {
            digits.push(b'0');
        }
        if is_negative {
            digits.insert(0, b'-');
        }
        Value(digits)
    }
}
impl std::str::FromStr for Number {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = Value::from_str(s)?;
        Ok(Self::from(value))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Value, number::Number};
    use rstest::rstest;

    #[rstest]
    #[case("")]
    #[case("-.")]
    #[case("+.")]
    //#[case("+.")] this is throwing an error `w +"+."`
    #[case("0.")]
    #[case("0.0")]
    #[case(".0")]
    #[case("-0.")]
    #[case::bar("-0.0")]
    #[case("-.0")]
    #[case("+0.")]
    #[case("+0.0")]
    #[case("+.0")]
    #[case("+a")]
    #[case("a")]
    #[case(".a")]
    fn edge_cases(#[case] given: Number) {
        use std::str::FromStr;
        assert_eq!(Value::from_str("0").unwrap(), given.into())
    }

    #[rstest]
    #[case("12345")]
    #[case(".9")]
    #[case("-10")]
    fn no_transformation_needed(#[case] value: Value) {
        assert_eq!(value, Number::from(value.clone()).into())
    }

    #[rstest]
    #[case("--9", "9")]
    #[case("---9", "-9")]
    #[case("----9", "9")]
    #[case("+9", "9")]
    #[case("-+-9", "9")]
    #[case("-+-+-9", "-9")]
    fn handling_signs(#[case] given: Number, #[case] cononical: Value) {
        assert_eq!(cononical, given.into())
    }

    #[rstest]
    #[case::leading("09", "9")]
    #[case::negative("-09", "-9")]
    #[case::positive("+09", "9")]
    #[case::multiple("009", "9")]
    #[case::multiple_neg("-009", "-9")]
    #[case::multiple_pos("+009", "9")]
    #[case::dont_strip_dec_leading("0.001", ".001")]
    #[case::strip_trailing_dot("9.", "9")]
    #[case::zero_int("0.9", ".9")]
    #[case::trailing(".90", ".9")]
    #[case::trailing_multiple(".9000", ".9")]
    #[case::trailing(".900090", ".90009")]
    #[case::everything("0.000010000", ".00001")]
    fn strip_zeros(#[case] given: Number, #[case] cononical: Value) {
        assert_eq!(cononical, given.into())
    }

    #[rstest]
    #[case("9a0", "9")]
    #[case("-+-99OO", "99")]
    fn stop_at_non_numaric(#[case] given: Number, #[case] cononical: Value) {
        assert_eq!(cononical, given.into())
    }

    #[rstest]
    #[case("0")]
    #[case("1")]
    #[case("9")]
    #[case("10")]
    fn negate(#[case] value: &str) {
        use std::str::FromStr;
        {
            let mut negated = Number::from_str(value).unwrap();
            negated.negate();
            let original = Number::from_str(&format!("-{value}")).unwrap();
            assert_eq!(negated, original);
            assert_eq!(negated, original);
        }
        {
            let mut negated = Number::from_str(&format!("-{value}")).unwrap();
            negated.negate();
            let original = Number::from_str(value).unwrap();
            assert_eq!(negated, original);
            assert_eq!(negated, original);
        }
    }

    //This is mostly here to satisfy mutation testing
    #[test]
    fn expect_neq() {
        let number = Number::from_str("1").unwrap();
        let mut negitive = number.clone();
        negitive.negate();
        assert_ne!(number, negitive)
    }
}
