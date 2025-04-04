use crate::typescript::FileTypes;
use glob::glob;
use std::fs::read_to_string;
use std::path::PathBuf;
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
    pub fn remove_types_from_file(&mut self, file_path: &PathBuf) {
        self.types.retain(|ttype| ttype.file_path != *file_path);
    }

    /// Generate the string containing all the typescript types
    /// found in the jar.
    fn generate_typescript(&self) -> String {
        let mut typescript = String::new();
        for ttype in &self.types {
            typescript.push_str(&ttype.types_as_string);
        }
        typescript
    }

    pub fn generate_typescript_file(&self, base_path: &PathBuf) -> std::io::Result<()> {
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
                            if let Ok(ttype) = FileTypes::try_from((file_path, file_str)) {
                                jar.types.push(ttype);
                            }
                        }
                    }
                });
            }
        }
        jar
    }
}
