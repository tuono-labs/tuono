use crate::{Payload, ssr::Js};
use axum::extract::Request;
use axum::response::Html;

pub async fn catch_all(request: Request) -> Html<String> {
    // TODO: remove unwrap
    let payload = Payload::new(&request, &"").client_payload().unwrap();

    let result = Js::render_to_string(Some(&payload));

    match result {
        Ok(html) => Html(html),
        _ => Html("500 internal server error".to_string()),
    }
}
