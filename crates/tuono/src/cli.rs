use fs_extra::dir::{copy, CopyOptions};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::app::App;
use crate::mode::Mode;
use crate::scaffold_project;
use crate::source_builder::{bundle_axum_source, check_tuono_folder, create_client_entry_files};
use crate::watch;

#[derive(Subcommand, Debug)]
enum Actions {
    /// Start the development environment
    Dev,
    /// Build the production assets
    Build {
        #[arg(short, long = "static")]
        /// Statically generate the website HTML
        ssg: bool,

        #[arg(short, long)]
        /// Prevent to export the js assets
        no_js_emit: bool,
    },
    /// Scaffold a new project
    New {
        /// The folder in which load the project. Default is the current directory.
        folder_name: Option<String>,
        /// The template to use to scaffold the project. The template should match one of the tuono
        /// examples
        #[arg(short, long)]
        template: Option<String>,
        /// Load the latest commit available on the main branch
        #[arg(long)]
        head: Option<bool>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about = "The React/Rust full-stack framework")]
struct Args {
    #[command(subcommand)]
    action: Actions,
}

fn check_for_tuono_config() {
    if !PathBuf::from("tuono.config.ts").exists() {
        eprintln!("Cannot find tuono.config.ts - is this a tuono project?");
        std::process::exit(1);
    }
}

fn init_tuono_folder(mode: Mode) -> std::io::Result<App> {
    check_for_tuono_config();
    check_tuono_folder()?;
    let app = bundle_axum_source(mode)?;
    create_client_entry_files()?;

    Ok(app)
}

pub fn app() -> std::io::Result<()> {
    let args = Args::parse();

    match args.action {
        Actions::Dev => {
            let mut app = init_tuono_folder(Mode::Dev)?;

            app.build_tuono_config()
                .expect("Failed to build tuono.config.ts");

            app.check_server_availability(Mode::Dev);

            watch::watch().unwrap();
        }
        Actions::Build { ssg, no_js_emit } => {
            let mut app = init_tuono_folder(Mode::Prod)?;

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

            app.check_server_availability(Mode::Prod);

            app.build_react_prod();

            if ssg {
                println!("SSG: generation started");

                let static_dir = PathBuf::from("out/static");

                if static_dir.is_dir() {
                    std::fs::remove_dir_all(&static_dir)
                        .expect("Failed to clear the out/static folder");
                }

                std::fs::create_dir(&static_dir).expect("Failed to create static output dir");

                copy(
                    "./out/client",
                    static_dir,
                    &CopyOptions::new().overwrite(true).content_only(true),
                )
                .expect("Failed to clone assets into static output folder");

                // the process is killed below so we don't really need to use wait() function
                #[allow(clippy::zombie_processes)]
                let mut rust_server = app.run_rust_server();

                let reqwest = reqwest::blocking::Client::builder()
                    .user_agent("")
                    .build()
                    .expect("Failed to build reqwest client");

                // Wait for server
                let mut is_server_ready = false;

                let config = app.config.as_ref().unwrap();

                while !is_server_ready {
                    let server_url =
                        format!("http://{}:{}", config.server.host, config.server.port);
                    if reqwest.get(server_url).send().is_ok() {
                        is_server_ready = true
                    }
                    // TODO: add maximum tries
                    sleep(Duration::from_secs(1))
                }

                for (_, route) in app.route_map {
                    route.save_ssg_file(&reqwest)
                }

                // Close server
                let _ = rust_server.kill();
            };

            println!("Build successfully finished");
        }
        Actions::New {
            folder_name,
            template,
            head,
        } => {
            scaffold_project::create_new_project(folder_name, template, head);
        }
    }

    Ok(())
}
