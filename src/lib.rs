use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ConstructInto)]
pub fn derive_construct_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("ConstructInto only supports structs with named fields"),
        },
        _ => panic!("ConstructInto only supports structs"),
    };

    let params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: impl Into<#ty> }
    });

    let assignments = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: #name.into() }
    });

    let expanded = quote! {
        impl #name {
            pub fn construct(#(#params),*) -> Self {
                Self {
                    #(#assignments),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
