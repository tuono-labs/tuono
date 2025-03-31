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
        Err((status_code, err)) => {
            return format!(
                "Error: {}. Status code: {}",
                err.to_string(),
                status_code
            );
        },
    };
    form.data
}
