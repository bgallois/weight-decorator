use derive_weight::{derive_weight_expr, derive_weight_fn, derive_weight_result};
use frame_support::pallet_prelude::Weight;

#[test]
fn test_expr() {
    let test = weighted_do_something(2);
    assert_eq!(test, Weight::zero());
}

#[test]
fn test_fn() {
    let test = weighted_do_something_else(2, 4);
    assert_eq!(test, Weight::from_parts(10, 10));
}

#[test]
fn test_result() {
    let test = weighted_ok_something(0);
    assert_eq!(test, Weight::zero());

    let test = weighted_ok_something(1);
    assert_eq!(test, Weight::from_parts(10, 10));
}

#[derive_weight_expr(Weight::zero())]
fn do_something(i: u32) -> u32 {
    i * i
}

#[allow(dead_code)]
fn weight_for_do_something_else(_i: u32, _j: u32) -> Weight {
    Weight::from_parts(10, 10)
}

#[derive_weight_fn(weight_for_do_something_else)]
fn do_something_else(i: u32, j: u32) -> u32 {
    i * j
}

#[derive_weight_result((Weight::zero(), Weight::from_parts(10, 10)))]
fn ok_something(i: u32) -> Result<(), ()> {
    if i == 0 {
        Ok(())
    } else {
        Err(())
    }
}
