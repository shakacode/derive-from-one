extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, token, DeriveInput, Error as SyntaxError, Fields, Ident, Type, Variant,
};

enum Field<'a> {
    Named(&'a syn::Field),
    Unnamed(&'a syn::Field),
}

#[proc_macro_derive(FromOne, attributes(from))]
pub fn derive_from_one(input: CompilerTokenStream) -> CompilerTokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    match &input.data {
        syn::Data::Enum(t) => from_enum(&input.ident, &t.variants),
        syn::Data::Struct(t) => from_struct(&input.ident, &t.fields),
        _ => {
            let error = SyntaxError::new_spanned(
                input,
                "FromOne can only be used with enums or structs with a single field",
            );
            error.to_compile_error().into()
        }
    }
}

fn from_enum(
    target_type: &Ident,
    variants: &Punctuated<Variant, token::Comma>,
) -> CompilerTokenStream {
    let non_unique_types: Vec<&Type> = {
        let mut type_counts = HashMap::new();

        for variant in variants {
            let fields = match &variant.fields {
                Fields::Named(fields) => &fields.named,
                Fields::Unnamed(fields) => &fields.unnamed,
                Fields::Unit => continue,
            };

            for field in fields {
                let count = type_counts.entry(&field.ty).or_insert(0);
                *count += 1;
            }
        }

        type_counts
            .into_iter()
            .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
            .collect()
    };

    let mut from_impls = Vec::new();

    for variant in variants {
        let field = match &variant.fields {
            Fields::Named(fields) => {
                if fields.named.len() != 1 {
                    continue;
                }

                let field = fields.named.first().unwrap();

                if non_unique_types.contains(&&field.ty) {
                    continue;
                }

                Field::Named(field)
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    continue;
                }

                let field = fields.unnamed.first().unwrap();

                if non_unique_types.contains(&&field.ty) {
                    continue;
                }

                Field::Unnamed(field)
            }
            Fields::Unit => continue,
        };

        let skip = get_skip_attribute(&variant.attrs);

        let from_impl = match skip {
            Ok(Some(_)) => continue,
            Ok(None) => match field {
                Field::Named(field) => {
                    let field_type = &field.ty;
                    let field_name = &field.ident;
                    let variant_name = &variant.ident;

                    quote! {
                        impl From<#field_type> for #target_type {
                            fn from(x: #field_type) -> Self {
                                Self::#variant_name { #field_name: x }
                            }
                        }
                    }
                }
                Field::Unnamed(field) => {
                    let field_type = &field.ty;
                    let variant_name = &variant.ident;

                    quote! {
                        impl From<#field_type> for #target_type {
                            fn from(x: #field_type) -> Self {
                                Self::#variant_name(x)
                            }
                        }
                    }
                }
            },
            Err(error) => return error.to_compile_error().into(),
        };

        from_impls.push(from_impl);
    }

    let tokens: TokenStream = from_impls.into_iter().collect();

    tokens.into()
}

fn get_skip_attribute(attrs: &[syn::Attribute]) -> Result<Option<()>, SyntaxError> {
    let mut skip_attr = None;

    for attr in attrs {
        if attr.path().is_ident("from") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    skip_attr = Some(());
                    Ok(())
                } else {
                    Err(meta.error("Expected `#[from(skip)]`"))
                }
            })?;
        }
    }

    Ok(skip_attr)
}

fn from_struct(target_type: &Ident, fields: &Fields) -> CompilerTokenStream {
    let from_impl = match fields {
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                return invalid_number_of_fields_in_struct_error(target_type)
                    .to_compile_error()
                    .into();
            }

            let field = fields.unnamed.first().unwrap();

            let field_type = &field.ty;

            quote! {
                impl From<#field_type> for #target_type {
                    fn from(x: #field_type) -> Self {
                        Self(x)
                    }
                }
            }
        }
        Fields::Named(fields) => {
            if fields.named.len() != 1 {
                return invalid_number_of_fields_in_struct_error(target_type)
                    .to_compile_error()
                    .into();
            }

            let field = fields.named.first().unwrap();

            let field_type = &field.ty;
            let field_name = &field.ident;

            quote! {
                impl From<#field_type> for #target_type {
                    fn from(x: #field_type) -> Self {
                        Self { #field_name: x }
                    }
                }
            }
        }
        Fields::Unit => {
            return invalid_number_of_fields_in_struct_error(target_type)
                .to_compile_error()
                .into()
        }
    };

    from_impl.into()
}

fn invalid_number_of_fields_in_struct_error(t: &Ident) -> SyntaxError {
    SyntaxError::new_spanned(
        t,
        "FromOne can only be used with structs with a single field",
    )
}
