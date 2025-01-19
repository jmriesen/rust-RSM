//mod add;

use std::{iter, str::FromStr};

use super::Value;
// Stores the number in scientific notation/9's complement form
//
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Number {
    exponent: usize,
    ///Note due to 9's complement
    /// positive numbers start with a leading 0
    /// negative numbers start with a leading 9
    mantica: Vec<i8>,
}

impl From<Value> for Number {
    fn from(value: Value) -> Self {
        let is_sign = |x: &&u8| **x == b'-' || **x == b'+';
        let value = &value.0[..];

        let sign = value.iter().take_while(is_sign);
        let tail = value.iter().skip_while(is_sign);
        //Removing leading white space
        let tail = tail.skip_while(|x| **x == b'0');

        //Pulling of integer part
        let integer = tail.clone().take_while(|x| x.is_ascii_digit());
        let mut tail = tail.skip_while(|x| x.is_ascii_digit());
        let exponent = integer.clone().count();

        //Pulling out decimal part.
        //Note could still have trailing zeros
        let has_decimal_point = tail.next() == Some(&b'.');
        let decimal = tail.take_while(|x| x.is_ascii_digit() && has_decimal_point);

        let digits = integer.chain(decimal).map(|x| (x - b'0') as i8);

        let non_neg =
            sign.filter(|x| **x == b'-').count() % 2 == 0 || digits.clone().all(|x| x == 0);

        let mantica: Vec<_> = if non_neg {
            iter::once(0).chain(digits).collect()
        } else {
            todo!("this should map into nines commploment");
            iter::once(9).chain(digits).collect()
        };

        //If we only have the sign bit the value is zero
        Number { mantica, exponent }
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        let Number {
            mut mantica,
            mut exponent,
        } = value;
        //TODO deal with sign byte
        let sign = *mantica
            .first()
            .expect("sign bit should allways be pressent");
        //Removing the sign bit
        mantica.remove(0);
        //Strip trailing zeros
        while mantica.len() > exponent && mantica.last() == Some(&0) {
            mantica.pop();
        }
        //Strip leading zeros
        while mantica.len() > 1 && mantica.first() == Some(&0) {
            mantica.remove(0);
            exponent -= 1;
        }
        let mut digits: Vec<_> = mantica.iter().map(|x| (x + b'0' as i8) as u8).collect();

        if digits.len() > exponent {
            digits.insert(exponent, b'.');
        }
        if digits.is_empty() {
            digits.push(b'0');
        }
        if sign == 9 {
            digits.insert(0, b'-');
        }
        if digits.is_empty() {
            Value(vec![b'0'])
        } else {
            Value(digits)
        }
    }
}
impl std::str::FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = Value::from_str(s)?;
        Ok(Self::from(value))
    }
}
#[derive(PartialEq, Debug)]
enum Sign {
    NonNegative,
    Negative,
}

#[cfg(test)]
mod test {
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
    #[case("09", "9")]
    #[case("-09", "-9")]
    #[case("+09", "9")]
    #[case("009", "9")]
    #[case("-009", "-9")]
    #[case("+009", "9")]
    #[case("9.", "9")]
    #[case("0.9", ".9")]
    #[case(".90", ".9")]
    #[case(".9000", ".9")]
    #[case(".900090", ".90009")]
    fn strip_zeros(#[case] given: Number, #[case] cononical: Value) {
        assert_eq!(cononical, given.into())
    }

    #[rstest]
    #[case("9a0", "9")]
    #[case("-+-99OO", "99")]
    fn stop_a_non_numaric(#[case] given: Number, #[case] cononical: Value) {
        assert_eq!(cononical, given.into())
    }
}
