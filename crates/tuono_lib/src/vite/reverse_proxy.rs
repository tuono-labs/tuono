use crate::config::GLOBAL_CONFIG;
use axum::body::Body;
use axum::extract::Path;

use axum::http::{HeaderName, HeaderValue};
use axum::response::{IntoResponse, Response};
use reqwest::Client;

pub async fn vite_reverse_proxy(Path(path): Path<String>) -> impl IntoResponse {
    let client = Client::new();

    let config = GLOBAL_CONFIG
        .get()
        .expect("Failed to get the internal config");

    let vite_url = format!(
        "http://{}:{}/vite-server",
        config.server.host,
        config.server.port + 1
    );

    match client.get(format!("{vite_url}/{path}")).send().await {
        Ok(res) => {
            let mut response_builder = Response::builder().status(res.status().as_u16());

            {
                let headers = response_builder.headers_mut().unwrap();
                res.headers().into_iter().for_each(|(name, value)| {
                    let name = HeaderName::from_bytes(name.as_ref()).unwrap();
                    let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
                    headers.insert(name, value);
                });
            }

            response_builder
                .body(Body::from_stream(res.bytes_stream()))
                .unwrap()
        }
        Err(_) => todo!(),
    }
}
