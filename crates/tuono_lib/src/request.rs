use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use axum::http::{HeaderMap, Uri};

/// Location must match client side interface
#[derive(Serialize, Debug)]
pub struct Location {
    href: String,
    pathname: String,
    #[serde(rename(serialize = "searchStr"))]
    search_str: String,
    search: HashMap<String, String>,
}

impl Location {
    pub fn pathname(&self) -> &String {
        &self.pathname
    }
}

impl From<Uri> for Location {
    fn from(uri: Uri) -> Self {
        let query = uri.query().unwrap_or("");
        Location {
            // TODO: build correct href
            href: uri.to_string(),
            pathname: uri.path().to_string(),
            search_str: query.to_string(),
            search: serde_urlencoded::from_str(query).unwrap_or(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub uri: Uri,
    pub headers: HeaderMap,
    pub params: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(
        uri: Uri,
        headers: HeaderMap,
        params: HashMap<String, String>,
        body: Option<Vec<u8>>,
    ) -> Request {
        Request {
            uri,
            headers,
            params,
            body,
        }
    }

    pub fn location(&self) -> Location {
        Location::from(self.uri.to_owned())
    }

    pub fn body<'de, T: Deserialize<'de>>(&'de self) -> Result<T, BodyParseError> {
        if let Some(body) = &self.body {
            let body = serde_json::from_slice::<T>(body)?;
            return Ok(body);
        }

        Err(BodyParseError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to read body",
        )))
    }
}

#[derive(Debug)]
pub enum BodyParseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<serde_json::Error> for BodyParseError {
    fn from(err: serde_json::Error) -> BodyParseError {
        BodyParseError::Serde(err)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, Deserialize)]
    struct FakeBody {
        field1: bool,
        field2: String,
    }

    #[test]
    fn it_correctly_parse_the_body() {
        let request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
            Some(r#"{"field1": true, "field2": "hello"}"#.as_bytes().to_vec()),
        );

        let body: FakeBody = request.body().expect("Failed to parse body");

        assert!(body.field1);
        assert_eq!(body.field2, "hello".to_string());
    }

    #[test]
    fn it_should_trigger_an_error_when_body_is_invalid() {
        let request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
            Some(r#"{"field1": true"#.as_bytes().to_vec()),
        );

        let body: Result<FakeBody, BodyParseError> = request.body();

        assert!(body.is_err());
    }
}
