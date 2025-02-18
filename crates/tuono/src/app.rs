use crate::mode::Mode;
use glob::glob;
use glob::GlobError;
use http::Method;
use std::collections::hash_set::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use tracing::error;
use tuono_internal::config::Config;

use crate::route::Route;

const IGNORE_EXTENSIONS: [&str; 3] = ["css", "scss", "sass"];
const IGNORE_FILES: [&str; 1] = ["__layout"];

#[cfg(target_os = "windows")]
const ROUTES_FOLDER_PATH: &str = "\\src\\routes";
#[cfg(target_os = "windows")]
const BUILD_JS_SCRIPT: &str = ".\\node_modules\\.bin\\tuono-build-prod.cmd";

#[cfg(target_os = "windows")]
const BUILD_TUONO_CONFIG: &str = ".\\node_modules\\.bin\\tuono-build-config.cmd";

#[cfg(not(target_os = "windows"))]
const ROUTES_FOLDER_PATH: &str = "/src/routes";
#[cfg(not(target_os = "windows"))]
const BUILD_JS_SCRIPT: &str = "./node_modules/.bin/tuono-build-prod";

#[cfg(not(target_os = "windows"))]
const BUILD_TUONO_CONFIG: &str = "./node_modules/.bin/tuono-build-config";

#[derive(Debug, Clone)]
pub struct App {
    pub route_map: HashMap<String, Route>,
    pub base_path: PathBuf,
    pub has_app_state: bool,
    pub config: Option<Config>,
}

fn has_app_state(base_path: PathBuf) -> std::io::Result<bool> {
    let file = File::open(base_path.join("src/app.rs"))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents.contains("pub fn main"))
}

impl App {
    pub fn new() -> Self {
        let base_path = std::env::current_dir().expect("Failed to read current_dir");

        let mut app = App {
            route_map: HashMap::new(),
            base_path: base_path.clone(),
            has_app_state: has_app_state(base_path).unwrap_or(false),
            config: None,
        };

        app.collect_routes();

        app
    }

    fn collect_routes(&mut self) {
        glob(
            self.base_path
                .join("src/routes/**/*.*")
                .to_str()
                .expect("Failed to glob routes folder"),
        )
        .expect("Failed to read glob pattern")
        .for_each(|entry| {
            if self.should_collect_route(&entry) {
                self.collect_route(entry)
            }
        })
    }

    fn should_collect_route(&self, entry: &Result<PathBuf, GlobError>) -> bool {
        let file_extension = entry
            .as_ref()
            .unwrap()
            .extension()
            .expect("Failed to read file extension");
        let file_name = entry
            .as_ref()
            .unwrap()
            .file_stem()
            .expect("Failed to read file name");

        if IGNORE_EXTENSIONS.iter().any(|val| val == &file_extension) {
            return false;
        }

        if IGNORE_FILES.iter().any(|val| val == &file_name) {
            return false;
        }
        true
    }

    fn collect_route(&mut self, path_buf: Result<PathBuf, GlobError>) {
        let entry = path_buf.expect("Failed to read glob path");

        let base_path_str = self
            .base_path
            .to_str()
            .expect("Failed to read as str base_path");
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
            if let Entry::Vacant(route_map) = self.route_map.entry(path.clone()) {
                let mut route = Route::new(path);
                route.update_axum_info();
                route_map.insert(route);
            } else {
                let route = self.route_map.get_mut(&path).unwrap();
                route.update_axum_info();
            }
            return;
        }
        if let Entry::Vacant(route_map) = self.route_map.entry(path.clone()) {
            let route = Route::new(path);
            route_map.insert(route);
        }
    }

    pub fn has_dynamic_routes(&self) -> bool {
        self.route_map.iter().any(|(_, route)| route.is_dynamic)
    }

    pub fn check_server_availability(&self, mode: Mode) {
        // At this point the config should be available
        let config = self.config.as_ref().unwrap();

        let rust_listener =
            std::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port));

        if let Err(_e) = rust_listener {
            eprintln!("Error: Failed to bind to port {}", config.server.port);
            eprintln!(
            "Please ensure that port {} is not already in use by another process or application.",
            config.server.port
        );
            std::process::exit(1);
        }

        if mode == Mode::Dev {
            let vite_port = config.server.port + 1;
            let vite_listener =
                std::net::TcpListener::bind(format!("{}:{}", config.server.host, vite_port));

            if let Err(_e) = vite_listener {
                eprintln!("Error: Failed to bind to port {}", vite_port);
                eprintln!(
                "Please ensure that port {} is not already in use by another process or application.",
                vite_port
            );
                std::process::exit(1);
            }
        }
    }

    pub fn build_react_prod(&self) {
        if !Path::new(BUILD_JS_SCRIPT).exists() {
            error!("Failed to find the build script. Please run `npm install`");
            std::process::exit(1);
        }

        let output = Command::new(BUILD_JS_SCRIPT).output().unwrap_or_else(|_| {
            error!("Failed to build the react source");
            std::process::exit(1);
        });

        if !output.status.success() {
            error!("Failed to build the react source");
            error!("Error: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
    }

    pub fn run_rust_server(&self) -> Child {
        Command::new("cargo")
            .arg("run")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run the rust server")
    }

    pub fn build_tuono_config(&mut self) -> Result<std::process::Output, std::io::Error> {
        if !Path::new(BUILD_TUONO_CONFIG).exists() {
            eprintln!("Failed to find the build script. Please run `npm install`");
            std::process::exit(1);
        }
        let config_build_command = Command::new(BUILD_TUONO_CONFIG)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        if let Ok(config) = Config::get() {
            self.config = Some(config);
        } else {
            eprintln!("[CLI] Failed to read tuono.config.ts");
            std::process::exit(1);
        };

        config_build_command
    }
    pub fn get_used_http_methods(&self) -> HashSet<Method> {
        let mut acc = HashSet::new();

        for (_, route) in self.route_map.clone().into_iter() {
            if route.axum_info.is_some() {
                acc.insert(Method::GET);
            }
            if !route.is_api() {
                continue;
            }
            for method in route.api_data.unwrap().methods.into_iter() {
                acc.insert(method);
            }
        }

        acc
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_collect_routes() {
        let mut app = App::new();
        #[cfg(target_os = "windows")]
        let base_path = "\\home\\user\\Documents\\tuono";

        #[cfg(not(target_os = "windows"))]
        let base_path = "/home/user/Documents/tuono";

        app.base_path = base_path.into();

        #[cfg(target_os = "windows")]
        let routes = [
            "\\home\\user\\Documents\\tuono\\src\\routes\\about.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\index.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\index.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\[post].rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\handle-this.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\handle-this\\[post].rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\UPPERCASE.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\sitemap.xml.rs",
        ];

        #[cfg(not(target_os = "windows"))]
        let routes = [
            "/home/user/Documents/tuono/src/routes/about.rs",
            "/home/user/Documents/tuono/src/routes/index.rs",
            "/home/user/Documents/tuono/src/routes/posts/index.rs",
            "/home/user/Documents/tuono/src/routes/posts/[post].rs",
            "/home/user/Documents/tuono/src/routes/posts/handle-this.rs",
            "/home/user/Documents/tuono/src/routes/posts/handle-this/[post].rs",
            "/home/user/Documents/tuono/src/routes/posts/UPPERCASE.rs",
            "/home/user/Documents/tuono/src/routes/sitemap.xml.rs",
        ];

        routes
            .into_iter()
            .for_each(|route| app.collect_route(Ok(PathBuf::from(route))));

        let results = [
            ("/index", "index"),
            ("/about", "about"),
            ("/posts/index", "posts_index"),
            ("/posts/[post]", "posts_dyn_post"),
            ("/posts/handle-this", "posts_handle_hyphen_this"),
            (
                "/posts/handle-this/[post]",
                "posts_handle_hyphen_this_dyn_post",
            ),
            ("/posts/UPPERCASE", "posts_uppercase"),
            ("/sitemap.xml", "sitemap_dot_xml"),
        ];

        results.into_iter().for_each(|(path, module_import)| {
            assert_eq!(
                app.route_map
                    .get(path)
                    .unwrap()
                    .axum_info
                    .as_ref()
                    .unwrap()
                    .module_import,
                String::from(module_import)
            )
        })
    }

    #[test]
    fn should_create_multi_level_axum_paths() {
        let mut app = App::new();

        #[cfg(target_os = "windows")]
        let base_path = "\\home\\user\\Documents\\tuono";

        #[cfg(not(target_os = "windows"))]
        let base_path = "/home/user/Documents/tuono";

        app.base_path = base_path.into();

        #[cfg(target_os = "windows")]
        let routes = [
            "\\home\\user\\Documents\\tuono\\src\\routes\\about.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\index.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\index.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\any-post.rs",
            "\\home\\user\\Documents\\tuono\\src\\routes\\posts\\[post].rs",
        ];

        #[cfg(not(target_os = "windows"))]
        let routes = [
            "/home/user/Documents/tuono/src/routes/about.rs",
            "/home/user/Documents/tuono/src/routes/index.rs",
            "/home/user/Documents/tuono/src/routes/posts/index.rs",
            "/home/user/Documents/tuono/src/routes/posts/any-post.rs",
            "/home/user/Documents/tuono/src/routes/posts/[post].rs",
        ];

        routes
            .into_iter()
            .for_each(|route| app.collect_route(Ok(PathBuf::from(route))));

        let results = [
            ("/index", "/"),
            ("/about", "/about"),
            ("/posts/index", "/posts"),
            ("/posts/any-post", "/posts/any-post"),
            ("/posts/[post]", "/posts/:post"),
        ];

        results.into_iter().for_each(|(path, expected_path)| {
            assert_eq!(
                app.route_map
                    .get(path)
                    .expect("Failed to get route path")
                    .axum_info
                    .as_ref()
                    .unwrap()
                    .axum_route,
                String::from(expected_path)
            )
        })
    }

    #[test]
    fn should_ignore_whitelisted_extensions() {
        let mut app = App::new();
        app.base_path = "/home/user/Documents/tuono".into();

        let routes = [
            "/home/user/Documents/tuono/src/routes/about.css",
            "/home/user/Documents/tuono/src/routes/index.scss",
            "/home/user/Documents/tuono/src/routes/posts/index.sass",
        ];

        routes.into_iter().for_each(|route| {
            if app.should_collect_route(&Ok(PathBuf::from(route))) {
                app.collect_route(Ok(PathBuf::from(route)))
            }
        });

        assert!(app.route_map.is_empty())
    }

    #[test]
    fn should_ignore_whitelisted_files() {
        let mut app = App::new();
        app.base_path = "/home/user/Documents/tuono".into();

        let routes = [
            "/home/user/Documents/tuono/src/routes/__layout.tsx",
            "/home/user/Documents/tuono/src/routes/posts/__layout.tsx",
        ];

        routes.into_iter().for_each(|route| {
            if app.should_collect_route(&Ok(PathBuf::from(route))) {
                app.collect_route(Ok(PathBuf::from(route)))
            }
        });

        assert!(app.route_map.is_empty())
    }

    #[test]
    fn should_correctly_parse_routes_with_server_handler() {
        let mut app = App::new();

        #[cfg(target_os = "windows")]
        let base_path = "\\home\\user\\Documents\\tuono";

        #[cfg(not(target_os = "windows"))]
        let base_path = "/home/user/Documents/tuono";

        app.base_path = base_path.into();

        #[cfg(target_os = "windows")]
        let routes = [
            "\\home\\user\\Documents\\tuono\\src\\routes\\about.rs",
            "\\home\\user\\Documents/tuono\\src\\routes\\about.tsx",
            "\\home\\user\\Documents\\tuono\\src\\routes\\index.tsx",
        ];

        #[cfg(not(target_os = "windows"))]
        let routes = [
            "/home/user/Documents/tuono/src/routes/about.rs",
            "/home/user/Documents/tuono/src/routes/about.tsx",
            "/home/user/Documents/tuono/src/routes/index.tsx",
        ];

        routes
            .into_iter()
            .for_each(|route| app.collect_route(Ok(PathBuf::from(route))));

        let results = [("/about", true), ("/index", false)];

        results
            .into_iter()
            .for_each(|(path, expected_has_server_handler)| {
                if expected_has_server_handler {
                    assert!(app
                        .route_map
                        .get(path)
                        .expect("Failed to get route path")
                        .axum_info
                        .is_some())
                } else {
                    assert!(app
                        .route_map
                        .get(path)
                        .expect("Failed to get route path")
                        .axum_info
                        .is_none())
                }
            })
    }

    #[test]
    fn has_dynamic_routes_works() {
        let mut app = App::new();
        app.base_path = "/home/user/Documents/tuono".into();

        let routes = [
            "/home/user/Documents/tuono/src/routes/index.rs",
            "/home/user/Documents/tuono/src/routes/posts/[post].rs",
        ];

        routes
            .into_iter()
            .for_each(|route| app.collect_route(Ok(PathBuf::from(route))));

        assert!(app.has_dynamic_routes());

        let mut app2 = App::new();
        app2.base_path = "/home/user/Documents/tuono".into();

        let routes = [
            "/home/user/Documents/tuono/src/routes/[post].rs",
            "/home/user/Documents/tuono/src/routes/posts/[post].rs",
        ];

        routes
            .into_iter()
            .for_each(|route| app2.collect_route(Ok(PathBuf::from(route))));

        assert!(app2.has_dynamic_routes());

        let mut app3 = App::new();
        app3.base_path = "/home/user/Documents/tuono".into();

        let routes = [
            "/home/user/Documents/tuono/src/routes/index.rs",
            "/home/user/Documents/tuono/src/routes/posts/index.rs",
        ];

        routes
            .into_iter()
            .for_each(|route| app3.collect_route(Ok(PathBuf::from(route))));

        assert!(!app3.has_dynamic_routes())
    }
}
