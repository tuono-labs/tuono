use axum::extract::Path;
use std::collections::HashMap;

#[tuono_lib::api(GET)]
async fn read_catch_all_parameter(params: Path<HashMap<String, String>>) -> String {
    let param = params
        .get("catch_all")
        .expect("Failed to get the catch_all param");

    param.to_string()
}
