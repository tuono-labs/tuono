use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use clap::crate_version;
use tracing::error;

use crate::app::App;
use crate::mode::Mode;
use crate::route::AxumInfo;
use crate::route::Route;
use crate::typescript::TypesJar;

#[cfg(not(target_os = "windows"))]
const FALLBACK_HTML: &str = include_str!("../templates/fallback.html");
#[cfg(not(target_os = "windows"))]
const SERVER_ENTRY_DATA: &str = include_str!("../templates/server.ts");
#[cfg(not(target_os = "windows"))]
const CLIENT_ENTRY_DATA: &str = include_str!("../templates/client.ts");
#[cfg(not(target_os = "windows"))]
const AXUM_ENTRY_POINT: &str = include_str!("../templates/server.rs");
#[cfg(not(target_os = "windows"))]
const AXUM_ROUTER: &str = include_str!("../templates/router.rs");

#[cfg(not(target_os = "windows"))]
const MAIN_FILE_PATH: &str = "./.tuono/main.rs";
#[cfg(not(target_os = "windows"))]
const ROUTER_FILE_PATH: &str = "./.tuono/router.rs";

#[cfg(not(target_os = "windows"))]
const FALLBACK_HTML_PATH: &str = "./.tuono/index.html";

const ROUTE_FOLDER: &str = "src/routes";
const DEV_FOLDER: &str = ".tuono";

#[cfg(target_os = "windows")]
const FALLBACK_HTML: &str = include_str!("..\\templates\\fallback.html");
#[cfg(target_os = "windows")]
const SERVER_ENTRY_DATA: &str = include_str!("..\\templates\\server.ts");
#[cfg(target_os = "windows")]
const CLIENT_ENTRY_DATA: &str = include_str!("..\\templates\\client.ts");
#[cfg(target_os = "windows")]
const AXUM_ENTRY_POINT: &str = include_str!("..\\templates\\server.rs");
#[cfg(target_os = "windows")]
const AXUM_ROUTER: &str = include_str!("..\\templates\\router.rs");

#[cfg(target_os = "windows")]
const MAIN_FILE_PATH: &str = ".\\.tuono\\main.rs";
#[cfg(target_os = "windows")]
const ROUTER_FILE_PATH: &str = ".\\.tuono\\router.rs";

#[cfg(target_os = "windows")]
const FALLBACK_HTML_PATH: &str = ".\\.tuono\\index.html";

// Use this function to instruct the users on how to
// fix their setup to make tuono work
fn recoverable_error(message: &str) -> ! {
    error!("{}", message);
    std::process::exit(1);
}

// Struct to build the source code
// on both "dev" and "build" commands
#[derive(Clone, Debug)]
pub struct SourceBuilder {
    pub app: App,
    mode: Mode,
    base_path: PathBuf,
    types_jar: TypesJar,
}

impl SourceBuilder {
    pub fn new(mode: Mode) -> io::Result<Self> {
        if !PathBuf::from("tuono.config.ts").exists() {
            recoverable_error("Cannot find tuono.config.ts - is this a tuono project?");
        }

        let dev_folder = Path::new(DEV_FOLDER);
        if !&dev_folder.is_dir() {
            fs::create_dir(dev_folder)?;
        }

        let app = App::new();

        let base_path = std::env::current_dir()?;

        Ok(Self {
            app,
            mode,
            types_jar: TypesJar::from(&base_path),
            base_path,
        })
    }

    // Build the source code needed for both build and dev
    pub fn base_build(&mut self) -> io::Result<()> {
        let mode = self.mode.clone();

        self.refresh_axum_source()?;
        let dev_folder = Path::new(DEV_FOLDER);
        self.create_file(dev_folder.join("server-main.tsx"), SERVER_ENTRY_DATA)?;
        self.create_file(dev_folder.join("client-main.tsx"), CLIENT_ENTRY_DATA)?;

        self.types_jar.generate_typescript_file(&self.base_path)?;

        if mode == Mode::Dev {
            self.app.build_tuono_config()?;
            let fallback_html = self.build_html_fallback();
            self.create_file(PathBuf::from(FALLBACK_HTML_PATH), &fallback_html)?;
        }

        Ok(())
    }

    fn generate_axum_source(&self) -> (Option<String>, String) {
        let Self { app, mode, .. } = &self;

        let router_src = AXUM_ROUTER
            .replace("\r", "")
            .replace("//ROUTE_BUILDER//", &self.create_routes_declaration())
            .replace("//MODULE_IMPORTS//", &self.create_modules_declaration())
            .replace("//VERSION//", crate_version!())
            .replace("//MODE//", mode.as_str())
            .replace(
                "//APP_STATE_IMPORT//",
                if app.has_app_state {
                    r#"#[path="../src/app.rs"]
                        mod tuono_main_state;
                        use tuono_main_state::ApplicationState;
                        "#
                } else {
                    ""
                },
            )
            .replace(
                "//APP_STATE_DECLARATION//",
                if app.has_app_state {
                    "let user_custom_state = tuono_main_state::main();"
                } else {
                    ""
                },
            )
            .replace(
                "//APP_STATE_USAGE//",
                if app.has_app_state {
                    "|router, user_custom_state| async { router.with_state(user_custom_state) }"
                } else {
                    "|router| async { router }"
                },
            )
            .replace(
                "//GET_ROUTER_SIGNATURE//",
                if app.has_app_state {
                    "F: Fn(Router<ApplicationState>, ApplicationState) -> Fut,"
                } else {
                    "F: Fn(Router) -> Fut,"
                },
            );

        let main_src = if !app.has_main {
            Some(AXUM_ENTRY_POINT.replace("\r", ""))
        } else {
            None
        };

        let mut import_http_handler = String::new();

        let used_http_methods = app.get_used_http_methods();

        for method in used_http_methods.into_iter() {
            let method = method.to_string().to_lowercase();
            import_http_handler.push_str(&format!("use tuono_lib::axum::routing::{method};\n"))
        }

        (
            main_src,
            router_src.replace("//AXUM_GET_ROUTE_HANDLER//", &import_http_handler),
        )
    }

    pub fn refresh_axum_source(&self) -> io::Result<()> {
        let (main_source, router_source) = self.generate_axum_source();

        if let Some(main_source) = main_source {
            self.create_file(PathBuf::from(MAIN_FILE_PATH), &main_source)?;
        }
        self.create_file(PathBuf::from(ROUTER_FILE_PATH), &router_source)?;

        Ok(())
    }

    fn create_file(&self, path: PathBuf, content: &str) -> io::Result<()> {
        let mut data_file = fs::File::create(self.base_path.join(path))?;

        data_file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn refresh_typescript_file(&mut self, path: PathBuf) {
        self.types_jar.refresh_file(path);
    }

    pub fn remove_typescript_file(&mut self, path: PathBuf) {
        self.types_jar.remove_file(path);
    }

    pub fn generate_typescript_file(&mut self) -> io::Result<()> {
        self.types_jar.generate_typescript_file(&self.base_path)
    }

    fn create_routes_declaration(&self) -> String {
        let routes = &self.app.route_map;
        let mut route_declarations = String::from("// ROUTE_BUILDER\n");

        for (_, route) in routes.iter() {
            let Route { axum_info, .. } = &route;

            if axum_info.is_some() {
                let AxumInfo {
                    axum_route,
                    module_import,
                } = axum_info.as_ref().unwrap();

                if !route.is_api() {
                    route_declarations.push_str(&format!(
                        r#".route("{axum_route}", get({module_import}::tuono_internal_route))"#
                    ));

                    route_declarations.push_str(&format!(
                        r#".route("/__tuono/data{axum_route}", get({module_import}::tuono_internal_api))"#
                    ));
                } else {
                    for method in route.api_data.as_ref().unwrap().methods.clone() {
                        let method = method.to_string().to_lowercase();
                        route_declarations.push_str(&format!(
                            r#".route("{axum_route}", {method}({module_import}::{method}_tuono_internal_api))"#
                        ));
                    }
                }
            }
        }

        route_declarations
    }

    fn create_modules_declaration(&self) -> String {
        let routes = &self.app.route_map;
        let mut route_declarations = String::from("// MODULE_IMPORTS\n");

        for (path, route) in routes.iter() {
            if route.axum_info.is_some() {
                let AxumInfo { module_import, .. } = route.axum_info.as_ref().unwrap();

                route_declarations.push_str(&format!(
                    r#"#[path="../{ROUTE_FOLDER}{path}.rs"]
                    mod {module_import};
                    "#
                ))
            }
        }

        route_declarations
    }

    fn build_html_fallback(&self) -> String {
        if let Some(config) = &self.app.config.as_ref() {
            if let Some(origin) = &config.server.origin {
                FALLBACK_HTML.replace("[BASE_URL]", origin)
            } else {
                let url = format!("http://{}:{}", config.server.host, config.server.port);
                FALLBACK_HTML.replace("[BASE_URL]", url.as_str())
            }
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_the_correct_mode() {
        let (_, dev_bundle) = SourceBuilder {
            app: App::new(),
            mode: Mode::Dev,
            base_path: PathBuf::new(),
            types_jar: TypesJar::default(),
        }
        .generate_axum_source();

        let (_, prod_bundle) = SourceBuilder {
            app: App::new(),
            mode: Mode::Prod,
            base_path: PathBuf::new(),
            types_jar: TypesJar::default(),
        }
        .generate_axum_source();

        assert!(dev_bundle.contains("const MODE: Mode = Mode::Dev;"));
        assert!(prod_bundle.contains("const MODE: Mode = Mode::Prod;"));
    }

    #[test]
    fn should_not_load_the_axum_get_function() {
        let (_, dev_bundle) = SourceBuilder {
            app: App::new(),
            mode: Mode::Dev,
            base_path: PathBuf::new(),
            types_jar: TypesJar::default(),
        }
        .generate_axum_source();

        assert!(!dev_bundle.contains("use tuono_lib::axum::routing::get;"));
    }

    #[test]
    fn should_load_the_axum_get_function() {
        let mut source_builder = SourceBuilder {
            app: App::new(),
            mode: Mode::Dev,
            base_path: PathBuf::new(),
            types_jar: TypesJar::default(),
        };

        let mut route = Route::new(String::from("index.tsx"));
        route.update_axum_info();

        source_builder
            .app
            .route_map
            .insert(String::from("index.rs"), route);

        let (_, dev_bundle) = source_builder.generate_axum_source();

        assert!(dev_bundle.contains("use tuono_lib::axum::routing::get;"));
    }

    #[test]
    fn should_create_fallback_html_with_default_config() {
        let mut app = App::new();
        app.config = Some(Default::default());

        let source_builder = SourceBuilder {
            app,
            mode: Mode::Dev,
            base_path: PathBuf::new(),
            types_jar: TypesJar::default(),
        };

        let fallback_html = source_builder.build_html_fallback();

        assert!(fallback_html.contains("http://localhost:3000/vite-server/@react-refresh"));
        assert!(fallback_html.contains("http://localhost:3000/vite-server/@vite/client"));
        assert!(fallback_html.contains("http://localhost:3000/vite-server/client-main.tsx"));
    }
}
