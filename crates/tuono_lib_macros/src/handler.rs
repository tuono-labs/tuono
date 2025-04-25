use crate::utils::{
    crate_application_state_extractor, create_struct_fn_arg, import_main_application_state,
    params_argument, request_argument,
};

use crate::axum_argument::AxumArgument;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    FnArg, Ident, ItemFn, Pat, PatIdent, Token, parenthesized, parse_macro_input, parse_quote,
};

/// Attributes for the handler proc macro
#[derive(Default)]
struct HandlerAttr {
    /// Which arguments should be passed to both axum routes and handler funciton, but
    /// excluded from state destructuring
    axum_arguments: Vec<AxumArgument>,
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
                    let axum_arguments;
                    parenthesized!(axum_arguments in input);
                    attr.axum_arguments =
                        Punctuated::<AxumArgument, Token![,]>::parse_terminated(&axum_arguments)
                            .map(|punctuated| {
                                punctuated.into_iter().collect::<Vec<AxumArgument>>()
                            })?;
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
                Pat::Ident(PatIdent { ident, .. }) => {
                    if let Some(AxumArgument::Tuple(axum_argument)) = handler_attribute
                        .axum_arguments
                        .iter()
                        .find(|axum_argument| {
                            matches!(axum_argument, AxumArgument::Tuple(a) if a.name == *ident)
                        })
                    {
                        let ty = &pat_type.ty;
                        let extracted_fn_arg: FnArg = if let Some(extractor) = axum_argument.extractor.as_ref() {
                            parse_quote!(#extractor(#ident): #extractor<#ty>)
                        } else {
                            parse_quote!(#ident: #ty)
                        };
                        axum_arguments.push(extracted_fn_arg);
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
                t => {
                    panic!("unsupported argument type {t:?}");
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
