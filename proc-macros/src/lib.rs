extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

#[proc_macro_derive(IncrementalEnum, attributes(base, incr))]
pub fn incremental_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let attrs = &input.attrs;

    let base = attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("base") {
                Some(attr.parse_args::<syn::LitInt>().unwrap())
            } else {
                None
            }
        })
        .expect("base attribute is required");

    let incr = attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("incr") {
                Some(attr.parse_args::<syn::LitInt>().unwrap())
            } else {
                None
            }
        })
        .expect("incr attribute is required");

    let data = match &input.data {
        Data::Enum(data) => data,
        _ => panic!("IncrementalEnum can only be applied to enums"),
    };

    // let variants = data.variants.iter().enumerate().map(|(i, variant)| {
    //     let id = &variant.ident;
    //     let value = base.base10_parse::<u32>().unwrap() + (i as u32) * incr.base10_parse::<u32>().unwrap();
    //     quote! { #id = #value, }
    // });

    let match_arms = data.variants.iter().enumerate().map(|(i, variant)| {
        let id = &variant.ident;
        let value = base.base10_parse::<isize>().unwrap()
            + (i as isize) * incr.base10_parse::<isize>().unwrap();
        quote! { #value => Some(Self::#id), }
    });

    let expanded = quote! {
        // #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        // enum #name {
        //     #(#variants)*
        // }

        impl #name {
            pub fn from_value(value: isize) -> Option<Self> {
                match value {
                    #(#match_arms)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
