use crate::utils::parse_next_value;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Ident, Path, Token, parenthesized};

/// Properties that can be specified on an axum argument spec
pub struct AxumArgumentTuple {
    pub name: Ident,
    pub extractor: Option<Path>,
}

impl AxumArgumentTuple {
    pub fn new(name: Ident) -> AxumArgumentTuple {
        Self {
            name,
            extractor: None,
        }
    }
}

impl Parse for AxumArgumentTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTES: &str = "name, extractor";

        // name always comes first
        let name = input.parse::<Ident>().map_err(|error| {
            Error::new(
                error.span(),
                format!("unexpected attribute, expected any of: {EXPECTED_ATTRIBUTES}, {error}"),
            )
        })?;
        // skip comma between this and other properties
        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        let mut axum_argument = AxumArgumentTuple::new(name);

        // iterate through remaining comma separated params
        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                Error::new(
                    error.span(),
                    format!(
                        "unexpected attribute, expected any of: {EXPECTED_ATTRIBUTES}, {error}"
                    ),
                )
            })?;
            let name = &*ident.to_string();
            match name {
                "extractor" => {
                    axum_argument.extractor =
                        Some(parse_next_value(input, || input.parse::<Path>())?);
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected attribute, expected any of: {EXPECTED_ATTRIBUTES}"),
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(axum_argument)
    }
}

pub enum AxumArgument {
    Tuple(AxumArgumentTuple),
}

impl Parse for AxumArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let axum_argument;
        // each argument is surrounded by parens
        parenthesized!(axum_argument in input);
        Ok(Self::Tuple(axum_argument.parse()?))
    }
}
