//! Support library for `hexf`. Do not use directly.

#[macro_use]
extern crate proc_macro_hack;
extern crate hexf_parse;
extern crate syn;

proc_macro_expr_impl! {
    /// Support function for `hexf32!` macro. Do not use directly.
    #[doc(hidden)]
    pub fn hexf32_impl(input: &str) -> String {
        let lit = syn::parse_str::<syn::LitStr>(input).expect("hexf32! requires a single string literal");
        match hexf_parse::parse_hexf32(&lit.value(), true) {
            Ok(v) => format!("{:?}f32", v), // should keep the sign even for -0.0
            Err(e) => panic!("hexf32! failed: {}", e),
        }
    }
}

proc_macro_expr_impl! {
    /// Support function for `hexf64!` macro. Do not use directly.
    #[doc(hidden)]
    pub fn hexf64_impl(input: &str) -> String {
        let lit = syn::parse_str::<syn::LitStr>(input).expect("hexf64! requires a single string literal");
        match hexf_parse::parse_hexf64(&lit.value(), true) {
            Ok(v) => format!("{:?}f64", v), // should keep the sign even for -0.0
            Err(e) => panic!("hexf64! failed: {}", e),
        }
    }
}
