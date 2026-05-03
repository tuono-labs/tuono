use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn middleware_core(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    quote! {
          #item
    }
    .into()
}
