use color_eyre::{eyre::ensure, Report, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt, Serialize, Deserialize)]
pub enum Skill {
    TooYoungToDie,
    HeyNotTooRough,
    HurtMePlenty,
    UltraViolence,
    Nightmare,
}

impl FromStr for Skill {
    type Err = String;

    fn from_str(input: &str) -> Result<Skill, Self::Err> {
        match input {
            "TooYoungToDie" => Ok(Skill::TooYoungToDie),
            "HeyNotTooRough" => Ok(Skill::HeyNotTooRough),
            "HurtMePlenty" => Ok(Skill::HurtMePlenty),
            "UltraViolence" => Ok(Skill::UltraViolence),
            "Nightmare" => Ok(Skill::Nightmare),
            _ => Err(format!("{} is not a valid skill", input)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SourcePortType {
    PrBoom,
    GzDoom,
    DoomRetro,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcePort {
    pub name: String,
    pub path: PathBuf,
    pub version: String,
}

impl SourcePort {
    pub fn new(name: &str, path: PathBuf, version: &str) -> Result<SourcePort, Report> {
        ensure!(!name.is_empty(), "The name of the source port must be set");
        ensure!(
            name.len() <= 20,
            "The name of the source port cannot exceed 20 characters"
        );
        ensure!(
            path.is_file(),
            "The source port must point to a valid exe file"
        );
        ensure!(
            !version.is_empty(),
            "The version of the source port must be set"
        );
        Ok(SourcePort {
            name: name.to_string(),
            path,
            version: version.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SourcePort;
    use assert_fs::prelude::*;

    #[test]
    fn constructor_should_set_fields_correctly() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        temp.write_binary(b"fake source port code").unwrap();
        let sp = SourcePort::new("prboom", temp.path().to_path_buf(), "2.6").unwrap();
        assert_eq!(sp.name, "prboom");
        assert_eq!(sp.path.to_str().unwrap(), temp.path().to_str().unwrap());
        assert_eq!(sp.version, "2.6");
    }

    #[test]
    fn constructor_should_return_error_for_name_not_set() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        temp.write_binary(b"fake source port code").unwrap();
        let sp = SourcePort::new("", temp.path().to_path_buf(), "2.6");
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The name of the source port must be set"
        );
    }

    #[test]
    fn constructor_should_return_error_for_name_greater_than_20_chars() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        temp.write_binary(b"fake source port code").unwrap();
        let sp = SourcePort::new(
            "prboomprboomprboomprboomprboomprboom",
            temp.path().to_path_buf(),
            "2.6",
        );
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The name of the source port cannot exceed 20 characters"
        );
    }

    #[test]
    fn constructor_should_return_error_if_path_does_not_exist() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        let sp = SourcePort::new("prboom", temp.path().to_path_buf(), "2.6");
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The source port must point to a valid exe file"
        );
    }

    #[test]
    fn constructor_should_return_error_if_path_is_not_a_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let sp = SourcePort::new("prboom", temp.path().to_path_buf(), "2.6");
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The source port must point to a valid exe file"
        );
    }

    #[test]
    fn constructor_should_return_error_for_version_not_set() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        temp.write_binary(b"fake source port code").unwrap();
        let sp = SourcePort::new("prboom", temp.path().to_path_buf(), "");
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The version of the source port must be set"
        );
    }
}
