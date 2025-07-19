use crate::config::GLOBAL_CONFIG;
use crate::manifest::MANIFEST;
use crate::mode::{GLOBAL_MODE, Mode};
use erased_serde::Serialize;
use serde::Serialize as SerdeSerialize;
use tuono_internal::config::ServerConfig;

use crate::request::{Location, Request};

#[derive(SerdeSerialize)]
/// This is the payload sent to the client for hydration
pub struct Payload<'a> {
    location: Location,
    data: &'a dyn Serialize,
    mode: Mode,
    #[serde(rename(serialize = "jsBundles"))]
    js_bundles: Option<Vec<String>>,
    #[serde(rename(serialize = "cssBundles"))]
    css_bundles: Option<Vec<String>>,
    #[serde(rename(serialize = "devServerConfig"))]
    dev_server_config: Option<&'a ServerConfig>,
}

impl<'a> Payload<'a> {
    pub fn new(req: &'a Request, data: &'a dyn Serialize) -> Payload<'a> {
        let config = GLOBAL_CONFIG
            .get()
            .expect("Failed to load the current config");

        let mode = *GLOBAL_MODE.get().expect("Failed to load the current mode");

        let dev_server_config = if mode == Mode::Dev {
            Some(&config.server)
        } else {
            None
        };

        Payload {
            location: req.location(),
            data,
            mode,
            js_bundles: None,
            css_bundles: None,
            dev_server_config,
        }
    }

    pub fn client_payload(&mut self) -> Result<String, serde_json::Error> {
        if self.mode == Mode::Prod {
            self.add_bundle_sources();
        }
        serde_json::to_string(&self)
    }

    fn add_bundle_sources(&mut self) {
        let manifest = MANIFEST.get().expect("Manifest not loaded");
        let bundles = manifest.get_bundle_from_pathname(self.location.pathname());
        self.js_bundles = Some(bundles.js_files);
        self.css_bundles = Some(bundles.css_files);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::manifest::ViteManifest;
    use axum::http::Uri;

    const MANIFEST_EXAMPLE: &str = r#"{
        "../src/routes/index.tsx": {
            "file": "assets/index-D-yFyCZo.js",
            "name": "index",
            "src": "../src/routes/index.tsx",
            "isDynamicEntry": true,
            "imports": [
                "client-main.tsx"
            ],
            "css": [
                "assets/index-CynfArjF.css"
            ]
        },
        "../src/routes/pokemons/[pokemon].tsx": {
            "file": "assets/_pokemon_-DlFInatQ.js",
            "name": "_pokemon_",
            "src": "../src/routes/pokemons/[pokemon].tsx",
            "isDynamicEntry": true,
            "imports": [
                "client-main.tsx"
            ],
            "css": [
                "assets/_pokemon_-BcJZaQaO.css"
            ]
        },
        "../src/routes/pokemons/__layout.tsx": {
            "file": "assets/__layout-BFnT3M7X.js",
            "name": "__layout",
            "src": "../src/routes/pokemons/__layout.tsx",
            "isDynamicEntry": true,
            "imports": [
                "client-main.tsx"
            ],
            "css": [
                "assets/__layout-CXGGqNw5.css"
            ]
        },
        "client-main.tsx": {
            "file": "assets/client-main-B9g1NVV7.js",
            "name": "client-main",
            "src": "client-main.tsx",
            "isEntry": true,
            "dynamicImports": [
                "../src/routes/pokemons/__layout.tsx",
                "../src/routes/index.tsx",
                "../src/routes/pokemons/[pokemon].tsx"
            ],  
            "css": [
                "assets/client-main-BS7N-NIa.css"
            ]
        } 
    }"#;

    fn prepare_payload(uri: Option<&str>, mode: Mode) -> Payload<'_> {
        let manifest_mock = serde_json::from_str::<ViteManifest>(MANIFEST_EXAMPLE)
            .expect("Failed to parse the manifest example");
        MANIFEST.get_or_init(|| manifest_mock.into());

        let uri = uri
            .unwrap_or("http://localhost:3000/")
            .parse::<Uri>()
            .unwrap();

        let location = Location::from(uri);

        Payload {
            location,
            data: &None::<Option<()>>,
            mode,
            js_bundles: None,
            css_bundles: None,
            dev_server_config: None,
        }
    }

    #[test]
    fn should_load_the_bundles_on_mode_prod() {
        let mut payload = prepare_payload(None, Mode::Prod);

        let _ = payload.client_payload();
        assert_eq!(
            payload.js_bundles,
            Some(vec![
                "assets/index-D-yFyCZo.js".to_string(),
                "assets/client-main-B9g1NVV7.js".to_string()
            ])
        );
        assert_eq!(
            payload.css_bundles,
            Some(vec![
                "assets/index-CynfArjF.css".to_string(),
                "assets/client-main-BS7N-NIa.css".to_string()
            ])
        );
    }

    #[test]
    fn should_not_load_the_bundles_on_mode_dev() {
        let mut payload = prepare_payload(None, Mode::Dev);
        let _ = payload.client_payload();
        assert!(payload.js_bundles.is_none());
        assert!(payload.css_bundles.is_none());
    }
}
