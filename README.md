# hexf

Hexadecimal float support for Rust 1.15 or later. See rust-lang/rust#1433 for the context.

```rust
#[macro_use] extern crate hexf;

assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
```

