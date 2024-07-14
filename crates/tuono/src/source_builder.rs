use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::app::App;
use crate::mode::Mode;
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

use tuono_lib::{tokio, Mode, Server, axum::Router, axum::routing::get};

const MODE: Mode = /*MODE*/;

// MODULE_IMPORTS

#[tokio::main]
async fn main() {
    let router = Router::new()
        // ROUTE_BUILDER
        ;

    Server::init(router, MODE).start().await
}
"##;

const ROOT_FOLDER: &str = "src/routes";
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
        let Route {
            has_server_handler,
            axum_route,
            module_import,
        } = &route;

        if *has_server_handler {
            route_declarations.push_str(&format!(
                r#".route("{axum_route}", get({module_import}::route))"#
            ));
            route_declarations.push_str(&format!(
                r#".route("/__tuono/data{axum_route}", get({module_import}::api))"#
            ));
        }
    }

    route_declarations
}

fn create_modules_declaration(routes: &HashMap<String, Route>) -> String {
    let mut route_declarations = String::from("// MODULE_IMPORTS\n");

    for (path, route) in routes.iter() {
        if route.has_server_handler {
            let module_name = &route.module_import;
            route_declarations.push_str(&format!(
                r#"#[path="../{ROOT_FOLDER}{path}.rs"]
                    mod {module_name};
                    "#
            ))
        }
    }

    route_declarations
}

pub fn bundle_axum_source(mode: Mode) -> io::Result<()> {
    let base_path = std::env::current_dir().unwrap();

    let mut source_builder = App::new();

    source_builder.collect_routes();

    dbg!(&source_builder);
    let bundled_file = generate_axum_source(&source_builder, mode);

    create_main_file(&base_path, &bundled_file);

    Ok(())
}

fn generate_axum_source(source_builder: &App, mode: Mode) -> String {
    AXUM_ENTRY_POINT
        .replace(
            "// ROUTE_BUILDER\n",
            &create_routes_declaration(&source_builder.route_map),
        )
        .replace(
            "// MODULE_IMPORTS\n",
            &create_modules_declaration(&source_builder.route_map),
        )
        .replace("/*MODE*/", mode.as_str())
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
        assert!(dev_bundle.contains("const MODE: Mode = Mode::Dev;"));

        let prod_bundle = generate_axum_source(&source_builder, Mode::Prod);

        assert!(prod_bundle.contains("const MODE: Mode = Mode::Prod;"));
    }
}
