use axum::extract::Path;
use std::collections::HashMap;

#[tuono_lib::api(GET)]
async fn read_dynamic_parameter(params: Path<HashMap<String, String>>) -> String {
    let param = params
        .get("parameter")
        .expect("Failed to get the catch_all param");

    param.to_string()
}
