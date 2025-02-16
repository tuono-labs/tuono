use fs_extra::dir::create_all;
use http::Method;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::Url;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::trace;

fn has_dynamic_path(route: &str) -> bool {
    let regex = Regex::new(r"\[(.*?)\]").expect("Failed to create the regex");
    regex.is_match(route)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AxumInfo {
    // Path for importing the module
    pub module_import: String,
    // path for the the axum router
    pub axum_route: String,
}

impl AxumInfo {
    pub fn new(route: &Route) -> Self {
        // Remove first slash
        let mut module = route.path.chars();
        module.next();

        let axum_route = route.path.replace("/index", "");

        let module_import = module
            .as_str()
            .to_string()
            .replace('/', "_")
            .replace('.', "_dot_")
            .replace('-', "_hyphen_")
            .to_lowercase();

        if axum_route.is_empty() {
            return AxumInfo {
                module_import,
                axum_route: "/".to_string(),
            };
        }

        if route.is_dynamic {
            return AxumInfo {
                module_import: module
                    .as_str()
                    .to_string()
                    .replace('/', "_")
                    .replace('-', "_hyphen_")
                    .replace('[', "dyn_")
                    .replace("...", "catch_all_")
                    .replace(']', ""),
                axum_route: axum_route
                    .replace("[...", "*")
                    .replace('[', ":")
                    .replace(']', ""),
            };
        }

        AxumInfo {
            module_import,
            axum_route,
        }
    }
}

// TODO: to be extended with common scenarios
const NO_HTML_EXTENSIONS: [&str; 1] = ["xml"];

// TODO: Refine this function to catch
// if the methods are commented.
fn read_http_methods_from_file(path: &String) -> Vec<Method> {
    let regex = Regex::new(r"tuono_lib::api\((.*?)\)]").expect("Failed to create API regex");

    let file = fs_extra::file::read_to_string(path).expect("Failed to read API file");

    regex
        .find_iter(&file)
        .map(|proc_macro| {
            let http_method = proc_macro
                .as_str()
                // Extract just the element surrounded by the phrantesist.
                .replace("tuono_lib::api(", "")
                .replace(")]", "");
            Method::from_str(http_method.as_str()).unwrap_or(Method::GET)
        })
        .collect::<Vec<Method>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ApiData {
    pub methods: Vec<Method>,
}

impl ApiData {
    pub fn new(path: &String) -> Option<Self> {
        if !path.starts_with("/api/") {
            return None;
        }

        let base_path = std::env::current_dir().expect("Failed to get the base_path");

        let file_path = base_path
            .join(format!("src/routes{path}.rs"))
            .to_str()
            .unwrap()
            .to_string();
        let methods = read_http_methods_from_file(&file_path);

        Some(ApiData { methods })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Route {
    path: String,
    pub is_dynamic: bool,
    pub axum_info: Option<AxumInfo>,
    pub api_data: Option<ApiData>,
}

impl Route {
    pub fn new(cleaned_path: String) -> Self {
        Route {
            path: cleaned_path.clone(),
            axum_info: None,
            is_dynamic: has_dynamic_path(&cleaned_path),
            api_data: ApiData::new(&cleaned_path),
        }
    }

    pub fn is_api(&self) -> bool {
        self.api_data.is_some()
    }

    pub fn update_axum_info(&mut self) {
        self.axum_info = Some(AxumInfo::new(self))
    }

    pub fn save_ssg_file(&self, reqwest: &Client) -> Result<(), String> {
        let path = &self.path.replace("index", "");

        let url = format!("http://localhost:3000{path}");

        trace!("Requesting the page: {}", url);
        let mut response = match reqwest.get(format!("http://localhost:3000{path}")).send() {
            Ok(response) => response,
            Err(_) => return Err(format!("Failed to get the response: {}", url)),
        };

        let file_path = self.output_file_path();

        let parent_dir = match file_path.parent() {
            Some(parent_dir) => parent_dir,
            None => {
                return Err(format!(
                    "Failed to get the parent directory {:?}",
                    file_path
                ))
            }
        };

        if !parent_dir.is_dir() {
            if let Err(_) = create_all(parent_dir, false) {
                return Err(format!(
                    "Failed to create the parent directory {:?}",
                    parent_dir
                ));
            }
        }

        trace!("Saving the HTML file: {:?}", file_path);

        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Failed to create the file: {:?}", file_path)),
        };

        if let Err(_) = io::copy(&mut response, &mut file) {
            return Err(format!("Failed to write the file: {:?}", file_path));
        }

        // Saving also the server response
        if self.axum_info.is_some() {
            trace!("The route is an axum route, saving the JSON file");

            let data_file_path = PathBuf::from(&format!("out/static/__tuono/data{path}.json"));

            let data_parent_dir = match data_file_path.parent() {
                Some(parent_dir) => parent_dir,
                None => {
                    return Err(format!(
                        "Failed to get the parent directory {:?}",
                        data_file_path
                    ))
                }
            };

            if !data_parent_dir.is_dir() {
                if let Err(_) = create_all(data_parent_dir, false) {
                    return Err(format!(
                        "Failed to create the parent directory {:?}",
                        data_parent_dir
                    ));
                }
            }

            let base = Url::parse("http://localhost:3000/__tuono/data").unwrap();

            let path = if path == "/" { "" } else { path };

            let pathname = &format!("/__tuono/data{path}");

            let url = base
                .join(pathname)
                .expect("Failed to build the reqwest URL");

            trace!("Requesting the JSON file: {}", url);

            let mut response = match reqwest.get(url.clone()).send() {
                Ok(response) => {
                    trace!("Successfully got the response for: {url}");
                    response
                }
                Err(_) => return Err(format!("Failed to get the response: {url}")),
            };

            let mut data_file = match File::create(&data_file_path) {
                Ok(file) => file,
                Err(_) => {
                    return Err(format!(
                        "Failed to create the JSON file: {:?}",
                        data_file_path
                    ))
                }
            };

            if let Err(_) = io::copy(&mut response, &mut data_file) {
                return Err(format!("Failed to write the JSON file"));
            }
        }

        Ok(())
    }

    fn output_file_path(&self) -> PathBuf {
        let cleaned_path = self.path.replace("index", "");

        if NO_HTML_EXTENSIONS
            .iter()
            .any(|extension| self.path.ends_with(extension))
        {
            return PathBuf::from(format!("out/static{}", cleaned_path));
        }

        PathBuf::from(format!("out/static{}/index.html", cleaned_path))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_find_dynamic_paths() {
        let routes = [
            ("/home/user/Documents/tuono/src/routes/about.rs", false),
            ("/home/user/Documents/tuono/src/routes/index.rs", false),
            (
                "/home/user/Documents/tuono/src/routes/posts/index.rs",
                false,
            ),
            (
                "/home/user/Documents/tuono/src/routes/posts/[post].rs",
                true,
            ),
        ];

        routes
            .into_iter()
            .for_each(|route| assert_eq!(has_dynamic_path(route.0), route.1));
    }

    #[test]
    fn should_correctly_create_the_axum_infos() {
        let info = AxumInfo::new(&Route::new("/index".to_string()));

        assert_eq!(info.axum_route, "/");
        assert_eq!(info.module_import, "index");

        let dyn_info = AxumInfo::new(&Route::new("/[posts]".to_string()));

        assert_eq!(dyn_info.axum_route, "/:posts");
        assert_eq!(dyn_info.module_import, "dyn_posts");
    }

    #[test]
    fn should_define_the_correct_html_build_path() {
        let routes = [
            ("/index", "out/static/index.html"),
            ("/documentation", "out/static/documentation/index.html"),
            ("/sitemap.xml", "out/static/sitemap.xml"),
            (
                "/documentation/routing",
                "out/static/documentation/routing/index.html",
            ),
        ];

        for (path, html) in routes {
            let route = Route::new(path.to_string());

            assert_eq!(route.output_file_path(), PathBuf::from(html))
        }
    }
}
