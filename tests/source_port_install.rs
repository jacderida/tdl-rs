/// Runs the `source-port install` command for each supported source port.
///
/// To avoid hitting the Github API, these tests are going to copy the Github responses to the
/// cache directory.
#[cfg(target_family = "windows")]
use {
    assert_cmd::Command,
    assert_fs::prelude::*,
    chrono::{Duration, Utc},
    predicates::prelude::*,
    test_helpers::cache::set_date_for_cache_entries,
};

#[cfg(target_family = "windows")]
const CHOCOLATE_LATEST_VERSION: &str = "3.0.0";
#[cfg(target_family = "windows")]
const CRISPY_LATEST_VERSION: &str = "5.10.3";
#[cfg(target_family = "windows")]
const DOOM_RETRO_LATEST_VERSION: &str = "4.3";
#[cfg(target_family = "windows")]
const ETERNITY_ENGINE_LATEST_VERSION: &str = "4.02.00";
#[cfg(target_family = "windows")]
const GZDOOM_LATEST_VERSION: &str = "4.7.1";
#[cfg(target_family = "windows")]
const LZDOOM_LATEST_VERSION: &str = "3.88a";
#[cfg(target_family = "windows")]
const ODAMEX_LATEST_VERSION: &str = "0.9.5";
#[cfg(target_family = "windows")]
const PRBOOM_LATEST_VERSION: &str = "2.6.1um";
#[cfg(target_family = "windows")]
const WOOF_LATEST_VERSION: &str = "7.0.0";

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_chocolate_doom() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!(
        "source-ports/chocolate-{}",
        CHOCOLATE_LATEST_VERSION
    ));
    let sp_exe_file = install_dir.child("chocolate-doom.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("Chocolate")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("Chocolate Doom.*{}.*Yes", CHOCOLATE_LATEST_VERSION))
                .unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_crispy_doom() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!("source-ports/crispy-{}", CRISPY_LATEST_VERSION));
    let sp_exe_file = install_dir.child("crispy-doom.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("Crispy")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("Crispy Doom.*{}.*Yes", CRISPY_LATEST_VERSION))
                .unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_doom_retro() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!(
        "source-ports/doomretro-{}",
        DOOM_RETRO_LATEST_VERSION
    ));
    let sp_exe_file = install_dir.child("doomretro.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("DoomRetro")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("Doom Retro.*{}.*Yes", DOOM_RETRO_LATEST_VERSION))
                .unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_fail_to_install_the_latest_version_of_dsda() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir =
        doom_home_dir.child(format!("source-ports/dsda-{}", DOOM_RETRO_LATEST_VERSION));

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("Dsda")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Failed to install the latest version of DSDA Doom",
        ))
        .stderr(predicate::str::contains(
            "You can try the command again with the --version argument to install a specific version",
        ))
        .stderr(predicate::str::contains(
            "Check the Github repository for DSDA Doom to see what versions are available",
        ));

    install_dir.assert(predicates::path::missing());
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_eternity_engine() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!(
        "source-ports/eternityengine-{}",
        ETERNITY_ENGINE_LATEST_VERSION
    ));
    let sp_exe_file = install_dir.child("eternity.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("EternityEngine")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!(
                "Eternity Engine.*{}.*Yes",
                ETERNITY_ENGINE_LATEST_VERSION
            ))
            .unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_gzdoom() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!("source-ports/gzdoom-{}", GZDOOM_LATEST_VERSION));
    let sp_exe_file = install_dir.child("gzdoom.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("GzDoom")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("GZDoom.*{}.*Yes", GZDOOM_LATEST_VERSION)).unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_lzdoom() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!("source-ports/lzdoom-{}", LZDOOM_LATEST_VERSION));
    let sp_exe_file = install_dir.child("lzdoom.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("LzDoom")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("LZDoom.*{}.*Yes", LZDOOM_LATEST_VERSION)).unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_odamex() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!("source-ports/odamex-{}", ODAMEX_LATEST_VERSION));
    let sp_exe_file = install_dir.child("odamex.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("Odamex")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("Odamex.*{}.*Yes", ODAMEX_LATEST_VERSION)).unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_prboom() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!("source-ports/prboom-{}", PRBOOM_LATEST_VERSION));
    let sp_exe_file = install_dir.child("prboom-plus.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("PrBoomPlus")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!("PrBoom Plus.*{}.*Yes", PRBOOM_LATEST_VERSION))
                .unwrap(),
        );
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_install_the_latest_version_of_woof() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let install_dir = doom_home_dir.child(format!("source-ports/woof-{}", WOOF_LATEST_VERSION));
    let archive_dir = install_dir.child(format!("Woof-{}-win32", WOOF_LATEST_VERSION));
    let sp_exe_file = install_dir.child("woof.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("Woof")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    install_dir.assert(predicates::path::is_dir());
    sp_exe_file.assert(predicates::path::is_file());
    // The archive for Woof contains a directory at its root level. The files should be moved to
    // the parent directory and the directory from the archive should be deleted.
    archive_dir.assert(predicates::path::missing());

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("ls")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!("Woof!.*{}.*Yes", WOOF_LATEST_VERSION)).unwrap());
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_fail_if_destination_dir_already_exists() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap().into_persistent();
    let install_dir = doom_home_dir.child(format!(
        "source-ports/chocolate-{}",
        CHOCOLATE_LATEST_VERSION
    ));
    install_dir.create_dir_all().unwrap();
    let sp_exe_file = install_dir.child("chocolate-doom.exe");

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("Chocolate")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Failed to install the latest version of Chocolate Doom",
        ))
        .stderr(predicate::str::contains(format!(
            "Remove the {} directory and run the command again",
            install_dir.path().display()
        )));
    sp_exe_file.assert(predicates::path::missing());
}

#[cfg(target_family = "windows")]
#[test]
fn source_port_install_should_fail_if_the_source_port_is_already_installed() {
    let cache_dt = Utc::now() - Duration::hours(1);

    let settings_dir = assert_fs::TempDir::new().unwrap();
    let release_cache_dir = settings_dir.child("release_cache");
    release_cache_dir.create_dir_all().unwrap();
    release_cache_dir
        .copy_from("resources/test_data/release_cache_entries", &["*.json"])
        .unwrap();
    set_date_for_cache_entries(release_cache_dir.path(), cache_dt).unwrap();

    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("PrBoomPlus")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("source-port")
        .arg("install")
        .arg("PrBoomPlus")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Version {} of PrBoom Plus is already installed",
            PRBOOM_LATEST_VERSION
        )));
}
