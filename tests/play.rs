use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::path::PathBuf;
use test_helpers::helpers::get_fake_source_port_path;

#[test]
fn play_should_run_the_game_with_the_default_profile() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM2")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Game called with -skill: 4"));
}

#[test]
fn play_should_run_the_game_using_the_specified_profile() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("second")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM2")
        .arg("--profile")
        .arg("second")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Game called with -skill: 4"))
        .stdout(predicate::str::contains("Game called with -nomusic"))
        .stdout(predicate::str::contains("Game called with -nofullscreen"));
}

#[test]
fn play_should_fail_if_non_existent_source_port_is_specified() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let fake_source_port_path = get_fake_source_port_path();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM2")
        .arg("--profile")
        .arg("badref")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .failure();
}

#[test]
fn play_should_run_the_game_with_the_selected_iwad() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let wad_dir_path = doom_home_dir.child("iwads");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut pb = PathBuf::from(wad_dir_path.path());
    pb.push("DOOM2.WAD");
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM2")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Game called with -iwad: {}",
            pb.as_path().display()
        )))
        .stdout(predicate::str::contains("Game called with -skill: 4"));
}

#[test]
fn play_should_run_the_game_with_the_selected_iwad_and_doom2_map() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let wad_dir_path = doom_home_dir.child("iwads");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut pb = PathBuf::from(wad_dir_path.path());
    pb.push("DOOM2.WAD");
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM2")
        .arg("--map")
        .arg("MAP12")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Game called with -iwad: {}",
            pb.as_path().display()
        )))
        .stdout(predicate::str::contains("Game called with -warp: 12"))
        .stdout(predicate::str::contains("Game called with -skill: 4"));
}

#[test]
fn play_should_run_the_game_with_an_invalid_map() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let wad_dir_path = doom_home_dir.child("iwads");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut pb = PathBuf::from(wad_dir_path.path());
    pb.push("DOOM2.WAD");
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM2")
        .arg("--map")
        .arg("MAP35")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Could not find MAP35 in DOOM2"));
}

#[test]
fn play_should_run_the_game_with_the_selected_iwad_and_doom_map() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let wad_dir_path = doom_home_dir.child("iwads");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--type")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .arg("--music")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut pb = PathBuf::from(wad_dir_path.path());
    pb.push("DOOM.WAD");
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("play")
        .arg("--megawad")
        .arg("DOOM")
        .arg("--map")
        .arg("E1M7")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Game called with -iwad: {}",
            pb.as_path().display()
        )))
        .stdout(predicate::str::contains("Game called with -warp: 1 7"))
        .stdout(predicate::str::contains("Game called with -skill: 4"));
}
