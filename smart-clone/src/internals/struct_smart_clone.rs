use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Field, Fields, LitInt, LitStr, Meta, Token};
use syn::punctuated::Punctuated;

enum StructType {
    Named,
    Unnamed,
    // Unit,
}

/// Clone an enum.
pub fn clone_struct_type(identity: &Ident, data_struct: DataStruct) -> TokenStream {
    match &data_struct.fields {
        Fields::Named(fields) => {
            let cloned_fields = clone_fields(StructType::Named, &fields.named);
            quote! { Self { #cloned_fields } }
        }
        Fields::Unnamed(fields) => {
            let cloned_fields = clone_fields(StructType::Unnamed, &fields.unnamed);
            quote! { #identity { #cloned_fields } }
        }
        Fields::Unit => quote! { Self { } }
    }
}

/// Convert fields according to there type.
fn clone_fields(struct_type: StructType, fields: &Punctuated<Field, Token![,]>) -> TokenStream {
    let clone_fields = fields.iter().enumerate().map(|(i, field)| {
        let field_name = &field.ident;
        let field_id = LitInt::new(&format!("{}", i), proc_macro2::Span::call_site());

        // Check for the `#[clone...]` attribute
        match &field.attrs.iter().find(|attr| attr.path().is_ident("clone")) {
            // Field is not marked: clone it as usual.
            None => match struct_type {
                StructType::Named => quote! { #field_name: self.#field_name.clone() },
                StructType::Unnamed => quote! { #field_id: self.#field_id.clone() },
            }
            // Field is marked: smart clone it!
            Some(attr) => match &attr.meta {
                // Handle `#[clone]` by cloning as usual
                Meta::Path(_) => match struct_type {
                    StructType::Named => quote! { #field_name: self.#field_name.clone() },
                    StructType::Unnamed => quote! { #field_id: self.#field_id.clone() },
                }
                // Handle #[clone = value].
                Meta::NameValue(item) => {
                    let value = &item.value;
                    match struct_type {
                        StructType::Named => quote! { #field_name: #value },
                        StructType::Unnamed => quote! { #field_id: #value },
                    }
                }
                // Handle `#[clone(item1, item2)]` as `#[clone(items)]`.
                Meta::List(items) => {
                    let tokens = items.tokens.clone();

                    // Case #[clone(...)]
                    let mut clone_value: Option<TokenStream> = None;
                    let _ = items.parse_nested_meta(|meta| {
                        // `#[clone(default)]` => clone with default value
                        if meta.path.is_ident("default") {
                            clone_value = Some(quote! {
                                Default::default()
                            });
                        }
                        // `#[clone(clone_with =func)]`
                        if meta.path.is_ident("clone_with") && meta.input.peek(Token![=]) {
                            let func: LitStr = meta.value()?.parse()?;
                            let func: TokenStream = func.parse()?;
                            clone_value = match struct_type {
                                StructType::Named => Some(quote! {
                                    #func(&self.#field_name)
                                }),
                                StructType::Unnamed => Some(quote! {
                                    #func(&self.#field_id)
                                }),
                            };
                        }
                        Ok(())
                    });

                    let value = clone_value.unwrap_or_else(|| tokens);
                    match struct_type {
                        StructType::Named => quote! { #field_name: #value },
                        StructType::Unnamed => quote! { #field_id: #value },
                    }
                }
            }
        }
    });

    quote! {
        #(#clone_fields,)*
    }
}