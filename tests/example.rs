#[macro_use] extern crate hexf;

use std::f64;

#[test]
fn test() {
    assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
    assert_eq!(hexf64!("0x1.999999999998ap-4"), 0.1f64 - f64::EPSILON);
}

