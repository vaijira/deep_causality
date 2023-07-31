// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use deep_causality::prelude::*;
use deep_causality::utils::test_utils::{get_test_causality_array, get_test_causality_data};

#[test]
fn test_all_active()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
}

#[test]
fn test_number_active()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
    assert_eq!(10.0, col.number_active());
}

#[test]
fn test_percent_active()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
    assert_eq!(10.0, col.number_active());
    assert_eq!(100.0, col.percent_active());
}

#[test]
fn test_get_all_items() {
    let col = get_test_causality_array();
    let all_items = col.get_all_items();

    let exp_len = col.len();
    let actual_len = all_items.len();
    assert_eq!(exp_len, actual_len);
}

#[test]
fn test_get_all_active_causes()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
    assert_eq!(10, col.get_all_active_causes().len());
}

#[test]
fn test_get_all_inactive_causes()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
    assert_eq!(0, col.get_all_inactive_causes().len());
}


#[test]
fn test_reason_all_causes()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let data = get_test_causality_data();

    let res = col.reason_all_causes(&data);
    assert!(res.is_ok());

    let valid = res.unwrap();
    assert!(valid);
}

#[test]
fn test_explain()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }

    let expected = format!("\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n\n * Causaloid: 1 tests whether data exceeds threshold of 0.55 on last data 0.99 evaluated to true\n");
    let actual = col.explain();
    assert_eq!(expected, actual);
}

#[test]
fn test_len()
{
    let col = get_test_causality_array();
    assert_eq!(10, col.len());
}

#[test]
fn test_is_empty()
{
    let col = get_test_causality_array();
    assert!(!col.is_empty());
}

#[test]
fn test_to_vec()
{
    let col = get_test_causality_array();
    assert_eq!(10, col.to_vec().len());
}