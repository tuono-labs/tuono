use convert_case::{Case, Casing};
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericArgument, GenericParam, PathArguments};

fn type_to_typescript(type_name: &str) -> &str {
    match type_name {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => type_name,
    }
}

pub fn get_field_name(field: &syn::Field) -> String {
    let field_name: String = parse_serde_attribute(&field.attrs, "rename");

    if !field_name.is_empty() {
        return field_name;
    }

    if let Some(field) = field.ident.as_ref() {
        field.to_string()
    } else {
        String::from("unknown")
    }
}

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

pub fn rust_to_typescript_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Tuple(tuple) => {
            let inner_types: Vec<String> =
                tuple.elems.iter().map(rust_to_typescript_type).collect();
            format!("[{}]", inner_types.join(", "))
        }
        syn::Type::Path(type_path) => {
            if let Some(last_segment) = type_path.path.segments.last() {
                let outer_type = last_segment.ident.to_string();
                if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    let inner_types: Vec<String> = args
                        .args
                        .iter()
                        .filter_map(|arg| {
                            if let GenericArgument::Type(inner_type) = arg {
                                match inner_type {
                                    syn::Type::Path(inner_type_path) => {
                                        Some(inner_type_path.path.segments[0].ident.to_string())
                                    }
                                    syn::Type::Reference(reference) => {
                                        if let syn::Type::Path(inner_type_path) = &*reference.elem {
                                            Some(inner_type_path.path.segments[0].ident.to_string())
                                        } else {
                                            Some("unknown".to_string())
                                        }
                                    }
                                    _ => Some("unknown".to_string()),
                                }
                            } else {
                                None
                            }
                        })
                        .collect();

                    match outer_type.as_str() {
                        "Option" => {
                            format!("{} | null", type_to_typescript(&inner_types[0]))
                        }
                        "Vec" => {
                            format!("{}[]", type_to_typescript(&inner_types[0]))
                        }
                        "HashMap" | "BTreeMap" => {
                            format!(
                                "Record<{}, {}>",
                                type_to_typescript(&inner_types[0]),
                                type_to_typescript(&inner_types[1])
                            )
                        }
                        _ => "unknown".to_string(),
                    }
                } else {
                    type_to_typescript(&outer_type).to_string()
                }
            } else {
                "unknown".to_string()
            }
        }
        syn::Type::Reference(reference) => {
            // Ignore lifetimes and treat references as their base type
            if let syn::Type::Path(type_path) = &*reference.elem {
                if let Some(base_type) = type_path.path.segments.last() {
                    type_to_typescript(&base_type.ident.to_string()).to_string()
                } else {
                    "unknown".to_string()
                }
            } else {
                "unknown".to_string()
            }
        }
        _ => "unknown".to_string(),
    }
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
