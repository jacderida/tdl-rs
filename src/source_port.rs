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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SourcePortType {
    PrBoom,
    PrBoomUmapInfo,
    Dsda,
    GzDoom,
    DoomRetro,
}

impl FromStr for SourcePortType {
    type Err = String;

    fn from_str(input: &str) -> Result<SourcePortType, Self::Err> {
        match input {
            "prboom" => Ok(SourcePortType::PrBoom),
            "prboomumapinfo" => Ok(SourcePortType::PrBoomUmapInfo),
            "dsda" => Ok(SourcePortType::Dsda),
            "gzdoom" => Ok(SourcePortType::GzDoom),
            "doomretro" => Ok(SourcePortType::DoomRetro),
            _ => Err(format!("{} is not a supported Source Port", input)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcePort {
    pub source_port_type: SourcePortType,
    pub path: PathBuf,
    pub version: String,
}

impl SourcePort {
    pub fn new(
        source_port_type: SourcePortType,
        path: PathBuf,
        version: &str,
    ) -> Result<SourcePort, Report> {
        ensure!(
            path.is_file(),
            "The source port must point to a valid exe file"
        );
        ensure!(
            !version.is_empty(),
            "The version of the source port must be set"
        );
        Ok(SourcePort {
            source_port_type,
            path,
            version: version.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SourcePort;
    use super::SourcePortType;
    use assert_fs::prelude::*;

    #[test]
    fn constructor_should_set_fields_correctly() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        temp.write_binary(b"fake source port code").unwrap();
        let sp = SourcePort::new(SourcePortType::PrBoom, temp.path().to_path_buf(), "2.6").unwrap();
        matches!(sp.source_port_type, SourcePortType::PrBoom);
        assert_eq!(sp.path.to_str().unwrap(), temp.path().to_str().unwrap());
        assert_eq!(sp.version, "2.6");
    }

    #[test]
    fn constructor_should_return_error_if_path_does_not_exist() {
        let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        let sp = SourcePort::new(SourcePortType::PrBoom, temp.path().to_path_buf(), "2.6");
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The source port must point to a valid exe file"
        );
    }

    #[test]
    fn constructor_should_return_error_if_path_is_not_a_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let sp = SourcePort::new(SourcePortType::PrBoom, temp.path().to_path_buf(), "2.6");
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
        let sp = SourcePort::new(SourcePortType::PrBoom, temp.path().to_path_buf(), "");
        assert!(sp.is_err());
        assert_eq!(
            sp.unwrap_err().to_string(),
            "The version of the source port must be set"
        );
    }
}
