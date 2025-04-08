use crate::typescript::FileTypes;
use glob::glob;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use tracing::error;

const TUONO_MACRO_TRAIT_NAME: &str = "Type";

#[derive(Debug, Clone, Default)]
pub struct TypesJar {
    types: Vec<FileTypes>,
}

impl TypesJar {
    pub fn new() -> Self {
        Self { types: vec![] }
    }
}

impl TypesJar {
    /// Removes all types from the jar that are
    /// present in the provided `file_path`.
    /// This function is triggered when a file is deleted
    pub fn remove_file(&mut self, file_path: PathBuf) {
        self.types.retain(|ttype| ttype.file_path != file_path);
    }

    pub fn refresh_file(&mut self, path: PathBuf) {
        if let Ok(file_str) = read_to_string(&path) {
            if file_str.contains(TUONO_MACRO_TRAIT_NAME) {
                if let Ok(ttype) = FileTypes::try_from((path.clone(), file_str)) {
                    self.types.retain(|t| t.file_path != path);
                    self.types.push(ttype);
                } else {
                    error!("Failed to parse file: {:?}", path);
                }
            }
        } else {
            error!("Failed to read file: {:?}", path);
        }
    }

    pub fn check_duplicate_types(&self) -> HashMap<String, (PathBuf, PathBuf)> {
        let mut duplicates: HashMap<String, (PathBuf, PathBuf)> = HashMap::new();
        let mut paths: Vec<&PathBuf> = vec![];
        let mut types: Vec<&Vec<String>> = vec![];

        for file_types in self.types.iter() {
            paths.push(&file_types.file_path);
            types.push(&file_types.types);
        }

        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                let types_i = types[i];
                let types_j = types[j];

                for type_i in types_i {
                    if types_j.contains(type_i) {
                        duplicates.insert(type_i.clone(), (paths[j].clone(), paths[i].clone()));
                    }
                }
            }
        }

        duplicates
    }

    /// Generate the string containing all the typescript types
    /// found in the jar.
    fn generate_typescript(&self) -> String {
        let duplicates = self.check_duplicate_types();
        let base_path = env::current_dir().unwrap_or_default();
        let base_path_str = base_path.to_string_lossy();

        for (type_name, file_paths) in duplicates.iter() {
            // TODO: replace this with tuono_println! macro
            println!(
                "  Duplicate \"{}\" type found in files:\n\n  - {}\n  - {}",
                type_name,
                file_paths
                    .0
                    .to_string_lossy()
                    .replace(&base_path_str.to_string(), ""),
                file_paths
                    .1
                    .to_string_lossy()
                    .replace(&base_path_str.to_string(), ""),
            );
        }

        let mut typescript = String::from("declare module \"tuono/types\" {\n");
        for ttype in &self.types {
            typescript.push_str(&format!(
                "// START [{}]\n",
                ttype.file_path.to_string_lossy()
            ));
            typescript.push_str(&ttype.types_as_string);
            typescript.push_str(&format!("// END [{}]\n", ttype.file_path.to_string_lossy()));
        }
        typescript.push_str("}\n");
        typescript
    }

    pub fn generate_typescript_file(&self, base_path: &Path) -> std::io::Result<()> {
        let typescript = self.generate_typescript();
        let typescript_file_path = base_path.join(".tuono").join("types.ts");
        std::fs::write(typescript_file_path, typescript)?;
        Ok(())
    }
}

impl From<&PathBuf> for TypesJar {
    /// Fill the TypesJar with all the Rust files found within
    /// the provided `base_path`.
    fn from(base_path: &PathBuf) -> Self {
        let mut jar = Self::new();

        if let Some(path) = base_path.join("src/**/*.rs").to_str() {
            if let Ok(files) = glob(path) {
                files.for_each(|path| {
                    let file_path = path.unwrap_or_default();
                    if let Ok(file_str) = read_to_string(&file_path) {
                        if file_str.contains(TUONO_MACRO_TRAIT_NAME) {
                            if let Ok(ttype) = FileTypes::try_from((file_path.clone(), file_str)) {
                                jar.types.push(ttype);
                            } else {
                                error!("Failed to parse file: {:?}", file_path);
                            }
                        }
                    } else {
                        error!("Failed to read file: {:?}", file_path);
                    }
                });
            } else {
                error!("Failed to read glob pattern: {:?}", path);
            }
        }
        jar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_finds_duplicate_types() {
        let file_type1 = FileTypes {
            file_path: PathBuf::from("src/types1.rs"),
            types_as_string: String::from("type1"),
            types: vec![
                String::from("Type1"),
                String::from("Type2"),
                String::from("Type3"),
            ],
        };
        let file_type2 = FileTypes {
            file_path: PathBuf::from("src/types2.rs"),
            types_as_string: String::from("type1"),
            types: vec![
                String::from("Type1"),
                String::from("Type2"),
                String::from("Type4"),
            ],
        };

        let file_type3 = FileTypes {
            file_path: PathBuf::from("src/types2.rs"),
            types_as_string: String::from("type1"),
            types: vec![String::from("Type3")],
        };

        let mut jar = TypesJar::new();
        jar.types.push(file_type1);
        jar.types.push(file_type2);
        jar.types.push(file_type3);

        let result = jar.check_duplicate_types();

        assert!(
            result
                .keys()
                .collect::<Vec<&String>>()
                .contains(&&"Type1".to_string())
        );
        assert!(
            result
                .keys()
                .collect::<Vec<&String>>()
                .contains(&&"Type2".to_string())
        );
        assert!(
            result
                .keys()
                .collect::<Vec<&String>>()
                .contains(&&"Type3".to_string())
        );

        assert!(
            !result
                .keys()
                .collect::<Vec<&String>>()
                .contains(&&"Type4".to_string())
        );
    }
}
