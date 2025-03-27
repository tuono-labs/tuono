use crate::utils::{
    crate_application_state_extractor, create_struct_fn_arg, import_main_application_state,
    params_argument, request_argument,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, ItemFn, Pat};

pub fn handler_core(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let fn_name = &item.sig.ident;

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
