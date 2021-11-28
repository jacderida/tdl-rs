/// Runs the `source-port ls` command.
///
/// This command should list the latest version for each supported source port, or, if there's no
/// version marked as latest, that will be indicated.
///
/// Due to the fact that these tests hit the Github API, they need to be prevented from running as
/// part of the normal test run, and that's why they used the `rate-limiting-tests` feature. Github
/// are actually fairly stringent about the number of unathenticated hits on the API.
use assert_cmd::Command;
use predicates::prelude::*;

#[cfg(feature = "rate-limiting-tests")]
const CHOCOLATE_LATEST_VERSION: &str = "3.0.0";
#[cfg(feature = "rate-limiting-tests")]
const CRISPY_LATEST_VERSION: &str = "5.10.3";
#[cfg(feature = "rate-limiting-tests")]
const DOOM_RETRO_LATEST_VERSION: &str = "4.3";
#[cfg(feature = "rate-limiting-tests")]
const DSDA_DOOM_LATEST_VERSION: &str = "0.22.1";
#[cfg(feature = "rate-limiting-tests")]
const ETERNITY_ENGINE_LATEST_VERSION: &str = "4.02.00";
#[cfg(feature = "rate-limiting-tests")]
const GZDOOM_LATEST_VERSION: &str = "4.7.1";
#[cfg(feature = "rate-limiting-tests")]
const LZDOOM_LATEST_VERSION: &str = "3.88a";
#[cfg(feature = "rate-limiting-tests")]
const ODAMEX_LATEST_VERSION: &str = "0.9.5";
#[cfg(feature = "rate-limiting-tests")]
const PRBOOM_LATEST_VERSION: &str = "2.6.1um";
#[cfg(feature = "rate-limiting-tests")]
const WOOF_LATEST_VERSION: &str = "8.1.0";

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
            predicate::str::is_match(format!("Crispy Doom.*{}", CRISPY_LATEST_VERSION)).unwrap(),
        )
        .stdout(
            predicate::str::is_match(format!("Doom Retro.*{}", DOOM_RETRO_LATEST_VERSION)).unwrap(),
        )
        .stdout(
            predicate::str::is_match(format!("DSDA Doom.*{}", DSDA_DOOM_LATEST_VERSION)).unwrap(),
        )
        .stdout(
            predicate::str::is_match(format!(
                "Eternity Engine.*{}",
                ETERNITY_ENGINE_LATEST_VERSION
            ))
            .unwrap(),
        )
        .stdout(predicate::str::is_match(format!("GZDoom.*{}", GZDOOM_LATEST_VERSION)).unwrap())
        .stdout(predicate::str::is_match(format!("LZDoom.*{}", LZDOOM_LATEST_VERSION)).unwrap())
        .stdout(predicate::str::is_match(format!("Odamex.*{}", ODAMEX_LATEST_VERSION)).unwrap())
        .stdout(
            predicate::str::is_match(format!("PrBoom Plus.*{}", PRBOOM_LATEST_VERSION)).unwrap(),
        )
        .stdout(predicate::str::is_match(format!("Woof.*{}", WOOF_LATEST_VERSION)).unwrap())
        .stdout(predicate::str::contains(
            "RUDE has no version marked as latest",
        ))
        .stdout(predicate::str::contains(
            "Zandronum has no version marked as latest",
        ));
}
