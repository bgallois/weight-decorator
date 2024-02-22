use frame_support::pallet_prelude::Weight;
use weight_decorator_derive::weight;
use weight_decorator_derive::weight_function;

#[test]
fn test_exp() {
    let test = weighted_do_something(2);
    assert_eq!(test, Weight::zero());
}

#[test]
fn test_func() {
    let test = weighted_do_something_else(2, 4);
    assert_eq!(test, Weight::from_parts(10, 10));
}

#[weight(Weight::zero())]
fn do_something(i: u32) -> u32 {
    i * i
}

#[allow(dead_code)]
fn weight_for_do_something_else(_i: u32, _j: u32) -> Weight {
    Weight::from_parts(10, 10)
}

#[weight_function(weight_for_do_something_else)]
fn do_something_else(i: u32, j: u32) -> u32 {
    i * j
}
