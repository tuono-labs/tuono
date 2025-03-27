use crate::utils::{
    crate_application_state_extractor, create_struct_fn_arg, import_main_application_state,
    params_argument, request_argument,
};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat};

pub fn api_core(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let http_method = parse_macro_input!(attrs as Ident)
        .to_string()
        .to_lowercase();

    let api_fn_name = Ident::new(
        &format!("{}_tuono_internal_api", http_method),
        Span::call_site().into(),
    );

    let fn_name = &item.sig.ident;
    let return_type = &item.sig.output;

    let mut argument_names: Punctuated<Pat, Comma> = Punctuated::new();
    let mut axum_arguments: Punctuated<FnArg, Comma> = Punctuated::new();

    // Fn Arguments minus the first which always is the request
    for (i, arg) in item.sig.inputs.iter().enumerate() {
        if i == 0 {
            axum_arguments.insert(i, params_argument());
            continue;
        }

        if i == 1 {
            axum_arguments.insert(1, create_struct_fn_arg())
        }

        if let FnArg::Typed(pat_type) = arg {
            let index = i - 1;
            let argument_name = *pat_type.pat.clone();
            argument_names.insert(index, argument_name.clone());
        }
    }

    axum_arguments.insert(axum_arguments.len(), request_argument());

    let application_state_extractor = crate_application_state_extractor(argument_names.clone());
    let application_state_import = import_main_application_state(argument_names.clone());

    let modified_request = if http_method == "post"
        || http_method == "put"
        || http_method == "patch"
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
