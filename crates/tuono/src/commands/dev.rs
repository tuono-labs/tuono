use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::error;
use tuono_internal::tuono_println;
use watchexec_events::Tag;
use watchexec_events::filekind::FileEventKind;

use crate::process_manager::{ProcessId, ProcessManager};

use miette::{IntoDiagnostic, Result};
use watchexec::Watchexec;
use watchexec_signals::Signal;

use crate::source_builder::SourceBuilder;
use console::Term;
use spinners::{Spinner, Spinners};

fn ssr_reload_needed(path: &Path) -> bool {
    let file_name_starts_with_env = path
        .file_name()
        .map(|f| f.to_string_lossy().starts_with(".env"))
        .unwrap_or(false);

    let file_path = path.to_string_lossy();

    file_name_starts_with_env || file_path.ends_with("sx") || file_path.ends_with("mdx")
}

#[allow(
    clippy::await_holding_lock,
    reason = "At this point there is no other thread waiting for the lock"
)]
async unsafe fn start_all_processes(process_manager: Arc<Mutex<ProcessManager>>) {
    if let Ok(mut pm) = process_manager.lock() {
        pm.start_dev_processes().await
    }
}

fn detect_existing_env_files() -> Vec<String> {
    if let Ok(dir) = fs::read_dir("./") {
        dir.filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_string_lossy().starts_with(".env"))
            .map(|entry| entry.path().to_string_lossy().into_owned())
            .collect::<Vec<String>>()
    } else {
        Vec::new()
    }
}

#[tokio::main]
pub async fn watch(source_builder: SourceBuilder) -> Result<()> {
    let source_builder = RwLock::new(source_builder);
    let term = Term::stdout();
    let mut sp = Spinner::new(Spinners::Dots, "Starting dev server...".into());

    let process_manager = Arc::new(Mutex::new(ProcessManager::new()));

    let env_files = detect_existing_env_files();

    unsafe {
        // It is safe to call this function because here
        // only one thread is running and the lock is not
        // needed by any other thread.
        start_all_processes(process_manager.clone()).await;
    }

    // Remove the spinner
    sp.stop();
    _ = term.clear_line();

    // Server address log for production
    // is done on the server process.
    if let Ok(builder) = source_builder.read() {
        if let Ok(pm) = process_manager.lock() {
            pm.log_server_address(builder.app.config.clone().unwrap_or_default());
        }
    }

    let wx = Watchexec::new(move |mut action| {
        let process_manager = process_manager.clone();
        // if Ctrl-C is received, quit
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            if let Ok(mut pm) = process_manager.lock() {
                pm.abort_all();
            }

            action.quit_gracefully(Signal::Quit, Duration::from_secs(30));
            return action;
        }

        // The best way to trigger the processes is to use these
        // flags.
        // This because the same event can be triggered multiple times
        // and we don't want to restart the process multiple times for the same
        // purpose.
        let mut should_reload_ssr_bundle = false;
        let mut should_reload_rust_server = false;
        let mut should_refresh_axum_source = false;
        let mut paths_to_refresh_types: Vec<PathBuf> = vec![];
        let mut removed_files_from_types: Vec<PathBuf> = vec![];

        for event in action.events.iter() {
            for event_type in event.tags.iter() {
                if let Tag::FileEventKind(kind) = event_type {
                    match kind {
                        FileEventKind::Remove(_) => event.paths().for_each(|(path, _)| {
                            if path.extension().is_some_and(|ext| ext == "rs") {
                                removed_files_from_types.push(path.to_path_buf());
                                should_refresh_axum_source = true;
                            }
                        }),
                        FileEventKind::Modify(_) => event.paths().for_each(|(path, _)| {
                            if path.extension().is_some_and(|ext| ext == "rs") {
                                should_reload_rust_server = true;
                                paths_to_refresh_types.push(path.to_path_buf());
                            }
                            if ssr_reload_needed(path) {
                                should_reload_ssr_bundle = true;
                            }
                        }),
                        _ => {}
                    }
                }
            }
        }

        if !paths_to_refresh_types.is_empty() {
            if let Ok(mut builder) = source_builder.write() {
                for path in paths_to_refresh_types {
                    // There is no need to check here if the `Type` trait is
                    // derived since it will be checked later by the TypeJar struct.
                    builder.refresh_typescript_file(path)
                }
                if builder.generate_typescript_file().is_err() {
                    error!("Failed to generate typescript file");
                };
            }
        }

        if !removed_files_from_types.is_empty() {
            if let Ok(mut builder) = source_builder.write() {
                for path in removed_files_from_types {
                    builder.remove_typescript_file(path);
                }
                if builder.generate_typescript_file().is_err() {
                    error!("Failed to generate typescript file");
                };
            }
        }

        if should_reload_rust_server || should_refresh_axum_source {
            tuono_println!("Reloading...");
            if should_refresh_axum_source {
                if let Ok(mut builder) = source_builder.write() {
                    builder.app.collect_routes();
                    _ = builder.refresh_axum_source();
                }
            }
            if let Ok(mut pm) = process_manager.lock() {
                pm.restart_process(ProcessId::RunRustDevServer);
            }
        }

        if should_reload_ssr_bundle {
            if let Ok(mut pm) = process_manager.lock() {
                pm.restart_process(ProcessId::BuildReactSSRSrc);
            }
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
