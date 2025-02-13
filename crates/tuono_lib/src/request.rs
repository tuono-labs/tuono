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
}

impl Request {
    pub fn new(uri: Uri, headers: HeaderMap, params: HashMap<String, String>) -> Request {
        Request {
            uri,
            headers,
            params,
        }
    }

    pub fn location(&self) -> Location {
        Location::from(self.uri.to_owned())
    }

    pub fn body<'de, T: Deserialize<'de>>(&'de self) -> Result<T, BodyParseError> {
        if let Some(body) = self.headers.get("body") {
            if let Ok(body) = body.to_str() {
                let body = serde_json::from_str::<T>(body)?;
                Ok(body)
            } else {
                Err(BodyParseError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to read body",
                )))
            }
        } else {
            Err(BodyParseError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No body found",
            )))
        }
    }
}

pub enum BodyParseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<serde_json::Error> for BodyParseError {
    fn from(err: serde_json::Error) -> BodyParseError {
        BodyParseError::Serde(err)
    }
}
