use crate::utils::{
    crate_application_state_extractor, create_struct_fn_arg, import_main_application_state,
    params_argument, parse_parethesized_terminated, request_argument,
};

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, Ident, ItemFn, Pat, parse_macro_input};

/// Attributes for the handler proc macro
#[derive(Default)]
pub struct HandlerAttr {
    /// Which arguments should be passed to both axum routes and handler funciton, but
    /// excluded from state destructuring
    axum_arguments: HashSet<String>,
}

impl Parse for HandlerAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE_MESSAGE: &str =
            "unexpected identifier, expected any of: axum_arguments";
        let mut attr = HandlerAttr::default();

        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                syn::Error::new(
                    error.span(),
                    format!("{EXPECTED_ATTRIBUTE_MESSAGE}, {error}"),
                )
            })?;
            let attribute_name = &*ident.to_string();

            match attribute_name {
                "axum_arguments" => {
                    attr.axum_arguments = parse_parethesized_terminated::<Ident, Comma>(input)?
                        .into_iter()
                        .map(|ident| ident.to_string())
                        .collect()
                }
                _ => {
                    return Err(syn::Error::new(ident.span(), EXPECTED_ATTRIBUTE_MESSAGE));
                }
            }
        }
        Ok(attr)
    }
}

pub fn handler_core(attr: TokenStream, item: TokenStream) -> TokenStream {
    let handler_attribute = syn::parse_macro_input!(attr as HandlerAttr);
    let item = parse_macro_input!(item as ItemFn);

    let fn_name = &item.sig.ident;

    let mut state_argument_names: Punctuated<Pat, Comma> = Punctuated::new();
    let mut argument_names: Punctuated<Pat, Comma> = Punctuated::new();
    let mut axum_arguments: Punctuated<FnArg, Comma> = Punctuated::new();

    let mut state_included = false;
    // The request argument
    axum_arguments.push(params_argument());
    // Fn Arguments minus the first which always is the request
    for arg in item.sig.inputs.iter().skip(1) {
        if let FnArg::Typed(pat_type) = arg {
            let argument_name = *pat_type.pat.clone();
            match &argument_name {
                Pat::Ident(ident) => {
                    if handler_attribute
                        .axum_arguments
                        .contains(&ident.ident.to_string())
                    {
                        axum_arguments.push(arg.to_owned());
                        argument_names.push(argument_name.clone());
                    } else {
                        // State extractor needs to be included if there are state arguments
                        if !state_included {
                            axum_arguments.push(create_struct_fn_arg());
                            state_included = true;
                        }
                        argument_names.push(argument_name.clone());
                        state_argument_names.push(argument_name.clone());
                    }
                }
                _ => {
                    // State extractor needs to be included if there are state arguments
                    if !state_included {
                        axum_arguments.push(create_struct_fn_arg());
                        state_included = true;
                    }
                    argument_names.push(argument_name.clone());
                    state_argument_names.push(argument_name.clone());
                }
            }
        }
    }

    axum_arguments.insert(axum_arguments.len(), request_argument());

    let application_state_extractor =
        crate_application_state_extractor(state_argument_names.clone());
    let application_state_import = import_main_application_state(state_argument_names.clone());

    quote! {
        #application_state_import

        #item

        pub async fn tuono_internal_route(
            #axum_arguments
        ) -> impl tuono_lib::axum::response::IntoResponse {

            #application_state_extractor

           let pathname = request.uri();
           let headers = request.headers();

           let req = tuono_lib::Request::new(pathname.to_owned(), headers.to_owned(), params, None);

           #fn_name(req.clone(), #argument_names).await.render_to_string(req)
        }

        pub async fn tuono_internal_api(
            #axum_arguments
        ) -> impl tuono_lib::axum::response::IntoResponse {

            #application_state_extractor

           let pathname = request.uri();
           let headers = request.headers();

           let req = tuono_lib::Request::new(pathname.to_owned(), headers.to_owned(), params, None);

           #fn_name(req.clone(), #argument_names).await.json()
        }
    }
    .into()
}
