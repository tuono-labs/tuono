use crate::typescript::FileTypes;
use glob::glob;
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
    #[allow(dead_code)]
    pub fn remove_types_from_file(&mut self, file_path: &PathBuf) {
        self.types.retain(|ttype| ttype.file_path != *file_path);
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

    /// Generate the string containing all the typescript types
    /// found in the jar.
    fn generate_typescript(&self) -> String {
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
