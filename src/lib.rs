use derive_weight::{derive_weight_expr, derive_weight_fn, derive_weight_result};
use frame_support::pallet_prelude::Weight;

#[test]
fn test_expr() {
    assert_eq!(weighted_do_something(2), Weight::zero());
}

#[test]
fn test_fn() {
    assert_eq!(weighted_do_something_else(0, 4), Weight::from_parts(10, 10));
    assert_eq!(weighted_do_something_else(1, 4), Weight::from_parts(20, 20));
}

#[test]
fn test_result() {
    assert_eq!(weighted_ok_something(0), Weight::zero());
    assert_eq!(weighted_ok_something(1), Weight::from_parts(10, 10));
}

#[derive_weight_expr(Weight::zero())]
fn do_something(i: u32) -> u32 {
    i * i
}

#[allow(dead_code)]
fn weight_for_do_something_else(i: u32, _j: u32) -> Weight {
    if i == 0 {
        Weight::from_parts(10, 10)
    }
    else {
        Weight::from_parts(20, 20)
    }
}

#[derive_weight_fn(weight_for_do_something_else)]
fn do_something_else(i: u32, j: u32) -> u32 {
    if i == 0 {
        i + j
    }
    else {
        i/j
    }
}

#[derive_weight_result((Weight::zero(), Weight::from_parts(10, 10)))]
fn ok_something(i: u32) -> Result<(), ()> {
    if i == 0 {
        Ok(())
    } else {
        Err(())
    }
}
