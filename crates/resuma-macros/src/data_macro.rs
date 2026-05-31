//! `#[data]` - concise Resuma DTO/model helper.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Item};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = match parse2(input) {
        Ok(item) => item,
        Err(e) => return e.to_compile_error(),
    };

    match item {
        Item::Struct(mut s) => {
            s.attrs.push(syn::parse_quote!(
                #[derive(Clone, ::resuma::__private::serde::Serialize, ::resuma::__private::serde::Deserialize)]
            ));
            s.attrs.push(syn::parse_quote!(
                #[serde(crate = "::resuma::__private::serde")]
            ));
            quote!(#s)
        }
        Item::Enum(mut e) => {
            e.attrs.push(syn::parse_quote!(
                #[derive(Clone, ::resuma::__private::serde::Serialize, ::resuma::__private::serde::Deserialize)]
            ));
            e.attrs.push(syn::parse_quote!(
                #[serde(crate = "::resuma::__private::serde")]
            ));
            quote!(#e)
        }
        other => {
            syn::Error::new_spanned(other, "#[data] supports structs and enums").to_compile_error()
        }
    }
}
