mod utils;
use assert_cmd::Command;
use serial_test::serial;
use utils::TempTuonoProject;

#[test]
#[serial]
fn it_inits_new_git_repo_by_default_with_git_installed() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new.arg("new").arg(".").assert().success();

    // Ensure the `.git` directory exists
    assert!(temp_tuono_project.path().join(".git").exists());
}

#[test]
#[serial]
fn it_does_not_init_new_git_repo_with_git_false() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg(".")
        .arg("--git-init=false")
        .assert()
        .success();

    // Ensure the `.git` directory does not exist
    assert!(!temp_tuono_project.path().join(".git").exists());
}

#[test]
#[serial]
fn it_creates_project_without_git_if_not_installed() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg(".")
        .env("PATH", "") // Simulate git not being installed
        .assert()
        .success();

    assert!(!temp_tuono_project.path().join(".git").exists());
}

#[test]
#[serial]
fn it_errors_if_git_not_installed_and_flag_set() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg(".")
        .arg("--git-init=true")
        .env("PATH", "") // Simulate git not being installed
        .assert()
        .failure()
        .stderr("You requested to use Git, but it is not installed.\n");
}
