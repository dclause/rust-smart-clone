use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, FieldsNamed, LitStr, Meta, Token};

/**
 * Clone an enum.
 */
pub fn clone_struct_type(data_struct: DataStruct) -> TokenStream {
    let cloned_fields = match &data_struct.fields {
        Fields::Named(named_fields) => clone_named_fields(named_fields),
        Fields::Unnamed(_) => quote! { compile_error!("Cannot use SmartClone on tuple struct types."); },
        Fields::Unit => quote! { compile_error!("Cannot use SmartClone on unit struct types."); },
    };

    quote! {
        Self {
            #cloned_fields
        }
    }
}

/**
 * Clone a named field structure type: `Point { x: u8, y: u8 }` annotated using smart clone #[clone...].
 */
fn clone_named_fields(fields: &FieldsNamed) -> TokenStream {
    // Loop through the fields of the named fields and clone it appropriately.
    let clone_fields = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        // Check for the `#[clone...]` attribute
        match &field.attrs.iter().find(|attr| attr.path().is_ident("clone")) {
            // Field is not marked: clone it as usual.
            None => quote! {
                #field_name: self.#field_name.clone()
            },
            // Field is marked: smart clone it!
            Some(attr) => match &attr.meta {
                // Handle `#[clone]` by cloning as usual
                Meta::Path(_) => quote! {
                    #field_name: self.#field_name.clone()
                },
                // Handle #[clone = value].
                Meta::NameValue(item) => {
                    let value = &item.value;
                    quote! {
                        #field_name: #value
                    }
                }
                // Handle `#[clone(item1, item2)]` as `#[clone(items)]`.
                Meta::List(items) => {
                    let tokens = &items.tokens;

                    // Case #[clone(...)]
                    let mut clone_value = None::<TokenStream>;
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
                            clone_value = Some(quote! {
                                #func(&self.#field_name)
                            });
                        }
                        Ok(())
                    });

                    match clone_value {
                        None => quote! { #field_name: #tokens },
                        Some(value) => quote! {
                            #field_name: #value
                        },
                    }
                }
            }
        }
    });
    quote! {
        #(#clone_fields,)*
    }
}