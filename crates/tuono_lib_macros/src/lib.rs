//! ## Tuono
//! Tuono is a full-stack web framework for building React applications using Rust as the backend with a strong focus on usability and performance.
//!
//! You can find the full documentation at [tuono.dev](https://tuono.dev/)

extern crate proc_macro;
use proc_macro::TokenStream;

mod api;
mod handler;

#[proc_macro_attribute]
pub fn handler(args: TokenStream, item: TokenStream) -> TokenStream {
    handler::handler_core(args, item)
}

#[proc_macro_attribute]
pub fn api(args: TokenStream, item: TokenStream) -> TokenStream {
    api::api_core(args, item)
}

/// Automatically generate typescript's types
/// from Rust's structs, types and enums.
///
/// The types will be exported on the client side
/// and it will be available from the `"tuono/types"` module.
#[proc_macro_derive(Type)]
pub fn derive_typescript_type(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
