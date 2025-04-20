use crate::typescript::parser::utils::{
    RenameSerdeOptions, get_field_name, parse_generics_to_typescript_string, parse_serde_attribute,
    rust_to_typescript_type, should_skip_element,
};

/// Parse a rust enum and returns a tuple of the enum name and the
/// enum compiled to a typescript type
pub fn parse_enum(element: &syn::ItemEnum) -> (String, String) {
    let enum_name = element.ident.to_string();

    let generics = parse_generics_to_typescript_string(element.generics.clone().params);
    let mut enum_variants: Vec<String> = Vec::new();

    let rename_option: RenameSerdeOptions = parse_serde_attribute(&element.attrs, "rename_all");

    for variant in &element.variants {
        if should_skip_element(&variant.attrs) {
            continue;
        }

        let mut parsed_variant = rename_option.transform(variant.ident.to_string());

        match &variant.fields {
            syn::Fields::Named(field) => {
                parsed_variant = format!("{{\"{}\": {{ ", parsed_variant);
                let mut variant_fields: Vec<String> = Vec::new();

                for field in &field.named {
                    if should_skip_element(&field.attrs) {
                        continue;
                    }

                    let field_name = rename_option.transform(get_field_name(field));

                    let field_type = rust_to_typescript_type(&field.ty);
                    variant_fields.push(format!("{field_name}: {field_type}"));
                }
                enum_variants.push(format!(
                    "{parsed_variant}{} }}}}",
                    variant_fields.join(", ")
                ));
            }
            syn::Fields::Unnamed(field) => {
                let mut variant_fields: Vec<String> = Vec::new();
                for field in &field.unnamed {
                    let field_type = rust_to_typescript_type(&field.ty);
                    variant_fields.push(field_type);
                }
                enum_variants.push(format!(
                    "{{\"{parsed_variant}\": [{}]}}",
                    variant_fields.join(", ")
                ));
            }
            syn::Fields::Unit => {
                enum_variants.push(format!("\"{parsed_variant}\""));
            }
        }
    }

    let enum_type = format!(
        "export type {enum_name}{generics} = {};\n",
        enum_variants.join(" | ")
    );
    (enum_name, enum_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_parse_a_simple_enum() {
        let enum_str = r#"
            #[derive(Type)]
            enum MyEnum {
                Variant1,
                Variant2,
                Variant3
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (enum_name, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(enum_name, "MyEnum");
        assert_eq!(
            typescript_definition,
            "export type MyEnum = \"Variant1\" | \"Variant2\" | \"Variant3\";\n"
        );
    }

    #[test]
    fn it_correctly_apply_rename_all_modifier() {
        let enum_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "lowercase")]
            enum MyEnum {
                Id,
                Name,
                UserAge
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (enum_name, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(enum_name, "MyEnum");
        assert_eq!(
            typescript_definition,
            "export type MyEnum = \"id\" | \"name\" | \"userage\";\n"
        );
    }

    #[test]
    fn it_correctly_skips_a_variant() {
        let enum_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "lowercase")]
            enum MyEnum {
                Id,
                Name,
                UserAge,
                #[serde(skip)]
                SkipMe
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (_, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(
            typescript_definition,
            "export type MyEnum = \"id\" | \"name\" | \"userage\";\n"
        );
    }

    #[test]
    fn it_correctly_parse_struct_variant() {
        let enum_str = r#"
            #[derive(Type)]
            enum MyEnum {
                Id,
                User {
                    name: String,
                    age: u32,
                },
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (_, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(
            typescript_definition,
            "export type MyEnum = \"Id\" | {\"User\": { name: string, age: number }};\n"
        );

        let enum_str = r#"
            #[derive(Type)]
            enum MyEnum {
                Id,
                User {
                    name: String,
                    #[serde(skip)]
                    age: u32,
                },
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (_, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(
            typescript_definition,
            "export type MyEnum = \"Id\" | {\"User\": { name: string }};\n"
        );

        let enum_str = r#"
            #[derive(Type)]
            enum MyEnum {
                Request {
                    body: Bytes
                },
                Response{
                    payload: Bytes
                },
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (_, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(
            typescript_definition,
            "export type MyEnum = {\"Request\": { body: Bytes }} | {\"Response\": { payload: Bytes }};\n"
        );
    }

    #[test]
    fn it_correctly_parse_the_generic_type() {
        let enum_str = r#"
            #[derive(Type)]
            enum MyEnum<T> {
                Id,
                User {
                    name: T,
                    age: u32
                },
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (enum_name, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(enum_name, "MyEnum");
        assert_eq!(
            typescript_definition,
            "export type MyEnum<T> = \"Id\" | {\"User\": { name: T, age: number }};\n"
        );
    }

    #[test]
    fn it_correctly_parse_the_tuple_variant() {
        let enum_str = r#"
            #[derive(Type)]
            enum MyEnum {
                Id,
                User(String, u32),
            }
        "#;

        let parsed_enum = syn::parse_str::<syn::ItemEnum>(&enum_str).unwrap();
        let (_, typescript_definition) = parse_enum(&parsed_enum);

        assert_eq!(
            typescript_definition,
            "export type MyEnum = \"Id\" | {\"User\": [string, number]};\n"
        );
    }
}
