#![allow(clippy::float_cmp)]

use hexf::{hexf32, hexf64};

use std::f64;

#[test]
fn basic() {
    assert_eq!(hexf32!("0x1.99999ap-4"), 0.1f32);
    assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
    assert_eq!(hexf64!("0x1.999999999998ap-4"), 0.1f64 - f64::EPSILON);
}

#[test]
fn negative() {
    assert_eq!(hexf32!("-0x1.99999ap-4"), -0.1f32);
    assert_eq!(hexf64!("-0x1.999999999999ap-4"), -0.1f64);
    assert_eq!(hexf64!("-0x1.999999999998ap-4"), -0.1f64 + f64::EPSILON);
    assert!(hexf32!("-0x0.0p0").is_sign_negative());
    assert!(hexf64!("-0x0.0p0").is_sign_negative());
}

#[test]
fn zeroes() {
    assert_eq!(1.0f64 / hexf64!("0x0.0p0"), f64::INFINITY);
    assert_eq!(1.0f64 / hexf64!("-0x0.0p0"), f64::NEG_INFINITY);
}

#[test]
fn syntax() {
    assert_eq!(hexf32!("0x1.0p0"), 1.0f32);
    assert_eq!(hexf64!("0x1.0p0"), 1.0f64);
    // Raw string literals are handled
    assert_eq!(hexf32!(r"0x1.0p0"), 1.0f32);
    assert_eq!(hexf64!(r"0x1.0p0"), 1.0f64);
    assert_eq!(hexf32!(r##"-0x2.8p0"##), -2.5f32);
    assert_eq!(hexf64!(r##"-0x2.8p0"##), -2.5f64);
}

#[test]
#[rustfmt::skip]
fn syntax_whitespace() {
    assert_eq!(hexf32!(  "0x1.0p0"  ), 1.0f32);
    assert_eq!(hexf64!(
        "-0x1.8p0"
    ), -1.5f64);
}
