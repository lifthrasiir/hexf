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

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

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

const EXPECTED_STRING_LITERAL: &str = "expected string literal";

fn parse_stream<F>(input: TokenStream, process_literal: F) -> TokenStream
where
    F: FnOnce(&str, Span) -> TokenStream,
{
    let literal = match get_literal(input) {
        Ok(lit) => lit,
        Err(tokens) => return tokens,
    };

    match extract_string_value(&format!("{}", literal)) {
        Some(s) => process_literal(s, literal.span()),
        None => compile_error_stream(EXPECTED_STRING_LITERAL, literal.span()),
    }
}

fn get_literal(input: TokenStream) -> Result<Literal, TokenStream> {
    let mut trees = input.into_iter();
    let tree = trees.next().ok_or_else(|| {
        compile_error_stream(
            "unexpected end of input, expected string literal",
            Span::call_site(),
        )
    })?;

    if let Some(next_tree) = trees.next() {
        return Err(compile_error_stream("unexpected token", next_tree.span()));
    }

    match tree {
        TokenTree::Literal(literal) => Ok(literal),
        _ => Err(compile_error_stream(EXPECTED_STRING_LITERAL, tree.span())),
    }
}

fn extract_string_value(literal: &str) -> Option<&str> {
    if literal.starts_with('"') {
        if literal.len() >= 2 && literal.ends_with('"') {
            // For simplicity, we do not handle any escape sequences. Given the
            // usage of these macros, they would be of questionable utility.
            Some(&literal[1..(literal.len() - 1)])
        } else {
            None
        }
    } else if literal.starts_with('r') {
        let mut forward = literal.char_indices().skip(1);
        let mut reverse = literal.char_indices().rev();
        loop {
            let (forward_pos, forward_char) = forward.next()?;
            let (reverse_pos, reverse_char) = reverse.next()?;
            if forward_char != reverse_char || forward_pos >= reverse_pos {
                return None;
            }

            if forward_char == '"' {
                return Some(&literal[(forward_pos + 1)..reverse_pos]);
            }

            if forward_char != '#' {
                return None;
            }
        }
    } else {
        None
    }
}

fn compile_error_stream(message: &str, span: Span) -> TokenStream {
    fn with_span(mut token_tree: TokenTree, span: Span) -> TokenTree {
        token_tree.set_span(span);
        token_tree
    }

    let message_token = with_span(TokenTree::Literal(Literal::string(message)), span);

    let mut tokens = TokenStream::new();
    tokens.extend([
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Joint)), span),
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Alone)), span),
        TokenTree::Ident(Ident::new("core", span)),
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Joint)), span),
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Alone)), span),
        TokenTree::Ident(Ident::new("compile_error", span)),
        with_span(TokenTree::Punct(Punct::new('!', Spacing::Alone)), span),
        with_span(
            TokenTree::Group(Group::new(Delimiter::Brace, message_token.into())),
            span,
        ),
    ]);

    tokens
}
