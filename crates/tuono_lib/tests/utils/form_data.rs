use serde::Deserialize;
use tuono_lib::Request;

#[derive(Deserialize)]
struct Payload {
    data: String,
}

#[tuono_lib::api(POST)]
async fn form_data(req: Request) -> String {
    let form = match req.form_data::<Payload>() {
        Ok(data) => data,
        Err(e) => {
            return format!("Error: {}", e.to_string());
        }
    };
    form.data
}
