use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::task::JoinHandle;
use watchexec_events::Tag;
use watchexec_events::filekind::FileEventKind;
use watchexec_supervisor::command::{Command, Program};

use miette::{IntoDiagnostic, Result};
use watchexec::Watchexec;
use watchexec_signals::Signal;
use watchexec_supervisor::job::{Job, start_job};

use crate::source_builder::SourceBuilder;
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

fn watch_react_src() -> (Job, JoinHandle<()>) {
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
}

fn run_rust_dev_server() -> (Job, JoinHandle<()>) {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: "cargo".into(),
            args: vec!["run".to_string(), "-q".to_string()],
        },
        options: Default::default(),
    }))
}

fn build_rust_src() -> (Job, JoinHandle<()>) {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: "cargo".into(),
            args: vec!["build".to_string(), "-q".to_string()],
        },
        options: Default::default(),
    }))
}

fn build_react_ssr_src() -> (Job, JoinHandle<()>) {
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
}

fn ssr_reload_needed(path: &Path) -> bool {
    let file_name_starts_with_env = path
        .file_name()
        .map(|f| f.to_string_lossy().starts_with(".env"))
        .unwrap_or(false);

    let file_path = path.to_string_lossy();

    file_name_starts_with_env || file_path.ends_with("sx") || file_path.ends_with("mdx")
}

#[tokio::main]
pub async fn watch(source_builder: SourceBuilder) -> Result<()> {
    let source_builder = RwLock::new(source_builder);
    let term = Term::stdout();
    let mut sp = Spinner::new(Spinners::Dots, "Starting dev server...".into());

    let (watch_react, watch_react_handle) = watch_react_src();

    let (rust_server, rust_server_handle) = run_rust_dev_server();
    let (build_rust_src, build_rust_src_hanlde) = build_rust_src();

    let (build_ssr_bundle, build_ssr_bundle_hanlde) = build_react_ssr_src();

    let env_files = fs::read_dir("./")
        .expect("Error reading env files from current directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().starts_with(".env"))
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect::<Vec<String>>();

    build_ssr_bundle.start().await;
    build_rust_src.start().await;
    watch_react.start().await;

    // Wait vite and rust builds
    build_ssr_bundle.to_wait().await;
    build_rust_src.to_wait().await;
    rust_server.start().await;

    // Remove the spinner
    sp.stop();
    _ = term.clear_line();

    let wx = Watchexec::new(move |mut action| {
        // if Ctrl-C is received, quit
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            rust_server.stop();
            build_ssr_bundle.stop();
            watch_react.stop();
            watch_react_handle.abort();
            rust_server_handle.abort();
            build_rust_src_hanlde.abort();
            build_ssr_bundle_hanlde.abort();

            action.quit_gracefully(Signal::Quit, Duration::from_secs(30));
            return action;
        }

        let mut should_reload_ssr_bundle = false;
        let mut should_reload_rust_server = false;
        let mut should_refresh_axum_source = false;

        for event in action.events.iter() {
            for event_type in event.tags.iter() {
                match event_type {
                    Tag::FileEventKind(kind) => match kind {
                        FileEventKind::Create(_) => {
                            if event.paths().any(|(path, _)| {
                                path.ends_with(".rs") || 
                            // APIs might define new HTTP methods that requires
                            // a refresh of the axum source
                            path.to_str().unwrap_or("").contains("api")
                            }) {
                                should_refresh_axum_source = true;
                            }
                        }
                        FileEventKind::Modify(_) => {
                            if event.paths().any(|(path, _)| path.ends_with(".rs")) {
                                should_reload_rust_server = true;
                            }

                            if event.paths().any(|(path, _)| ssr_reload_needed(path)) {
                                should_reload_ssr_bundle = true;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        if should_refresh_axum_source {
            rust_server.stop();
            if let Ok(mut builder) = source_builder.write() {
                builder.app.collect_routes();
                _ = builder.refresh_axum_source();
            }
            rust_server.start();
        }

        if should_reload_rust_server {
            println!("  Reloading...");
            rust_server.stop();
            rust_server.start();
        }

        if should_reload_ssr_bundle {
            build_ssr_bundle.stop();
            build_ssr_bundle.start();
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
