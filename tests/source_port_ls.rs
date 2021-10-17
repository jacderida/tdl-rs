/// These tests are for parsing IWAD files. The only legal way to obtain IWADs are by purchasing
/// them, so these tests shouldn't run on the public, remote CI environment, since that would involve
/// copying the IWADs there. These tests will just need to be executed locally.
use assert_cmd::Command;
use predicates::prelude::*;

#[cfg(feature = "rate-limiting-tests")]
const CHOCOLATE_LATEST_VERSION: &str = "3.0.0";
#[cfg(feature = "rate-limiting-tests")]
const DOOM_RETRO_LATEST_VERSION: &str = "4.2.3";

#[cfg(feature = "rate-limiting-tests")]
#[test]
fn source_port_ls_should_list_latest_versions_for_supported_source_ports() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("Chocolate Doom.*{}", CHOCOLATE_LATEST_VERSION))
                .unwrap(),
        )
        .stdout(
            predicate::str::is_match(format!("Doom Retro.*{}", DOOM_RETRO_LATEST_VERSION)).unwrap(),
        );
}
