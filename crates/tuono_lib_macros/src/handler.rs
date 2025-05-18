use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Expr, FnArg, Ident, ItemFn, Pat, PatIdent, Type, parse_macro_input, parse_quote};

pub fn handler_core(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let fn_name = &item.sig.ident;

    let mut arguments: Punctuated<FnArg, Comma> = Punctuated::new();
    let mut passed_arguments: Punctuated<Expr, Comma> = Punctuated::new();

    let mut request_arg_index: Option<usize> = None;

    for (i, arg) in item.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = arg {
            match &*pat_type.pat {
                Pat::Ident(PatIdent { ident, .. }) => {
                    // Check if the type of this is Request, and remember its index if it is
                    if let Type::Path(ty) = &*pat_type.ty {
                        if let Some(segment) = ty.path.segments.last() {
                            if segment.ident == "Request" {
                                request_arg_index = Some(i);
                            }
                        }
                    }
                    arguments.push(arg.to_owned());
                    passed_arguments.push(parse_quote!(#ident));
                }
                // If it's anything except an ident, give it a marker arg
                // so we don't mess up any destructuring
                _ => {
                    let arg_name = Ident::new(&format!("arg_{}", i), item.span());
                    let arg_type = &pat_type.ty;
                    let argument: FnArg = parse_quote!(#arg_name: #arg_type);
                    arguments.push(argument);
                    passed_arguments.push(parse_quote!(#arg_name));
                }
            }
        }
    }

    let content = match request_arg_index {
        // If request arg is passed, use it in render_to_string
        Some(request_arg_index) => {
            let request_arg = passed_arguments[request_arg_index].clone();
            // When we call the function, we need to clone the request arg so we can use it
            // in render
            passed_arguments[request_arg_index] = parse_quote!(#request_arg.clone());

            quote! {
                #item

                pub async fn tuono_internal_route(
                    #arguments
                ) -> impl tuono_lib::axum::response::IntoResponse {
                   #fn_name(#passed_arguments).await.render_to_string(#request_arg)
                }

                pub async fn tuono_internal_api(
                    #arguments
                ) -> impl tuono_lib::axum::response::IntoResponse {
                   #fn_name(#passed_arguments).await.json()
                }
            }
        }
        // If request arg is not passed, we need to create one for the route handler
        None => {
            let mut route_arguments = arguments.clone();
            route_arguments.push(parse_quote!(req: tuono_lib::axum::extract::Request));

            quote! {
                #item

                pub async fn tuono_internal_route(
                    #route_arguments
                ) -> impl tuono_lib::axum::response::IntoResponse {
                   #fn_name(#passed_arguments).await.render_to_string(req)
                }

                pub async fn tuono_internal_api(
                    #arguments
                ) -> impl tuono_lib::axum::response::IntoResponse {
                   #fn_name(#passed_arguments).await.json()
                }
            }
        }
    };

    content.into()
}
