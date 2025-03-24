use tracing::error;
use tracing_subscriber::EnvFilter;
use tuono::cli::app;

fn main() {
    tracing_subscriber::fmt()
        // Time not needed since the execution is synchronous
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    if let Err(e) = app() {
        // Generic error.
        // Recoverable errors should be managed locally
        error!("Failed to run the tuono CLI: {}", e);
        std::process::exit(1);
    }
}
