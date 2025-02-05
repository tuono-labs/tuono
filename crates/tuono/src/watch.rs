use miette::{IntoDiagnostic, Result};
use std::path::Path;
use std::sync::Arc;
use watchexec::Watchexec;
use watchexec_signals::Signal;
use watchexec_supervisor::command::{Command, Program};
use watchexec_supervisor::job::{start_job, Job};

use crate::mode::Mode;
use crate::source_builder::bundle_axum_source;

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

fn build_rust_src() -> Job {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: "cargo".into(),
            args: vec!["run".to_string(), "-q".to_string()],
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
    watch_react_src().start().await;

    let run_server = build_rust_src();

    let build_ssr_bundle = build_react_ssr_src();

    build_ssr_bundle.start().await;

    run_server.start().await;

    build_ssr_bundle.to_wait().await;

    let wx = Watchexec::new(move |mut action| {
        let mut should_reload_ssr_bundle = false;
        let mut should_reload_rust_server = false;

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
            }
        }

        if should_reload_rust_server {
            println!("  Reloading...");
            run_server.stop();
            bundle_axum_source(Mode::Dev).expect("Failed to bundle rust source");
            run_server.start();
        }

        if should_reload_ssr_bundle {
            build_ssr_bundle.stop();
            build_ssr_bundle.start();
        }

        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit_gracefully(Signal::Interrupt, std::time::Duration::from_secs(2000));
            eprintln!("Tuono gracefully shutting down...")
        }

        action
    })?;

    // watch the current directory
    wx.config.pathset(["./src"]);

    let _ = wx.main().await.into_diagnostic()?;

    Ok(())
}
