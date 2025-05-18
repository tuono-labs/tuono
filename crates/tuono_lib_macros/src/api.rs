use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Expr, FnArg, Ident, ItemFn, Pat, PatIdent, parse_macro_input, parse_quote};

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

    let mut arguments: Punctuated<FnArg, Comma> = Punctuated::new();
    let mut passed_arguments: Punctuated<Expr, Comma> = Punctuated::new();

    for (i, arg) in item.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = arg {
            match &*pat_type.pat {
                Pat::Ident(PatIdent { ident, .. }) => {
                    arguments.push(arg.to_owned());
                    passed_arguments.push(parse_quote!(#ident));
                }
                // if it's anything except an ident, give it a marker arg
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

    quote! {
        #item

        pub async fn #api_fn_name(#arguments)#return_type {
           #fn_name(#passed_arguments).await
        }
    }
    .into()
}
