# hexf

[![Chrono on Travis CI][travis-image]][travis]
[![Chrono on crates.io][cratesio-image]][cratesio]
[![Chrono on docs.rs][docsrs-image]][docsrs]

[travis-image]: https://travis-ci.org/lifthrasiir/hexf.svg?branch=master
[travis]: https://travis-ci.org/lifthrasiir/hexf
[cratesio-image]: https://img.shields.io/crates/v/hexf.svg
[cratesio]: https://crates.io/crates/hexf
[docsrs-image]: https://docs.rs/hexf/badge.svg
[docsrs]: https://docs.rs/hexf/

Hexadecimal float support for Rust 1.45 or later.

```rust
use hexf::hexf64;

assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
```

The literal is explicitly typed,
and should match to the pattern `SIGN "0x" INTEGRAL "." FRACTIONAL "p" EXPSIGN EXPDIGITS`, where:

* All Latin letters are matched case-insensitively;

* `SIGN` and `EXPSIGN` are either `+`, `-` or empty;

* `INTEGRAL` and `FRACTIONAL` are one or more hexadecimal digits,
  optionally separated by or ending with exactly one underscore (`_`) (but cannot begin with it);

* At least one of `INTEGRAL` or `FRACTIONAL` should be present
  (`1.0` or `.0` or `1.` is allowed, `1` is not);

* `EXPDIGITS` is decimal digits,
  optionally separated by or beginning or ending with exactly one underscore (`_`).

It is a compile-time error to put an invalid literal.

```rust,ignore
// hexf32! failed: invalid hexadecimal float literal
let invalid = hexf32!("42");
```

It is also a compile-time error to put a literal
that would be not exactly representable in the target type.

```rust,ignore
// hexf32! failed: cannot exactly represent float in target type
let inexact = hexf32!("0x1.99999bp-4");

// hexf32! failed: cannot exactly represent float in target type
let inexact_subnormal = hexf32!("0x1.8p-149");

// hexf64! failed: cannot exactly represent float in target type
let overflow = hexf64!("0x1.0p1024");

// hexf64! failed: cannot exactly represent float in target type
let underflow = hexf64!("0x1.0p-1075");
```

The crate (and also a standalone `hexf-parse` crate) provides
`parse_hexf32` and `parse_hexf64` functions,
which allows parsing failures (reported via a `ParseHexfError` type).
These functions will allow for interleaved underscores only if the second parameter is true;
this is added for the consistency, because Rust allows for underscores in numeric literals,
but not in the standard library (`"3_4".parse::<i32>()` is an error).

## How does it work?

This crate heavily relies on the fact that
the recent enough Rust compiler can correctly print *and* read a floating point number.
So the actual implementation of this crate is, well, done by
printing the parsed hexadecimal float back to the correct decimal digits,
which is picked up by the compiler to produce an exact bit pattern.

Wait, then what's the point of hexadecimal floats?
The answer is that **they are "invented" by ISO C99 to avoid implementation pitfalls**.
Ideally it should be possible to enumerate enough fractional digits
to get the correctly rounded bit pattern,
but many implementations didn't
(quite understandably, because it is actually [quite hard][dec2flt-paper]).
So the Standard has made a compromise:
in the conforming implementation decimal floats should parse to
very close to, but not exactly, the correctly rounded number:

> The significand part is interpreted as a (decimal or hexadecimal) rational number;
> the digit sequence in the exponent part is interpreted as a decimal integer. [...]
> For decimal floating constants,
> and also for hexadecimal floating constants when FLT_RADIX is not a power of 2,
> the result is **either the nearest representable value,
> or the larger or smaller representable value
> immediately adjacent to the nearest representable value**,
> chosen in an implementation-defined manner. [...]
>
> —ISO C99, Section 6.4.4.2 Floating constants, Paragraph 3 (emphases mine)

Indeed, it is relatively easier to parse decimal floats in that accuracy.
Hexadecimal floats are born out of this legacy, but Rust doesn't have to!
Hexadecimal floats can be still useful for manually writing float bits down,
or for converting from other languages, however.
This crate exists for those rarer use cases.

See [rust-lang/rust#1433][issue-1433] for the more context.

[dec2flt-paper]: http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.45.4152
[issue-1433]: https://github.com/rust-lang/rust/issues/1433#issuecomment-288184018

