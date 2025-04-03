use serde::Deserialize;
use tuono_lib::Request;

#[derive(Deserialize)]
struct Payload {
    data: String,
}

#[tuono_lib::api(POST)]
async fn form_data(req: Request) -> String {
    let form = req.form_data::<Payload>().unwrap();
    form.data
}
