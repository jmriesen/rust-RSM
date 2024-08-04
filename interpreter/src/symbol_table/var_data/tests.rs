use crate::{
    symbol_table::{m_var::helpers::var_m, Table},
    value::Value,
};
mod data {
    use super::*;

    #[test]
    fn root_data() {
        let mut table = Table::new();
        let var = var_m("var", &[]);
        assert!(!table.data(&var).1);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&var, &data);
        assert!(table.data(&var).1);
    }
    #[test]
    fn root_descendants() {
        let mut table = Table::new();
        let root = var_m("var", &[]);
        let sub = var_m("var", &["sub"]);
        assert!(!table.data(&root).0);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&sub, &data);
        assert!(table.data(&root).0);
    }
    #[test]
    fn sub_key_data() {
        let mut table = Table::new();
        let var = var_m("var", &["sub"]);
        assert!(!table.data(&var).1);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&var, &data);
        assert!(table.data(&var).1);
    }
    #[test]
    fn sub_key_descendants() {
        let mut table = Table::new();
        let sub = var_m("var", &[]);
        let sub_sub = var_m("var", &["sub"]);
        assert!(!table.data(&sub).0);
        let data: Value = "data".try_into().unwrap();
        let _ = table.set(&sub_sub, &data);
        assert!(table.data(&sub).0);
    }
}

mod query {

    use super::*;
    use crate::symbol_table::var_data::Direction;

    #[test]
    fn forward_and_backward() {
        let keys: [&[&str]; 4] = [&["-1"], &["0"], &["0", "1"], &["a"]];
        let mut m_vars: Vec<_> = keys.map(|x| var_m("foo", x)).to_vec();

        let mut table = Table::new();
        for var in &m_vars {
            table.set(var, &Value::try_from("Value").unwrap()).unwrap();
        }

        m_vars.insert(0, var_m("foo", &[""]));
        m_vars.push(var_m("foo", &[""]));

        for i in 0..m_vars.len() - 2 {
            assert_eq!(
                table.query(&m_vars[i], Direction::Forward),
                format!("{}", m_vars[i + 1])
            );
        }
        for i in (2..m_vars.len()).rev() {
            assert_eq!(
                table.query(&m_vars[i], Direction::Backward),
                format!("{}", m_vars[i - 1])
            );
        }
    }

    #[test]
    fn value_with_no_subscripts() {
        let mut table = super::Table::new();
        let _ = table.set(&var_m("foo", &[]), &Value::try_from("Value").unwrap());
        assert_eq!(table.query(&var_m("foo", &[""]), Direction::Forward), "");
        assert_eq!(table.query(&var_m("foo", &["bar"]), Direction::Forward), "");
        assert_eq!(table.query(&var_m("foo", &[""]), Direction::Backward), "");
        assert_eq!(
            table.query(&var_m("foo", &["bar"]), Direction::Backward),
            ""
        );
    }

    ///A trailing null is treated as the fist key while moving forward and the last key when moving
    ///backwards.
    #[test]
    fn moving_backwards_null_subscript() {
        let keys: [&[&str]; 5] = [&["-1"], &["0"], &["0", "-1"], &["0", "1"], &["1"]];
        let m_vars = keys.map(|x| var_m("foo", x));

        let mut table = Table::new();
        for var in &m_vars {
            table.set(var, &Value::try_from("Value").unwrap()).unwrap();
        }
        let null_last_key = var_m("foo", &["0", ""]);
        assert_eq!(
            table.query(&null_last_key, Direction::Forward),
            format!("{}", m_vars[2])
        );
        assert_eq!(
            table.query(&null_last_key, Direction::Backward),
            format!("{}", m_vars[3])
        );
    }

    ///If in the middle of the key null is treated normally (null< everything else)
    #[test]
    fn null_in_the_middle() {
        let keys: [&[&str]; 4] = [&["-1"], &["0", "1", "-1"], &["0", "1", "1"], &["1"]];
        let m_vars = keys.map(|x| var_m("foo", x));

        let mut table = Table::new();
        for var in &m_vars {
            table.set(var, &Value::try_from("Value").unwrap()).unwrap();
        }

        let null_in_middle = var_m("foo", &["0", "", "0"]);
        assert_eq!(
            table.query(&null_in_middle, Direction::Forward),
            format!("{}", m_vars[1])
        );
        assert_eq!(
            table.query(&null_in_middle, Direction::Backward),
            format!("{}", m_vars[0])
        );
    }
}

mod order {

    use crate::symbol_table::var_data::Direction;

    use super::*;

    #[test]
    fn forward_and_backward() {
        let mut table = Table::new();
        let data = Value::try_from("data").unwrap();
        table.set(&var_m("foo", &["0"]), &data).unwrap();
        table.set(&var_m("foo", &["1", "a"]), &data).unwrap();
        table.set(&var_m("foo", &["1", "b"]), &data).unwrap();
        table.set(&var_m("foo", &["2"]), &data).unwrap();

        //Top level forward
        assert_eq!(
            table.order(&var_m("foo", &[""]), Direction::Forward),
            "0".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["0"]), Direction::Forward),
            "1".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["1"]), Direction::Forward),
            "2".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["2"]), Direction::Forward),
            "".try_into().unwrap(),
        );

        //Top level Backwords
        assert_eq!(
            table.order(&var_m("foo", &[""]), Direction::Backward),
            "2".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["2"]), Direction::Backward),
            "1".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["1"]), Direction::Backward),
            "0".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["0"]), Direction::Backward),
            "".try_into().unwrap(),
        );

        //sub layer Forward
        assert_eq!(
            table.order(&var_m("foo", &["1", ""]), Direction::Forward),
            "a".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["1", "a"]), Direction::Forward),
            "b".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["1", "b"]), Direction::Forward),
            "".try_into().unwrap(),
        );

        //Top level Backwords
        assert_eq!(
            table.order(&var_m("foo", &["1", ""]), Direction::Backward),
            "b".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["1", "b"]), Direction::Backward),
            "a".try_into().unwrap(),
        );
        assert_eq!(
            table.order(&var_m("foo", &["1", "a"]), Direction::Backward),
            "".try_into().unwrap(),
        );
    }

    #[test]
    fn value_with_no_subscripts() {
        let mut table = ffi::symbol_table::Table::new();
        let foo = var_m("foo", &[]).into_cmvar();
        let bar = var_m("bar", &["subscript"]).into_cmvar();
        let () = table.set(&foo, &Value::try_from("Value").unwrap().into_cstring());
        let () = table.set(&bar, &Value::try_from("Value").unwrap().into_cstring());
        assert_eq!("", String::from_utf8(table.order(&foo, false)).unwrap());
        assert_eq!("", String::from_utf8(table.order(&bar, false)).unwrap());
        assert_eq!("", String::from_utf8(table.order(&foo, true)).unwrap());
        assert_eq!("", String::from_utf8(table.order(&bar, true)).unwrap());
    }
}
