use super::Value;
/// Stores a value that has been converted into the canonical numeric representation
/// If negative there will be exactly one leading '-'
/// If positive there will be no leading sign
#[derive(Debug, PartialEq, Eq)]
pub struct Number(Value);

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

        //Pulling out decimal part.
        //Note could still have trailing zeros or dot
        let decimal = if let Some(&b'.') = tail.next() {
            Some(tail.take_while(|x| x.is_ascii_digit()))
        } else {
            None
        };

        let mut output = vec![];
        //Counting the number of negative signs
        if sign.filter(|x| **x == b'-').count() % 2 == 1 {
            output.push(b'-');
        };

        output.extend(integer);
        if let Some(decimal) = decimal {
            output.push(b'.');
            output.extend(decimal);
            //Striping trailing zeros
            while let Some(&b'0') = output.last() {
                output.pop();
            }
            //Stringing trailing dot
            if let Some(&b'.') = output.last() {
                output.pop();
            }
        }

        //Handling edge cases that evaluate to zero
        if output.is_empty() {
            Number(Value(vec![b'0']))
        } else if output.len() == 1 && is_sign(&&output[0]) {
            Number(Value(vec![b'0']))
        } else {
            Number(Value(output))
        }
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
    #[case("-0.0")]
    #[case("-.0")]
    #[case("+0.")]
    #[case("+0.0")]
    #[case("+.0")]
    #[case("+a")]
    #[case("a")]
    #[case(".a")]
    fn edge_cases(#[case] given: Value) {
        use std::str::FromStr;
        assert_eq!(Number::from(given).0, Value::from_str("0").unwrap())
    }

    #[rstest]
    #[case("12345")]
    #[case(".9")]
    fn no_transformation_needed(#[case] value: Value) {
        assert_eq!(Number::from(value.clone()).0, value)
    }

    #[rstest]
    #[case("--9", "9")]
    #[case("---9", "-9")]
    #[case("----9", "9")]
    #[case("+9", "9")]
    #[case("-+-9", "9")]
    #[case("-+-+-9", "-9")]
    fn handling_signs(#[case] given: Value, #[case] cononical: Value) {
        assert_eq!(Number::from(given).0, cononical)
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
    fn strip_zeros(#[case] given: Value, #[case] cononical: Value) {
        assert_eq!(Number::from(given).0, cononical)
    }

    #[rstest]
    #[case("9a0", "9")]
    #[case("-+-99OO", "99")]
    fn stop_a_non_numaric(#[case] given: Value, #[case] cononical: Value) {
        assert_eq!(Number::from(given).0, cononical)
    }
}
