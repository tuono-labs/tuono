use crate::config::GLOBAL_CONFIG;
use axum::body::Body;
use axum::extract::{Path, Query};
use std::collections::HashMap;

use axum::http::{HeaderName, HeaderValue};
use axum::response::{IntoResponse, Response};
use reqwest::Client;

pub async fn vite_reverse_proxy(
    Path(path): Path<String>,
    query: Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let client = Client::new();

    let config = GLOBAL_CONFIG
        .get()
        .expect("Failed to get the internal config");

    let vite_url = format!(
        "http://{}:{}/vite-server",
        config.server.host,
        config.server.port + 1
    );

    let query_string = query
        .0
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let query_string = if query_string.is_empty() {
        String::new()
    } else {
        format!("?{query_string}")
    };

    match client
        .get(format!("{vite_url}/{path}{query_string}"))
        .send()
        .await
    {
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
