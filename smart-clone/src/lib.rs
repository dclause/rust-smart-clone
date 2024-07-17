extern crate proc_macro;

use proc_macro::TokenStream;
use smart_clone_macros::smart_clone_derive;

#[proc_macro_derive(SmartClone, attributes(clone))]
pub fn smart_clone_derive_macro(input: TokenStream) -> TokenStream {
    smart_clone_derive(input.into()).into()
}
