[![Build Status](https://github.com/dclause/rust-smart-clone/workflows/Build/badge.svg)](https://github.com/dclause/rust-smart-clone/actions/workflows/build.yml)
[![Test Status](https://github.com/dclause/rust-smart-clone/workflows/Test/badge.svg)](https://github.com/dclause/rust-smart-clone/actions/workflows/test.yml)
[![Code Coverage](https://codecov.io/gh/dclause/rust-smart-clone/graph/badge.svg?token=BKN5I1G5CU)](https://codecov.io/gh/dclause/rust-smart-clone)
[![Latest Version](https://img.shields.io/crates/v/smart-clone.svg)](https://crates.io/crates/smart-clone)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/smart-clone)

# Rust SmartClone

Rust custom-derive macro for implementing `Clone` trait with more control on the fields cloned values.

```rust
use smart_clone::SmartClone;

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
            g: SimpleStruct::vec_clone(&self.g),
            h: "banana".to_owned(),
        }
    }
}
```

## Install

- Add `smart-clone = "0.1.0"` as a dependency in Cargo.toml.
- On structs and enums that you want to clone, import the derive macro as `use smart_clone::SmartClone;` within the same
  module and write `#[derive(SmartClone)]` on the struct or enum.
- Use the attribute `#[clone(...)]` with the option you need on the desired structure field.

## API options

- `#[clone]`: will perform cloning as usual for your field. Equivalent to no annotation.
- `#[clone = xxx]`: will set the value `xxx` to the field when the structure is cloned
- `#[clone(xxx)]`: same as above, but `xxx` can be whatever you want here, not just a literal
- `#[clone(clone_with = "xxx")]`: the field will be passed by reference to a function called `xxx` and the
  returned value will be used when the structure is cloned.

## Examples

See the [examples](https://github.com/dclause/rust-smart-clone/blob/develop/examples) folder for various use cases.
