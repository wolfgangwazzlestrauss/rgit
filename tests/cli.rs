use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use rstest::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile;

/// Create a repository and initialize it for version control.
#[fixture]
fn temp_repo() -> PathBuf {
    let tmp_dir = tempfile::tempdir().unwrap().path().to_owned();
    fs::create_dir(&tmp_dir).unwrap();

    let text = "I am some mock text for a file.";
    let file_path = tmp_dir.join("code.txt");
    fs::write(file_path, text).unwrap();

    env::set_current_dir(&tmp_dir).unwrap();

    tmp_dir
}

#[rstest]
fn init_success(_temp_repo: PathBuf) {
    let mut cmd = Command::cargo_bin("rgit").unwrap();
    cmd.arg("init");
    cmd.assert().success();
}
