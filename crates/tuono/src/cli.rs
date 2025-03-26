use clap::{Parser, Subcommand};
use tracing::{span, Level};

use crate::commands::{build, dev, new};
use crate::mode::Mode;
use crate::source_builder::SourceBuilder;

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

pub fn app() -> std::io::Result<()> {
    let args = Args::parse();

    match args.action {
        Actions::Dev => {
            let span = span!(Level::TRACE, "DEV");

            let _guard = span.enter();

            let mut source_builder = SourceBuilder::new(Mode::Dev)?;

            source_builder.base_build()?;

            source_builder.app.check_server_availability(Mode::Dev);

            dev::watch(source_builder).unwrap();
        }
        Actions::Build { ssg, no_js_emit } => {
            let span = span!(Level::TRACE, "BUILD");

            let _guard = span.enter();

            let mut source_builder = SourceBuilder::new(Mode::Prod)?;
            source_builder.base_build()?;
            build::build(source_builder.app, ssg, no_js_emit);
        }
        Actions::New {
            folder_name,
            template,
            head,
        } => {
            let span = span!(Level::TRACE, "NEW");

            let _guard = span.enter();

            new::create_new_project(folder_name, template, head);
        }
    }

    Ok(())
}
