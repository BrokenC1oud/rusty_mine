use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Data, Fields};

#[proc_macro_derive(Packet, attributes(packet_id))]
pub fn derive_trait_func(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("#[derive(Packet)] only supports named fields"),
        }
        _ => panic!("#[derive(Packet)] is only defined for structs"),
    };

    let field_names = fields
        .iter()
        .map(|field| (field.ident.as_ref().unwrap(), field.ty.clone()))
        .collect::<Vec<_>>();

    let read_calls = field_names.iter().map(|(name, ty)| {
        quote! { #name: #ty::read(reader)? }
    });

    let write_calls = field_names.iter().map(|(name, _)| {
        quote! { self.#name.write(writer)? }
    });

    let expanded = quote! {
        impl crate::protocol::packets::Packet for #name {
            fn read(reader: &mut std::io::Cursor<&[u8]>) -> eyre::Result<Self> {
                use crate::protocol::types::Type;
                Ok(Self {
                    #(#read_calls,)*
                })
            }
            fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
                use crate::protocol::types::Type;
                #(#write_calls;)*

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
