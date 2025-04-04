use super::utils::type_to_typescript;
use syn::{self, GenericArgument, PathArguments};

/// Parse a rust struct and returns a tuple of the struct name and the
/// struct compiled to typescript
pub fn parse_struct(element: &syn::ItemStruct) -> (String, String) {
    let struct_name = element.ident.to_string();
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

    let generics_as_string = if !generics.is_empty() {
        format!("<{}>", generics.join(", "))
    } else {
        String::new()
    };

    let mut fields_as_string = String::from(format!(
        "export interface {struct_name}{generics_as_string} {{\n"
    ));

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
                field1: &str,
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
            "export interface MyStruct<T, R> {\n  field1: string;\n  field2: number;\n  field3: string | null;\n  field4: number[];\n  record: Record<string, number>;\n  user: User;\n  generic: T;\n  generic2: Record<string, R>;\n  btree: Record<string, number>;\n}\n"
        );
    }
}
