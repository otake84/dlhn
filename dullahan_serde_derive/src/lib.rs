mod leb128;

use std::slice::Iter;
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span};
use quote::{ToTokens, quote};
use syn::{Attribute, DeriveInput, Meta, NestedMeta, parse_macro_input};
use crate::leb128::Leb128;

const STRUCT_CODE: u8 = 20;
const ENUM_CODE: u8 = 22;
const SERDE_ATTRIBUTE: &str = "serde";
const SKIP_ATTRIBUTE: &str = "skip";
const SKIP_SERIALIZING_ATTRIBUTE: &str = "skip_serializing";

#[proc_macro_derive(SerializeHeader, attributes(serde))]
pub fn derive_serialize_header(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let type_name = item.ident;

    match item.data {
        syn::Data::Struct(data) => {
            let mut types = Vec::new();

            for field in data.fields.iter() {
                if !is_skip_field(field.attrs.iter()) {
                    types.push(field.ty.to_token_stream());
                }
            }

            let fields_count = types.len().encode_leb128_vec().iter().map(ToTokens::to_token_stream).collect::<Vec<proc_macro2::TokenStream>>();

            let gen = quote! {
                impl dullahan_serde::header::serialize_header::SerializeHeader for #type_name {
                    fn serialize_header<W: std::io::Write>(writer: &mut W) -> std::io::Result<()> {
                        writer.write_all(&[
                            #STRUCT_CODE,
                            #(
                                #fields_count,
                            )*
                        ])?;
                        #(
                            <#types>::serialize_header(writer)?;
                        )*
                        Ok(())
                    }
                }
            };

            gen.into()
        }
        syn::Data::Enum(data) => {
            let mut types = Vec::new();

            data.variants.iter().for_each(|variant| {
                if !is_skip_field(variant.attrs.iter()) {
                    let mut inner_types = variant.fields.iter().map(|field| field.ty.to_token_stream()).collect::<Vec<proc_macro2::TokenStream>>();
                    if inner_types.is_empty() {
                        inner_types.push(Group::new(Delimiter::Parenthesis, proc_macro2::TokenStream::new()).into_token_stream());
                    }
                    types.push(inner_types);
                }
            });

            let variants_count = types.len().encode_leb128_vec().iter().map(ToTokens::to_token_stream).collect::<Vec<proc_macro2::TokenStream>>();
            let types_count = types.iter().map(|v| {
                v.len().encode_leb128_vec().iter().map(ToTokens::to_token_stream).collect::<Vec<proc_macro2::TokenStream>>()
            }).collect::<Vec<Vec<proc_macro2::TokenStream>>>();

            let gen = quote! {
                impl dullahan_serde::header::serialize_header::SerializeHeader for #type_name {
                    fn serialize_header<W: std::io::Write>(writer: &mut W) -> std::io::Result<()> {
                        writer.write_all(&[
                            #ENUM_CODE,
                            #(
                                #variants_count,
                            )*
                        ])?;
                        #(
                            writer.write_all(&[
                                #(
                                    #types_count,
                                )*
                            ])?;
                            #(
                                <#types>::serialize_header(writer)?;
                            )*
                        )*
                        Ok(())
                    }
                }
            };

            gen.into()
        }
        syn::Data::Union(_) => {
            syn::Error::new(Span::call_site(), "union is not supported").to_compile_error().into()
        }
    }
}

fn is_skip_field(mut attributes: Iter<Attribute>) -> bool {
    attributes.any(|attribute| {
        attribute.path.get_ident().map(ToString::to_string) == Some(SERDE_ATTRIBUTE.to_string()) &&
            match attribute.parse_meta() {
                Ok(Meta::List(v)) => {
                    v.nested.iter().any(|v| {
                        match v {
                            NestedMeta::Meta(v) => {
                                let ident = v.path().get_ident().map(ToString::to_string);
                                ident == Some(SKIP_ATTRIBUTE.to_string()) || ident == Some(SKIP_SERIALIZING_ATTRIBUTE.to_string())
                            }
                            _ => false
                        }
                    })
                },
                _ => false
            }
    })
}
