# derive-from-one
[<img alt="github" src="https://img.shields.io/badge/github-shakacode/derive-from-one-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/alex35mil/derive-from-one)
[<img alt="crates.io" src="https://img.shields.io/crates/v/derive-from-one.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/derive-from-one)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-derive-from-one-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/derive-from-one)
<!-- cargo-sync-readme start -->

Autogeneration of `From` impls for enums and structs.

## Installation

Add to `Cargo.toml`:

```toml
derive-from-one = "0.1"
```

## Usage
This macro generates `From` impls for enum constructors with a single field.

If you don't want some impls to be generated, apply `#[from(skip)]` to this tag.

Also, the macro would automatically skip ambiguous types, i.e., when a type appears in multiple tags regardless of amount of fields.

Example:

```rust
use derive_from_one::FromOne;

#[derive(FromOne)]
enum Enum {
    A { a: u32 },           // GENERATES From impl
    B(bool),                // GENERATES From impl
    #[from(skip)]
    C(String),              // DOES NOT GENERATE From impl due to #[from(skip)] attribute
    D {                     // DOES NOT GENERATE From impl due to multiple fields
        foo: Vec<String>,
        bar: &'static str,
    },
    E(Vec<String>)          // DOES NOT GENERATE From impl due to Vec<String> appears in D tag
}
```

Generated code:

```rust
enum Enum {
    A { a: u32 },
    B(bool),
    C(String),
    D {
        foo: Vec<String>,
        bar: &'static str,
    },
    E(Vec<String>)
}

impl From<u32> for Enum {
    fn from(x: u32) -> Self {
        Self::A { a: x }
    }
}

impl From<bool> for Enum {
    fn from(x: bool) -> Self {
        Self::B(x)
    }
}
```

You can also apply this macro to structs with a single field.

```rust
use derive_from_one::FromOne;

#[derive(FromOne)]
struct StructOne(usize);

#[derive(FromOne)]
struct StructTwo { a: usize }
```

<!-- cargo-sync-readme end -->

## License
MIT.
