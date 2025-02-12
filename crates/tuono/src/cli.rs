use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::app::App;
use crate::build;
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
            let app = init_tuono_folder(Mode::Prod)?;

            let _ = build::build(app, ssg, no_js_emit);
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
