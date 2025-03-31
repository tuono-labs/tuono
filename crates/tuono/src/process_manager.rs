use std::collections::HashMap;

use std::path::Path;
use std::sync::Arc;
use tokio::task::JoinHandle;

use tracing::trace;
use watchexec_supervisor::command::{Command, Program};
use watchexec_supervisor::job::{Job, start_job};

#[cfg(target_os = "windows")]
const DEV_WATCH_BIN_SRC: &str = "node_modules\\.bin\\tuono-dev-watch.cmd";
#[cfg(target_os = "windows")]
const DEV_SSR_BIN_SRC: &str = "node_modules\\.bin\\tuono-dev-ssr.cmd";

#[cfg(not(target_os = "windows"))]
const DEV_WATCH_BIN_SRC: &str = "node_modules/.bin/tuono-dev-watch";
#[cfg(not(target_os = "windows"))]
const DEV_SSR_BIN_SRC: &str = "node_modules/.bin/tuono-dev-ssr";

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ProcessId {
    WatchReactSrc,
    RunRustDevServer,
    BuildRustSrc,
    BuildReactSSRSrc,
}

// This struct manages all the processes spawned by the dev server
// That are not the dev server itself
#[derive(Debug)]
pub struct ProcessManager {
    processes: HashMap<ProcessId, (Job, JoinHandle<()>)>,
}

impl ProcessManager {
    pub fn new() -> Self {
        trace!("Creating process manager");
        if !Path::new(DEV_SSR_BIN_SRC).exists() {
            eprintln!("Failed to find script to run dev watch. Please run `npm install`");
            std::process::exit(1);
        }
        let mut processes = HashMap::new();
        processes.insert(
            ProcessId::WatchReactSrc,
            start_supervisor_job(DEV_WATCH_BIN_SRC, vec![]),
        );

        processes.insert(
            ProcessId::RunRustDevServer,
            start_supervisor_job("cargo", vec!["run", "-q"]),
        );

        processes.insert(
            ProcessId::BuildRustSrc,
            start_supervisor_job("cargo", vec!["build", "-q"]),
        );

        processes.insert(
            ProcessId::BuildReactSSRSrc,
            start_supervisor_job(DEV_SSR_BIN_SRC, vec![]),
        );

        Self { processes }
    }

    pub fn start_process(&mut self, id: ProcessId) {
        trace!("start process {:?}", id);
        if let Some((job, _)) = self.processes.get(&id) {
            job.start();
        }
    }

    pub async fn start_dev_processes(&mut self) {
        trace!("Starting dev processes");
        self.start_process(ProcessId::WatchReactSrc);
        self.start_process(ProcessId::BuildRustSrc);
        self.start_process(ProcessId::BuildReactSSRSrc);

        self.wait_for_process(ProcessId::BuildRustSrc).await;
        self.wait_for_process(ProcessId::BuildReactSSRSrc).await;

        // This needs to start after having built the rust and react sources
        self.start_process(ProcessId::RunRustDevServer);
    }

    pub fn restart_process(&mut self, id: ProcessId) {
        trace!("Restarting process {:?}", id);
        if let Some((job, _)) = self.processes.get(&id) {
            job.stop();
            job.start();
        }
    }

    pub async fn wait_for_process(&mut self, id: ProcessId) {
        trace!("Waiting for process {:?}", id);
        if let Some((job, _)) = self.processes.get(&id) {
            job.to_wait().await;
        }
    }

    pub fn abort_all(&mut self) {
        trace!("Aborting all processes");
        for (_, (job, handle)) in self.processes.iter() {
            job.stop();
            handle.abort();
        }
    }
}

fn start_supervisor_job(prog: &str, args: Vec<&str>) -> (Job, JoinHandle<()>) {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: prog.into(),
            args: args.into_iter().map(|arg| arg.to_string()).collect(),
        },
        options: Default::default(),
    }))
}
