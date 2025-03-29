use crate::{Payload, ssr::Js};
use axum::extract::{Path, Request};
use axum::response::Html;
use std::collections::HashMap;

pub async fn catch_all(
    Path(params): Path<HashMap<String, String>>,
    request: Request,
) -> Html<String> {
    let pathname = request.uri();
    let headers = request.headers();

    let req = crate::Request::new(pathname.to_owned(), headers.to_owned(), params, None);

    // TODO: remove unwrap
    let payload = Payload::new(&req, &"").client_payload().unwrap();

    let result = Js::render_to_string(Some(&payload));

    match result {
        Ok(html) => Html(html),
        _ => Html("500 internal server error".to_string()),
    }
}
