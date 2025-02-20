use fs_extra::dir::{copy, CopyOptions};
use spinners::{Spinner, Spinners};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use tracing::{error, trace};

use crate::app::App;
use crate::mode::Mode;

fn exit_gracefully_with_error(msg: &str) -> ! {
    error!(msg);
    std::process::exit(1);
}

pub fn build(mut app: App, ssg: bool, no_js_emit: bool) {
    if no_js_emit {
        println!("Rust build successfully finished");
        std::process::exit(0);
    }

    if ssg && app.has_dynamic_routes() {
        // TODO: allow dynamic routes static generation
        println!("Cannot statically build dynamic routes");
        std::process::exit(1);
    }

    app.build_tuono_config()
        .unwrap_or_else(|_| exit_gracefully_with_error("Failed to build tuono.config.ts"));

    let mut app_build_spinner = Spinner::new(Spinners::Dots, "Building app...".into());

    app.check_server_availability(Mode::Prod);

    app.env_var_manager.refresh_env_files();

    app.build_react_prod();

    // Remove the spinner
    app_build_spinner.stop_with_message("\u{2705}Build completed".into());

    if ssg {
        let mut app_build_static_spinner =
            Spinner::new(Spinners::Dots, "Static site generation".into());

        let mut exit_gracefully_with_error = |msg: &str| -> ! {
            app_build_static_spinner.stop_with_message("\u{274C} Build failed\n".into());
            exit_gracefully_with_error(msg)
        };

        let static_dir = PathBuf::from("out/static");

        if static_dir.is_dir() {
            std::fs::remove_dir_all(&static_dir).unwrap_or_else(|_| {
                exit_gracefully_with_error("Failed to clear the out/static folder")
            });
        }

        std::fs::create_dir(&static_dir)
            .unwrap_or_else(|_| exit_gracefully_with_error("Failed to create static output dir"));

        copy(
            "./out/client",
            static_dir,
            &CopyOptions::new().overwrite(true).content_only(true),
        )
        .unwrap_or_else(|_| {
            exit_gracefully_with_error("Failed to clone assets into static output folder")
        });

        // Start the server
        #[allow(clippy::zombie_processes)]
        let mut rust_server = app.run_rust_server();

        let mut exit_and_shut_server = |msg: &str| -> ! {
            _ = rust_server.kill();
            exit_gracefully_with_error(msg)
        };

        let reqwest_client = reqwest::blocking::Client::builder()
            .user_agent("")
            .build()
            .unwrap_or_else(|_| exit_and_shut_server("Failed to build reqwest client"));

        // Wait for server
        let mut is_server_ready = false;
        let config = app.config.as_ref().unwrap();

        while !is_server_ready {
            trace!("Checking server availability");
            let server_url = format!("http://{}:{}", config.server.host, config.server.port);
            if reqwest_client.get(&server_url).send().is_ok() {
                is_server_ready = true;
            } else {
                trace!("Server not ready yet. Sleeping for 1 second");
            }
            sleep(Duration::from_secs(1));
        }

        trace!("Server is ready, starting static site generation");

        for (_, route) in app.route_map {
            if let Err(msg) = route.save_ssg_file(&reqwest_client) {
                exit_and_shut_server(&msg);
            }
        }

        // Close server
        let _ = rust_server.kill();

        app_build_static_spinner
            .stop_with_message("\u{2705}Static site generation completed".into());
    }
}
