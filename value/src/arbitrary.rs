use super::{MAX_STR_LEN, Value};
use arbitrary::Arbitrary;

impl<'a> Arbitrary<'a> for Value {
    #[cfg_attr(test, mutants::skip)]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let len: usize = u.int_in_range(0..=MAX_STR_LEN)?;
        Ok(Value::try_from(u.bytes(len)?).expect("len is less then max len"))
    }
}
