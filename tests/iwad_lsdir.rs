/// These tests are for parsing IWAD files. The only legal way to obtain IWADs are by purchasing
/// them, so these tests shouldn't run on the public, remote CI environment, since that would involve
/// copying the IWADs there. These tests will just need to be executed locally.
use assert_cmd::Command;
use assert_fs::prelude::*;
use duct::cmd;
use predicates::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

#[test]
fn wad_lsdir_command_should_fail_for_a_non_wad_file() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let invalid_wad = assert_fs::NamedTempFile::new("invalid.wad").unwrap();
    invalid_wad.write_binary(b"this is not a wad file").unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("wad")
        .arg("lsdir")
        .arg("--path")
        .arg(invalid_wad.path().to_str().unwrap())
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Failed to parse {}",
            invalid_wad.path().display()
        )))
        .stderr(predicate::str::contains(
            "This file is likely not a WAD file",
        ));
}

#[test]
fn doom2_iwad_lsdir_command_should_correctly_parse_header() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("wad")
        .arg("lsdir")
        .arg("--path")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("type: IWAD"))
        .stderr(predicate::str::contains("directory entries: 2919"))
        .stderr(predicate::str::contains("directory offset: 14557880"));
}

#[test]
fn doom2_iwad_lsdir_command_gets_the_correct_number_of_directory_entries() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("wad")
        .arg("lsdir")
        .arg("--path")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("Directory has 2919 entries"));
}

#[test]
fn doom2_iwad_lsdir_command_parse_the_directory_correctly() {
    let lswad_directory = get_directory_from_lswad(&PathBuf::from("test_iwads/DOOM2.WAD"));
    let directory = get_directory_from_tdl(&PathBuf::from("test_iwads/DOOM2.WAD"));
    for (i, entry) in directory.iter().enumerate() {
        assert_eq!(entry.0, lswad_directory[i].0);
        assert_eq!(entry.1, lswad_directory[i].1);
        assert_eq!(entry.2, lswad_directory[i].2);
    }
}

#[test]
fn doom_iwad_lsdir_command_parse_the_directory_correctly() {
    let lswad_directory = get_directory_from_lswad(&PathBuf::from("test_iwads/DOOM.WAD"));
    let directory = get_directory_from_tdl(&PathBuf::from("test_iwads/DOOM.WAD"));
    for (i, entry) in directory.iter().enumerate() {
        assert_eq!(entry.0, lswad_directory[i].0);
        assert_eq!(entry.1, lswad_directory[i].1);
        assert_eq!(entry.2, lswad_directory[i].2);
    }
}

#[test]
fn doom_iwad_lsdir_command_should_correctly_parse_header() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("wad")
        .arg("lsdir")
        .arg("--path")
        .arg("test_iwads/DOOM.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("type: IWAD"))
        .stderr(predicate::str::contains("directory entries: 2306"))
        .stderr(predicate::str::contains("directory offset: 12371396"));
}

#[test]
fn doom_iwad_lsdir_command_gets_the_correct_number_of_directory_entries() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("wad")
        .arg("lsdir")
        .arg("--path")
        .arg("test_iwads/DOOM.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("Directory has 2306 entries"));
}

fn get_directory_from_lswad(wad_path: &impl AsRef<Path>) -> Vec<(String, u32, u32)> {
    let handle = cmd!(
        "/home/chris/dev/misc/xwadtools/lswad/lswad",
        "-ls",
        &wad_path.as_ref().to_str().unwrap()
    )
    .reader()
    .unwrap();

    let mut lswad_directory: Vec<(String, u32, u32)> = Vec::new();
    let reader = BufReader::new(handle);
    for result in reader.lines() {
        let line = result.unwrap();
        let mut split = line.split_whitespace();
        let lump_name = split.next().unwrap();
        let lump_size: u32 = split.next().unwrap().parse().unwrap();
        let lump_offset: u32 = split.next().unwrap().parse().unwrap();
        lswad_directory.push((lump_name.to_string(), lump_size, lump_offset));
    }
    lswad_directory
}

fn get_directory_from_tdl(wad_path: &impl AsRef<Path>) -> Vec<(String, u32, u32)> {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut directory: Vec<(String, u32, u32)> = Vec::new();

    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("wad")
        .arg("lsdir")
        .arg("--path")
        .arg(&wad_path.as_ref().to_str().unwrap())
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success();
    for result in cmd.output().unwrap().stdout.lines() {
        let line = result.unwrap();
        if line.starts_with('+') || line == "| Lump Name | Lump Size | Lump Offset |" {
            // skips the header and separator rows.
            continue;
        }
        let split: Vec<&str> = line.split('|').collect();
        let lump_name = split[1].trim().trim_matches(char::from(0));
        let lump_size: u32 = split[2].trim().parse().unwrap();
        let lump_offset: u32 = split[3].trim().parse().unwrap();
        directory.push((lump_name.to_string(), lump_size, lump_offset));
    }
    directory
}
