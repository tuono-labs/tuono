use convert_case::{Case, Casing};
use std::str::FromStr;
use syn::GenericParam;
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Parse the struct generics and return them collected into a "<...>" string.
/// If no generics are present, return an empty string.
pub fn parse_generics_to_typescript_string(generics: Punctuated<GenericParam, Comma>) -> String {
    let generics = generics
        .iter()
        .map(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                type_param.ident.to_string()
            } else {
                String::new()
            }
        })
        .filter(|name| !name.is_empty())
        .collect::<Vec<String>>();

    if !generics.is_empty() {
        return format!("<{}>", generics.join(", "));
    }

    String::new()
}

/// Parse any "assign" `#[serde(... = "...")]` attribute and return the value of the specified
/// attribute.
pub fn parse_serde_attribute<T>(attrs: &[syn::Attribute], attribute_name: &str) -> T
where
    T: Default + FromStr,
{
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Ok(meta) = attr.parse_args::<syn::Expr>() {
                match meta {
                    syn::Expr::Assign(assign) => {
                        if let syn::Expr::Path(path) = *assign.left {
                            if !path.path.is_ident(attribute_name) {
                                return T::default();
                            }
                        }
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *assign.right
                        {
                            return T::from_str(&lit_str.value()).unwrap_or_default();
                        }
                    }
                    _ => return T::default(),
                }
            }
        }
    }
    T::default()
}

/// Check if the element should be skipped based on the presence of
/// `#[serde(skip)]` or `#[serde(skip_serializing)]` attributes.
pub fn should_skip_element(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Ok(meta) = attr.parse_args::<syn::Ident>() {
                if meta == "skip" || meta == "skip_serializing" {
                    return true;
                }
            }
        }
    }
    false
}

// This enum matches serde's RenameRule enum
#[derive(Debug, Eq, PartialEq, Default)]
pub enum RenameSerdeOptions {
    #[default]
    None,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl FromStr for RenameSerdeOptions {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "lowercase" => Ok(Self::LowerCase),
            "UPPERCASE" => Ok(Self::UpperCase),
            "PascalCase" => Ok(Self::PascalCase),
            "camelCase" => Ok(Self::CamelCase),
            "snake_case" => Ok(Self::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(Self::ScreamingSnakeCase),
            "kebab-case" => Ok(Self::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(Self::ScreamingKebabCase),
            _ => Err(()),
        }
    }
}

impl RenameSerdeOptions {
    pub fn transform(&self, input: String) -> String {
        match self {
            Self::LowerCase => input.to_lowercase(),
            Self::UpperCase => input.to_uppercase(),
            Self::CamelCase => input.to_case(Case::Camel),
            Self::PascalCase => input.to_case(Case::Pascal),
            _ => input,
        }
    }
}
