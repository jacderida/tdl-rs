use assert_cmd::Command;
use predicates::prelude::*;
use test_helpers::helpers::get_fake_source_port_path;

#[test]
fn profile_ls_with_2_profiles_should_list_the_added_profiles() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let fake_source_port_path = get_fake_source_port_path();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(&fake_source_port_path)
        .arg("2.6")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("add")
        .arg("PrBoomPlus")
        .arg(&fake_source_port_path)
        .arg("2.6um")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("add")
        .arg("--name")
        .arg("default")
        .arg("--source-port")
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
        .arg("profile2")
        .arg("--source-port")
        .arg("PrBoomPlus")
        .arg("--version")
        .arg("2.6um")
        .arg("--skill")
        .arg("UltraViolence")
        .arg("--fullscreen")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("ls")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("Listing 2 profiles"))
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Source Port"))
        .stdout(predicate::str::contains("Version"))
        .stdout(predicate::str::contains("Is Default?"))
        .stdout(predicate::str::contains("default"))
        .stdout(predicate::str::contains("PrBoom Plus"))
        .stdout(predicate::str::contains("2.6"))
        .stdout(predicate::str::contains("profile2"))
        .stdout(predicate::str::contains("PrBoom Plus"))
        .stdout(predicate::str::contains("2.6um"));
}

#[test]
fn profile_ls_with_no_profiles_should_print_empty_message() {
    let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("profile")
        .arg("ls")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_file.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("No profiles have been added yet."))
        .stderr(predicate::str::contains(
            "Run the `profile add` command to create a new profile.",
        ));
}
