//! Hexadecimal float support for Rust 1.43 or later.
//!
//! ```rust
//! use hexf::{hexf32, hexf64};
//!
//! # fn main() {
//! assert_eq!(hexf32!("0x1.99999ap-4"), 0.1f32);
//! assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
//! # }
//! ```

mod tokens;

use proc_macro::{Literal, TokenStream, TokenTree};
use tokens::{compile_error_stream, parse_stream};

/// Expands to a `f32` value with given hexadecimal representation.
///
/// # Example
///
/// ```rust
/// # use hexf::hexf32; fn main() {
/// assert_eq!(hexf32!("0x1.99999ap-4"), 0.1f32);
/// # }
/// ```
#[proc_macro]
pub fn hexf32(input: TokenStream) -> TokenStream {
    parse_stream(input, |s, span| match hexf_parse::parse_hexf32(s, true) {
        Ok(v) => TokenTree::Literal(Literal::f32_suffixed(v)).into(),
        Err(e) => compile_error_stream(&format!("hexf32! failed: {}", e), span),
    })
}

/// Expands to a `f64` value with given hexadecimal representation.
///
/// # Example
///
/// ```rust
/// # use hexf::hexf64; fn main() {
/// assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
/// # }
/// ```
#[proc_macro]
pub fn hexf64(input: TokenStream) -> TokenStream {
    parse_stream(input, |s, span| match hexf_parse::parse_hexf64(s, true) {
        Ok(v) => TokenTree::Literal(Literal::f64_suffixed(v)).into(),
        Err(e) => compile_error_stream(&format!("hexf64! failed: {}", e), span),
    })
}
