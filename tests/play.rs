use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn play_should_run_the_game_with_the_default_profile() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let settings_file_path = settings_file.path().to_str().unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("prboom")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("default")
        .arg("prboom")
        .arg("2.6")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("DOOM2")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Game called with -skill: 4"));
}

#[test]
fn play_should_run_the_game_using_the_specified_profile() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let settings_file_path = settings_file.path().to_str().unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("prboom")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("default")
        .arg("prboom")
        .arg("2.6")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("second")
        .arg("prboom")
        .arg("2.6")
        .arg("UltraViolence")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("DOOM2")
        .arg("--profile")
        .arg("second")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Game called with -skill: 4"))
        .stdout(predicate::str::contains("Game called with -nomusic"))
        .stdout(predicate::str::contains("Game called with -nofullscreen"));
}

#[test]
fn play_should_fail_if_non_existent_source_port_is_specified() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let settings_file_path = settings_file.path().to_str().unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("prboom")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("default")
        .arg("prboom")
        .arg("2.6")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("DOOM2")
        .arg("--profile")
        .arg("badref")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .assert()
        .failure();
}

fn get_fake_source_port_path() -> String {
    let mut fake_source_port_path = std::env::current_dir().unwrap();
    fake_source_port_path.push("target");
    fake_source_port_path.push("debug");
    fake_source_port_path.push("fake_source_port");
    String::from(fake_source_port_path.as_path().to_str().unwrap())
}
