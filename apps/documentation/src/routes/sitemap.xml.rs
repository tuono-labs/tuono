use glob::glob;
use time::OffsetDateTime;
use tuono_lib::axum::http::{header, HeaderMap, StatusCode};
use tuono_lib::{Request, Response};

const FILES_TO_EXCLUDE: [&str; 3] = ["sitemap.xml", "__layout", "404"];

const SITEMAP: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    [PLACEHOLDER]
</urlset>"#;

fn load_routes() -> Vec<String> {
    let mut paths: Vec<String> = vec![];

    for entry in glob("./src/routes/**/*")
        .expect("Failed to glob src/routes folder")
        .flatten()
    {
        if !entry.is_dir() {
            let path = clean_path(format!("/{}", entry.to_string_lossy()));

            if !FILES_TO_EXCLUDE
                .iter()
                .any(|exclude| path.ends_with(exclude))
            {
                paths.push(path)
            }
        }
    }
    paths
}

fn clean_path(value: String) -> String {
    value
        .replace("src/routes/", "")
        .replace(".mdx", "")
        .replace(".tsx", "")
        .replace(".rs", "")
        .replace("index", "")
}

#[tuono_lib::handler]
async fn generate_sitemap(_req: Request) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/xml".parse().unwrap());

    let routes = load_routes();

    let mut sitemaps = String::new();

    for path in routes {
        let mut url = format!("https://tuono.dev{}", path);

        if url.ends_with('/') {
            url.pop();
        }

        sitemaps.push_str(&format!(
            r#"<url><loc>{}</loc><lastmod>{}</lastmod></url>"#,
            url,
            OffsetDateTime::now_utc().date()
        ))
    }

    Response::Custom((
        StatusCode::OK,
        headers,
        SITEMAP.replace("[PLACEHOLDER]", &sitemaps),
    ))
}
