mod leb128;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemStruct, parse_macro_input};
use crate::leb128::Leb128;

#[proc_macro_derive(SerializeHeader)]
pub fn derive_serialize_header(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let type_name = item.ident;
    let fields_count = item.fields.len().encode_leb128_vec().into_iter().map(|v| v.to_token_stream()).collect::<Vec<proc_macro2::TokenStream>>();
    let mut types = Vec::new();

    item.fields.iter().for_each(|field| {
        types.push(field.ty.to_token_stream());
    });

    let gen = quote! {
        impl dullahan_serde::header::serialize_header::SerializeHeader for #type_name {
            fn serialize_header<W: std::io::Write>(writer: &mut W) -> std::io::Result<()> {
                writer.write_all(&[
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
