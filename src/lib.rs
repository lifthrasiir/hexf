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

use proc_macro::TokenStream;

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
    let lit = syn::parse_macro_input!(input as syn::LitStr);
    match hexf_parse::parse_hexf32(&lit.value(), true) {
        Ok(v) => format!("{:?}f32", v) // should keep the sign even for -0.0
            .parse()
            .expect("formatted a f32 literal"),
        Err(e) => syn::Error::new(lit.span(), format!("hexf32! failed: {}", e))
            .to_compile_error()
            .into(),
    }
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
    let lit = syn::parse_macro_input!(input as syn::LitStr);
    match hexf_parse::parse_hexf64(&lit.value(), true) {
        Ok(v) => format!("{:?}f64", v) // should keep the sign even for -0.0
            .parse()
            .expect("formatted a f64 literal"),
        Err(e) => syn::Error::new(lit.span(), format!("hexf64! failed: {}", e))
            .to_compile_error()
            .into(),
    }
}
