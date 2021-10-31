use crate::storage::{ObjectRepository, StorageError};
use chrono::DateTime;
#[cfg(not(test))]
use chrono::Utc;
use color_eyre::{eyre::ensure, Report, Result};
use lazy_static::lazy_static;
use log::{debug, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use strum_macros::EnumIter;
#[cfg(test)]
use test_helpers::FakeUtc as Utc;
use thiserror::Error;

const GITHUB_API_URL: &str = "https://api.github.com";
lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(r#"\d+(\.\d+|[a-z]+)+"#).unwrap();
    static ref RELEASE_ASSET_MAP: HashMap<&'static str, Regex> = {
        maplit::hashmap! {
            "chocolate-doom/chocolate-doom/windows" => Regex::new("chocolate-doom-.*-win32.zip").unwrap(),
            "chocolate-doom/chocolate-doom/macos" => Regex::new("chocolate-doom-.*.dmg").unwrap(),
            "fabiangreffrath/crispy-doom/windows" => Regex::new("crispy-doom-.*-win32.zip").unwrap(),
            "bradharding/doomretro/windows" => Regex::new("doomretro-.*-win64.zip").unwrap(),
            "team-eternity/eternity/windows" => Regex::new("ee-.*-win64.zip").unwrap(),
            "team-eternity/eternity/macos" => Regex::new("ee-.*-macos.dmg").unwrap(),
            "coelckers/gzdoom/windows" => Regex::new("gzdoom-.*Windows-64bit.zip").unwrap(),
            "coelckers/gzdoom/macos" => Regex::new("gzdoom-.*macOS.zip").unwrap(),
            "drfrag666/gzdoom/windows" => Regex::new("LZDoom_.*_x64.zip").unwrap(),
            "drfrag666/gzdoom/macos" => Regex::new("LZDoom_.*_macOS.zip").unwrap(),
            "odamex/odamex/windows" => Regex::new("odamex-win64-.*zip").unwrap(),
            "odamex/odamex/macos" => Regex::new("odamex-macos-.*dmg").unwrap(),
            "coelckers/prboom-plus/windows" => Regex::new("prboom-plus-.*-w32.zip").unwrap(),
            "fabiangreffrath/woof/windows" => Regex::new("Woof-.*-win32.zip").unwrap(),
        }
    };
    static ref SOURCE_PORT_OWNERS_MAP: HashMap<SourcePort, (&'static str, &'static str)> = {
        maplit::hashmap! {
            SourcePort::Chocolate => ("chocolate-doom", "chocolate-doom"),
            SourcePort::Crispy => ("fabiangreffrath", "crispy-doom"),
            SourcePort::DoomRetro => ("bradharding", "doomretro"),
            SourcePort::Dsda => ("kraflab", "dsda-doom"),
            SourcePort::EternityEngine => ("team-eternity", "eternity"),
            SourcePort::GzDoom => ("coelckers", "gzdoom"),
            SourcePort::LzDoom => ("drfrag666", "gzdoom"),
            SourcePort::Odamex => ("odamex", "odamex"),
            SourcePort::PrBoomPlus => ("coelckers", "prboom-plus"),
            SourcePort::Rude => ("drfrag666", "RUDE"),
            SourcePort::Woof => ("fabiangreffrath", "woof"),
            SourcePort::Zandronum => ("TorrSamaho", "zandronum"),
        }
    };
}

#[derive(Debug, Error)]
pub enum SourcePortError {
    #[error("The source port {0} has no releases marked as latest")]
    NoLatestRelease(SourcePort),
    #[error("Could not parse version number from {0} for {1} source port")]
    VersionParsing(String, String),
    #[error("Failed to retrieve release request response from Github API")]
    GithubApiResponseError(#[from] reqwest::Error),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

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

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SourcePort {
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

impl FromStr for SourcePort {
    type Err = String;

    fn from_str(input: &str) -> Result<SourcePort, Self::Err> {
        match input {
            "Chocolate" => Ok(SourcePort::Chocolate),
            "Crispy" => Ok(SourcePort::Crispy),
            "DoomRetro" => Ok(SourcePort::DoomRetro),
            "Dsda" => Ok(SourcePort::Dsda),
            "EternityEngine" => Ok(SourcePort::EternityEngine),
            "GzDoom" => Ok(SourcePort::GzDoom),
            "LzDoom" => Ok(SourcePort::LzDoom),
            "Odamex" => Ok(SourcePort::Odamex),
            "PrBoomPlus" => Ok(SourcePort::PrBoomPlus),
            "Rude" => Ok(SourcePort::PrBoomPlus),
            "Woof" => Ok(SourcePort::Woof),
            "Zandronum" => Ok(SourcePort::Zandronum),
            _ => Err(format!("{} is not a supported source port", input)),
        }
    }
}

impl std::fmt::Display for SourcePort {
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
    pub name: SourcePort,
    pub path: PathBuf,
    pub version: String,
}

impl InstalledSourcePort {
    pub fn new(
        name: SourcePort,
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
            name,
            path,
            version: version.to_string(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcePortRelease {
    pub source_port: SourcePort,
    pub owner: String,
    pub repository: String,
    pub version: String,
    pub assets: Vec<(String, String)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedSourcePortRelease {
    pub cached_date: DateTime<chrono::Utc>,
    pub release: SourcePortRelease,
}

pub trait ReleaseRepository {
    fn get_latest_release(
        &self,
        source_port: SourcePort,
    ) -> Result<SourcePortRelease, SourcePortError>;
    fn get_releases(
        &self,
        source_port: SourcePort,
    ) -> Result<Vec<SourcePortRelease>, SourcePortError>;
}

pub struct GithubReleaseRepository {
    pub api_base_url: String,
}

impl GithubReleaseRepository {
    pub fn new() -> GithubReleaseRepository {
        GithubReleaseRepository {
            api_base_url: String::from(GITHUB_API_URL),
        }
    }
}

impl ReleaseRepository for GithubReleaseRepository {
    fn get_latest_release(
        &self,
        source_port: SourcePort,
    ) -> Result<SourcePortRelease, SourcePortError> {
        let (owner, repository) = SOURCE_PORT_OWNERS_MAP.get(&source_port).unwrap();
        info!("Getting latest version for {}/{}", owner, repository);
        let latest_release_url = format!(
            "{}/repos/{}/{}/releases/latest",
            self.api_base_url, owner, repository
        );
        let response = reqwest::blocking::Client::new()
            .get(latest_release_url)
            .header(
                reqwest::header::USER_AGENT,
                format!("tdl {}", get_current_tdl_version()),
            )
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .send()?;
        let response_json = response.json::<serde_json::Value>()?;
        get_latest_source_port_release_from_response(source_port, &response_json)
    }

    fn get_releases(
        &self,
        source_port: SourcePort,
    ) -> Result<Vec<SourcePortRelease>, SourcePortError> {
        Ok(Vec::new())
    }
}

fn get_latest_source_port_release_from_response(
    source_port: SourcePort,
    response: &Value,
) -> Result<SourcePortRelease, SourcePortError> {
    let tag = response["tag_name"].as_str();
    if tag.is_none() {
        return Err(SourcePortError::NoLatestRelease(source_port));
    }
    let tag = tag.unwrap();
    let (owner, repository) = SOURCE_PORT_OWNERS_MAP.get(&source_port).unwrap();
    let version = if let Some(regex_match) = VERSION_REGEX.find(tag) {
        regex_match.as_str()
    } else {
        return Err(SourcePortError::VersionParsing(
            tag.to_string(),
            source_port.to_string(),
        ));
    };

    let mut release_assets = Vec::new();
    let assets = response["assets"].as_array().unwrap();
    for platform in vec!["windows", "linux", "macos"].iter() {
        if let Some(asset_regex) =
            RELEASE_ASSET_MAP.get(format!("{}/{}/{}", owner, repository, *platform).as_str())
        {
            let asset = assets
                .iter()
                .find(|v| asset_regex.is_match(v["name"].as_str().unwrap()))
                .map(|v| {
                    (
                        String::from(*platform),
                        String::from(v["browser_download_url"].as_str().unwrap()),
                    )
                });
            if let Some(a) = asset {
                release_assets.push(a);
            }
        }
    }

    Ok(SourcePortRelease {
        source_port,
        owner: String::from(*owner),
        repository: String::from(*repository),
        version: String::from(version),
        assets: release_assets,
    })
}

pub fn get_latest_source_port_release(
    source_port: SourcePort,
    release_repository: &impl ReleaseRepository,
    object_repository: &ObjectRepository,
) -> Result<SourcePortRelease, SourcePortError> {
    let (owner, repository) = SOURCE_PORT_OWNERS_MAP.get(&source_port).unwrap();
    let id = format!("{}.{}.latest", owner, repository.to_lowercase());
    let cache_result: Result<CachedSourcePortRelease, StorageError> = object_repository.get(&id);
    if let Ok(cache_entry) = cache_result {
        debug!("Github release cache has entry for {}", id);
        let duration = Utc::now() - cache_entry.cached_date;
        if duration.num_hours() < 24 {
            debug!(
                "Cache entry is {} old so another Github API call will be avoided",
                duration.num_hours()
            );
            if cache_entry.release.version == "no_latest_release" {
                return Err(SourcePortError::NoLatestRelease(source_port));
            }
            return Ok(cache_entry.release);
        }
        debug!("Cache entry is older than 24 hours so it will be deleted");
        object_repository.delete(&id)?;
    }

    debug!("No cached entry for {} so Github will be queried...", id);
    match release_repository.get_latest_release(source_port) {
        Ok(latest_release) => {
            let cache_entry = CachedSourcePortRelease {
                release: latest_release.clone(),
                cached_date: Utc::now(),
            };
            object_repository.save(&id, &cache_entry)?;
            Ok(latest_release)
        }
        Err(error) => {
            debug!(
                "Error retrieving latest release for {}: {}",
                source_port, error
            );
            match error {
                SourcePortError::NoLatestRelease(_) => {
                    let cache_missing_release = CachedSourcePortRelease {
                        cached_date: Utc::now(),
                        release: SourcePortRelease {
                            source_port,
                            owner: String::from(*owner),
                            repository: String::from(*repository),
                            version: String::from("no_latest_release"),
                            assets: Vec::new(),
                        },
                    };
                    object_repository.save(&id, &cache_missing_release)?;
                }
                _ => {}
            }
            // We want to just return back whatever the original error was, including if it was the
            // missing latest release error.
            Err(error)
        }
    }
}

fn get_current_tdl_version() -> String {
    format!(
        "{}.{}.{}",
        pkg_version::pkg_version_major!(),
        pkg_version::pkg_version_minor!(),
        pkg_version::pkg_version_patch!()
    )
}

#[cfg(test)]
pub mod test {
    use super::{
        get_latest_source_port_release_from_response, ReleaseRepository, SourcePort,
        SourcePortError, SourcePortRelease, SOURCE_PORT_OWNERS_MAP,
    };
    use serde_json::Value;
    use std::path::{Path, PathBuf};

    pub struct FakeReleaseRepository {
        pub response_directory: PathBuf,
    }

    impl ReleaseRepository for FakeReleaseRepository {
        fn get_releases(
            &self,
            source_port: SourcePort,
        ) -> Result<Vec<SourcePortRelease>, SourcePortError> {
            Ok(Vec::new())
        }

        fn get_latest_release(
            &self,
            source_port: SourcePort,
        ) -> Result<SourcePortRelease, SourcePortError> {
            let (owner, repository) = SOURCE_PORT_OWNERS_MAP.get(&source_port).unwrap();
            let cached_github_response_path = Path::new(&self.response_directory).join(format!(
                "{}.{}.latest.json",
                owner,
                repository.to_lowercase()
            ));
            let github_response = std::fs::read_to_string(cached_github_response_path)?;
            let github_response_json: Value = serde_json::from_str(&github_response)?;
            get_latest_source_port_release_from_response(source_port, &github_response_json)
        }
    }
}

#[cfg(test)]
mod get_latest_source_port_release_from_response {
    use super::{get_latest_source_port_release_from_response, SourcePort, SourcePortError};
    use serde_json::Value;
    use std::path::Path;

    #[test]
    fn should_return_windows_and_linux_assets_for_chocolate_doom() {
        let response_path = Path::new(
            "resources/test_data/github_responses/chocolate-doom.chocolate-doom.latest.json",
        );
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::Chocolate, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "chocolate-doom");
        assert_eq!(release.repository, "chocolate-doom");
        assert_eq!(release.version, "3.0.0");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(release.assets[0].1, "https://github.com/chocolate-doom/chocolate-doom/releases/download/chocolate-doom-3.0.0/chocolate-doom-3.0.0-win32.zip");
        assert_eq!(release.assets[1].0, "macos");
        assert_eq!(release.assets[1].1, "https://github.com/chocolate-doom/chocolate-doom/releases/download/chocolate-doom-3.0.0/chocolate-doom-3.0.0.dmg");
    }

    #[test]
    fn should_return_a_windows_asset_for_crispy_doom() {
        let response_path = Path::new(
            "resources/test_data/github_responses/fabiangreffrath.crispy-doom.latest.json",
        );
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::Crispy, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "fabiangreffrath");
        assert_eq!(release.repository, "crispy-doom");
        assert_eq!(release.version, "5.10.3");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(release.assets[0].1, "https://github.com/fabiangreffrath/crispy-doom/releases/download/crispy-doom-5.10.3/crispy-doom-5.10.3-win32.zip");
    }

    #[test]
    fn should_return_a_windows_asset_for_doom_retro() {
        let response_path =
            Path::new("resources/test_data/github_responses/bradharding.doomretro.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::DoomRetro, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "bradharding");
        assert_eq!(release.repository, "doomretro");
        assert_eq!(release.version, "4.2.3");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(release.assets[0].1, "https://github.com/bradharding/doomretro/releases/download/v4.2.3/doomretro-4.2.3-win64.zip");
    }

    #[test]
    fn should_return_no_assets_for_dsda_doom() {
        let response_path =
            Path::new("resources/test_data/github_responses/kraflab.dsda-doom.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result = get_latest_source_port_release_from_response(SourcePort::Dsda, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "kraflab");
        assert_eq!(release.repository, "dsda-doom");
        assert_eq!(release.version, "0.21.3");
        assert_eq!(release.assets.len(), 0);
    }

    #[test]
    fn should_return_windows_and_macos_asset_for_eternity_engine() {
        let response_path =
            Path::new("resources/test_data/github_responses/team-eternity.eternity.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result = get_latest_source_port_release_from_response(
            SourcePort::EternityEngine,
            &response_json,
        );

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "team-eternity");
        assert_eq!(release.repository, "eternity");
        assert_eq!(release.version, "4.02.00");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(release.assets[0].1, "https://github.com/team-eternity/eternity/releases/download/4.02.00/ee-4.02.00-win64.zip");
        assert_eq!(release.assets[1].0, "macos");
        assert_eq!(release.assets[1].1, "https://github.com/team-eternity/eternity/releases/download/4.02.00/ee-4.02.00-macos.dmg");
    }

    #[test]
    fn should_return_windows_and_macos_asset_for_gzdoom() {
        let response_path =
            Path::new("resources/test_data/github_responses/coelckers.gzdoom.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::GzDoom, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "coelckers");
        assert_eq!(release.repository, "gzdoom");
        assert_eq!(release.version, "4.7.0");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(release.assets[0].1, "https://github.com/coelckers/gzdoom/releases/download/g4.7.0/gzdoom-4-7-0-Windows-64bit.zip");
        assert_eq!(release.assets[1].0, "macos");
        assert_eq!(
            release.assets[1].1,
            "https://github.com/coelckers/gzdoom/releases/download/g4.7.0/gzdoom-4-7-0-macOS.zip"
        );
    }

    #[test]
    fn should_return_windows_and_macos_asset_for_lzdoom() {
        let response_path =
            Path::new("resources/test_data/github_responses/drfrag666.gzdoom.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::LzDoom, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "drfrag666");
        assert_eq!(release.repository, "gzdoom");
        assert_eq!(release.version, "3.88a");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(
            release.assets[0].1,
            "https://github.com/drfrag666/gzdoom/releases/download/3.88a/LZDoom_3.88a_x64.zip"
        );
        assert_eq!(release.assets[1].0, "macos");
        assert_eq!(
            release.assets[1].1,
            "https://github.com/drfrag666/gzdoom/releases/download/3.88a/LZDoom_3.88a_macOS.zip"
        );
    }

    #[test]
    fn should_return_windows_and_macos_asset_for_odamex() {
        let response_path =
            Path::new("resources/test_data/github_responses/odamex.odamex.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::Odamex, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "odamex");
        assert_eq!(release.repository, "odamex");
        assert_eq!(release.version, "0.9.5");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(
            release.assets[0].1,
            "https://github.com/odamex/odamex/releases/download/0.9.5/odamex-win64-0.9.5.zip"
        );
        assert_eq!(release.assets[1].0, "macos");
        assert_eq!(
            release.assets[1].1,
            "https://github.com/odamex/odamex/releases/download/0.9.5/odamex-macos-0.9.5.dmg"
        );
    }

    #[test]
    fn should_return_a_windows_asset_for_prboom_plus() {
        let response_path =
            Path::new("resources/test_data/github_responses/coelckers.prboom-plus.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result =
            get_latest_source_port_release_from_response(SourcePort::PrBoomPlus, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "coelckers");
        assert_eq!(release.repository, "prboom-plus");
        assert_eq!(release.version, "2.6.1um");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(
            release.assets[0].1,
            "https://github.com/coelckers/prboom-plus/releases/download/v2.6.1um/prboom-plus-261um-w32.zip"
        );
    }

    #[test]
    fn should_return_a_windows_asset_for_woof() {
        let response_path =
            Path::new("resources/test_data/github_responses/fabiangreffrath.woof.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result = get_latest_source_port_release_from_response(SourcePort::Woof, &response_json);

        assert!(result.is_ok());
        let release = result.unwrap();
        assert_eq!(release.owner, "fabiangreffrath");
        assert_eq!(release.repository, "woof");
        assert_eq!(release.version, "7.0.0");
        assert_eq!(release.assets[0].0, "windows");
        assert_eq!(
            release.assets[0].1,
            "https://github.com/fabiangreffrath/woof/releases/download/woof_7.0.0/Woof-7.0.0-win32.zip"
        );
    }

    #[test]
    fn should_return_an_error_for_source_port_with_no_latest_release_marked() {
        let response_path =
            Path::new("resources/test_data/github_responses/drfrag666.rude.latest.json");
        let response = std::fs::read_to_string(response_path).unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();

        let result = get_latest_source_port_release_from_response(SourcePort::Rude, &response_json);

        assert!(result.is_err());
        let error = result.unwrap_err();
        matches!(error, SourcePortError::NoLatestRelease(_));
        assert_eq!(
            error.to_string(),
            "The source port RUDE has no releases marked as latest"
        );
    }
}

#[cfg(test)]
mod get_latest_source_port_version {
    use super::test::FakeReleaseRepository;
    use super::{get_latest_source_port_release, CachedSourcePortRelease, SourcePortError};
    use crate::source_port::SourcePort;
    use crate::storage::ObjectRepository;
    use assert_fs::prelude::*;
    use chrono::{Datelike, Duration, TimeZone, Timelike, Utc};
    use test_helpers::FakeUtc;

    #[test]
    fn should_get_correct_version() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Chocolate,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_ok());
        let source_port = result.unwrap();
        assert_eq!(source_port.version, "3.0.0");
    }

    #[test]
    fn should_create_a_cache_entry_for_retrieved_version() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let chocolate_doom_cache_entry =
            github_cache_dir.child("chocolate-doom.chocolate-doom.latest.json");
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Chocolate,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_ok());
        chocolate_doom_cache_entry.assert(predicates::path::is_file());
    }

    #[test]
    fn should_create_a_cache_entry_with_correct_date_time_stamp() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let chocolate_doom_cache_entry =
            github_cache_dir.child("chocolate-doom.chocolate-doom.latest.json");
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Chocolate,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_ok());
        chocolate_doom_cache_entry.assert(predicates::path::is_file());

        // Testing to the resolution of an hour is good enough for this test.
        let dt = Utc::now();
        let cache_entry: CachedSourcePortRelease = object_repository
            .get("chocolate-doom.chocolate-doom.latest")
            .unwrap();
        assert_eq!(dt.year(), cache_entry.cached_date.year());
        assert_eq!(dt.month(), cache_entry.cached_date.month());
        assert_eq!(dt.day(), cache_entry.cached_date.day());
        assert_eq!(dt.hour(), cache_entry.cached_date.hour());
    }

    #[test]
    fn should_return_the_existing_cache_entry() {
        // This date and time is completely arbitrary. It's just assigned so that the test isn't
        // dependent on the current date or any timezone issues and such.
        let dt = chrono::prelude::Utc.ymd(2021, 10, 1).and_hms(10, 10, 10);
        FakeUtc::set_date_time(dt).unwrap();
        let serialized_cache_entry = r#"
        {
            "cached_date": "__DATE__",
            "release": {
                "source_port": "Crispy",
                "owner": "fabiangreffrath",
                "repository": "crispy-doom",
                "version": "5.10.3",
                "assets": [
                    [
                        "windows",
                        "https://github.com/fabiangreffrath/crispy-doom/releases/download/crispy-doom-5.10.3/crispy-doom-5.10.3-win32.zip"
                    ]
                ]
            }
        }"#;
        let serialized_cache_entry = serialized_cache_entry.replace("__DATE__", &dt.to_string());

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let chocolate_doom_cache_entry =
            github_cache_dir.child("fabiangreffrath.crispy-doom.latest.json");
        chocolate_doom_cache_entry
            .write_str(&serialized_cache_entry)
            .unwrap();
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Crispy,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_ok());
        let source_port = result.unwrap();
        assert_eq!(source_port.version, "5.10.3");
    }

    #[test]
    fn should_not_use_stale_cache_entry() {
        // This date and time is completely arbitrary. It's just assigned so that the test isn't
        // dependent on the current date or any timezone issues and such.
        let dt = chrono::prelude::Utc.ymd(2021, 10, 1).and_hms(10, 10, 10);
        FakeUtc::set_date_time(dt).unwrap();
        let dt = dt - Duration::hours(25);
        let serialized_stale_cache_entry = r#"
        {
            "cached_date": "__DATE__",
            "release": {
                "source_port": "Crispy",
                "owner": "fabiangreffrath",
                "repository": "crispy-doom",
                "version": "5.10.2",
                "assets": [
                    [
                        "windows",
                        "https://github.com/fabiangreffrath/crispy-doom/releases/download/crispy-doom-5.10.3/crispy-doom-5.10.3-win32.zip"
                    ]
                ]
            }
        }"#;
        let serialized_stale_cache_entry =
            serialized_stale_cache_entry.replace("__DATE__", &dt.to_string());

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let chocolate_doom_cache_entry =
            github_cache_dir.child("fabiangreffrath.crispy-doom.latest.json");
        chocolate_doom_cache_entry
            .write_str(&serialized_stale_cache_entry)
            .unwrap();
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Crispy,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_ok());
        let source_port = result.unwrap();
        assert_eq!(source_port.version, "5.10.3");
    }

    #[test]
    fn should_return_an_error_for_no_release_marked_latest() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Rude,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_err());
        matches!(
            result.unwrap_err(),
            SourcePortError::NoLatestRelease(SourcePort::Rude)
        );
    }

    #[test]
    fn should_return_error_and_cache_source_port_with_no_latest_release() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let rude_cache_entry = github_cache_dir.child("drfrag666.rude.latest.json");
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Rude,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_err());
        matches!(
            result.unwrap_err(),
            SourcePortError::NoLatestRelease(SourcePort::Rude)
        );
        rude_cache_entry.assert(predicates::path::is_file());
    }

    #[test]
    fn should_return_error_when_missing_release_is_cached() {
        // This date and time is completely arbitrary. It's just assigned so that the test isn't
        // dependent on the current date or any timezone issues and such.
        let dt = chrono::prelude::Utc.ymd(2021, 10, 1).and_hms(10, 10, 10);
        FakeUtc::set_date_time(dt).unwrap();
        let serialized_cache_entry = r#"
        {
            "cached_date": "__DATE__",
            "release": {
                "source_port": "Rude",
                "owner": "drfrag666",
                "repository": "rude",
                "version": "no_latest_release",
                "assets": []
            }
        }"#;
        let serialized_cache_entry = serialized_cache_entry.replace("__DATE__", &dt.to_string());

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let github_response_dir = temp_dir.child("github_responses");
        github_response_dir
            .copy_from("resources/test_data/github_responses", &["**"])
            .unwrap();
        let github_cache_dir = temp_dir.child("github_cache");
        let rude_cache_entry = github_cache_dir.child("drfrag666.rude.latest.json");
        rude_cache_entry.write_str(&serialized_cache_entry).unwrap();
        let release_repository = FakeReleaseRepository {
            response_directory: github_response_dir.to_path_buf(),
        };
        let object_repository = ObjectRepository::new(&github_cache_dir.to_path_buf()).unwrap();

        let result = get_latest_source_port_release(
            SourcePort::Rude,
            &release_repository,
            &object_repository,
        );

        assert!(result.is_err());
        matches!(
            result.unwrap_err(),
            SourcePortError::NoLatestRelease(SourcePort::Rude)
        );
    }
}

#[cfg(test)]
mod installed_source_port {
    mod new {
        use super::super::InstalledSourcePort;
        use super::super::SourcePort;
        use assert_fs::prelude::*;

        #[test]
        fn should_set_fields() {
            let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            temp.write_binary(b"fake source port code").unwrap();
            let sp =
                InstalledSourcePort::new(SourcePort::PrBoomPlus, temp.path().to_path_buf(), "2.6")
                    .unwrap();
            matches!(sp.name, SourcePort::PrBoomPlus);
            assert_eq!(sp.path.to_str().unwrap(), temp.path().to_str().unwrap());
            assert_eq!(sp.version, "2.6");
        }

        #[test]
        fn should_return_error_if_path_does_not_exist() {
            let temp = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            let sp =
                InstalledSourcePort::new(SourcePort::PrBoomPlus, temp.path().to_path_buf(), "2.6");
            assert!(sp.is_err());
            assert_eq!(
                sp.unwrap_err().to_string(),
                "The source port must point to a valid exe file"
            );
        }

        #[test]
        fn should_return_error_if_path_is_not_a_file() {
            let temp = assert_fs::TempDir::new().unwrap();
            let sp =
                InstalledSourcePort::new(SourcePort::PrBoomPlus, temp.path().to_path_buf(), "2.6");
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
                InstalledSourcePort::new(SourcePort::PrBoomPlus, temp.path().to_path_buf(), "");
            assert!(sp.is_err());
            assert_eq!(
                sp.unwrap_err().to_string(),
                "The version of the source port must be set"
            );
        }
    }
}

#[cfg(test)]
mod github_release_repository {
    mod new {
        use super::super::GithubReleaseRepository;
        use super::super::GITHUB_API_URL;

        #[test]
        fn should_set_the_api_base_url() {
            let repo = GithubReleaseRepository::new();
            assert_eq!(GITHUB_API_URL, repo.api_base_url);
        }
    }
}
