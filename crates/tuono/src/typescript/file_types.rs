use super::utils::has_derive_type;
use crate::typescript::parser::{parse_enum, parse_struct};
use std::error::Error;
use std::path::PathBuf;
use tracing::trace;

/// Represents all the valid typescript types found in a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTypes {
    /// Rust file when the type was found
    pub file_path: PathBuf,
    /// All the types found in the file
    /// ready to be printed in the typescript file
    pub types_as_string: String,
    /// The types found in the file.
    /// Used to check that the types are not duplicated across files.
    pub types: Vec<String>,
}

impl TryFrom<(PathBuf, String)> for FileTypes {
    type Error = Box<dyn Error>;

    fn try_from((file_path, file_str): (PathBuf, String)) -> Result<Self, Self::Error> {
        trace!("Parsing file: {:?}", &file_path);
        let file = syn::parse_file(&file_str)?;

        let mut types_as_string = String::new();
        let mut types = Vec::new();

        for item in file.items {
            match item {
                syn::Item::Struct(element) => {
                    if !has_derive_type(&element.attrs) {
                        continue;
                    }
                    trace!("Found struct in file: {:?}", &file_path);
                    let (struct_name, typescript_definition) = parse_struct(&element);
                    types_as_string.push_str(&typescript_definition);
                    types.push(struct_name);
                }
                syn::Item::Enum(element) => {
                    if !has_derive_type(&element.attrs) {
                        continue;
                    }
                    trace!("Found enum in file: {:?}", &file_path);

                    let (enum_name, typescript_definition) = parse_enum(&element);
                    types_as_string.push_str(&typescript_definition);
                    types.push(enum_name);
                }
                _ => {}
            }
        }

        if types_as_string.is_empty() {
            return Err("No types found in the file".into());
        }

        Ok(Self {
            file_path,
            types_as_string,
            types,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_correctly_creates_type_from_pathbuf_and_string() {
        let file_path = PathBuf::from("src/types.rs");
        let file_str = r#"
            #[derive(Type)]
            struct MyStruct {
                field1: String,
                field2: i32,
            }

            #[derive(Type)]
            enum MyEnum {
                Variant1,
                Variant2,
            }
        "#
        .to_string();

        let ttype = FileTypes::try_from((file_path.clone(), file_str)).unwrap();

        assert_eq!(ttype.file_path, file_path);
        assert_eq!(
            ttype.types_as_string,
            "export interface MyStruct {\n  field1: string;\n  field2: number;\n}\nexport type MyEnum = \"Variant1\" | \"Variant2\";\n"
        );
        assert_eq!(ttype.types, vec!["MyStruct", "MyEnum"]);
    }
}
