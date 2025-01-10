use crate::Value;

use super::Number;
use std::ops;

impl ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self.0.0.clone();
        let mut rhs = rhs.0.0.clone();
        let lhs_int_len = lhs.iter().position(|x| *x == b'.').unwrap_or(lhs.len());
        let rhs_int_len = rhs.iter().position(|x| *x == b'.').unwrap_or(rhs.len());

        //Leading up leading digit
        for _ in lhs_int_len..rhs_int_len {
            lhs.insert(0, b'0');
        }
        for _ in rhs_int_len..lhs_int_len {
            rhs.insert(0, b'0');
        }

        let lhs_dec_pos = lhs.iter().position(|x| *x == b'.');
        let rhs_dec_pos = rhs.iter().position(|x| *x == b'.');
        // Set up trailing decimals
        if let Some(lhs_len) = lhs_dec_pos {
            if let Some(rhs_len) = rhs_dec_pos {
                for _ in rhs_len..lhs_len {
                    rhs.push(b'0');
                }
            } else {
                rhs.push(b'.');
                for _ in 0..lhs_len {
                    rhs.push(b'0');
                }
            }
        }

        if let Some(rhs_len) = rhs_dec_pos {
            if let Some(lhs_len) = lhs_dec_pos {
                for _ in lhs_len..rhs_len {
                    lhs.push(b'0');
                }
            } else {
                lhs.push(b'.');
                for _ in 0..rhs_len {
                    lhs.push(b'0');
                }
            }
        }

        // Temporarily remove decimal point (it makes the + logic simpler)
        let mut dec_pos = lhs_dec_pos.or(rhs_dec_pos);
        if let Some(dec_pos) = dec_pos {
            rhs.remove(dec_pos);
            lhs.remove(dec_pos);
        }

        let mut result: Vec<_> = lhs
            .iter()
            .zip(rhs.iter())
            .map(|(x, y)| x + y - b'0')
            .collect();
        // Handle carry over
        for i in (1..result.len()).rev() {
            if result[i] > b'9' {
                result[i] -= 10;
                result[i - 1] += 1;
            }
        }
        if result[0] > b'9' {
            result[0] -= 10;
            result.insert(0, b'1');
            if let Some(dec_pos) = &mut dec_pos {
                *dec_pos += 1;
            }
        }

        if let Some(dec_pos) = dec_pos {
            result.insert(dec_pos, b'.');
            while result.last() == Some(&b'0') {
                result.pop();
            }
            if result.last() == Some(&b'.') {
                result.pop();
            }
        }
        Number(Value(result))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::Number;

    #[rstest]
    #[case("1", "1", "2")]
    #[case("10", "1", "11")]
    #[case(".1", "1", "1.1")]
    #[case("18", "7", "25")]
    #[case("8", "7", "15")]
    #[case("99.9", ".2", "100.1")]
    #[case(".19", ".01", ".2")]
    #[case(".9", ".1", "1")]
    #[case("9", "1", "10")]
    fn add(#[case] a: Number, #[case] b: Number, #[case] sum: Number) {
        assert_eq!(a.clone() + b.clone(), sum);
        assert_eq!(b + a, sum);
    }
}
