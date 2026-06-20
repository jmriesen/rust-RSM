use std::sync::LazyLock;

use crate::{Number, Value};

impl Value {
    /// False value.
    pub fn false_v() -> &'static Self {
        static FALSE: LazyLock<Value> = LazyLock::new(|| {
            "0".parse()
                .expect("hard coded string litteral should not panic")
        });
        &FALSE
    }
    pub fn true_v() -> &'static Self {
        static TRUE: LazyLock<Value> = LazyLock::new(|| {
            "1".parse()
                .expect("hard coded string litteral should not panic")
        });
        &TRUE
    }
}
impl Number {
    pub fn zero() -> &'static Self {
        static ZERO: LazyLock<Number> = LazyLock::new(|| {
            "0".parse()
                .expect("hard coded string litteral should not panic")
        });
        &ZERO
    }
}
#[cfg(test)]
mod test {
    use crate::{Number, Value};

    #[test]
    fn truth_values() {
        assert_eq!("1".parse::<Value>().unwrap(), true.into());
        assert_eq!("0".parse::<Value>().unwrap(), false.into());
        assert_eq!(Number::zero().clone(), Value::false_v().clone().into());
        assert_eq!(true, (&"5".parse::<Number>().unwrap()).into());
        assert_eq!(false, Number::zero().into());
    }
}
