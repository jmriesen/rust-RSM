use std::borrow::Borrow;

use super::Table;
use crate::m_var::test_helpers::var_m;
use pretty_assertions::{assert_eq, assert_ne};

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
    let data = "Data".try_into().unwrap();
    table.set(m_var.borrow(), &data).unwrap();

    let m_var = var_m("foo", &["bar"]);
    assert_eq!(table.get(&m_var), None);
}

#[test]
fn set_root_value() {
    let mut table = Table::new();
    let m_var = var_m("foo", &[]);
    let data = "Data".try_into().unwrap();

    table.set(&m_var, &data).unwrap();
    assert_eq!(table.get(&m_var), Some(&data));
}

#[test]
fn set_index_value() {
    let mut table = Table::new();
    let m_var = var_m("foo", &["keys"]);
    let data = "Data".try_into().unwrap();
    table.set(&m_var, &data).unwrap();
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
    //These should be no ops.
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

    //Hash table should have freed the entire.
    assert_eq!(table.table.locate(&var.name), None);
}

#[test]
fn keep_slot_open_if_variable_was_new_ed() {
    let mut table = Table::new();
    let var = var_m("foo", &[]);
    table.push_new_frame();
    table.new_var(&[&var.name]).unwrap();
    table.kill(&var);

    //Hash table should still have the slot reserved.
    assert_ne!(table.table.locate(&var.name), None);
    table.pop_new_frame();
    //Hash table should have freed the slot since it was not new-ed
    //by something else and the restored value holds no data.
    assert_eq!(table.table.locate(&var.name), None);
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

    table.keep(&[retain_a.name.clone(), retain_b.name.clone()]);

    assert_eq!(table.get(&normal), None);
    for var in &[dolor, retain_a, retain_b] {
        assert_eq!(table.get(var), Some(&data));
    }
}

#[test]
fn newing_stores_a_copy() {
    let mut table = Table::new();
    let var = var_m("var", &[]);

    let level_zero = "zero".try_into().unwrap();
    let level_one = "one".try_into().unwrap();
    let level_two = "two".try_into().unwrap();
    table.set(&var, &level_zero).unwrap();

    table.push_new_frame();
    table.new_var(&[&var.name]).unwrap();
    table.set(&var, &level_one).unwrap();

    table.push_new_frame();
    table.new_var(&[&var.name]).unwrap();
    table.set(&var, &level_two).unwrap();

    table.push_new_frame();
    table.new_var(&[&var.name]).unwrap();
    assert_eq!(table.get(&var), None);

    table.pop_new_frame();
    assert_eq!(table.get(&var), Some(&level_two));

    table.pop_new_frame();
    assert_eq!(table.get(&var), Some(&level_one));

    table.pop_new_frame();
    assert_eq!(table.get(&var), Some(&level_zero));
}

#[test]
fn assumed_variables_are_accessible() {
    let mut table = Table::new();
    let new_ed_var = var_m("var", &[]);
    let assumed_var = var_m("assumed", &[]);

    let assumed_value = "new-ed".try_into().unwrap();
    let new_value = "new-ed".try_into().unwrap();
    table.set(&assumed_var, &assumed_value).unwrap();

    table.push_new_frame();
    table.new_var(&[&new_ed_var.name]).unwrap();
    table.set(&new_ed_var, &assumed_value).unwrap();

    assert_eq!(table.get(&assumed_var), Some(&assumed_value));
    assert_eq!(table.get(&new_ed_var), Some(&new_value));
    table.pop_new_frame();
    assert_eq!(table.get(&assumed_var), Some(&assumed_value));
    assert_eq!(table.get(&new_ed_var), None);
}

#[test]
fn calling_new_on_the_same_variable_multiple_times() {
    let mut table = Table::new();
    let var = var_m("var", &[]);

    let initial_value = "inital".try_into().unwrap();
    let second_value = "second".try_into().unwrap();
    let third_value = "thired".try_into().unwrap();
    table.set(&var, &initial_value).unwrap();

    table.push_new_frame();

    table.new_var(&[&var.name]).unwrap();
    assert_eq!(table.get(&var), None);
    table.set(&var, &second_value).unwrap();

    table.new_var(&[&var.name]).unwrap();
    assert_eq!(table.get(&var), None);
    table.set(&var, &third_value).unwrap();

    table.pop_new_frame();
    assert_eq!(table.get(&var), Some(&initial_value));
}

#[test]
fn new_all_does_not_new_excluded_or_intrinsic_vars() {
    let mut table = Table::new();
    let intrinsic = var_m("$var", &[]);
    let included = var_m("included", &[]);
    let excluded = var_m("excluded", &[]);
    let value = "value".try_into().unwrap();

    table.set(&intrinsic, &value).unwrap();
    table.set(&included, &value).unwrap();
    table.set(&excluded, &value).unwrap();

    table.new_all_but(&[&excluded.name]).unwrap();
    //New-ed value
    assert_eq!(table.get(&included), None);
    //Not new-ed
    assert_ne!(table.get(&excluded), None);
    assert_ne!(table.get(&intrinsic), None);
}
