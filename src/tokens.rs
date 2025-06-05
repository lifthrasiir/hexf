use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

const EXPECTED_STRING_LITERAL: &str = "expected string literal";

enum ExtractStringError {
    BadLiteral,
    EscapeSequence,
}

pub fn parse_stream<F>(input: TokenStream, process_literal: F) -> TokenStream
where
    F: FnOnce(&str, Span) -> TokenStream,
{
    let literal = match get_literal(input) {
        Ok(lit) => lit,
        Err(tokens) => return tokens,
    };

    match extract_string_value(&format!("{}", literal)) {
        Ok(s) => process_literal(s, literal.span()),
        Err(ExtractStringError::BadLiteral) => {
            compile_error_stream(EXPECTED_STRING_LITERAL, literal.span())
        }
        Err(ExtractStringError::EscapeSequence) => {
            compile_error_stream("Escape sequences are not supported", literal.span())
        }
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

fn extract_string_value(literal: &str) -> Result<&str, ExtractStringError> {
    if literal.starts_with('"') {
        if literal.len() >= 2 && literal.ends_with('"') {
            // For simplicity, we do not handle any escape sequences. Given the
            // usage of these macros, they would be of questionable utility.
            if literal.contains('\\') {
                return Err(ExtractStringError::EscapeSequence);
            }

            Ok(&literal[1..(literal.len() - 1)])
        } else {
            Err(ExtractStringError::BadLiteral)
        }
    } else if literal.starts_with('r') {
        let mut forward: std::iter::Skip<std::str::CharIndices<'_>> =
            literal.char_indices().skip(1);
        let mut reverse = literal.char_indices().rev();
        loop {
            let (forward_pos, forward_char) =
                forward.next().ok_or(ExtractStringError::BadLiteral)?;
            let (reverse_pos, reverse_char) =
                reverse.next().ok_or(ExtractStringError::BadLiteral)?;
            if forward_char != reverse_char || forward_pos >= reverse_pos {
                return Err(ExtractStringError::BadLiteral);
            }

            if forward_char == '"' {
                return Ok(&literal[(forward_pos + 1)..reverse_pos]);
            }

            if forward_char != '#' {
                return Err(ExtractStringError::BadLiteral);
            }
        }
    } else {
        return Err(ExtractStringError::BadLiteral);
    }
}

pub fn compile_error_stream(message: &str, span: Span) -> TokenStream {
    let with_span = |mut token_tree: TokenTree| {
        token_tree.set_span(span);
        token_tree
    };

    let message_token = with_span(TokenTree::Literal(Literal::string(message)));

    // ::core::compile_error!(<message_token>)
    let mut tokens = TokenStream::new();
    tokens.extend([
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Joint))),
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
        TokenTree::Ident(Ident::new("core", span)),
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Joint))),
        with_span(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
        TokenTree::Ident(Ident::new("compile_error", span)),
        with_span(TokenTree::Punct(Punct::new('!', Spacing::Alone))),
        with_span(TokenTree::Group(Group::new(
            Delimiter::Brace,
            message_token.into(),
        ))),
    ]);

    tokens
}
