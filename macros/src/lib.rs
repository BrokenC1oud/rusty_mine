use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, parse_macro_input};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive_trait_func(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let packet_id = parse_container_attribute(&input).unwrap_or(0);

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("#[derive(Packet)] only supports named fields"),
        },
        _ => panic!("#[derive(Packet)] is only defined for structs"),
    };

    let field_names = fields
        .iter()
        .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
        .collect::<Vec<_>>();

    let read_calls = field_names.iter().map(|(name, ty)| {
        quote! { #name: <#ty>::read(reader)? }
    });

    let write_calls = field_names.iter().map(|(name, _)| {
        quote! { self.#name.write(writer)?; }
    });

    let expanded = quote! {
        impl crate::protocol::packets::Packet for #name {
            const PACKET_ID: u8 = #packet_id;
            fn read(reader: &mut std::io::Cursor<&[u8]>) -> eyre::Result<Self> {
                use crate::protocol::types::Type;
                Ok(Self {
                    #(#read_calls,)*
                })
            }
            fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
                use crate::protocol::types::Type;
                writer.push(Self::PACKET_ID);
                #(#write_calls;)*

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_container_attribute(input: &DeriveInput) -> Option<u8> {
    for attr in &input.attrs {
        if attr.path().is_ident("packet") {
            if let Meta::List(meta_list) = &attr.meta {
                let nested = meta_list
                    .parse_args_with(
                        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                    )
                    .ok()?;

                for meta in nested {
                    if let Meta::NameValue(name_value) = meta {
                        if name_value.path.is_ident("id") {
                            if let syn::Expr::Lit(expr_lit) = &name_value.value {
                                if let Lit::Int(lit_int) = &expr_lit.lit {
                                    return lit_int.base10_parse::<u8>().ok();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}
