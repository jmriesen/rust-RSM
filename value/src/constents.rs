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
impl From<Value> for bool {
    fn from(value: Value) -> Self {
        (&Number::from(value)).into()
    }
}
impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        (&Number::from(value.clone())).into()
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        if value {
            Value::true_v()
        } else {
            Value::false_v()
        }
        .clone()
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
impl From<&Number> for bool {
    fn from(value: &Number) -> Self {
        value != Number::zero()
    }
}

#[cfg(test)]
mod test {
    use crate::{Number, Value};

    #[test]
    fn truth_values() {
        assert_eq!(true, Value::true_v().into());
        assert_eq!(false, Value::false_v().into());
        assert_eq!(false, Number::zero().into());
        assert_eq!(true, (&Number::from(Value::true_v().clone())).into());
    }
}
