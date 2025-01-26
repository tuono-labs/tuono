use dotenvy::from_filename;
use std::env;
use tuono::cli::app;

fn main() {
    // // Determine which .env file to load based on the ENVIRONMENT variable
    // let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "local".to_string());

    // let env_file = match environment.as_str() {
    //     "production" => ".env.production",
    //     "test" => ".env.test",
    //     _ => ".env.local",
    // };
  
    // // // Load the environment variables from the selected .env file
    dotenvy::dotenv().ok();

    app().expect("Failed to start the CLI")
}
