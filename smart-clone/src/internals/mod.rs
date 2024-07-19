extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use enum_smart_clone::clone_enum_type;
use struct_smart_clone::clone_struct_type;

mod struct_smart_clone;
mod enum_smart_clone;

/// Defines if a structure or a field uses its default cloning
/// Or if its value is overridden by the given TokenStream.
enum CloneMode {
    Standard,
    Overridden(TokenStream),
}

/// Implementation for the #[derive(SmartClone)] macros.
pub fn smart_clone_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = syn::parse2(input).expect("Impossible to parse macro SmartClone");

    // Get the name of the struct
    let structure_name = &input.ident;

    // Process the data associated with the #[derive(SmartClone)].
    let cloned = match input.data {
        Data::Struct(data_struct) => clone_struct_type(structure_name, data_struct),
        Data::Enum(enum_struct) => clone_enum_type(structure_name, enum_struct),
        Data::Union(_) => return quote! { compile_error!("Cannot use SmartClone on union types.") },
    };

    // Generate the implementation of the Clone trait
    quote! {
        impl Clone for #structure_name {
            fn clone(&self) -> Self {
                #cloned
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::internals::smart_clone_derive;

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
                #[clone((u8, u8))]
                b: (u8, u8)
            }
        };
        let output = quote! {
            impl Clone for Foo {
                 fn clone(&self) -> Self {
                    Self {
                        a: foobar,
                        b: (u8, u8),
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
    fn test_clone_with() {
        let input = quote! {
            struct Baz {
                #[clone(clone_with = "youpi")]
                a: Vec<i32>,
                #[clone(clone_with = "Foo::bar")]
                z: Vec<i32>,
            }
        };
        let output = quote! {
            impl Clone for Baz {
                fn clone(&self) -> Self {
                    Self {
                        a: youpi(&self.a),
                        z: Foo::bar(&self.z),
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Impl with #[clone(clone_with =...)] tag: {}", result);
    }

    #[test]
    fn test_clone_default() {
        let input = quote! {
            struct Baz {
                #[clone(default)]
                a: Vec<i32>,
            }
        };
        let output = quote! {
            impl Clone for Baz {
                fn clone(&self) -> Self {
                    Self {
                        a: Default::default(),
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Impl with #[clone(default)] tag: {}", result);
    }

    #[test]
    fn test_enum_basic() {
        let input = quote! {
        enum SimpleEnum {
            A,
            B(i32, u32),
            C { x: u8, y: u8 },
        }
    };
        let output = quote! {
        impl Clone for SimpleEnum {
            fn clone(&self) -> Self {
                match self {
                    SimpleEnum::A => SimpleEnum::A,
                    SimpleEnum::B(v0, v1) => SimpleEnum::B(v0.clone(), v1.clone()),
                    SimpleEnum::C { x, y } => SimpleEnum::C { x: x.clone(), y: y.clone() },
                }
            }
        }
    };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Enum basic: {}", result);
    }

    #[test]
    fn test_enum_with_clone_attr() {
        let input = quote! {
            enum CustomCloneEnum {
                A,
                #[clone = CustomCloneEnum::B(8, 12)]
                B(i32, u32),
                #[clone(custom_clone_expr)]
                C { x: u8, y: u8 },
                #[clone(clone_with = "Try::func")]
                D,
                #[clone]
                E,
                #[clone(default)]
                F
            }
        };
        let output = quote! {
            impl Clone for CustomCloneEnum {
                fn clone(&self) -> Self {
                    match self {
                        CustomCloneEnum::A => CustomCloneEnum::A,
                        CustomCloneEnum::B(..) => CustomCloneEnum::B(8, 12),
                        CustomCloneEnum::C { x, y } => custom_clone_expr,
                        CustomCloneEnum::D => Try::func(self),
                        CustomCloneEnum::E => CustomCloneEnum::E,
                        CustomCloneEnum::F => Default::default (),
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Enum various tags: {}", result);
    }

    #[test]
    fn test_unit_type() {
        let input = quote! {
            struct UnitStruct;
        };
        let output = quote! {
            impl Clone for UnitStruct {
                fn clone (& self) -> Self {
                    Self { }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Unit type structures: {}", result);
    }

    #[test]
    fn test_unnamed_struct_type() {
        let input = quote! {
            struct UnnamedStruct(
                i32,
                #[clone = 3]
                u8,
                #[clone]
                Vec<u32>,
                #[clone(default)]
                i32,
                #[clone(custom_value)]
                vec![],
                #[clone(clone_with = "wow")]
                vec![1, 2, 3],
            );
        };
        let output = quote! {
            impl Clone for UnnamedStruct {
                fn clone (& self) -> Self {
                    UnnamedStruct {
                        0: self.0.clone(),
                        1: 3,
                        2: self.2.clone(),
                        3: Default::default(),
                        4: custom_value,
                        5: wow(&self.5),
                    }
                }
            }
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Unnamed structures: {}", result);
    }

    #[test]
    fn test_unsupported_types() {
        let input = quote! {
            union MyUnion {
                i: i32,
                f: f32,
            }
        };
        let output = quote! {
            compile_error ! ("Cannot use SmartClone on union types.")
        };
        let result = smart_clone_derive(input).to_string();
        assert_eq!(result, output.to_string(), "Unsupported union error: {}", result);
    }
}

