use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed, LitStr, Meta, Token, Variant};

use crate::CloneMode;

/**
 * Clone an enum type.
 */
pub(crate) fn clone_enum_type(identity: &Ident, fields: DataEnum) -> TokenStream {
    let clone_variants = fields.variants.iter().map(|variant| {
        // Check for the `#[clone...]` attribute
        match &variant.attrs.iter().find(|attr| attr.path().is_ident("clone")) {
            // Field is not marked: clone it as usual.
            None => clone_variant_fields(identity, variant, CloneMode::Standard),
            // Field is marked: smart clone it!
            Some(attr) => match &attr.meta {
                // Handle `#[clone]` by cloning as usual
                Meta::Path(_) => clone_variant_fields(identity, variant, CloneMode::Standard),
                // Handle #[clone = value].
                Meta::NameValue(item) => {
                    let value = &item.value;
                    clone_variant_fields(identity, variant, CloneMode::Overridden(quote! { #value }))
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
                                #func(self)
                            });
                        }
                        Ok(())
                    });

                    match clone_value {
                        None => clone_variant_fields(identity, variant, CloneMode::Overridden(quote! { #tokens })),
                        Some(value) => clone_variant_fields(identity, variant, CloneMode::Overridden(quote! { #value })),
                    }
                }
            }
        }
    });

    quote! {
        match self {
            #(#clone_variants,)*
        }
    }
}

/**
 * Clone unit type variant.
 */
fn clone_variant_fields(identity: &Ident, variant: &Variant, mode: CloneMode) -> TokenStream {
    match &variant.fields {
        Fields::Unit => clone_unit_fields(identity, &variant.ident, &variant.fields, mode),
        Fields::Unnamed(fields) => clone_unnamed_fields(identity, &variant.ident, fields, mode),
        Fields::Named(fields) => clone_named_fields(identity, &variant.ident, fields, mode),
    }
}

/**
 * Clone an unit field type: `A` annotated using smart clone #[clone...].
 */
fn clone_unit_fields(identity: &Ident, variant: &Ident, _: &Fields, mode: CloneMode) -> TokenStream {
    match mode {
        CloneMode::Standard => quote! { #identity::#variant => #identity::#variant },
        CloneMode::Overridden(value) => quote! { #identity::#variant => #value },
    }
}

/**
 * Clone an unnamed field type: `B(i32, u32)` annotated using smart clone #[clone...].
 */
fn clone_unnamed_fields(identity: &Ident, variant: &Ident, fields: &FieldsUnnamed, mode: CloneMode) -> TokenStream {
    // Construction of the fields identities (v0, v1, ....).
    let field_idents: Vec<_> = fields.unnamed.iter().enumerate().map(|(i, _)| {
        Ident::new(&format!("v{}", i), proc_macro2::Span::call_site())
    }).collect();

    match mode {
        CloneMode::Standard => quote! { #identity::#variant(#(#field_idents),*) => #identity::#variant(#(#field_idents.clone()),* ) },
        CloneMode::Overridden(value) => quote! { #identity::#variant(..) => #value },
    }
}

/**
 * Clone a named field type: `Point { x: u8, y: u8, ... }` annotated using smart clone #[clone...].
 */
fn clone_named_fields(identity: &Ident, variant: &Ident, fields: &FieldsNamed, mode: CloneMode) -> TokenStream {
    // Construction of the fields identities (x, y, ...).
    let field_idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

    match mode {
        CloneMode::Overridden(value) => quote! {  #identity::#variant { #(#field_idents),* } => #value },
        CloneMode::Standard => {
            // Loop through the fields of the named fields and clone it appropriately.
            let cloned_fields = fields.named.iter().map(|field| {
                let field_name = &field.ident;
                quote! { #field_name.clone() }
            });
            quote! {  #identity::#variant { #(#field_idents),* } => #identity::#variant { #(#field_idents: #cloned_fields),* } }
        }
    }
}