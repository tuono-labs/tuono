use fs_extra::dir::create_all;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

#[derive(Debug)]
pub struct TempTuonoProject {
    original_dir: PathBuf,
    temp_dir: TempDir,
}

impl Default for TempTuonoProject {
    fn default() -> Self {
        Self::new()
    }
}

impl TempTuonoProject {
    pub fn new() -> Self {
        let original_dir = env::current_dir().expect("Failed to read current_dir");
        let temp_dir = tempdir().expect("Failed to create temp_dir");

        env::set_current_dir(temp_dir.path()).expect("Failed to change current dir into temp_dir");

        let project = TempTuonoProject {
            original_dir,
            temp_dir,
        };

        project.add_file("./tuono.config.ts");

        project
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn add_file(&self, path: &str) -> File {
        let path = PathBuf::from(path);
        create_all(
            path.parent().expect("Route path does not have any parent"),
            false,
        )
        .expect("Failed to create parent route directory");
        File::create(path).expect("Failed to create the route file")
    }

    pub fn add_file_with_content<'a>(&self, path: &'a str, content: &'a str) {
        let path = PathBuf::from(path);
        create_all(
            path.parent().expect("Route path does not have any parent"),
            false,
        )
        .expect("Failed to create parent route directory");

        let mut file = File::create(path).expect("Failed to create the route file");
        file.write_all(content.as_bytes())
            .expect("Failed to write into API file");
    }
}

impl Drop for TempTuonoProject {
    fn drop(&mut self) {
        // Set back the current dir in the previous state
        env::set_current_dir(&self.original_dir)
            .expect("Failed to restore the original directory.");
    }
}
