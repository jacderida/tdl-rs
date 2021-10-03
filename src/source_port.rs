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
    Chocolate,
    Crispy,
    DoomRetro,
    Dsda,
    EternityEngine,
    GzDoom,
    LzDoom,
    Odamex,
    PrBoomPlus,
    Rude,
    Woof,
    Zandronum,
}

impl FromStr for SourcePortType {
    type Err = String;

    fn from_str(input: &str) -> Result<SourcePortType, Self::Err> {
        match input {
            "Chocolate" => Ok(SourcePortType::Chocolate),
            "Crispy" => Ok(SourcePortType::Crispy),
            "DoomRetro" => Ok(SourcePortType::DoomRetro),
            "Dsda" => Ok(SourcePortType::Dsda),
            "EternityEngine" => Ok(SourcePortType::EternityEngine),
            "GzDoom" => Ok(SourcePortType::GzDoom),
            "LzDoom" => Ok(SourcePortType::LzDoom),
            "Odamex" => Ok(SourcePortType::Odamex),
            "PrBoomPlus" => Ok(SourcePortType::PrBoomPlus),
            "Rude" => Ok(SourcePortType::PrBoomPlus),
            "Woof" => Ok(SourcePortType::Woof),
            "Zandronum" => Ok(SourcePortType::Zandronum),
            _ => Err(format!("{} is not a supported source port", input)),
        }
    }
}

impl std::fmt::Display for SourcePortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chocolate => write!(f, "Chocolate Doom"),
            Self::Crispy => write!(f, "Crispy Doom"),
            Self::DoomRetro => write!(f, "Doom Retro"),
            Self::Dsda => write!(f, "DSDA Doom"),
            Self::EternityEngine => write!(f, "Eternity Engine"),
            Self::GzDoom => write!(f, "GZDoom"),
            Self::LzDoom => write!(f, "LZDoom"),
            Self::Odamex => write!(f, "Odamex"),
            Self::PrBoomPlus => write!(f, "PrBoom Plus"),
            Self::Rude => write!(f, "RUDE"),
            Self::Woof => write!(f, "Woof!"),
            Self::Zandronum => write!(f, "Zandronum"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstalledSourcePort {
    pub source_port_type: SourcePortType,
    pub path: PathBuf,
    pub version: String,
}

impl InstalledSourcePort {
    pub fn new(
        source_port_type: SourcePortType,
        path: PathBuf,
        version: &str,
    ) -> Result<InstalledSourcePort, Report> {
        ensure!(
            path.is_file(),
            "The source port must point to a valid exe file"
        );
        ensure!(
            !version.is_empty(),
            "The version of the source port must be set"
        );
        Ok(InstalledSourcePort {
            source_port_type,
            path,
            version: version.to_string(),
        })
    }
}

#[cfg(test)]
mod installed_source_port {
    mod new {
        use super::super::InstalledSourcePort;
        use super::super::SourcePortType;
        use assert_fs::prelude::*;

        #[test]
        fn should_set_fields() {
            let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            temp.write_binary(b"fake source port code").unwrap();
            let sp = InstalledSourcePort::new(
                SourcePortType::PrBoomPlus,
                temp.path().to_path_buf(),
                "2.6",
            )
            .unwrap();
            matches!(sp.source_port_type, SourcePortType::PrBoomPlus);
            assert_eq!(sp.path.to_str().unwrap(), temp.path().to_str().unwrap());
            assert_eq!(sp.version, "2.6");
        }

        #[test]
        fn should_return_error_if_path_does_not_exist() {
            let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            let sp = InstalledSourcePort::new(
                SourcePortType::PrBoomPlus,
                temp.path().to_path_buf(),
                "2.6",
            );
            assert!(sp.is_err());
            assert_eq!(
                sp.unwrap_err().to_string(),
                "The source port must point to a valid exe file"
            );
        }

        #[test]
        fn should_return_error_if_path_is_not_a_file() {
            let temp = assert_fs::TempDir::new().unwrap();
            let sp = InstalledSourcePort::new(
                SourcePortType::PrBoomPlus,
                temp.path().to_path_buf(),
                "2.6",
            );
            assert!(sp.is_err());
            assert_eq!(
                sp.unwrap_err().to_string(),
                "The source port must point to a valid exe file"
            );
        }

        #[test]
        fn should_return_error_for_empty_version() {
            let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            temp.write_binary(b"fake source port code").unwrap();
            let sp =
                InstalledSourcePort::new(SourcePortType::PrBoomPlus, temp.path().to_path_buf(), "");
            assert!(sp.is_err());
            assert_eq!(
                sp.unwrap_err().to_string(),
                "The version of the source port must be set"
            );
        }
    }
}
