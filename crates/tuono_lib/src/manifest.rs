use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const VITE_MANIFEST_PATH: &str = "./out/client/.vite/manifest.json";

fn has_dynamic_path(pathname: &str) -> bool {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[(.*?)\]").expect("Invalid regex for dynamic path detection"));
    RE.is_match(pathname)
}

/// ViteManifest is the mapping between the vite output bundled files
/// and the originals.
/// Vite doc: https://vitejs.dev/config/build-options.html#build-manifest
pub type ViteManifest = HashMap<String, BundleInfo>;

fn empty_vector() -> Vec<String> {
    Vec::with_capacity(0)
}

/// Interface representing the bundle information
/// as they are in the vite manifest.json.
///
/// Used for deserialization
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BundleInfo {
    pub file: String,
    #[serde(default = "empty_vector")]
    pub css: Vec<String>,
    #[serde(default = "empty_vector")]
    pub imports: Vec<String>,
    // TODO: Add also dynamic imports
}

#[derive(Debug, Default, Clone)]
pub struct RouteBundle {
    pub css_files: Vec<String>,
    pub js_files: Vec<String>,
}

#[derive(Debug)]
pub struct Manifest {
    /// The mapping between the route and the bundle
    bundles: HashMap<String, RouteBundle>,
}

fn clean_route_path(path: String) -> String {
    let path = path
        .replace("../src/routes", "")
        .replace(".tsx", "")
        .replace(".mdx", "")
        .replace(".md", "")
        .replace(".jsx", "");

    if path == "/index" {
        return "/".to_string();
    }

    path.replace("/index", "")
}

impl From<ViteManifest> for Manifest {
    fn from(manifest: ViteManifest) -> Self {
        let mut bundles = HashMap::new();
        let client_main = manifest
            .get("client-main.tsx")
            // client-main.tsx is the entry point and always exists
            .expect("client-main.tsx not found in the manifest")
            .clone();

        for (key, bundle) in &manifest {
            if key.contains("__layout") {
                continue;
            }

            if key == "client-main.tsx" {
                bundles.insert(
                    "client-main".to_string(),
                    RouteBundle {
                        css_files: bundle.css.clone(),
                        js_files: vec![bundle.file.clone()],
                    },
                );
                continue;
            }

            let route = clean_route_path(key.clone());

            // Skip components/utils files
            if !route.starts_with("/") {
                continue;
            }

            let css_files = [bundle.css.clone(), client_main.css.clone()].concat();
            let js_files = vec![bundle.file.clone(), client_main.file.clone()];

            let mut route_bundle = RouteBundle {
                css_files,
                js_files,
            };

            // the imports bundle always contains at least the client-main
            if bundle.imports.len() > 1 {
                for import in &bundle.imports {
                    if import == "client-main.tsx" {
                        continue;
                    }

                    if let Some(import_bundle) = manifest.get(import) {
                        route_bundle.js_files.push(import_bundle.file.clone());
                        route_bundle.css_files.extend(import_bundle.css.clone());
                    }
                }
            }

            bundles.insert(route, route_bundle);
        }

        // Add __layout imports
        for (key, layout_bundle) in &manifest {
            let route = clean_route_path(key.clone());
            if route.contains("__layout") {
                let path_included_in_layout = route.replace("__layout", "");

                let mut layout_css_files: Vec<String> = Vec::new();
                let mut layout_js_files: Vec<String> = Vec::new();

                for import in &layout_bundle.imports {
                    if import == "client-main.tsx" {
                        continue;
                    }

                    if let Some(import_bundle) = manifest.get(import) {
                        layout_js_files.push(import_bundle.file.clone());
                        layout_css_files.extend(import_bundle.css.clone());
                    }
                }

                for (key, route_bundles) in &mut bundles {
                    if key.starts_with(path_included_in_layout.as_str()) {
                        route_bundles.js_files.push(layout_bundle.file.clone());
                        route_bundles.css_files.extend(layout_bundle.css.clone());
                        route_bundles.js_files.extend(layout_js_files.clone());
                        route_bundles.css_files.extend(layout_css_files.clone());
                    }
                }
            }
        }

        Manifest { bundles }
    }
}

impl Manifest {
    /// This method adds the route specific bundles to the server
    /// side rendered HTML.
    ///
    /// The same matching algorithm is implemented on the client side in
    /// this file (packages/tuono/src/router/components/Matches.ts).
    ///
    /// Optimizations should occour on both.
    pub fn get_bundle_from_pathname(&self, pathname: &str) -> RouteBundle {
        // Exact match
        if let Some(bundle) = self.bundles.get(pathname) {
            return bundle.clone();
        }

        let dynamic_routes = self
            .bundles
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

                let mut route_segments_collector: Vec<&str> = Vec::new();

                for i in 0..dyn_route_segments.len() {
                    // Catch all dynamic route
                    if dyn_route_segments[i].starts_with("[...") {
                        route_segments_collector.push(dyn_route_segments[i]);

                        let manifest_key = route_segments_collector.join("/");

                        let route_data = self.bundles.get(&format!("/{manifest_key}"));

                        if let Some(data) = route_data {
                            return data.clone();
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

                    let route_data = self.bundles.get(&format!("/{manifest_key}"));
                    if let Some(data) = route_data {
                        return data.clone();
                    }
                    break;
                }
            }
        }

        // No dynamic routes, return the client main bundle
        if let Some(bundle) = self.bundles.get("client-main") {
            return bundle.clone();
        }

        // This should never happen because client-main always exists
        RouteBundle::default()
    }
}

pub static MANIFEST: OnceCell<Manifest> = OnceCell::new();

/// Load the vite manifest from the file system and set it in the MANIFEST
/// static variable.
pub fn load_manifest() -> std::io::Result<()> {
    let file = File::open(PathBuf::from(VITE_MANIFEST_PATH))?;
    let reader = BufReader::new(file);
    let manifest: ViteManifest = serde_json::from_reader(reader)?;
    MANIFEST
        .set(Manifest::from(manifest))
        .map_err(|_| std::io::Error::other("Failed to set the manifest"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // This manifest is an example of a complex vite manifest.json
    // It includes dynamic routes, static routes, catch all routes, nested
    // __layout and shared components.
    const MANIFEST_EXAMPLE: &str = r#"{
      "../src/routes/about.tsx": {
        "file": "assets/about-C3UqHfGb.js",
        "name": "about",
        "src": "../src/routes/about.tsx",
        "isDynamicEntry": true,
        "imports": [
          "client-main.tsx",
          "_FileWithCssOnly.js"
        ],
        "css": [
          "assets/about-DUhMJ_Ze.css"
        ]
      },
      "_FileWithCssOnly.js": {
        "file": "assets/FileWithCssOnly.js",
        "name": "FileWithCssOnly",
        "imports": [
          "client-main.tsx"
        ],
        "css": [
          "assets/FileWithCssOnly.css"
        ]
      },
      "../src/routes/catch_all/[...slug].tsx": {
        "file": "assets/_...slug_-CpJyPnPj.js",
        "name": "_...slug_",
        "src": "../src/routes/catch_all/[...slug].tsx",
        "isDynamicEntry": true,
        "imports": [
          "client-main.tsx"
        ],
        "css": [
          "assets/_..-CipbPoTl.css"
        ]
      },
      "../src/routes/index.tsx": {
        "file": "assets/index-B3tnHOzi.js",
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
      "../src/routes/pokemons/[pokemon]/[type].tsx": {
        "file": "assets/_type_-B-sJOcVJ.js",
        "name": "_type_",
        "src": "../src/routes/pokemons/[pokemon]/[type].tsx",
        "isDynamicEntry": true,
        "imports": [
          "client-main.tsx",
          "_PokemonView-jNGFFO0j.js"
        ],
        "css": [
          "assets/_type_-B8vgxybx.css"
        ]
      },
      "../src/routes/pokemons/[pokemon]/index.tsx": {
        "file": "assets/index-ByRBj7WK.js",
        "name": "index",
        "src": "../src/routes/pokemons/[pokemon]/index.tsx",
        "isDynamicEntry": true,
        "imports": [
          "client-main.tsx",
          "_PokemonView-jNGFFO0j.js"
        ],
        "css": [
          "assets/index-CM86zKWq.css"
        ]
      },
      "../src/routes/pokemons/__layout.tsx": {
        "file": "assets/__layout-2v3JiSeL.js",
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
      "_PokemonView-BcJZaQaO.css": {
        "file": "assets/PokemonView-BcJZaQaO.css",
        "src": "_PokemonView-BcJZaQaO.css"
      },
      "_PokemonView-jNGFFO0j.js": {
        "file": "assets/PokemonView-jNGFFO0j.js",
        "name": "PokemonView",
        "imports": [
          "client-main.tsx"
        ],
        "css": [
          "assets/PokemonView-BcJZaQaO.css"
        ]
      },
      "client-main.tsx": {
        "file": "assets/client-main-DOdr9gvl.js",
        "name": "client-main",
        "src": "client-main.tsx",
        "isEntry": true,
        "dynamicImports": [
          "../src/routes/pokemons/__layout.tsx",
          "../src/routes/about.tsx",
          "../src/routes/index.tsx",
          "../src/routes/catch_all/[...slug].tsx",
          "../src/routes/pokemons/[pokemon]/[type].tsx",
          "../src/routes/pokemons/[pokemon]/index.tsx"
        ],
        "css": [
          "assets/client-main-BS7N-NIa.css"
        ]
      }
    }"#;

    #[test]
    fn it_correctly_cleans_the_route_path() {
        let cleaned_path = clean_route_path("../src/routes/index.tsx".to_string());
        assert_eq!(cleaned_path, "/");

        let cleaned_path =
            clean_route_path("../src/routes/pokemons/[pokemon]/index.tsx".to_string());
        assert_eq!(cleaned_path, "/pokemons/[pokemon]");

        let cleaned_path = clean_route_path("../src/routes/pokemons/__layout.tsx".to_string());
        assert_eq!(cleaned_path, "/pokemons/__layout");

        let cleaned_path =
            clean_route_path("../src/routes/pokemons/[pokemon]/[type].mdx".to_string());
        assert_eq!(cleaned_path, "/pokemons/[pokemon]/[type]");

        let cleaned_path = clean_route_path("../src/routes/about.md".to_string());
        assert_eq!(cleaned_path, "/about");
    }

    #[test]
    fn correctly_parse_the_manifest_json() {
        let parsed_manifest = serde_json::from_str::<ViteManifest>(MANIFEST_EXAMPLE).unwrap();

        let manifest = Manifest::from(parsed_manifest);
        assert_eq!(manifest.bundles.len(), 6);
        let index_route = manifest.get_bundle_from_pathname("/");

        assert_eq!(
            index_route.css_files,
            vec![
                "assets/index-CynfArjF.css",
                "assets/client-main-BS7N-NIa.css"
            ]
        );

        assert_eq!(
            index_route.js_files,
            vec!["assets/index-B3tnHOzi.js", "assets/client-main-DOdr9gvl.js"]
        );
    }

    #[test]
    fn should_load_the_correct_single_dyn_path() {
        let parsed_manifest = serde_json::from_str::<ViteManifest>(MANIFEST_EXAMPLE).unwrap();

        let manifest = Manifest::from(parsed_manifest);
        let nested_route = manifest.get_bundle_from_pathname("/pokemons/ditto");

        assert_eq!(
            nested_route.css_files,
            vec![
                "assets/index-CM86zKWq.css",
                "assets/client-main-BS7N-NIa.css",
                "assets/PokemonView-BcJZaQaO.css",
                "assets/__layout-CXGGqNw5.css"
            ]
        );
        assert_eq!(
            nested_route.js_files,
            vec![
                "assets/index-ByRBj7WK.js",
                "assets/client-main-DOdr9gvl.js",
                "assets/PokemonView-jNGFFO0j.js",
                "assets/__layout-2v3JiSeL.js"
            ]
        );
    }

    #[test]
    fn should_load_the_correct_nested_dyn_path_bundles() {
        let parsed_manifest = serde_json::from_str::<ViteManifest>(MANIFEST_EXAMPLE).unwrap();

        let manifest = Manifest::from(parsed_manifest);
        let route = manifest.get_bundle_from_pathname("/pokemons/charizard/fire");

        assert_eq!(
            route.css_files,
            vec![
                "assets/_type_-B8vgxybx.css",
                "assets/client-main-BS7N-NIa.css",
                "assets/PokemonView-BcJZaQaO.css",
                "assets/__layout-CXGGqNw5.css"
            ]
        );

        assert_eq!(
            route.js_files,
            vec![
                "assets/_type_-B-sJOcVJ.js",
                "assets/client-main-DOdr9gvl.js",
                "assets/PokemonView-jNGFFO0j.js",
                "assets/__layout-2v3JiSeL.js"
            ]
        );
    }
    #[test]
    fn should_load_the_correct_catch_all_bundles() {
        let parsed_manifest = serde_json::from_str::<ViteManifest>(MANIFEST_EXAMPLE).unwrap();

        let manifest = Manifest::from(parsed_manifest);
        let route = manifest.get_bundle_from_pathname("/catch_all/some/random/path");

        assert_eq!(
            route.css_files,
            vec!["assets/_..-CipbPoTl.css", "assets/client-main-BS7N-NIa.css"]
        );

        assert_eq!(
            route.js_files,
            vec![
                "assets/_...slug_-CpJyPnPj.js",
                "assets/client-main-DOdr9gvl.js"
            ]
        );
    }
    #[test]
    fn should_load_the_defined_path_bundles() {
        let parsed_manifest = serde_json::from_str::<ViteManifest>(MANIFEST_EXAMPLE).unwrap();

        let manifest = Manifest::from(parsed_manifest);
        let route = manifest.get_bundle_from_pathname("/about");
        assert_eq!(
            route.css_files,
            vec![
                "assets/about-DUhMJ_Ze.css",
                "assets/client-main-BS7N-NIa.css",
                "assets/FileWithCssOnly.css"
            ]
        );
        assert_eq!(
            route.js_files,
            vec![
                "assets/about-C3UqHfGb.js",
                "assets/client-main-DOdr9gvl.js",
                "assets/FileWithCssOnly.js"
            ]
        );
    }
}
