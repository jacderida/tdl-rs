use crate::profile::Profile;
use crate::source_port::SourcePort;
use color_eyre::{eyre::ensure, eyre::eyre, Report, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub iwads_path: PathBuf,
    pub wads_path: PathBuf,
    pub source_ports_path: PathBuf,
}

impl UserSettings {
    pub fn set_from_doom_home() -> Result<UserSettings, Report> {
        let path_var = std::env::var("TDL_DOOM_HOME_PATH")?;
        let doom_home_path = Path::new(&path_var);
        let iwad_pb = doom_home_path.join("iwads");
        if !iwad_pb.exists() {
            std::fs::create_dir_all(iwad_pb.as_path())?;
        }
        let wad_pb = doom_home_path.join("wads");
        if !wad_pb.exists() {
            std::fs::create_dir_all(wad_pb.as_path())?;
        }
        let sp_pb = doom_home_path.join("source-ports");
        if !sp_pb.exists() {
            std::fs::create_dir_all(sp_pb.as_path())?;
        }
        Ok(UserSettings {
            iwads_path: iwad_pb,
            wads_path: wad_pb,
            source_ports_path: sp_pb,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsRegistry {
    pub source_ports: Vec<SourcePort>,
    pub profiles: Vec<Profile>,
}

pub trait SettingsRepository {
    fn get(&self) -> Result<SettingsRegistry, Report>;
    fn save(&self, settings: SettingsRegistry) -> Result<(), Report>;
}

pub struct FileSettingsRepository {
    settings_path: PathBuf,
}

impl FileSettingsRepository {
    pub fn new(settings_path: PathBuf) -> Result<FileSettingsRepository, Report> {
        if !settings_path.exists() {
            std::fs::write(
                settings_path.to_str().unwrap(),
                r#"{"source_ports":[], "profiles": []}"#,
            )?;
        }
        Ok(FileSettingsRepository { settings_path })
    }
}

impl SettingsRepository for FileSettingsRepository {
    fn get(&self) -> Result<SettingsRegistry, Report> {
        let path = self
            .settings_path
            .to_owned()
            .into_os_string()
            .into_string()
            .map_err(|e| eyre!(format!("Could not convert {:?} to string", e)))?;
        let serialized = std::fs::read_to_string(&path)?;
        let settings: SettingsRegistry = serde_json::from_str(&serialized)?;
        Ok(settings)
    }

    fn save(&self, settings: SettingsRegistry) -> Result<(), Report> {
        let serialized = serde_json::to_string(&settings)?;
        std::fs::write(self.settings_path.to_str().unwrap(), serialized)?;
        Ok(())
    }
}

/// Gets the location of the app settings directory.
///
/// If the `TDL_SETTINGS_PATH` environment variable is defined, that will be used. Otherwise, the
/// directory will be $HOME/.config/tdl on Linux.
///
/// Windows support has still yet to be implemented.
///
/// If the path that either of these point to doesn't exist, the directory will be created.
///
/// ## Errors
///
/// If `TDL_SETTINGS_PATH` points to an existing file.
pub fn get_app_settings_dir_path() -> Result<PathBuf, Report> {
    let mut home_path = dirs::home_dir().unwrap();
    home_path.push(".config");
    home_path.push("tdl");
    let result = std::env::var("TDL_SETTINGS_PATH").map(PathBuf::from);
    let pb = result.unwrap_or(home_path);
    ensure!(
        !pb.is_file(),
        "The settings path cannot point to an existing file. \
        It must be either an existing directory or a path that does not exist."
    );
    if !pb.exists() {
        std::fs::create_dir_all(pb.as_path())?;
    }
    Ok(pb)
}

/// Retrieves the user settings.
///
/// There will be 3 different mechanisms for getting the user settings:
/// * If the `TDL_DOOM_HOME_PATH` is set, the various paths available in the user settings will be
/// set automatically, relative to that directory.
/// * Look for environment variables for each of the settings.
/// * Read them from a file located at `TDL_SETTINGS_PATH/user_settings.json`
pub fn get_user_settings() -> Result<UserSettings, Report> {
    let settings;
    if std::env::var("TDL_DOOM_HOME_PATH").is_ok() {
        settings = UserSettings::set_from_doom_home()?;
    } else {
        panic!("Alternative method for retrieving user settings is not yet supported");
    }
    Ok(settings)
}

#[cfg(test)]
mod app_settings_repo {
    use super::FileSettingsRepository;
    use super::SettingsRegistry;
    use super::SettingsRepository;
    use super::SourcePort;
    use crate::source_port::SourcePortType;
    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn constructor_should_set_fields_correctly() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        settings_file.write_str("json document").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        assert_eq!(
            repo.settings_path.to_str().unwrap(),
            settings_file.path().to_str().unwrap()
        );
    }

    #[test]
    fn constructor_should_create_empty_json_object_if_settings_file_does_not_exist() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let _ = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        settings_file.assert(predicate::path::exists());
        let settings_contents =
            std::fs::read_to_string(settings_file.path().to_str().unwrap()).unwrap();
        assert_eq!(settings_contents, r#"{"source_ports":[], "profiles": []}"#);
    }

    #[test]
    fn save_should_serialize_the_settings_file_to_json() {
        let sp_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        sp_exe.write_binary(b"fake source port code").unwrap();
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort::new(
                SourcePortType::PrBoom,
                sp_exe.to_path_buf(),
                "2.6",
            )
            .unwrap()],
            profiles: Vec::new(),
        };
        let serialized_settings = serde_json::to_string(&settings).unwrap();

        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let _ = repo.save(settings);

        settings_file.assert(predicate::path::exists());
        let settings_contents =
            std::fs::read_to_string(settings_file.path().to_str().unwrap()).unwrap();
        assert_eq!(settings_contents, serialized_settings);
    }

    #[test]
    fn get_should_deserialize_the_settings_file_and_return_the_settings_registry() {
        let sp_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        sp_exe.write_binary(b"fake source port code").unwrap();
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort::new(
                SourcePortType::PrBoom,
                sp_exe.to_path_buf(),
                "2.6",
            )
            .unwrap()],
            profiles: Vec::new(),
        };
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let _ = repo.save(settings);

        let retrieved_settings = repo.get().unwrap();

        settings_file.assert(predicate::path::exists());
        let sp = &retrieved_settings.source_ports[0];
        matches!(sp.source_port_type, SourcePortType::PrBoom);
        assert_eq!(sp.path.to_str().unwrap(), sp_exe.path().to_str().unwrap());
        assert_eq!(sp.version, "2.6");
    }

    #[test]
    fn get_should_return_an_empty_initialised_settings_registry_if_no_settings_have_yet_been_saved()
    {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let retrieved_settings = repo.get().unwrap();
        assert_eq!(retrieved_settings.source_ports.len(), 0);
    }
}

#[cfg(test)]
mod user_settings {
    use super::UserSettings;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::env::set_var;

    #[test]
    fn set_from_doom_home_should_set_fields_correctly() {
        let doom_home = assert_fs::TempDir::new().unwrap();
        let iwads_dir = doom_home.child("iwads");
        let wads_dir = doom_home.child("wads");
        let source_ports_dir = doom_home.child("source-ports");
        set_var("TDL_DOOM_HOME_PATH", doom_home.path().to_str().unwrap());
        let _ = UserSettings::set_from_doom_home().unwrap();
        iwads_dir.assert(predicate::path::is_dir());
        wads_dir.assert(predicate::path::is_dir());
        source_ports_dir.assert(predicate::path::is_dir());
    }

    #[test]
    fn set_from_doom_home_should_create_directories_if_they_do_not_exist() {
        let doom_home = assert_fs::TempDir::new().unwrap();
        let iwads_dir = doom_home.child("iwads");
        let wads_dir = doom_home.child("wads");
        let source_ports_dir = doom_home.child("source-ports");
        set_var("TDL_DOOM_HOME_PATH", doom_home.path().to_str().unwrap());
        let _ = UserSettings::set_from_doom_home().unwrap();
        iwads_dir.assert(predicate::path::is_dir());
        wads_dir.assert(predicate::path::is_dir());
        source_ports_dir.assert(predicate::path::is_dir());
    }
}

/// These tests won't test the use of the home directory, since it would
/// involve creating and removing real directories on my development machine.
///
/// This code can be tested using an integration test, or be one of the very few parts of the
/// application that won't have code coverage.
#[cfg(test)]
mod get_app_settings_path {
    use super::get_app_settings_dir_path;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::env::set_var;

    #[test]
    fn get_app_settings_dir_path_returns_env_var_value() {
        let app_settings_dir = assert_fs::TempDir::new().unwrap();
        set_var(
            "TDL_SETTINGS_PATH",
            app_settings_dir.path().to_str().unwrap(),
        );

        let app_settings_path = get_app_settings_dir_path().unwrap();
        assert_eq!(app_settings_path.as_path(), app_settings_dir.path());
    }

    #[test]
    fn get_app_settings_dir_path_creates_directory_if_it_does_not_exist() {
        let app_settings_dir = assert_fs::TempDir::new().unwrap();
        let child = app_settings_dir.child("tdl");
        set_var("TDL_SETTINGS_PATH", child.path().to_str().unwrap());

        let _ = get_app_settings_dir_path().unwrap();
        child.assert(predicate::path::is_dir());
    }

    #[test]
    #[ignore = "This test sometimes randomly fails. Apparently, somtimes the temporary file is *not* actually a file. Cannot figure out why."]
    fn get_app_settings_dir_path_ensures_value_is_not_a_file() {
        let app_settings_dir = assert_fs::TempDir::new().unwrap();
        let child = app_settings_dir.child("tdl");
        child.write_str("existing file").unwrap();
        set_var("TDL_SETTINGS_PATH", child.path().to_str().unwrap());

        let result = get_app_settings_dir_path();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The settings path cannot point to an existing file. \
            It must be either an existing directory or a path that does not exist."
        );
    }
}

#[cfg(test)]
mod get_user_settings {
    use super::get_user_settings;
    use assert_fs::prelude::*;
    use std::env::set_var;

    #[test]
    fn get_user_settings_should_set_paths_relative_to_doom_home_if_env_var_is_set() {
        let doom_home_dir = assert_fs::TempDir::new().unwrap();
        let iwads_dir = doom_home_dir.child("iwads");
        let wads_dir = doom_home_dir.child("wads");
        let source_ports_dir = doom_home_dir.child("source-ports");
        set_var("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap());

        let user_settings = get_user_settings().unwrap();
        assert_eq!(user_settings.iwads_path.as_path(), iwads_dir.path());
        assert_eq!(user_settings.wads_path.as_path(), wads_dir.path());
        assert_eq!(
            user_settings.source_ports_path.as_path(),
            source_ports_dir.path()
        );
    }
}
