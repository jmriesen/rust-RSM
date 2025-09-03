use crate::{
    SymbolTable,
    m_var::test_helpers::{var_m, var_m_nullable},
    var_data::Direction,
};
use value::Value;
mod data {
    use crate::var_data::DataResult;

    use super::*;

    #[test]
    fn root_data() {
        let mut table = SymbolTable::new();
        let var = var_m("var", &[]);
        assert!(!table.data(&var).has_value);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&var, &data);
        assert!(table.data(&var).has_value);
    }
    #[test]
    fn root_descendants() {
        let mut table = SymbolTable::new();
        let root = var_m("var", &[]);
        let sub = var_m("var", &["sub"]);
        assert!(!table.data(&root).has_descendants);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&sub, &data);
        assert!(table.data(&root).has_descendants);
    }
    #[test]
    fn sub_key_data() {
        let mut table = SymbolTable::new();
        let var = var_m("var", &["sub"]);
        assert!(!table.data(&var).has_value);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&var, &data);
        assert!(table.data(&var).has_value);
    }

    #[test]
    fn sub_key_descendants() {
        let mut table = SymbolTable::new();
        let data: Value = "data".try_into().unwrap();
        let var = var_m("var", &["sub"]);
        let descendant = var_m("var", &["sub", "sub"]);
        assert!(!table.data(&var).has_descendants);
        let _ = table.set(&descendant, &data);
        assert!(table.data(&var).has_descendants);
    }

    ///All of the variables keys are stored in one sorted list
    #[test]
    fn adjacent_keys_are_not_sub_keys() {
        let mut table = SymbolTable::new();
        let data: Value = "data".try_into().unwrap();
        let a = var_m("var", &["a"]);
        let b = var_m("var", &["b"]);
        let c = var_m("var", &["c"]);
        let _ = table.set(&a, &data);
        let _ = table.set(&b, &data);
        let _ = table.set(&c, &data);
        assert!(!dbg!(table.data(&b)).has_descendants);
    }

    #[test]
    fn into_value() {
        assert_eq!(
            Value::try_from("0").unwrap(),
            Value::from(DataResult {
                has_value: false,
                has_descendants: false,
            }),
        );
        assert_eq!(
            Value::try_from("1").unwrap(),
            Value::from(DataResult {
                has_value: true,
                has_descendants: false,
            }),
        );
        assert_eq!(
            Value::try_from("10").unwrap(),
            Value::from(DataResult {
                has_value: false,
                has_descendants: true,
            }),
        );
        assert_eq!(
            Value::try_from("11").unwrap(),
            Value::from(DataResult {
                has_value: true,
                has_descendants: true,
            }),
        );
    }
}

mod query {

    use super::*;

    #[test]
    fn forward_and_backward() {
        let keys: [&[&str]; 4] = [&["-1"], &["0"], &["0", "1"], &["a"]];
        let m_vars: Vec<_> = keys.map(|x| var_m("foo", x)).to_vec();

        let mut table = SymbolTable::new();
        for var in &m_vars {
            table.set(var, &Value::try_from("Value").unwrap()).unwrap();
        }
        assert_eq!(
            table.query(m_vars.first().unwrap(), Direction::Backward),
            None
        );
        for [a, b] in m_vars.array_windows() {
            assert_eq!(table.query(a, Direction::Forward).as_ref(), Some(b));
            assert_eq!(table.query(b, Direction::Backward).as_ref(), Some(a));
        }

        assert_eq!(
            table.query(m_vars.last().unwrap(), Direction::Forward),
            None
        );
    }

    #[test]
    fn nullable_subscripts_can_be_used() {
        let keys: [&[&str]; 4] = [&["-1"], &["0", "-1"], &["0", "1"], &["a"]];
        let m_vars: Vec<_> = keys.map(|x| var_m("foo", x)).to_vec();

        let mut table = SymbolTable::new();
        for var in &m_vars {
            table.set(var, &Value::try_from("Value").unwrap()).unwrap();
        }
        let boundary = var_m_nullable("foo", &["0", ""]);
        assert_eq!(
            table.query(&boundary, Direction::Forward).as_ref(),
            m_vars.get(1)
        );
        assert_eq!(
            table.query(&boundary, Direction::Backward).as_ref(),
            m_vars.get(2)
        );
    }

    #[test]
    fn value_with_no_subscripts() {
        let mut table = super::SymbolTable::new();
        let root = var_m("foo", &[]);
        let _ = table.set(&root, &Value::try_from("Value").unwrap());

        assert_eq!(
            table.query(&var_m_nullable("foo", &[""]), Direction::Forward),
            None
        );
        assert_eq!(
            table.query(&var_m_nullable("foo", &["bar"]), Direction::Forward),
            None
        );
        assert_eq!(
            table.query(&var_m_nullable("foo", &[""]), Direction::Backward),
            Some(root.clone())
        );
        assert_eq!(
            table.query(&var_m_nullable("foo", &["bar"]), Direction::Backward),
            Some(root)
        );
    }

    ///The behavior documented below is counter intuitive and was discovered during fuzz/AB
    ///testing
    ///$Q(var("subscript")) is being affected by presents or absents of *ANY* var subscript.
    ///This seems like a bug
    ///TODO File an issue with the upstream repo
    #[test]
    fn root_value_is_handled() {
        let mut table = super::SymbolTable::new();
        //Root value is absent
        assert_eq!(
            table.query(&var_m_nullable("foo", &["bar"]), Direction::Backward),
            None
        );

        //Root value is present
        let _ = table.set(&var_m("foo", &[]), &Value::try_from("Value").unwrap());
        assert_eq!(
            table.query(&var_m_nullable("foo", &["bar"]), Direction::Backward),
            Some(var_m("foo", &[]))
        );
    }

    ///A trailing null is treated as the fist key while moving forward and the last key when moving
    ///backwards.
    #[test]
    fn moving_backwards_null_subscript() {
        let keys: [&[&str]; 5] = [&["-1"], &["0"], &["0", "-1"], &["0", "1"], &["1"]];
        let m_vars = keys.map(|x| var_m("foo", x));

        let mut table = SymbolTable::new();
        for var in &m_vars {
            table.set(var, &Value::try_from("Value").unwrap()).unwrap();
        }
        let null_last_key = var_m_nullable("foo", &["0", ""]);
        assert_eq!(
            table.query(&null_last_key, Direction::Forward).as_ref(),
            Some(&m_vars[2])
        );
        assert_eq!(
            table.query(&null_last_key, Direction::Backward).as_ref(),
            Some(&m_vars[3])
        );
    }
}

mod order {
    use super::*;

    #[test]
    fn forward_and_backward() {
        use Direction::{Backward, Forward};
        let mut table = SymbolTable::new();
        let data = Value::try_from("data").unwrap();
        table.set(&var_m("foo", &["0"]), &data).unwrap();
        table.set(&var_m("foo", &["1", "a"]), &data).unwrap();
        table.set(&var_m("foo", &["1", "b"]), &data).unwrap();
        table.set(&var_m("foo", &["2"]), &data).unwrap();

        let assert = |var, direction, value: Option<&str>| {
            assert_eq!(
                table.order(&var, direction).map(Value::from),
                value.map(|x| x.try_into().unwrap()),
            );
        };

        //Top level forward
        assert(var_m_nullable("foo", &[""]), Forward, Some("0"));
        assert(var_m_nullable("foo", &["0"]), Forward, Some("1"));
        assert(var_m_nullable("foo", &["1"]), Forward, Some("2"));
        assert(var_m_nullable("foo", &["2"]), Forward, None);

        //Top level Backwards
        assert(var_m_nullable("foo", &[""]), Backward, Some("2"));
        assert(var_m_nullable("foo", &["2"]), Backward, Some("1"));
        assert(var_m_nullable("foo", &["1"]), Backward, Some("0"));
        assert(var_m_nullable("foo", &["0"]), Backward, None);

        //Sub layer Forward
        assert(var_m_nullable("foo", &["1", ""]), Forward, Some("a"));
        assert(var_m_nullable("foo", &["1", "a"]), Forward, Some("b"));
        assert(var_m_nullable("foo", &["1", "b"]), Forward, None);

        //Top level Backboards
        assert(var_m_nullable("foo", &["1", ""]), Backward, Some("b"));
        assert(var_m_nullable("foo", &["1", "b"]), Backward, Some("a"));
        assert(var_m_nullable("foo", &["1", "a"]), Backward, None);
    }

    #[test]
    fn value_with_no_subscripts() {
        let mut table = SymbolTable::new();
        let foo = var_m("foo", &[]);
        let bar = var_m("bar", &["subscript"]);
        table.set(&foo, &Value::try_from("Value").unwrap()).unwrap();
        table.set(&bar, &Value::try_from("Value").unwrap()).unwrap();
        assert_eq!(None, table.order(&foo, Direction::Forward));
        assert_eq!(None, table.order(&bar, Direction::Forward));
        assert_eq!(None, table.order(&foo, Direction::Backward));
        assert_eq!(None, table.order(&bar, Direction::Backward));
    }
}
