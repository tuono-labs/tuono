use crate::config::GLOBAL_CONFIG;
use crate::manifest::MANIFEST;
use crate::mode::{Mode, GLOBAL_MODE};
use erased_serde::Serialize;
use regex::Regex;
use serde::Serialize as SerdeSerialize;
use tuono_internal::config::ServerConfig;

use crate::request::{Location, Request};

fn has_dynamic_path(route: &str) -> bool {
    let regex = Regex::new(r"\[(.*?)\]").expect("Failed to create the regex");
    regex.is_match(route)
}

#[derive(SerdeSerialize)]
/// This is the payload sent to the client for hydration
pub struct Payload<'a> {
    router: Location,
    props: &'a dyn Serialize,
    mode: Mode,
    #[serde(rename(serialize = "jsBundles"))]
    js_bundles: Option<Vec<&'a String>>,
    #[serde(rename(serialize = "cssBundles"))]
    css_bundles: Option<Vec<&'a String>>,
    #[serde(rename(serialize = "devServerConfig"))]
    dev_server_config: Option<&'a ServerConfig>,
}

impl<'a> Payload<'a> {
    pub fn new(req: &'a Request, props: &'a dyn Serialize) -> Payload<'a> {
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
            router: req.location(),
            props,
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

    /// This method adds the route specific bundles to the server
    /// side rendered HTML.
    ///
    /// The same matching algorithm is implemented on the client side in
    /// this file (packages/tuono/src/router/components/Matches.ts).
    ///
    /// Optimizations should occour on both.
    fn add_bundle_sources(&mut self) {
        // Manifest should always be loaded. The load happen before starting
        // the server.
        let manifest = MANIFEST.get().expect("Failed to load manifest");

        // The main bundle should always exist.
        // The extension should be tsx even with JS only projects.
        let main_bundle = manifest
            .get("client-main")
            .expect("Failed to get client-main bundle");

        let mut js_bundles_sources = vec![&main_bundle.file];
        let mut css_bundles_sources = main_bundle.css.iter().collect::<Vec<&String>>();

        let pathname = &self.router.pathname();

        let bundle_data = manifest.get(*pathname);

        if let Some(data) = bundle_data {
            js_bundles_sources.push(&data.file);

            data.css
                .iter()
                .for_each(|source| css_bundles_sources.push(source))
        } else {
            let dynamic_routes = manifest
                .keys()
                .filter(|path| has_dynamic_path(path))
                .collect::<Vec<&String>>();

            if !dynamic_routes.is_empty() {
                let path_segments = pathname
                    .split('/')
                    .filter(|path| !path.is_empty())
                    .collect::<Vec<&str>>();

                '_dynamic_routes_loop: for dyn_route in dynamic_routes.iter() {
                    let dyn_route_segments = dyn_route
                        .split('/')
                        .filter(|path| !path.is_empty())
                        .collect::<Vec<&str>>();

                    let mut route_segments_collector: Vec<&str> = vec![];

                    for i in 0..dyn_route_segments.len() {
                        if dyn_route_segments[i].starts_with("[...") {
                            route_segments_collector.push(dyn_route_segments[i]);

                            let manifest_key = route_segments_collector.join("/");

                            let route_data = manifest.get(&format!("/{manifest_key}"));
                            if let Some(data) = route_data {
                                js_bundles_sources.push(&data.file);
                                data.css
                                    .iter()
                                    .for_each(|source| css_bundles_sources.push(source))
                            }
                            break '_dynamic_routes_loop;
                        }
                        if path_segments.len() == i {
                            break;
                        }
                        if dyn_route_segments[i] == path_segments[i]
                            || has_dynamic_path(dyn_route_segments[i])
                        {
                            route_segments_collector.push(dyn_route_segments[i])
                        } else {
                            break;
                        }
                    }

                    if route_segments_collector.len() == path_segments.len() {
                        let manifest_key = route_segments_collector.join("/");

                        let route_data = manifest.get(&format!("/{manifest_key}"));
                        if let Some(data) = route_data {
                            js_bundles_sources.push(&data.file);
                            data.css
                                .iter()
                                .for_each(|source| css_bundles_sources.push(source))
                        }
                        break;
                    }
                }
            }
        }

        self.js_bundles = Some(js_bundles_sources);
        self.css_bundles = Some(css_bundles_sources);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use axum::http::Uri;
    use std::collections::HashMap;

    use crate::manifest::BundleInfo;

    fn prepare_payload<'a>(uri: Option<&'a str>, mode: Mode) -> Payload<'a> {
        let mut manifest_mock = HashMap::new();
        manifest_mock.insert(
            "client-main".to_string(),
            BundleInfo {
                file: "assets/bundled-file.js".to_string(),
                css: vec!["assets/bundled-file.css".to_string()],
            },
        );
        manifest_mock.insert(
            "/".to_string(),
            BundleInfo {
                file: "assets/index.js".to_string(),

                css: vec!["assets/index.css".to_string()],
            },
        );
        manifest_mock.insert(
            "/posts/[post]".to_string(),
            BundleInfo {
                file: "assets/posts/[post].js".to_string(),

                css: vec!["assets/posts/[post].css".to_string()],
            },
        );
        manifest_mock.insert(
            "/posts/[post]/[comment]".to_string(),
            BundleInfo {
                file: "assets/posts/[post]/[comment].js".to_string(),

                css: vec!["assets/posts/[post]/[comment].css".to_string()],
            },
        );
        manifest_mock.insert(
            "/pokemons/[...catch_all]".to_string(),
            BundleInfo {
                file: "assets/catch_all.js".to_string(),
                css: vec!["assets/catch_all.css".to_string()],
            },
        );

        manifest_mock.insert(
            "/posts/custom-post".to_string(),
            BundleInfo {
                file: "assets/custom-post.js".to_string(),

                css: vec!["assets/custom-post.css".to_string()],
            },
        );
        manifest_mock.insert(
            "/about".to_string(),
            BundleInfo {
                file: "assets/about.js".to_string(),

                css: vec!["assets/about.css".to_string()],
            },
        );
        MANIFEST.get_or_init(|| manifest_mock);

        let uri = uri
            .unwrap_or("http://localhost:3000/")
            .parse::<Uri>()
            .unwrap();

        let location = Location::from(uri);

        Payload {
            router: location,
            props: &None::<Option<()>>,
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
                &"assets/bundled-file.js".to_string(),
                &"assets/index.js".to_string()
            ])
        );
        assert_eq!(
            payload.css_bundles,
            Some(vec![
                &"assets/bundled-file.css".to_string(),
                &"assets/index.css".to_string()
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

    #[test]
    fn should_load_the_correct_single_dyn_path_bundles() {
        let mut payload = prepare_payload(Some("http://localhost:3000/posts/a-post"), Mode::Prod);
        let _ = payload.client_payload();
        assert_eq!(
            payload.js_bundles,
            Some(vec![
                &"assets/bundled-file.js".to_string(),
                &"assets/posts/[post].js".to_string()
            ])
        );
        assert_eq!(
            payload.css_bundles,
            Some(vec![
                &"assets/bundled-file.css".to_string(),
                &"assets/posts/[post].css".to_string()
            ])
        );
    }

    #[test]
    fn should_load_the_correct_nested_dyn_path_bundles() {
        let mut payload = prepare_payload(
            Some("http://localhost:3000/posts/a-post/a-comment"),
            Mode::Prod,
        );
        let _ = payload.client_payload();
        assert_eq!(
            payload.js_bundles,
            Some(vec![
                &"assets/bundled-file.js".to_string(),
                &"assets/posts/[post]/[comment].js".to_string()
            ])
        );
        assert_eq!(
            payload.css_bundles,
            Some(vec![
                &"assets/bundled-file.css".to_string(),
                &"assets/posts/[post]/[comment].css".to_string()
            ])
        );
    }

    #[test]
    fn should_load_the_correct_catch_all_bundles() {
        let mut payload = prepare_payload(
            Some("http://localhost:3000/pokemons/a-poke/a-poke"),
            Mode::Prod,
        );
        let _ = payload.client_payload();
        assert_eq!(
            payload.js_bundles,
            Some(vec![
                &"assets/bundled-file.js".to_string(),
                &"assets/catch_all.js".to_string()
            ])
        );
        assert_eq!(
            payload.css_bundles,
            Some(vec![
                &"assets/bundled-file.css".to_string(),
                &"assets/catch_all.css".to_string()
            ])
        );
    }
    #[test]
    fn should_load_the_defined_path_bundles() {
        let mut payload = prepare_payload(Some("http://localhost:3000/about"), Mode::Prod);
        let _ = payload.client_payload();
        assert_eq!(
            payload.js_bundles,
            Some(vec![
                &"assets/bundled-file.js".to_string(),
                &"assets/about.js".to_string()
            ])
        );
        assert_eq!(
            payload.css_bundles,
            Some(vec![
                &"assets/bundled-file.css".to_string(),
                &"assets/about.css".to_string()
            ])
        );
    }
}
