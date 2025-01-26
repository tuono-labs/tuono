use dotenvy::from_filename;
use std::env;
use tuono::cli::app;

fn main() {
    dotenvy::dotenv().ok();

    app().expect("Failed to start the CLI")
}
