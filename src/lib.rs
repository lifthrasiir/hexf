//! Hexadecimal float support for Rust 1.15 or later.
//!
//! ```rust
//! #[macro_use] extern crate hexf;
//!
//! # fn main() {
//! assert_eq!(hexf32!("0x1.99999ap-4"), 0.1f32);
//! assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
//! # }
//! ```

pub use hexf_impl::{hexf32, hexf64};
pub use hexf_parse::{parse_hexf32, parse_hexf64, ParseHexfError};
