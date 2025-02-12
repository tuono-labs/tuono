use console::Term;
use fs_extra::dir::{copy, CopyOptions};
use spinners::{Spinner, Spinners};
use std::io;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use crate::app::App;
use crate::mode::Mode;

#[tokio::main]
pub async fn build(mut app: App, ssg: bool, no_js_emit: bool) -> Result<(), io::Error> {
    if no_js_emit {
        println!("Rust build successfully finished");
        return Ok(());
    }

    if ssg && app.has_dynamic_routes() {
        // TODO: allow dynamic routes static generation
        println!("Cannot statically build dynamic routes");
        return Ok(());
    }

    app.build_tuono_config()
        .expect("Failed to build tuono.config.ts");

    // start showing the spinner
    let term = Term::stdout();
    let mut sp = Spinner::new(Spinners::Dots, "Building app...".into());

    app.check_server_availability(Mode::Prod);

    app.build_react_prod();

    if ssg {
        println!("SSG: generation started");

        let static_dir = PathBuf::from("out/static");

        if static_dir.is_dir() {
            std::fs::remove_dir_all(&static_dir).expect("Failed to clear the out/static folder");
        }

        std::fs::create_dir(&static_dir).expect("Failed to create static output dir");

        copy(
            "./out/client",
            static_dir,
            &CopyOptions::new().overwrite(true).content_only(true),
        )
        .expect("Failed to clone assets into static output folder");

        // Start the server
        #[allow(clippy::zombie_processes)]
        let mut rust_server = app.run_rust_server();

        let reqwest_client = reqwest::blocking::Client::builder()
            .user_agent("")
            .build()
            .expect("Failed to build reqwest client");

        // Wait for server
        let mut is_server_ready = false;
        let config = app.config.as_ref().unwrap();

        while !is_server_ready {
            let server_url = format!("http://{}:{}", config.server.host, config.server.port);
            if reqwest_client.get(&server_url).send().is_ok() {
                is_server_ready = true;
            }
            sleep(Duration::from_secs(1));
        }

        for (_, route) in app.route_map {
            route.save_ssg_file(&reqwest_client);
        }

        // Close server
        let _ = rust_server.kill();
    }

    // Remove the spinner
    sp.stop();
    term.clear_line().unwrap();

    println!("Build successfully finished");
    Ok(())
}
