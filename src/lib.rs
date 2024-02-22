use weight_decorator_derive::weight;
use frame_support::pallet_prelude::Weight;

#[test]
fn test() {
    let test = weighted_do_something(2);
    println!("{}", test);
}

#[weight(Weight::zero())]
fn do_something(i: u32) -> u32 {
    i * i
}
