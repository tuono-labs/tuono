use axum::extract::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Payload {
    data: String,
}

#[tuono_lib::api(POST)]
async fn form_data(Form(form_data): Form<Payload>) -> String {
    form_data.data
}
