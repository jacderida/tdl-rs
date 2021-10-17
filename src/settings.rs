use crate::profile::Profile;
use crate::source_port::InstalledSourcePort;
#[cfg(target_family = "windows")]
use color_eyre::eyre::eyre;
use color_eyre::{eyre::ensure, Report, Result};
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
pub struct AppSettings {
    pub source_ports: Vec<InstalledSourcePort>,
    pub profiles: Vec<Profile>,
    pub release_cache_path: PathBuf,
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
#[cfg(target_family = "unix")]
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

#[cfg(target_family = "windows")]
pub fn get_app_settings_dir_path() -> Result<PathBuf, Report> {
    let mut app_settings_path =
        dirs::data_local_dir().ok_or_else(|| eyre!("Could not retrieve app settings directory"))?;
    app_settings_path.push("tdl");
    let pb = std::env::var("TDL_SETTINGS_PATH").map_or(app_settings_path, PathBuf::from);
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
mod user_settings {
    mod set_from_doom_home {
        use super::super::UserSettings;
        use assert_fs::prelude::*;
        use predicates::prelude::*;
        use std::env::set_var;

        #[test]
        fn should_set_fields() {
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
        fn should_create_directories() {
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
}

/// These tests won't test the use of the home directory, since it would
/// involve creating and removing real directories on my development machine.
///
/// This code can be tested using an integration test, or be one of the very few parts of the
/// application that won't have code coverage.
#[cfg(test)]
mod get_app_settings_dir_path {
    use super::get_app_settings_dir_path;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::env::set_var;

    #[test]
    fn should_return_value_of_tdl_settings_path_env_var() {
        let app_settings_dir = assert_fs::TempDir::new().unwrap();
        set_var(
            "TDL_SETTINGS_PATH",
            app_settings_dir.path().to_str().unwrap(),
        );

        let app_settings_path = get_app_settings_dir_path().unwrap();
        assert_eq!(app_settings_path.as_path(), app_settings_dir.path());
    }

    #[test]
    fn should_create_directory_specified_by_tdl_settings_path_env_var() {
        let app_settings_dir = assert_fs::TempDir::new().unwrap();
        let child = app_settings_dir.child("tdl");
        set_var("TDL_SETTINGS_PATH", child.path().to_str().unwrap());

        let _ = get_app_settings_dir_path().unwrap();
        child.assert(predicate::path::is_dir());
    }

    #[test]
    fn should_return_error_if_tdl_settings_env_var_points_to_file() {
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
    fn should_set_paths_relative_to_doom_home_if_env_var_is_set() {
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
