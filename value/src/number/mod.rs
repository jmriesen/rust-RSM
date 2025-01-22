mod add;
use super::Value;
use std::iter;
// Stores the number in scientific notation/9's complement form
// TODO 9's complement form
#[derive(Debug, Clone)]
pub struct Number {
    exponent: usize,
    ///Note due to 9's complement
    /// positive numbers start with a leading 0
    /// negative numbers start with a leading 9
    mantica: Vec<i8>,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        // NOTE:
        // I am currently cloning so I can re use the match_padding function.
        // Cloning should not be needed, but this is the simplest way I can think of implementing
        // equality right now.
        //
        // If this becomes a performance issue consider rewriting this logic.
        let mut left = self.clone();
        let mut right = other.clone();
        Number::match_padding(&mut [&mut left, &mut right]);
        left.mantica == right.mantica && left.exponent == right.exponent
    }
}

// Handles propagating the carry bit.
// This should be called after any operation that could result in a carry.
// Note this is specifically NOT a method on Number since since it is invalid
// to have a "number" that contains digits other then 0-9
fn carry_logic(mut mantica: Vec<i8>, mut exponent: usize) -> Number {
    for i in (1..mantica.len()).rev() {
        if mantica[i] > 9 {
            mantica[i] -= 10;
            mantica[i - 1] += 1;
        }
    }
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
    Number { exponent, mantica }
}

impl Number {
    /// Adds padding leading and trailing zeros until all numbers are the same level of precision
    /// and order of magnitude
    fn match_padding(numbers: &mut [&mut Number]) {
        let get_padding_digit = |x: &Number| if x.is_negative() { 9 } else { 0 };
        //NOTE this does not handle negative numbers properly
        if !numbers.is_empty() {
            //Match order of magnitude
            let new_exponent = numbers
                .iter()
                .map(|x| x.exponent)
                .max()
                .expect("non-empty slice");
            for number in &mut *numbers {
                let padding_digit = get_padding_digit(&number);
                for _ in number.exponent..new_exponent {
                    number.mantica.insert(0, padding_digit);
                    number.exponent += 1;
                }
            }
            // Match precision
            let new_mantica_len = numbers
                .iter()
                .map(|x| x.mantica.len())
                .max()
                .expect("non-empty slice");
            for number in &mut *numbers {
                let padding_digit = get_padding_digit(&number);
                for _ in number.mantica.len()..new_mantica_len {
                    number.mantica.push(padding_digit);
                }
            }
        }
    }

    fn is_negative(&self) -> bool {
        self.mantica[0] == 9
    }

    fn negate(&mut self) {
        self.mantica = self.mantica.iter().map(|x| 9 - x).collect();
        *self
            .mantica
            .last_mut()
            .expect("mantica is always non Empty") += 1;
        *self = carry_logic(std::mem::take(&mut self.mantica), self.exponent);
    }
}

impl From<Value> for Number {
    fn from(value: Value) -> Self {
        let is_sign = |x: &&u8| **x == b'-' || **x == b'+';
        let value = &value.0[..];

        let sign = value.iter().take_while(is_sign);
        let tail = value.iter().skip_while(is_sign);

        //Pulling of integer part
        let integer = tail.clone().take_while(|x| x.is_ascii_digit());
        let mut tail = tail.skip_while(|x| x.is_ascii_digit());
        let exponent = integer.clone().count();

        //Pulling out decimal part.
        //Note could still have trailing zeros
        let has_decimal_point = tail.next() == Some(&b'.');
        let decimal = tail.take_while(|x| x.is_ascii_digit() && has_decimal_point);

        let digits = integer.chain(decimal).map(|x| (x - b'0') as i8);

        let mut num = Number {
            mantica: iter::once(0).chain(digits).collect(),
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
            mut mantica,
            mut exponent,
        } = value;

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
        if is_negative {
            digits.insert(0, b'-');
        }
        Value(digits)
    }
}
impl std::str::FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = Value::from_str(s)?;
        Ok(Self::from(value))
    }
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
}
