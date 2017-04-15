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

#[macro_use] extern crate proc_macro_hack;
#[macro_use] #[allow(unused_imports)] extern crate hexf_impl;
extern crate hexf_parse;

pub use hexf_parse::{ParseHexfError, parse_hexf32, parse_hexf64};
#[doc(hidden)] pub use hexf_impl::*;

proc_macro_expr_decl! {
    /// Expands to a `f32` value with given hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate hexf; fn main() {
    /// assert_eq!(hexf32!("0x1.99999ap-4"), 0.1f32);
    /// # }
    /// ```
    hexf32! => hexf32_impl
}

proc_macro_expr_decl! {
    /// Expands to a `f64` value with given hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate hexf; fn main() {
    /// assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
    /// # }
    /// ```
    hexf64! => hexf64_impl
}

