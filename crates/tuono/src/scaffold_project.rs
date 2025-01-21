use clap::crate_version;
use reqwest::blocking;
use serde::Deserialize;
use std::env;
use std::fs::{self, create_dir, File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

const GITHUB_TUONO_TAGS_URL: &str = "https://api.github.com/repos/tuono-labs/tuono/git/ref/tags/";

const GITHUB_TUONO_TAG_COMMIT_TREES_URL: &str =
    "https://api.github.com/repos/tuono-labs/tuono/git/trees/";

const GITHUB_RAW_CONTENT_URL: &str = "https://raw.githubusercontent.com/tuono-labs/tuono";

#[derive(Deserialize, Debug)]
enum GithubFileType {
    #[serde(rename = "blob")]
    Blob,
    #[serde(rename = "tree")]
    Tree,
}

#[derive(Deserialize, Debug)]
struct GithubTagObject {
    sha: String,
}

#[derive(Deserialize, Debug)]
struct GithubTagResponse {
    object: GithubTagObject,
}

#[derive(Deserialize, Debug)]
struct GithubTreeResponse<T> {
    tree: Vec<T>,
}

fn exit_with_error(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

#[derive(Deserialize, Debug)]
struct GithubFile {
    path: String,
    #[serde(rename(deserialize = "type"))]
    element_type: GithubFileType,
}

fn create_file(path: PathBuf, content: String) -> std::io::Result<()> {
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => {
            exit_with_error(&format!("Failed to create file {}: {}", path.display(), err));
        }
    };
    let _ = file.write_all(content.as_bytes());

    Ok(())
}

pub fn create_new_project(folder_name: Option<String>, template: Option<String>) {
    let folder = folder_name.unwrap_or(".".to_string());

    // In case of missing select the tuono example
    let template = template.unwrap_or("tuono-app".to_string());

    let client = match blocking::Client::builder().user_agent("").build() {
        Ok(client) => client,
        Err(_) => exit_with_error("Error: Failed to build request client"),
    };

    // This string does not include the "v" version prefix
    let cli_version: &str = crate_version!();

    let res_tag: GithubTagResponse = client
        .get(format!("{}v{}", GITHUB_TUONO_TAGS_URL, cli_version))
        .send()
        .and_then(|response| response.json::<GithubTagResponse>())
        .unwrap_or_else(|_| {
            exit_with_error(&format!(
                "Error: Failed to call or parse the tag github API for v{}",
                cli_version
            ))
        });

    let sha_tagged_commit = res_tag.object.sha;

    let res_tree = client
        .get(format!(
            "{}{}?recursive=1",
            GITHUB_TUONO_TAG_COMMIT_TREES_URL, sha_tagged_commit
        ))
        .send()
        .unwrap_or_else(|_| {
            panic!("Failed to call the tagged commit tree github API for v{cli_version}")
        })
        .json::<GithubTreeResponse<GithubFile>>()
        .expect("Failed to parse the tree structure");

    let new_project_files = res_tree
        .tree
        .iter()
        .filter(|GithubFile { path, .. }| path.starts_with(&format!("examples/{template}/")))
        .collect::<Vec<&GithubFile>>();

    if new_project_files.is_empty() {
        eprintln!("Error: Template '{template}' not found");
        exit_with_error(&format!("Error: Template '{template}' not found. Hint: you can view the available templates at https://github.com/tuono-labs/tuono/tree/main/examples"));
    }

    if folder != "." {
        if Path::new(&folder).exists() {
            eprintln!("Error: Directory '{folder}' already exists");
            exit_with_error(&format!("Error: Directory '{folder}' already exists. Hint: you can scaffold a tuono project within an existing folder with 'cd {folder} && tuono new .'"));
        }
        create_dir(&folder).unwrap();
    }

    let folder_name = PathBuf::from(&folder);
    let current_dir = env::current_dir().expect("Failed to get current working directory");

    let folder_path = current_dir.join(folder_name);

    create_directories(&new_project_files, &folder_path, &template)
        .unwrap_or_else(|err| exit_with_error(&format!("Failed to create directories: {}", err)));

    for GithubFile {
        element_type, path, ..
    } in new_project_files.iter()
    {
        if let GithubFileType::Blob = element_type {
            let file_content = client
                .get(format!("{GITHUB_RAW_CONTENT_URL}/v{cli_version}/{path}"))
                .send()
                .map_err(|_| exit_with_error("Failed to call the folder github API"))
                .and_then(|response| {
                    response
                        .text()
                        .map_err(|_| exit_with_error("Failed to parse the repo structure"))
                })
                .unwrap();

            let path = PathBuf::from(&path.replace(&format!("examples/{template}/"), ""));

            let file_path = folder_path.join(&path);

            if let Err(err) = create_file(file_path, file_content) {
                exit_with_error(&format!("Failed to create file: {}", err));
            }
        }
    }

    update_package_json_version(&folder_path).expect("Failed to update package.json version");
    update_cargo_toml_version(&folder_path).expect("Failed to update Cargo.toml version");
    outro(folder);
}

fn create_directories(
    new_project_files: &[&GithubFile],
    folder_path: &Path,
    template: &String,
) -> io::Result<()> {
    for GithubFile {
        element_type, path, ..
    } in new_project_files.iter()
    {
        if let GithubFileType::Tree = element_type {
            let path = PathBuf::from(&path.replace(&format!("examples/{template}/"), ""));

            let dir_path = folder_path.join(&path);
            create_dir(&dir_path).unwrap();
        }
    }
    Ok(())
}
fn update_package_json_version(folder_path: &Path) -> io::Result<()> {
    let v = crate_version!();
    let package_json_path = folder_path.join(PathBuf::from("package.json"));
    let package_json = match fs::read_to_string(&package_json_path) {
        Ok(content) => content,
        Err(err) => exit_with_error(&format!("Failed to read package.json: {}", err)),
    };
    let package_json = package_json.replace("link:../../packages/tuono", v);

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(package_json_path)
    {
        Ok(file) => file,
        Err(err) => exit_with_error(&format!("Failed to open package.json: {}", err)),
    };

    match file.write_all(package_json.as_bytes()) {
        Ok(_) => (),
        Err(err) => exit_with_error(&format!("Failed to write to package.json: {}", err)),
    }

    Ok(())
}

fn update_cargo_toml_version(folder_path: &Path) -> io::Result<()> {
    let v = crate_version!();
    let cargo_toml_path = folder_path.join(PathBuf::from("Cargo.toml"));
    let cargo_toml = match fs::read_to_string(&cargo_toml_path) {
        Ok(file) => file,
        Err(err) => exit_with_error(&format!("Failed to write to package.json: {}", err)),
    };
    let cargo_toml =
        cargo_toml.replace("{ path = \"../../crates/tuono_lib/\"}", &format!("\"{v}\""));

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(cargo_toml_path)
    {
        Ok(file) => file,
        Err(err) => exit_with_error(&format!("Failed to write to Cargo.toml: {}", err)),
    };

    match file.write_all(cargo_toml.as_bytes()) {
        Ok(_) => (),
        Err(err) => exit_with_error(&format!("Failed to write to Cargo.toml: {}", err)),
    }

    Ok(())
}

fn outro(folder_name: String) {
    println!("Success! 🎉");

    if folder_name != "." {
        println!("\nGo to the project directory:");
        println!("cd {folder_name}/");
    }

    println!("\nInstall the dependencies:");
    println!("npm install");

    println!("\nRun the local environment:");
    println!("tuono dev");
}
