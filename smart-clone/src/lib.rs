extern crate proc_macro;

use proc_macro::TokenStream;

use crate::internals::smart_clone_derive;

mod internals;

/// # Smart Clone
///
/// This crate provides a custom derive macro called `SmartClone`.
///
/// `SmartClone` is a derive macro that replaces the standard `#[derive(Clone)]` in order to
/// implement a custom `impl Clone`.
/// The main difference is that `#[derive(SmartClone)]` allows you to use `#[clone(...)]` attributes
/// to customize the clone method for each field individually.
///
/// This means you can implement the `Clone` trait for a Rust structure (struct, enum, etc.) even if
/// not all fields implement `Clone`.
///
/// # Examples
///
/// ```
/// use smart_clone::SmartClone;
///
/// fn main() {
///   #[derive(SmartClone)]
///   struct Foo {
///       #[clone(12)]
///       a: i32, // will always be clone to value 12
///       #[clone("banana".to_owned())]
///       b: String, // this field will always clone to String `banana`
///       #[clone(default)]
///       c: Option<i32>, // this field will always be reset to default when Foo is cloned
///       #[clone(clone_with = "Foo::vec_clone")]
///       d: Vec<u32>, // uses a custom method to clone this field
///       #[clone("banana".to_owned())]
///       e: String,
///   }
///
///   #[derive(SmartClone, Default)]
///   enum SimpleEnum {
///       #[default]
///       A,
///       B(usize, usize), // will behave as usual
///       C { x: u8, y: u8 },
///       #[clone(SimpleEnum::D(8, 12))]
///       D(i32, u32),
///       #[clone(SimpleEnum::E { x: 3, y: 4 })]
///       E { x: u8, y: u8 },
///       #[clone(clone_with = "double")]
///       F { x: u8, y: u8 },
///       #[clone(default)]
///       G { x: u8, y: u8 },
///   }
/// }
/// ```
#[proc_macro_derive(SmartClone, attributes(clone))]
pub fn smart_clone_derive_macro(input: TokenStream) -> TokenStream {
    smart_clone_derive(input.into()).into()
}
