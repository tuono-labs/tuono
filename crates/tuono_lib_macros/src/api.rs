use crate::axum_argument::AxumArgument;
use crate::utils::{
    crate_application_state_extractor, create_struct_fn_arg, import_main_application_state,
    params_argument, request_argument,
};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    FnArg, Ident, ItemFn, Pat, PatIdent, Token, parenthesized, parse_macro_input, parse_quote,
};

/// Attributes for the handler proc macro
#[derive(Default)]
struct ApiAttr {
    http_method: String,
    /// Which arguments should be passed to both axum routes and handler function, but
    /// excluded from state destructuring
    axum_arguments: Vec<AxumArgument>,
}

impl Parse for ApiAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE_MESSAGE: &str =
            "unexpected identifier, expected any of: axum_arguments";
        let mut attr = ApiAttr::default();

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
                    attr.http_method = ident.to_string().to_lowercase();

                    // skip comma between this and other properties
                    if !input.is_empty() {
                        input.parse::<Token![,]>()?;
                    }
                }
            }
        }
        Ok(attr)
    }
}

pub fn api_core(attr: TokenStream, item: TokenStream) -> TokenStream {
    let api_attribute = syn::parse_macro_input!(attr as ApiAttr);
    let item = parse_macro_input!(item as ItemFn);

    let api_fn_name = Ident::new(
        &format!("{}_tuono_internal_api", api_attribute.http_method),
        Span::call_site().into(),
    );

    let fn_name = &item.sig.ident;
    let return_type = &item.sig.output;

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
                    if let Some(AxumArgument::Tuple(axum_argument)) = api_attribute
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

    let modified_request = if api_attribute.http_method == "post"
        || api_attribute.http_method == "put"
        || api_attribute.http_method == "patch"
    {
        quote! {
            let (parts, body) = request.into_parts();
            let path = parts.uri.clone();
            let headers = parts.headers.clone();

            let body = tuono_lib::axum::body::to_bytes(body, usize::MAX).await.unwrap_or(Vec::new().into()).to_vec();

            let req = tuono_lib::Request::new(path, headers, params, Some(body));
        }
    } else {
        quote! {
           let pathname = request.uri();
           let headers = request.headers();

           let req = tuono_lib::Request::new(request.uri().to_owned(), request.headers().to_owned(), params, None);
        }
    };

    quote! {
        #application_state_import

        #item

        pub async fn #api_fn_name(#axum_arguments)#return_type {

           #application_state_extractor

           #modified_request

           #fn_name(req.clone(), #argument_names).await
        }
    }
    .into()
}
