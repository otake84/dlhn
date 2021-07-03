mod leb128;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group};
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};
use crate::leb128::Leb128;

const MAP_CODE: u8 = 20;
const ENUM_CODE: u8 = 22;
const UNIT_ENUM_CODE: u8 = 23;

#[proc_macro_derive(SerializeHeader)]
pub fn derive_serialize_header(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let type_name = item.ident;

    match item.data {
        syn::Data::Struct(data) => {
            let fields_count = data.fields.len().encode_leb128_vec().into_iter().map(|v| v.to_token_stream()).collect::<Vec<proc_macro2::TokenStream>>();
            let mut types = Vec::new();

            data.fields.iter().for_each(|field| {
                types.push(field.ty.to_token_stream());
            });

            let gen = quote! {
                impl dullahan_serde::header::serialize_header::SerializeHeader for #type_name {
                    fn serialize_header<W: std::io::Write>(writer: &mut W) -> std::io::Result<()> {
                        writer.write_all(&[
                            #MAP_CODE,
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
            let variants_count = data.variants.len().encode_leb128_vec().into_iter().map(|v| v.to_token_stream()).collect::<Vec<proc_macro2::TokenStream>>();
            let mut variant_names = Vec::new();
            let mut types = Vec::new();

            data.variants.iter().for_each(|variant| {
                variant_names.push(variant.ident.to_token_stream());
                let mut inner_types = variant.fields.iter().map(|field| field.to_token_stream()).collect::<Vec<proc_macro2::TokenStream>>();
                if inner_types.is_empty() {
                    inner_types.push(Group::new(Delimiter::Parenthesis, proc_macro2::TokenStream::new()).into_token_stream());
                }
                types.push(inner_types);
            });

            let types_count = types.iter().map(|v| {
                v.len().encode_leb128_vec().into_iter().map(|v| {
                    v.to_token_stream()
                }).collect::<Vec<proc_macro2::TokenStream>>()
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
        syn::Data::Union(_) => todo!(),
    }
}
