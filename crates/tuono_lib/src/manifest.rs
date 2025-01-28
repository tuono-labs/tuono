use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const VITE_MANIFEST_PATH: &str = "./out/client/.vite/manifest.json";

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BundleInfo {
    /// TODO: Add also the import field and load the dynamic
    /// values in the payload bundles.
    pub file: String,
    #[serde(default = "default_css_vector")]
    pub css: Vec<String>,
}

fn default_css_vector() -> Vec<String> {
    Vec::with_capacity(0)
}

/// Manifest is the mapping between the vite output bundled files
/// and the originals.
/// Vite doc: https://vitejs.dev/config/build-options.html#build-manifest
pub static MANIFEST: OnceCell<HashMap<String, BundleInfo>> = OnceCell::new();

pub fn load_manifest() {
    let file = File::open(PathBuf::from(VITE_MANIFEST_PATH)).unwrap();
    let reader = BufReader::new(file);
    let manifest = serde_json::from_reader(reader).unwrap();
    let _ = MANIFEST.set(remap_manifest_keys(manifest));
}

fn remap_manifest_keys(manifest: HashMap<String, BundleInfo>) -> HashMap<String, BundleInfo> {
    let mut new_hashmap = HashMap::new();

    manifest.keys().for_each(|key| {
        let new_key = key
            .replace("../src/routes", "")
            .replace(".tsx", "")
            .replace(".jsx", "")
            .replace("index", "");

        new_hashmap.insert(new_key, manifest.get(key).unwrap().clone());
    });

    new_hashmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parse_the_manifest_json() {
        let manifest_example = r#"{
  "../src/routes/index.tsx": {
    "file": "assets/index.js",
    "name": "index",
    "src": "../src/routes/index.tsx",
    "isDynamicEntry": true,
    "imports": [
      "client-main.tsx"
    ],
    "css": [
      "assets/index.css"
    ]
  },
  "meta-tags-lib": {
    "file": "assets/meta-lib.js",
    "name": "meta-tags-lib",
    "imports": [
      "client-main.tsx"
    ]
  },
  "client-main.tsx": {
    "file": "assets/client-main.js",
    "name": "client-main",
    "src": "client-main.tsx",
    "isEntry": true,
    "dynamicImports": [
      "../src/routes/index.tsx",
      "../src/routes/pokemons/[pokemon].tsx"
    ],
    "css": [
      "assets/client-main.css"
    ]
  }
}"#;

        let parsed_manifest =
            serde_json::from_str::<HashMap<String, BundleInfo>>(manifest_example).unwrap();

        let mut result = HashMap::new();
        result.insert(
            "../src/routes/index.tsx".to_string(),
            BundleInfo {
                file: "assets/index.js".to_string(),
                css: vec!["assets/index.css".to_string()],
            },
        );
        result.insert(
            "client-main.tsx".to_string(),
            BundleInfo {
                file: "assets/client-main.js".to_string(),
                css: vec!["assets/client-main.css".to_string()],
            },
        );

        result.insert(
            "meta-tags-lib".to_string(),
            BundleInfo {
                file: "assets/meta-lib.js".to_string(),
                css: Vec::new(),
            },
        );

        assert_eq!(parsed_manifest, result);
    }

    #[test]
    fn should_correctly_remap_the_manifest() {
        let mut parsed_manifest: HashMap<String, BundleInfo> = HashMap::new();
        parsed_manifest.insert(
            "../src/routes/index.tsx".to_string(),
            BundleInfo {
                file: "assets/index.js".to_string(),
                css: vec!["assets/index.css".to_string()],
            },
        );
        parsed_manifest.insert(
            "../src/routes/about.jsx".to_string(),
            BundleInfo {
                file: "assets/about.js".to_string(),
                css: vec!["assets/about.css".to_string()],
            },
        );
        parsed_manifest.insert(
            "../src/routes/posts/[post].tsx".to_string(),
            BundleInfo {
                file: "assets/posts/[post].js".to_string(),
                css: vec!["assets/posts/[post].css".to_string()],
            },
        );
        parsed_manifest.insert(
            "client-main.tsx".to_string(),
            BundleInfo {
                file: "assets/main.js".to_string(),
                css: vec!["assets/main.css".to_string()],
            },
        );

        let remapped = remap_manifest_keys(parsed_manifest);

        assert_eq!(
            remapped.get("/").unwrap().file,
            "assets/index.js".to_string()
        );
        assert_eq!(
            remapped.get("/about").unwrap().file,
            "assets/about.js".to_string()
        );
        assert_eq!(
            remapped.get("/posts/[post]").unwrap().file,
            "assets/posts/[post].js".to_string()
        );
        assert_eq!(
            remapped.get("client-main").unwrap().file,
            "assets/main.js".to_string()
        );
    }
}
