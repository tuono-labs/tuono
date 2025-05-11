//! ## Tuono
//! Tuono is a full-stack web framework for building React applications using Rust as the backend with a strong focus on usability and performance.
//!
//! You can find the full documentation at [tuono.dev](https://tuono.dev/)

extern crate proc_macro;
use proc_macro::TokenStream;

mod api;
mod handler;
mod utils;

/// Marks an asynchronous function as a Tuono server-side handler.
///
/// This attribute macro registers a function as a server-side data loader
/// that will run during server-side rendering (SSR) in a Tuono application.
/// It expects a function with the signature:
///
/// ```rust
/// #[tuono_lib::handler]
/// async fn my_handler(req: Request) -> Response { ... }
/// ```
///
/// The function should return a `Response`,
/// with serializable props for a React component.
#[proc_macro_attribute]
pub fn handler(args: TokenStream, item: TokenStream) -> TokenStream {
    handler::handler_core(args, item)
}

/// Marks an asynchronous function as an API route handler in Tuono.
///
/// This attribute macro registers a function as an HTTP API endpoint. It must
/// be used on an `async fn` and annotated with the HTTP method it handles, such as:
/// `#[api(GET)]`, `#[api(POST)]`, etc.
///
/// The function should accept a `Request` (from `tuono_lib`) as input and return
/// a type compatible with Axum, such as `StatusCode`, `Json<T>`, or any type
/// implementing `IntoResponse`.
///
/// This makes the function accessible as an HTTP `GET /health_check` route
/// in the Tuono server.
#[proc_macro_attribute]
pub fn api(args: TokenStream, item: TokenStream) -> TokenStream {
    api::api_core(args, item)
}

/// Automatically generate typescript's types
/// from Rust's structs, types and enums.
///
/// The types will be exported on the client side,
/// and it will be available from the `"tuono/types"` module.
#[proc_macro_derive(Type)]
pub fn derive_typescript_type(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
