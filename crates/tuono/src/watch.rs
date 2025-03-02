use std::fs;
use std::path::Path;
use std::sync::Arc;
use watchexec_supervisor::command::{Command, Program};

use miette::{IntoDiagnostic, Result};
use watchexec::Watchexec;
use watchexec_signals::Signal;
use watchexec_supervisor::job::{start_job, Job};

use crate::mode::Mode;
use crate::source_builder::bundle_axum_source;
use console::Term;
use spinners::{Spinner, Spinners};

#[cfg(target_os = "windows")]
const DEV_WATCH_BIN_SRC: &str = "node_modules\\.bin\\tuono-dev-watch.cmd";
#[cfg(target_os = "windows")]
const DEV_SSR_BIN_SRC: &str = "node_modules\\.bin\\tuono-dev-ssr.cmd";

#[cfg(not(target_os = "windows"))]
const DEV_WATCH_BIN_SRC: &str = "node_modules/.bin/tuono-dev-watch";
#[cfg(not(target_os = "windows"))]
const DEV_SSR_BIN_SRC: &str = "node_modules/.bin/tuono-dev-ssr";

fn watch_react_src() -> Job {
    if !Path::new(DEV_SSR_BIN_SRC).exists() {
        eprintln!("Failed to find script to run dev watch. Please run `npm install`");
        std::process::exit(1);
    }
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: DEV_WATCH_BIN_SRC.into(),
            args: vec![],
        },
        options: Default::default(),
    }))
    .0
}

fn run_rust_dev_server() -> Job {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: "cargo".into(),
            args: vec!["run".to_string(), "-q".to_string()],
        },
        options: Default::default(),
    }))
    .0
}

fn build_rust_src() -> Job {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: "cargo".into(),
            args: vec!["build".to_string(), "-q".to_string()],
        },
        options: Default::default(),
    }))
    .0
}

fn build_react_ssr_src() -> Job {
    if !Path::new(DEV_SSR_BIN_SRC).exists() {
        eprintln!("Failed to find script to run dev ssr. Please run `npm install`");
        std::process::exit(1);
    }
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: DEV_SSR_BIN_SRC.into(),
            args: vec![],
        },
        options: Default::default(),
    }))
    .0
}

#[tokio::main]
pub async fn watch() -> Result<()> {
    let term = Term::stdout();
    let mut sp = Spinner::new(Spinners::Dots, "Starting dev server...".into());

    watch_react_src().start().await;

    let rust_server = run_rust_dev_server();
    let build_rust_src = build_rust_src();

    let build_ssr_bundle = build_react_ssr_src();

    let env_files = fs::read_dir("./")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().starts_with(".env"))
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect::<Vec<String>>();

    build_ssr_bundle.start().await;
    build_rust_src.start().await;

    // Wait vite and rust builds
    build_ssr_bundle.to_wait().await;
    build_rust_src.to_wait().await;

    rust_server.start().await;

    // Remove the spinner
    sp.stop();
    term.clear_line().unwrap();

    let wx = Watchexec::new(move |mut action| {
        let mut should_reload_ssr_bundle = false;
        let mut should_reload_rust_server = false;
        let mut should_reload_env_file = false;

        for event in action.events.iter() {
            for path in event.paths() {
                let file_path = path.0.to_string_lossy();
                if file_path.ends_with(".rs") {
                    should_reload_rust_server = true
                }

                // Either tsx, jsx and mdx
                if file_path.ends_with("sx") || file_path.ends_with("mdx") {
                    should_reload_ssr_bundle = true
                }

                if path
                    .0
                    .file_name()
                    .map(|f| f.to_string_lossy().starts_with(".env"))
                    .unwrap_or(false)
                {
                    should_reload_env_file = true;
                    should_reload_ssr_bundle = true
                }
            }
        }

        if should_reload_rust_server {
            println!("  Reloading...");
            rust_server.stop();
            bundle_axum_source(Mode::Dev).expect("Failed to bundle rust source");
            rust_server.start();
        }

        if should_reload_ssr_bundle {
            build_ssr_bundle.stop();
            build_ssr_bundle.start();
        }

        if should_reload_env_file {
            println!("  Reloading environment variables, and restarting rust server...");
            rust_server.stop();
            bundle_axum_source(Mode::Dev).expect("Failed to bundle rust source");
            rust_server.start();
        }

        // if Ctrl-C is received, quit
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit();
        }

        action
    })?;

    // watch the current directory and all types of .env file
    let mut paths_to_watch = vec!["./src".to_string()];
    paths_to_watch.extend(env_files);

    wx.config.pathset(paths_to_watch);

    let _ = wx.main().await.into_diagnostic()?;
    Ok(())
}
