# hexf

Hexadecimal float support for Rust 1.15 or later.

```rust
#[macro_use] extern crate hexf;

assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
```

See [rust-lang/rust#1433](https://github.com/rust-lang/rust/issues/1433#issuecomment-288184018) for the context.

