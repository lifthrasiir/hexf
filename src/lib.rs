#[macro_use] extern crate proc_macro_hack;
#[macro_use] #[allow(unused_imports)] extern crate hexf_impl;
extern crate hexf_parse;

pub use hexf_parse::{ParseHexfError, parse_hexf32, parse_hexf64};
pub use hexf_impl::*;

proc_macro_expr_decl!(hexf32! => hexf32_impl);
proc_macro_expr_decl!(hexf64! => hexf64_impl);

