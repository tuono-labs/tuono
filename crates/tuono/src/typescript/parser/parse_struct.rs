use crate::typescript::parser::utils::{
    RenameSerdeOptions, get_field_name, parse_generics_to_typescript_string, parse_serde_attribute,
    rust_to_typescript_type,
};

use super::utils::should_skip_element;

/// Parse a rust struct and returns a tuple of the struct name and the
/// struct compiled to a typescript interface
pub fn parse_struct(element: &syn::ItemStruct) -> (String, String) {
    let struct_name = element.ident.to_string();
    let generics = parse_generics_to_typescript_string(element.generics.clone().params);

    let mut fields_as_string = format!("export interface {struct_name}{generics} {{\n");

    let rename_option: RenameSerdeOptions = parse_serde_attribute(&element.attrs, "rename_all");

    for field in &element.fields {
        if should_skip_element(&field.attrs) {
            continue;
        }

        let field_name = get_field_name(field);

        let field_type = rust_to_typescript_type(&field.ty);

        let field_name = rename_option.transform(field_name);

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
    fn it_correctly_parse_tuple_fields() {
        let struct_str = r#"
            #[derive(Type)]
            struct MyStruct {
                tuple: (i32, i32, String, User),

            }"#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let (_, typescript_definition) = parse_struct(&parsed_struct);

        assert_eq!(
            typescript_definition,
            "export interface MyStruct {\n  tuple: [number, number, string, User];\n}\n"
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
        let rename_option: RenameSerdeOptions =
            parse_serde_attribute(&parsed_struct.attrs, "rename_all");

        assert_eq!(rename_option, RenameSerdeOptions::PascalCase);

        let struct_str = r#"
            #[derive(Type)]
            #[serde(rename_all = "camelCase")]
            struct MyStruct {
                field_one: &str,
            }"#;
        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let rename_option: RenameSerdeOptions =
            parse_serde_attribute(&parsed_struct.attrs, "rename_all");
        assert_eq!(rename_option, RenameSerdeOptions::CamelCase);
    }

    #[test]
    fn it_correctly_identifies_skip_fields() {
        let struct_str = r#"
            #[derive(Type)]
            struct MyStruct {
                #[serde(skip)]
                field_one: &str,
                field_two: i32,
                #[serde(skip_serializing)]
                field_three: i32,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();

        let (_, typescript_definition) = parse_struct(&parsed_struct);
        assert_eq!(
            typescript_definition,
            "export interface MyStruct {\n  field_two: number;\n}\n"
        );
    }

    #[test]
    fn it_correctly_override_a_field_name() {
        let struct_str = r#"
            #[derive(Type)]
            struct MyStruct {
                #[serde(rename = "field_one")]
                field_two: i32,
            }
        "#;

        let parsed_struct = syn::parse_str::<syn::ItemStruct>(struct_str).unwrap();
        let (_, typescript_definition) = parse_struct(&parsed_struct);
        assert_eq!(
            typescript_definition,
            "export interface MyStruct {\n  field_one: number;\n}\n"
        );
    }
}
