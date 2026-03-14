use crate::app::{IGNORE_EXTENSIONS, IGNORE_FILES, ROUTES_FOLDER_PATH};
use crate::route::Route;
use std::collections::{HashMap, hash_map::Entry};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use syn::{Attribute, Item};

pub const MIDDLEWARE_FILENAME: &str = "middlewares";

#[derive(Clone, Debug, Default)]
pub struct RouteDirectoryInfo {
    pub path: String,
    pub directories: Vec<RouteDirectoryInfo>,
    pub routes: HashMap<String, Route>,
    pub middlewares: Vec<String>,
}

impl RouteDirectoryInfo {
    pub fn new(path: &Path) -> io::Result<RouteDirectoryInfo> {
        if path.is_dir() {
            let mut directories = Vec::new();
            let mut routes: HashMap<String, Route> = HashMap::new();
            let mut middlewares = Vec::new();

            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();

                // recursively search directories
                if entry_path.is_dir() {
                    let sub_dir_info = RouteDirectoryInfo::new(&entry_path)?;
                    directories.push(sub_dir_info);
                // handle files
                } else if entry_path.is_file() {
                    if let Some(name) = entry_path.file_name() {
                        let name_str = name.to_string_lossy().to_string();
                        // check if middlewares file
                        if name_str == format!("{MIDDLEWARE_FILENAME}.rs") {
                            // scan middleware file for middleware layer functions
                            let middleware_data: Option<MiddlewareData> = MiddlewareData::new(
                                &entry_path.to_str().expect("Invalid filepath").to_string(),
                            );
                            middlewares = middleware_data.unwrap_or_default().middlewares;
                        } else {
                            // Generate Routes from file, add to routes
                            if RouteDirectoryInfo::should_collect_route(&entry_path) {
                                routes = RouteDirectoryInfo::collect_route(entry_path, routes);
                            }
                        }
                    }
                }
            }

            let dir_info = RouteDirectoryInfo {
                path: path.to_string_lossy().to_string(),
                directories,
                routes: routes,
                middlewares,
            };

            Ok(dir_info)
        } else {
            // If it's not a directory, return an empty DirectoryInfo (though we don't push for non-dirs)
            Ok(RouteDirectoryInfo {
                path: path.to_string_lossy().to_string(),
                directories: Vec::new(),
                routes: HashMap::new(),
                middlewares: Vec::new(),
            })
        }
    }

    pub fn has_middlewares(&self) -> bool {
        !self.middlewares.is_empty()
    }


    pub fn get_middleware_module_import(&self) -> String {
        let base_path = RouteDirectoryInfo::get_base_path();
        let base_path_str = base_path.to_string_lossy();
        let routes_path_str = format!("{base_path_str}{ROUTES_FOLDER_PATH}");
        let mut module_import = self
            .path
            .as_str()
            .to_string()
            .replace(&routes_path_str, "")
            .replacen('/', "", 1)
            .replace('/', "_")
            .replace('.', "_dot_")
            .replace('-', "_hyphen_")
            .to_lowercase();
        if module_import != "" {
            module_import += "_"
        }

        format!("{module_import}{MIDDLEWARE_FILENAME}")
    }

    pub fn get_base_path() -> PathBuf {
        std::env::current_dir().expect("Failed to read current_dir")
    }

    pub fn should_collect_route(entry: &PathBuf) -> bool {
        let file_extension = entry.extension().expect("Failed to read file extension");
        let file_name = entry.file_stem().expect("Failed to read file name");

        if IGNORE_EXTENSIONS.iter().any(|val| val == &file_extension) {
            return false;
        }

        if IGNORE_FILES.iter().any(|val| val == &file_name) {
            return false;
        }
        true
    }

    fn collect_route(entry: PathBuf, routes: HashMap<String, Route>) -> HashMap<String, Route> {
        let mut ret_routes = routes.clone();
        let base_path = RouteDirectoryInfo::get_base_path();
        let base_path_str = base_path.to_string_lossy();
        let path = entry
            .to_str()
            .expect("Failed to read entry as str")
            .replace(&format!("{base_path_str}{ROUTES_FOLDER_PATH}"), "")
            // Cleanup windows paths
            .replace("\\", "/")
            .replace(".rs", "")
            .replace(".mdx", "")
            .replace(".tsx", "");

        if entry.extension().expect("failed to read entry extension") == "rs" {
            if let Entry::Vacant(routes) = ret_routes.entry(path.clone()) {
                let mut route = Route::new(path);
                route.update_axum_info();
                routes.insert(route);
            } else {
                let route = ret_routes.get_mut(&path).unwrap();
                route.update_axum_info();
            }
            return ret_routes;
        }
        if let Entry::Vacant(routes) = ret_routes.entry(path.clone()) {
            let route = Route::new(path);
            routes.insert(route);
        }
        ret_routes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MiddlewareData {
    pub middlewares: Vec<String>,
}

impl MiddlewareData {
    pub fn new(path: &String) -> Option<Self> {
        if !(std::fs::exists(&path).unwrap_or_default()) {
            return None;
        }
        let middlewares = MiddlewareData::read_middleware_methods_from_file(&path);

        Some(MiddlewareData { middlewares })
    }

    // Given an array of syn::Attribute, returns true if the segments are "tuono_lib" and "middleware"
    fn has_middleware_attr(attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            let path = attr.path();

            let segments: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();

            segments == ["tuono_lib", "middleware"]
        })
    }

    // Reads a middlewares.rs file and returns a Vector of Strings representing functions that were decorated with the tuono_lib::middleware macro
    fn read_middleware_methods_from_file(path: &str) -> Vec<String> {
        let file = fs_extra::file::read_to_string(path).expect("Failed to read API file");
        let syntax = syn::parse_file(&file).expect("Unable to parse file");
        let mut result = Vec::new();

        for item in syntax.items {
            if let Item::Fn(func) = item {
                if MiddlewareData::has_middleware_attr(&func.attrs) {
                    result.push(func.sig.ident.to_string());
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_has_middlewares() {
        let dir_info = RouteDirectoryInfo {
            middlewares: vec!["middleware1".to_string()],
            ..Default::default()
        };
        assert!(dir_info.has_middlewares());

        let dir_info_empty = RouteDirectoryInfo::default();
        assert!(!dir_info_empty.has_middlewares());
    }

    #[test]
    fn test_get_middleware_module_import() {
        let dir_info = RouteDirectoryInfo {
            path: "/some/path/src/routes".to_string(),
            ..Default::default()
        };
        // Assuming base path is current dir, but this might vary
        // For test, we can check the format
        let import = dir_info.get_middleware_module_import();
        assert!(import.ends_with("_middlewares"));
    }

    #[test]
    fn test_get_base_path() {
        let path = RouteDirectoryInfo::get_base_path();
        assert!(path.is_absolute());
    }

    #[test]
    fn test_should_collect_route() {
        let temp_dir = TempDir::new().unwrap();
        let rs_file = temp_dir.path().join("test.rs");
        File::create(&rs_file).unwrap();
        assert!(RouteDirectoryInfo::should_collect_route(&rs_file));

        let tsx_file = temp_dir.path().join("test.tsx");
        File::create(&tsx_file).unwrap();
        assert!(RouteDirectoryInfo::should_collect_route(&tsx_file));

        let md_file = temp_dir.path().join("test.md");
        File::create(&md_file).unwrap();
        assert!(RouteDirectoryInfo::should_collect_route(&md_file));
    }

    #[test]
    fn test_collect_route() {
        let temp_dir = TempDir::new().unwrap();
        let rs_file = temp_dir.path().join("index.rs");
        File::create(&rs_file).unwrap();

        let routes = HashMap::new();
        let new_routes = RouteDirectoryInfo::collect_route(rs_file, routes);
        assert!(!new_routes.is_empty());
    }

    #[test]
    fn test_route_directory_info_new() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        let file = temp_dir.path().join("test.rs");
        File::create(&file).unwrap();
        let middlewares_file = temp_dir.path().join("middlewares.rs");
        let mut file = File::create(&middlewares_file).unwrap();
        writeln!(file, "#[tuono_lib::middleware]\nfn test_middleware() {{}}").unwrap();

        let dir_info = RouteDirectoryInfo::new(temp_dir.path()).unwrap();
        assert_eq!(dir_info.path, temp_dir.path().to_string_lossy());
        assert!(!dir_info.directories.is_empty());
        assert!(!dir_info.middlewares.is_empty());
    }

    #[test]
    fn test_middleware_data_new() {
        let temp_dir = TempDir::new().unwrap();
        let middlewares_file = temp_dir.path().join("middlewares.rs");
        let mut file = File::create(&middlewares_file).unwrap();
        writeln!(file, "#[tuono_lib::middleware]\nfn test_middleware() {{}}").unwrap();

        let middleware_data = MiddlewareData::new(&middlewares_file.to_string_lossy().to_string()).unwrap();
        assert_eq!(middleware_data.middlewares, vec!["test_middleware"]);
    }

    #[test]
    fn test_has_middleware_attr() {
        let attr: Attribute = syn::parse_quote!(#[tuono_lib::middleware]);
        assert!(MiddlewareData::has_middleware_attr(&[attr]));

        let attr2: Attribute = syn::parse_quote!(#[other_attr]);
        assert!(!MiddlewareData::has_middleware_attr(&[attr2]));
    }

    #[test]
    fn test_read_middleware_methods_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let middlewares_file = temp_dir.path().join("middlewares.rs");
        let mut file = File::create(&middlewares_file).unwrap();
        writeln!(file, "#[tuono_lib::middleware]\nfn test_middleware() {{}}\nfn other_fn() {{}}").unwrap();

        let methods = MiddlewareData::read_middleware_methods_from_file(&middlewares_file.to_string_lossy());
        assert_eq!(methods, vec!["test_middleware"]);
    }
}
