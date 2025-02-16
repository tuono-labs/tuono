use reqwest::Client;

#[derive(Clone)]
pub struct ApplicationState {
    pub fetch: Client,
}

pub fn main() -> ApplicationState {
    let fetch = Client::new();
    return ApplicationState { fetch };
}
