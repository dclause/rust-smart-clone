[![Build Status](https://github.com/dclause/rust-smart-clone/workflows/Build/badge.svg)](https://github.com/dclause/rust-smart-clone/actions/workflows/build.yml)
[![Test Status](https://github.com/dclause/rust-smart-clone/workflows/Test/badge.svg)](https://github.com/dclause/rust-smart-clone/actions/workflows/test.yml)
[![Code Coverage](https://codecov.io/gh/dclause/rust-smart-clone/graph/badge.svg?token=BKN5I1G5CU)](https://codecov.io/gh/dclause/rust-smart-clone)
[![Latest Version](https://img.shields.io/crates/v/smart-clone.svg)](https://crates.io/crates/smart-clone)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](???)

# Rust SmartClone

Rust custom-derive macro for Clone with more control on the fields cloned values.

```rust
#[derive(SmartClone)]
struct Foo {
    a: u8, // left for standard clone
    #[clone = 12]
    b: u8, // Override with hardcoded value (same as `#[clone(12)]`)
    #[clone(TEST)]
    c: u8, // In general, prefer this syntax for hardcode, variable or const
    #[clone((42, 69))]
    d: (i32, u32),
    #[clone(default)]
    e: Vec<Vec<Vec<(u8, u8)>>>, // Reserved 'skip' keyword to clone to Default::default() value (g type must implement `Default`)
    #[clone(Some(Default::default()))] // `Some(Default::default())` is not `None` but `Some(0)` !
    f: Option<i32>,
    #[clone(clone_with = "SimpleStruct::vec_clone")]
    g: Vec<u32>,
    #[clone("banana".to_owned())]
    h: String,
}
 ```

Will generate:

```rust
impl Clone for SimpleStruct {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: 12,
            c: TEST,
            d: (42, 69),
            e: Default::default(),
            f: Some(Default::default()),
            g: SimpleStruct::vec_clone(&self.j),
            h: "banana".to_owned(),
        }
    }
}
```

## Install

@todo when published on cargo

## Examples

See the [examples](https://github.com/dclause/rust-smart-clone/blob/develop/examples) folder for more usage with enums,
unnamed structures, etc...
