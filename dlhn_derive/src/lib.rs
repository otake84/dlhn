mod leb128;

use std::{slice::Iter, str::FromStr};
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
const SKIP_SERIALIZING_IF_ATTRIBUTE: &str = "skip_serializing_if";

#[proc_macro_derive(SerializeHeader, attributes(serde))]
pub fn derive_serialize_header(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let type_name = item.ident;

    match item.data {
        syn::Data::Struct(data) => {
            let mut types = Vec::new();

            for field in data.fields.iter() {
                if has_skip_serializing_if(field.attrs.iter()) {
                    return syn::Error::new(Span::call_site(), "skip_serializing_if is not supported").to_compile_error().into()
                }

                if !is_skip_field(field.attrs.iter()) {
                    types.push(field.ty.to_token_stream());
                }
            }

            let fields_count = types.len().encode_leb128_vec().iter().map(ToTokens::to_token_stream).collect::<Vec<proc_macro2::TokenStream>>();

            let gen = quote! {
                impl dlhn::header::ser::SerializeHeader for #type_name {
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
            let mut outers = Vec::new();
            let mut inners = Vec::new();

            for variant in data.variants.iter() {
                if has_skip_serializing_if(variant.attrs.iter()) {
                    return syn::Error::new(Span::call_site(), "skip_serializing_if is not supported").to_compile_error().into()
                }

                if !is_skip_field(variant.attrs.iter()) {
                    if variant.fields.is_empty() {
                        outers.push(Group::new(Delimiter::Bracket, proc_macro2::TokenStream::new()).into_token_stream());
                        inners.push(vec![Group::new(Delimiter::Parenthesis, proc_macro2::TokenStream::new()).into_token_stream()]);
                    } else {
                        if variant.fields.len() > 1 {
                            match &variant.fields {
                                syn::Fields::Named(fields) => {
                                    let mut buf = vec![20];
                                    buf.append(&mut variant.fields.len().encode_leb128_vec());
                                    outers.push(proc_macro2::TokenStream::from_str(format!("{:?}", buf).as_str()).unwrap());

                                    let mut field_types = Vec::new();
                                    fields.named.iter().for_each(|field| {
                                        field_types.push(field.ty.to_token_stream());
                                    });
                                    inners.push(field_types);
                                }
                                syn::Fields::Unnamed(fields) => {
                                    let mut buf = vec![19];
                                    buf.append(&mut variant.fields.len().encode_leb128_vec());
                                    outers.push(proc_macro2::TokenStream::from_str(format!("{:?}", buf).as_str()).unwrap());

                                    inners.push(fields.unnamed.iter().map(|field| {
                                        field.ty.to_token_stream()
                                    }).collect());
                                }
                                syn::Fields::Unit => todo!(),
                            }
                        } else {
                            outers.push(Group::new(Delimiter::Bracket, proc_macro2::TokenStream::new()).into_token_stream());
                            let mut field_types = Vec::new();
                            variant.fields.iter().for_each(|field| {
                                field_types.push(field.ty.to_token_stream());
                            });
                            inners.push(field_types);
                        }
                    }
                }
            }

            let variants_count = outers.len().encode_leb128_vec().iter().map(ToTokens::to_token_stream).collect::<Vec<proc_macro2::TokenStream>>();

            let gen = quote! {
                impl dlhn::header::ser::SerializeHeader for #type_name {
                    fn serialize_header<W: std::io::Write>(writer: &mut W) -> std::io::Result<()> {
                        writer.write_all(&[
                            #ENUM_CODE,
                            #(
                                #variants_count,
                            )*
                        ])?;
                        #(
                            writer.write_all(&#outers)?;
                            #(
                                <#inners>::serialize_header(writer)?;
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

fn has_skip_serializing_if(mut attributes: Iter<Attribute>) -> bool {
    attributes.any(|attribute| {
        attribute.path.get_ident().map(ToString::to_string) == Some(SERDE_ATTRIBUTE.to_string()) &&
            match attribute.parse_meta() {
                Ok(Meta::List(v)) => {
                    v.nested.iter().any(|v| {
                        match v {
                            NestedMeta::Meta(v) => {
                                let ident = v.path().get_ident().map(ToString::to_string);
                                ident == Some(SKIP_SERIALIZING_IF_ATTRIBUTE.to_string())
                            }
                            _ => false
                        }
                    })
                },
                _ => false
            }
    })
}
