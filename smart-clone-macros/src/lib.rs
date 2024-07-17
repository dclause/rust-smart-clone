extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Expr, Fields, Meta, Token};

pub fn smart_clone_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = match syn::parse2(input) {
        Ok(parsed_input) => parsed_input,
        Err(error) => return error.to_compile_error().into(),
    };

    // Get the name of the struct
    let name = input.ident;

    // Get the fields of the struct
    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => return quote! { compile_error!("Only named fields are supported"); }.into(),
        },
        _ => return quote! { compile_error!("Only structs are supported"); }.into(),
    };

    // Process each field to determine how it should be cloned.
    let clone_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let mut custom_clone = None;

        // Check for the `clone` attribute
        for attr in &field.attrs {
            if attr.path().is_ident("clone") {
                custom_clone = Some(attr);
            }
        }

        match custom_clone {
            // no #[custom...] found
            None => {
                // Default clone behavior
                quote! {
                    #field_name: self.#field_name.clone()
                }
            }
            Some(attr) => {
                match &attr.meta {
                    // #[clone = ....]
                    Meta::NameValue(value) => {
                        match &value.value {
                            data => {
                                quote! {
                                    #field_name: #data
                                }
                            }
                        }
                    }
                    // #[clone(...)]
                    Meta::List(list) => {
                        let mut expr: Option<TokenStream> = None;
                        match list.parse_args::<Expr>() {
                            Ok(data) => {
                                expr = Some(data.to_token_stream());
                            }
                            Err(_) => {
                                let _ = list.parse_nested_meta(|meta| {
                                    // #[clone(fn = ...)]
                                    if meta.path.is_ident("fn") {
                                        if meta.input.peek(Token![=]) {
                                            if expr.is_none() {
                                                let func: Expr = meta.value()?.parse()?;
                                                expr = Some(quote!(#func(self.#field_name.clone())));
                                            }
                                            // return Err(syn::Error::new(field.span(), "Only one fn function supported"))
                                        }
                                    }
                                    Ok(())
                                });
                            }
                        }

                        quote! {
                            #field_name: #expr
                        }
                    }
                    _ => {
                        // Default clone behavior
                        quote! {
                            #field_name: self.#field_name.clone()
                        }
                    }
                }
            }
        }
    });

    // Generate the implementation of the Clone trait
    let gen = quote! {
        impl Clone for #name {
            fn clone(&self) -> Self {
                Self {
                    #(#clone_fields,)*
                }
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    gen.into()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::smart_clone_derive;

    #[test]
    fn test_struct_with_no_clone_attr() {
        let input = quote! {
            struct Bar {
                x: f64,
                #[clone]
                y: String,
            }
        };
        let output = quote! {
            impl Clone for Bar {
                fn clone(&self) -> Self {
                    Self {
                        x: self.x.clone(),
                        y: self.y.clone(),
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Impl with #[clone] or NO tag: {}", result);
    }

    #[test]
    fn test_with_equal_syntax() {
        let input = quote! {
            struct Foo {
                #[clone = foobar]
                a: u32,
            }
        };
        let output = quote! {
            impl Clone for Foo {
                 fn clone(&self) -> Self {
                    Self {
                        a: foobar,
                   }
                }
            }
        };

        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Impl with #[clone = ...] tag: {}", result);
    }

    #[test]
    fn test_with_expr() {
        let input = quote! {
            struct Baz {
                #[clone(custom_clone_expr)]
                z: Vec<i32>,
            }
        };
        let output = quote! {
            impl Clone for Baz {
                fn clone(&self) -> Self {
                    Self {
                        z: custom_clone_expr,
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Impl with #[clone(...)] tag: {}", result);
    }

    #[test]
    fn test_with_fn() {
        let input = quote! {
            struct Baz {
                #[clone(fn = youpi)]
                a: Vec<i32>,
                #[clone(fn = Foo::bar)]
                z: Vec<i32>,
            }
        };
        let output = quote! {
            impl Clone for Baz {
                fn clone(&self) -> Self {
                    Self {
                        a: youpi(self.a.clone()),
                        z: Foo::bar(self.z.clone()),
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Impl with #[clone(fn = ...)] tag: {}", result);
    }

    // #[test]
    // fn test_enum_basic() {
    //     let input = quote! {
    //     enum SimpleEnum {
    //         A,
    //         B(i32),
    //         C { x: u8, y: u8 },
    //     }
    // };
    //     let output = quote! {
    //     impl Clone for SimpleEnum {
    //         fn clone(&self) -> Self {
    //             match self {
    //                 SimpleEnum::A => SimpleEnum::A,
    //                 SimpleEnum::B(val) => SimpleEnum::B(val.clone()),
    //                 SimpleEnum::C { x, y } => SimpleEnum::C { x: x.clone(), y: y.clone() },
    //             }
    //         }
    //     }
    // };
    //     let result = smart_clone_derive(input).to_string();
    //     assert_eq!(result, output.to_string(), "Enum basic: {}", result);
    // }
    // 
    // #[test]
    // fn test_enum_with_clone_attr() {
    //     let input = quote! {
    //         enum CustomCloneEnum {
    //             A,
    //             #[clone = Some(42)]
    //             B,
    //             #[clone(custom_clone_expr)]
    //             C,
    //         }
    //     };
    //     let output = quote! {
    //         impl Clone for CustomCloneEnum {
    //             fn clone(&self) -> Self {
    //                 match self {
    //                     CustomCloneEnum::A => CustomCloneEnum::A,
    //                     CustomCloneEnum::B => CustomCloneEnum::B(Some(42)),
    //                     CustomCloneEnum::C => CustomCloneEnum::C(custom_clone_expr),
    //                 }
    //             }
    //         }
    //     };
    //     let result = smart_clone_derive(input).to_string();
    //     assert_eq!(result, output.to_string(), "Enum various tags: {}", result);
    // }
}

