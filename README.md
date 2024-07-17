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
    #[clone = 12]
    a: i32,
    b: i32,
    #[clone(Some(Default::default()))]
    c: Option<i32>,
    #[clone(fn = Foo::vec_clone)]
    d: Vec<u32>,
    #[clone("banana".to_owned())]
    e: String,
}
 ```

Will generate:

```rust
impl Clone for Foo {
    fn clone(&self) -> Self {
        Self {
            a: 12,
            b: self.b.clone(),
            c: Some(Default::default()),
            d: Foo::vec_clone(self.d.clone()),
            e: "banana".to_owned()
        }
    }
}
```

## Install

@todo when published on cargo

## Examples

See the [examples](https://github.com/dclause/rust-smart-clone/blob/develop/examples) folder for more.
