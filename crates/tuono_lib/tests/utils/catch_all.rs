use tuono_lib::Request;

#[tuono_lib::api(GET)]
async fn read_catch_all_parameter(req: Request) -> String {
    let param = req
        .params
        .get("catch_all")
        .expect("Failed to get the catch_all param");

    param.to_string()
}
