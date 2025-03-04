use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use clap::crate_version;

use crate::app::App;
use crate::mode::Mode;
use crate::route::AxumInfo;
use crate::route::Route;

pub const SERVER_ENTRY_DATA: &str = "// File automatically generated by tuono
// Do not manually update this file
import { routeTree } from './routeTree.gen'
import { serverSideRendering } from 'tuono/ssr'

export const renderFn = serverSideRendering(routeTree)
";

pub const CLIENT_ENTRY_DATA: &str = "// File automatically generated by tuono
// Do not manually update this file
import 'vite/modulepreload-polyfill'
import { hydrate } from 'tuono/hydration'
import '../src/styles/global.css'

// Import the generated route tree
import { routeTree } from './routeTree.gen'

hydrate(routeTree)
";

pub const AXUM_ENTRY_POINT: &str = r##"
// File automatically generated
// Do not manually change it

use tuono_lib::{tokio, Mode, Server, axum::Router, tuono_internal_init_v8_platform};
// AXUM_GET_ROUTE_HANDLER

const MODE: Mode = /*MODE*/;

// MODULE_IMPORTS

//MAIN_FILE_IMPORT//

#[tokio::main]
async fn main() {
    tuono_internal_init_v8_platform();
    println!("\n  ⚡ Tuono v/*VERSION*/");

    //MAIN_FILE_DEFINITION//

    let router = Router::new()
        // ROUTE_BUILDER
        //MAIN_FILE_USAGE//;

    Server::init(router, MODE).await.start().await
}
"##;

const ROUTE_FOLDER: &str = "src/routes";
const DEV_FOLDER: &str = ".tuono";

fn create_main_file(base_path: &Path, bundled_file: &String) {
    let mut data_file =
        fs::File::create(base_path.join(".tuono/main.rs")).expect("creation failed");

    data_file
        .write_all(bundled_file.as_bytes())
        .expect("write failed");
}

fn create_routes_declaration(routes: &HashMap<String, Route>) -> String {
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

fn create_modules_declaration(routes: &HashMap<String, Route>) -> String {
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

pub fn bundle_axum_source(mode: Mode) -> io::Result<App> {
    let base_path = std::env::current_dir()?;

    let app = App::new();
    let bundled_file = generate_axum_source(&app, mode);
    create_main_file(&base_path, &bundled_file);

    Ok(app)
}

fn generate_axum_source(app: &App, mode: Mode) -> String {
    let src = AXUM_ENTRY_POINT
        .replace(
            "// ROUTE_BUILDER\n",
            &create_routes_declaration(&app.route_map),
        )
        .replace(
            "// MODULE_IMPORTS\n",
            &create_modules_declaration(&app.route_map),
        )
        .replace("/*VERSION*/", crate_version!())
        .replace("/*MODE*/", mode.as_str())
        .replace(
            "//MAIN_FILE_IMPORT//",
            if app.has_app_state {
                r#"#[path="../src/app.rs"]
                    mod tuono_main_state;
                    "#
            } else {
                ""
            },
        )
        .replace(
            "//MAIN_FILE_DEFINITION//",
            if app.has_app_state {
                "let user_custom_state = tuono_main_state::main();"
            } else {
                ""
            },
        )
        .replace(
            "//MAIN_FILE_USAGE//",
            if app.has_app_state {
                ".with_state(user_custom_state)"
            } else {
                ""
            },
        );

    let mut import_http_handler = String::new();

    let used_http_methods = app.get_used_http_methods();

    for method in used_http_methods.into_iter() {
        let method = method.to_string().to_lowercase();
        import_http_handler.push_str(&format!("use tuono_lib::axum::routing::{method};\n"))
    }

    src.replace("// AXUM_GET_ROUTE_HANDLER", &import_http_handler)
}

pub fn check_tuono_folder() -> io::Result<()> {
    let dev_folder = Path::new(DEV_FOLDER);
    if !&dev_folder.is_dir() {
        fs::create_dir(dev_folder)?;
    }

    Ok(())
}

pub fn create_client_entry_files() -> io::Result<()> {
    let dev_folder = Path::new(DEV_FOLDER);

    let mut server_entry = fs::File::create(dev_folder.join("server-main.tsx"))?;
    let mut client_entry = fs::File::create(dev_folder.join("client-main.tsx"))?;

    server_entry.write_all(SERVER_ENTRY_DATA.as_bytes())?;
    client_entry.write_all(CLIENT_ENTRY_DATA.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_set_the_correct_mode() {
        let source_builder = App::new();

        let dev_bundle = generate_axum_source(&source_builder, Mode::Dev);
        let prod_bundle = generate_axum_source(&source_builder, Mode::Prod);

        assert!(dev_bundle.contains("const MODE: Mode = Mode::Dev;"));
        assert!(prod_bundle.contains("const MODE: Mode = Mode::Prod;"));
    }

    #[test]
    fn should_not_load_the_axum_get_function() {
        let source_builder = App::new();

        let dev_bundle = generate_axum_source(&source_builder, Mode::Dev);

        assert!(!dev_bundle.contains("use tuono_lib::axum::routing::get;"));
    }

    #[test]
    fn should_load_the_axum_get_function() {
        let mut source_builder = App::new();

        let mut route = Route::new(String::from("index.tsx"));
        route.update_axum_info();

        source_builder
            .route_map
            .insert(String::from("index.rs"), route);

        let dev_bundle = generate_axum_source(&source_builder, Mode::Dev);

        assert!(dev_bundle.contains("use tuono_lib::axum::routing::get;"));
    }
}
