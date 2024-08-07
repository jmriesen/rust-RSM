use super::Table;
use crate::symbol_table::m_var::helpers::var_m;
use ffi::UCI_IS_LOCALVAR;
use pretty_assertions::assert_eq;

#[test]
fn get_unset_variable() {
    let table = Table::new();
    let m_var = var_m("foo", &[]);
    assert_eq!(table.get(&m_var), None);
}
#[test]
fn get_unset_key() {
    let mut table = Table::new();
    let m_var = var_m("foo", &[]);
    let mut data = "Data".try_into().unwrap();
    table.set(&m_var, &mut data).unwrap();

    let m_var = var_m("foo", &["bar"]);
    assert_eq!(table.get(&m_var), None);
}

#[test]
fn set_root_value() {
    let mut table = Table::new();
    let m_var = var_m("foo", &[]);
    let mut data = "Data".try_into().unwrap();

    table.set(&m_var, &mut data).unwrap();
    assert_eq!(table.get(&m_var), Some(&data));
}

#[test]
fn set_index_value() {
    let mut table = Table::new();
    let m_var = var_m("foo", &["keys"]);
    let mut data = "Data".try_into().unwrap();
    table.set(&m_var, &mut data).unwrap();
    assert_eq!(Some(&data), table.get(&m_var));
}

#[test]
fn set_root_then_index() {
    let root = var_m("foo", &[]);
    let root_data = "root Data".try_into().unwrap();
    let with_key = var_m("foo", &["keys"]);
    let key_data = "key Data".try_into().unwrap();
    {
        let mut table = Table::new();

        table.set(&root, &root_data).unwrap();
        table.set(&with_key, &key_data).unwrap();
        assert_eq!(Some(&root_data), table.get(&root));
        assert_eq!(Some(&key_data), table.get(&with_key));
    }
    {
        let mut table = Table::new();

        table.set(&with_key, &key_data).unwrap();
        table.set(&root, &root_data).unwrap();
        assert_eq!(Some(&root_data), table.get(&root));
        assert_eq!(Some(&key_data), table.get(&with_key));
    }
}

#[test]
fn set_null_value() {
    let mut table = Table::new();
    let m_var = var_m("foo", &[]);
    let data = "".try_into().unwrap();

    table.set(&m_var, &data).unwrap();
    assert_eq!(Some(&data), table.get(&m_var));
}

#[test]
fn set_works_while_with_prefixs() {
    let mut table = Table::new();
    let prefix = var_m("foo", &["prefix"]);
    let full = var_m("foo", &["prefixAndMore"]);
    let prefix_data = "prefix".try_into().unwrap();
    let full_data = "full".try_into().unwrap();

    table.set(&prefix, &prefix_data).unwrap();
    table.set(&full, &full_data).unwrap();
    assert_eq!(Some(&prefix_data), table.get(&prefix));
    assert_eq!(Some(&full_data), table.get(&full));
}

#[test]
fn set_overrides_value() {
    let mut table = Table::new();
    let m_var = var_m("foo", &[]);
    let initial_value = "inital".try_into().unwrap();
    let end_value = "end".try_into().unwrap();

    table.set(&m_var, &initial_value).unwrap();
    assert_eq!(Some(&initial_value), table.get(&m_var));

    table.set(&m_var, &end_value).unwrap();
    assert_eq!(Some(&end_value), table.get(&m_var));
}

#[test]
fn do_a_bunch_of_sets() {
    let test_data = [
        (vec![], ""),
        (vec!["SomeKey0"], "someKey0"),
        (vec!["SomeKey1"], "someKey1"),
        (vec!["SomeKey2"], "someKey2"),
        (vec!["SomeKey3"], "someKey3"),
        (vec!["SomeKey4"], "someKey4"),
        (vec!["lots", "of", "Keys", "even", "more"], "lots of keys"),
    ];
    let test_data = test_data.map(|(keys, value)| (var_m("foo", &keys), value.try_into().unwrap()));

    let mut table = Table::new();
    for (var, value) in &test_data {
        table.set(var, value).unwrap();
    }

    for (var, value) in test_data {
        assert_eq!(Some(&value), table.get(&var));
    }
}

#[test]
fn kill_uninitialized_var() {
    let mut table = Table::new();
    //These should be noops.
    table.kill(&var_m("foo", &[]));
    table.kill(&var_m("foo", &["arg"]));
}

#[test]
fn kill_initialized_root() {
    let mut table = Table::new();
    let var = var_m("foo", &[]);
    let var_i = var_m("foo", &["i"]);
    let var_ii = var_m("foo", &["i", "ii"]);
    let data = "data".try_into().unwrap();
    table.set(&var, &data).unwrap();
    table.set(&var_i, &data).unwrap();
    table.set(&var_ii, &data).unwrap();
    table.kill(&var);
    assert_eq!(table.get(&var), None);
    assert_eq!(table.get(&var_i), None);
    assert_eq!(table.get(&var_i), None);

    //hash table should have freed the entire.
    assert_eq!(table.0.locate(&var.name), None);
}

#[test]
fn kill_initialized_index() {
    let mut table = Table::new();
    let var = var_m("foo", &[]);
    let var_i = var_m("foo", &["i"]);
    let var_ii = var_m("foo", &["i", "ii"]);
    let data = "data".try_into().unwrap();
    table.set(&var, &data).unwrap();
    table.set(&var_i, &data).unwrap();
    table.set(&var_ii, &data).unwrap();
    table.kill(&var_i);
    assert_eq!(table.get(&var), Some(&data));
    assert_eq!(table.get(&var_i), None);
    assert_eq!(table.get(&var_i), None);
}

#[test]
fn kill_removes_only_specified_index() {
    let mut table = Table::new();
    let data = "data".try_into().unwrap();
    let a = var_m("foo", &["a"]);
    let b = var_m("foo", &["b"]);
    let c = var_m("foo", &["c"]);

    table.set(&a, &data).unwrap();
    table.set(&b, &data).unwrap();
    table.set(&c, &data).unwrap();
    table.kill(&b);
    assert_eq!(table.get(&a), Some(&data));
    assert_eq!(table.get(&b), None);
    assert_eq!(table.get(&c), Some(&data));
}

#[test]
fn keep_vars() {
    let mut table = Table::new();
    let data = "data".try_into().unwrap();
    let dolor = var_m("$dolor", &[]);
    let normal = var_m("normal", &[]);
    let retain_a = var_m("retain_a", &[]);
    let retain_b = var_m("retain_b", &[]);
    table.set(&dolor, &data).unwrap();
    table.set(&normal, &data).unwrap();
    table.set(&retain_a, &data).unwrap();
    table.set(&retain_b, &data).unwrap();
    let mut part_tab = Default::default();

    table.keep(
        &[retain_a.name.clone(), retain_b.name.clone()],
        &mut part_tab,
    );
    assert_eq!(part_tab.src_var.uci, UCI_IS_LOCALVAR as u8);
    assert_eq!(part_tab.src_var.slen, 0);
    assert_eq!(part_tab.src_var.volset, 0);

    assert_eq!(table.get(&normal), None);
    for var in &[dolor, retain_a, retain_b] {
        assert_eq!(table.get(var), Some(&data));
    }
}
