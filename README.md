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
    b: (i32, u32),
    c: Option<i32>,
    d: Vec<u32>,
    #[clone = 12]
    e: u8, // Override with hardcoded value (same as `#[clone(12)]`)
    #[clone(TEST)]
    f: u8, // In general, prefer this syntax for hardcode, variable or const
    #[clone((42, 69))]
    g: (i32, u32),
    #[clone(default)]
    h: Vec<Vec<Vec<(u8, u8)>>>, // Reserved 'skip' keyword to clone to Default::default() value (g type must implement `Default`)
    #[clone(Some(Default::default()))] // `Some(Default::default())` is not `None` but `Some(0)` !
    i: Option<i32>,
    #[clone(clone_with = "SimpleStruct::vec_clone")]
    j: Vec<u32>,
    #[clone("banana".to_owned())]
    k: String,
}
 ```

Will generate:

```rust
impl Clone for SimpleStruct {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            d: self.d.clone(),
            e: 12,
            f: TEST,
            g: (42, 69),
            h: Default::default(),
            i: Some(Default::default()),
            j: SimpleStruct::vec_clone(&self.j),
            k: "banana".to_owned(),
        }
    }
}
```

## Install

@todo when published on cargo

## Examples

See the [examples](https://github.com/dclause/rust-smart-clone/blob/develop/examples) folder for more usage with enums,
unnamed structures, etc...
