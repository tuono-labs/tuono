use std::str::FromStr;

use crate::typescript::utils::type_to_typescript;

use convert_case::{Case, Casing};
use syn::{self, GenericArgument, PathArguments};

// This enum matches serde's RenameRule enum
#[derive(Debug, Eq, PartialEq)]
enum RenameSerdeOptions {
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
            "lowercase" => Ok(RenameSerdeOptions::LowerCase),
            "UPPERCASE" => Ok(RenameSerdeOptions::UpperCase),
            "PascalCase" => Ok(RenameSerdeOptions::PascalCase),
            "camelCase" => Ok(RenameSerdeOptions::CamelCase),
            "snake_case" => Ok(RenameSerdeOptions::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(RenameSerdeOptions::ScreamingSnakeCase),
            "kebab-case" => Ok(RenameSerdeOptions::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(RenameSerdeOptions::ScreamingKebabCase),
            _ => Err(()),
        }
    }
}

/// Check if the struct has the `#[serde(rename_all = "...")]` attribute
fn get_rename_option(attrs: &[syn::Attribute]) -> RenameSerdeOptions {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Ok(meta) = attr.parse_args::<syn::Expr>() {
                match meta {
                    syn::Expr::Assign(assign) => {
                        if let syn::Expr::Path(path) = *assign.left {
                            if !path.path.is_ident("rename_all") {
                                return RenameSerdeOptions::None;
                            }
                        }
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *assign.right
                        {
                            return RenameSerdeOptions::from_str(&lit_str.value())
                                .unwrap_or(RenameSerdeOptions::None);
                        }
                    }
                    _ => return RenameSerdeOptions::None,
                }
            }
        }
    }
    RenameSerdeOptions::None
}

/// Parse the struct generics and return them collected into a "<...>" string.
/// If no generics are present, return an empty string.
fn parse_generics_to_typescript_string(element: &syn::ItemStruct) -> String {
    let generics = element
        .generics
        .clone()
        .params
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

/// Parse a rust struct and returns a tuple of the struct name and the
/// struct compiled to a typescript interface
pub fn parse_struct(element: &syn::ItemStruct) -> (String, String) {
    let struct_name = element.ident.to_string();
    let generics = parse_generics_to_typescript_string(element);

    let mut fields_as_string = format!("export interface {struct_name}{generics} {{\n");

    let rename_option = get_rename_option(&element.attrs);

    for field in &element.fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_type = match &field.ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path.path.segments.last().unwrap();
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
            }
            syn::Type::Reference(reference) => {
                // Ignore lifetimes and treat references as their base type
                if let syn::Type::Path(type_path) = &*reference.elem {
                    let base_type = type_path.path.segments.last().unwrap().ident.to_string();
                    type_to_typescript(&base_type).to_string()
                } else {
                    "unknown".to_string()
                }
            }
            _ => "unknown".to_string(),
        };

        let field_name = match rename_option {
            RenameSerdeOptions::LowerCase => field_name.to_lowercase(),
            RenameSerdeOptions::UpperCase => field_name.to_uppercase(),

            RenameSerdeOptions::CamelCase => field_name.to_case(Case::Camel),
            RenameSerdeOptions::PascalCase => field_name.to_case(Case::Pascal),
            _ => field_name,
        };

        fields_as_string.push_str(&format!("  {field_name}: {field_type};\n"));
    }

    fields_as_string.push_str("}\n");

    (struct_name, fields_as_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_parses_struct() {
        let struct_str = r#"
            #[derive(Type)]
            struct MyStruct<'a, T, R>{
                field_1: &str,
                field2: i32,
                field3: Option<String>,
                field4: Vec<i32>,
                record: HashMap<&'a str, i32>,
                user: User,
                generic: T,
                generic2: HashMap<&mut str, R>,
                btree: BTreeMap<String, i32>,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let (struct_name, typescript_definition) = parse_struct(&parsed_struct);

        assert_eq!(struct_name, "MyStruct");
        assert_eq!(
            typescript_definition,
            "export interface MyStruct<T, R> {\n  field_1: string;\n  field2: number;\n  field3: string | null;\n  field4: number[];\n  record: Record<string, number>;\n  user: User;\n  generic: T;\n  generic2: Record<string, R>;\n  btree: Record<string, number>;\n}\n"
        );
    }

    #[test]
    fn it_correctly_parses_struct_with_no_generics() {
        let struct_str = r#"
            #[derive(Type)]
            struct MyStruct {
                field_1: &str,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let (struct_name, typescript_definition) = parse_struct(&parsed_struct);

        assert_eq!(struct_name, "MyStruct");
        assert_eq!(
            typescript_definition,
            "export interface MyStruct {\n  field_1: string;\n}\n"
        );
    }

    #[test]
    fn it_correctly_turn_the_keys_camel_case() {
        let struct_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "camelCase")]
            struct MyStruct {
                field_one: &str,
                field_two: i32,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let (struct_name, typescript_definition) = parse_struct(&parsed_struct);

        assert_eq!(struct_name, "MyStruct");
        assert_eq!(
            typescript_definition,
            "export interface MyStruct {\n  fieldOne: string;\n  fieldTwo: number;\n}\n"
        );
    }

    #[test]
    fn it_correctly_turn_the_keys_pascal_case() {
        let struct_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "PascalCase")]
            struct MyStruct {
                field_one: &str,
                field_two: i32,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let (struct_name, typescript_definition) = parse_struct(&parsed_struct);

        assert_eq!(struct_name, "MyStruct");
        assert_eq!(
            typescript_definition,
            "export interface MyStruct {\n  FieldOne: string;\n  FieldTwo: number;\n}\n"
        );
    }

    #[test]
    fn it_correctly_retrieve_the_serde_rename_option() {
        let struct_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "PascalCase")]
            struct MyStruct {
                field_one: &str,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let rename_option = get_rename_option(&parsed_struct.attrs);

        assert_eq!(rename_option, RenameSerdeOptions::PascalCase);

        let struct_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "camelCase")]
            struct MyStruct {
                field_one: &str,
            }"#;
        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let rename_option = get_rename_option(&parsed_struct.attrs);
        assert_eq!(rename_option, RenameSerdeOptions::CamelCase);
    }
}
