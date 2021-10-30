use crate::settings::AppSettings;
use color_eyre::{eyre::eyre, Report, Result};
use log::{debug, info};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("{0}")]
    ObjectIdError(String),
    #[error("The object path cannot be set to an existing file")]
    InvalidObjectPath,
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// A repository for saving 'objects'.
///
/// One concrete example of an 'object' is a WadEntry. I'm making this repository completely
/// generic, as I can imagine at least one more thing I'm going to want to save. All this
/// repository is going to do is just serialize the object to JSON.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectRepository {
    /// The directory to save the object to.
    ///
    /// Typically the object you're saving is going to sit alongside other objects of the same type
    /// in the same directory.
    object_path: PathBuf,
}

impl ObjectRepository {
    /// Creates an instance of the ObjectRepository.
    ///
    /// The `object_path` should either be an existing directory, or a directory will be created at
    /// that location. Typically there will be many objects of the same type being stored at this
    /// directory.
    ///
    /// # Errors
    ///
    /// Result will be an error if `object_path` is an existing file.
    ///
    /// Any other errors would be from file IO.
    pub fn new(object_path: &impl AsRef<Path>) -> Result<ObjectRepository, StorageError> {
        let pb = PathBuf::from(object_path.as_ref());
        if pb.is_file() {
            return Err(StorageError::InvalidObjectPath);
        }
        if !pb.exists() {
            std::fs::create_dir_all(&pb)?;
        }
        Ok(ObjectRepository { object_path: pb })
    }

    /// Gets any object that's been saved with the ObjectRepository.
    ///
    /// Will read the object from the file at `ojbect_path/<id>.json`, deserialize it and then
    /// return the object.
    ///
    /// # Errors
    ///
    /// Should only be related to IO or if the object being retrieved is not valid JSON (which
    /// shouldn't happen if it's been saved by this application).
    pub fn get<T: DeserializeOwned>(&self, id: &str) -> Result<T, StorageError> {
        let mut pb = PathBuf::from(&self.object_path);
        pb.push(format!("{}.json", id));
        debug!("Deserializing {}", pb.as_path().display());
        let serialized = std::fs::read_to_string(pb.as_path().to_str().unwrap())?;
        Ok(serde_json::from_str(&serialized)?)
    }

    /// Saves any struct to JSON, provided it implements Serialize.
    ///
    /// The object will be serialized to JSON and saved at `object_path/<id>.json`.
    ///
    /// # Errors
    ///
    /// Result will be an error if ID is set to empty.
    ///
    /// Result will be an error if there is already an object with the specified ID.
    ///
    /// Any other errors would be from file IO or the JSON library.
    pub fn save<T: Serialize>(&self, id: &str, object: &T) -> Result<(), StorageError> {
        if id.is_empty() {
            return Err(StorageError::ObjectIdError(String::from(
                "To save the object, its ID must be set.",
            )));
        }
        let serialized = serde_json::to_string(&object)?;
        let mut save_pb = PathBuf::from(&self.object_path);
        save_pb.push(format!("{}.json", id));
        if save_pb.exists() {
            return Err(StorageError::ObjectIdError(String::from(format!(
                "The ID '{}' is already taken.",
                id
            ))));
        }
        info!("Saving entry for {}", id);
        std::fs::write(save_pb.as_path(), serialized)?;
        Ok(())
    }

    /// Deletes a saved object.
    ///
    /// This exists for use with the Github release cache to remove stale cache entries.
    pub fn delete(&self, id: &str) -> Result<(), StorageError> {
        let object_path = Path::new(&self.object_path).join(format!("{}.json", id));
        if object_path.exists() {
            std::fs::remove_file(object_path)?;
        }
        Ok(())
    }
}

pub struct AppSettingsRepository {
    settings_path: PathBuf,
}

impl AppSettingsRepository {
    pub fn new(settings_path: PathBuf) -> Result<AppSettingsRepository, Report> {
        if !settings_path.exists() {
            let mut release_cache_path = settings_path.clone();
            release_cache_path.pop();
            release_cache_path.push("release_cache");
            std::fs::create_dir_all(&release_cache_path)?;
            let settings = AppSettings {
                source_ports: Vec::new(),
                profiles: Vec::new(),
                release_cache_path,
            };
            let serialized = serde_json::to_string(&settings)?;
            std::fs::write(settings_path.to_str().unwrap(), serialized)?;
        }
        Ok(AppSettingsRepository { settings_path })
    }

    pub fn get(&self) -> Result<AppSettings, Report> {
        let path = self
            .settings_path
            .to_owned()
            .into_os_string()
            .into_string()
            .map_err(|e| eyre!(format!("Could not convert {:?} to string", e)))?;
        let serialized = std::fs::read_to_string(&path)?;
        let settings: AppSettings = serde_json::from_str(&serialized)?;
        Ok(settings)
    }

    pub fn save(&self, settings: AppSettings) -> Result<(), Report> {
        let serialized = serde_json::to_string(&settings)?;
        std::fs::write(self.settings_path.to_str().unwrap(), serialized)?;
        Ok(())
    }
}

#[cfg(test)]
mod object_repository {
    mod new {
        use super::super::ObjectRepository;
        use assert_fs::prelude::*;
        use predicates::prelude::*;

        #[test]
        fn should_set_fields_correctly() {
            let wad_dir = assert_fs::TempDir::new().unwrap();
            let child = wad_dir.child("wads");
            child.create_dir_all().unwrap();
            let sut = ObjectRepository::new(&child).unwrap();
            assert_eq!(sut.object_path.as_path(), child.path());
        }

        #[test]
        fn should_create_objects_path_if_it_does_not_exist() {
            let wad_dir = assert_fs::TempDir::new().unwrap();
            let sub = wad_dir.child("sub");
            let child = sub.child("wads");
            let _ = ObjectRepository::new(&child).unwrap();
            child.assert(predicate::path::exists());
            child.assert(predicate::path::is_dir());
        }

        #[test]
        fn should_ensure_existing_object_path_is_not_a_file() {
            let wad_dir = assert_fs::TempDir::new().unwrap();
            let child = wad_dir.child("wads");
            child.write_str("some file content").unwrap();
            let result = ObjectRepository::new(&child);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "The object path cannot be set to an existing file"
            );
        }
    }

    mod save {
        use super::super::ObjectRepository;
        use crate::wad::MapInfo;
        use crate::wad::WadEntry;
        use assert_fs::prelude::*;
        use predicates::prelude::*;

        #[test]
        fn should_persist_a_wad_entry() {
            let maps = vec![
                MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
                MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
                MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
            ];
            let entry = WadEntry::new(
                "DOOM2".to_string(),
                "DOOM2.WAD".to_string(),
                "Doom II: Hell on Earth".to_string(),
                "1994-09-30".to_string(),
                "id Software".to_string(),
                maps,
            )
            .unwrap();

            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let wad_dir = tmp_dir.child("wads");
            wad_dir.create_dir_all().unwrap();
            let saved = wad_dir.child("DOOM2.json");

            let sut = ObjectRepository::new(&wad_dir).unwrap();
            sut.save(&entry.id, &entry).unwrap();
            saved.assert(predicate::path::is_file());
        }

        #[test]
        fn should_ensure_id_is_set() {
            let maps = vec![
                MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
                MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
                MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
            ];
            let entry = WadEntry::new(
                "DOOM2".to_string(),
                "DOOM2.WAD".to_string(),
                "Doom II: Hell on Earth".to_string(),
                "1994-09-30".to_string(),
                "id Software".to_string(),
                maps,
            )
            .unwrap();

            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let wad_dir = tmp_dir.child("wads");
            wad_dir.create_dir_all().unwrap();

            let sut = ObjectRepository::new(&wad_dir).unwrap();
            let result = sut.save(&"".to_string(), &entry);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "To save the object, its ID must be set."
            );
        }

        #[test]
        fn should_ensure_id_is_not_taken() {
            let maps = vec![
                MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
                MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
                MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
            ];
            let entry = WadEntry::new(
                "DOOM2".to_string(),
                "DOOM2.WAD".to_string(),
                "Doom II: Hell on Earth".to_string(),
                "1994-09-30".to_string(),
                "id Software".to_string(),
                maps,
            )
            .unwrap();

            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let wad_dir = tmp_dir.child("wads");
            wad_dir.create_dir_all().unwrap();
            let doom2_entry = wad_dir.child("DOOM2.json");
            doom2_entry.write_str("file already exists").unwrap();

            let sut = ObjectRepository::new(&wad_dir).unwrap();
            let result = sut.save(&entry.id, &entry);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "The ID 'DOOM2' is already taken."
            );
        }
    }

    mod get {
        use super::super::ObjectRepository;
        use crate::wad::MapInfo;
        use crate::wad::WadEntry;
        use assert_fs::prelude::*;

        #[test]
        fn should_retrieve_existing_wad_entry() {
            let maps = vec![
                MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
                MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
                MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
            ];
            let entry = WadEntry::new(
                "DOOM2".to_string(),
                "DOOM2.WAD".to_string(),
                "Doom II: Hell on Earth".to_string(),
                "1994-09-30".to_string(),
                "id Software".to_string(),
                maps,
            )
            .unwrap();

            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let wad_dir = tmp_dir.child("wads");
            wad_dir.create_dir_all().unwrap();

            let sut = ObjectRepository::new(&wad_dir).unwrap();
            sut.save(&entry.id, &entry).unwrap();

            let saved: WadEntry = sut.get(&entry.id).unwrap();
            assert_eq!(saved.id, entry.id);
            assert_eq!(saved.name, entry.name);
            assert_eq!(saved.title, entry.title);
            assert_eq!(saved.release_date, entry.release_date);
            assert_eq!(saved.author, entry.author);
            assert_eq!(saved.maps.len(), entry.maps.len());
        }
    }

    mod delete {
        use super::super::ObjectRepository;
        use crate::wad::MapInfo;
        use crate::wad::WadEntry;
        use assert_fs::prelude::*;

        #[test]
        fn should_delete_an_existing_wad_entry() {
            let maps = vec![
                MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
                MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
                MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
            ];
            let entry = WadEntry::new(
                "DOOM2".to_string(),
                "DOOM2.WAD".to_string(),
                "Doom II: Hell on Earth".to_string(),
                "1994-09-30".to_string(),
                "id Software".to_string(),
                maps,
            )
            .unwrap();

            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let wad_dir = tmp_dir.child("wads");
            wad_dir.create_dir_all().unwrap();
            let doom2_wad_file = wad_dir.child("DOOM2.json");

            let sut = ObjectRepository::new(&wad_dir).unwrap();
            sut.save(&entry.id, &entry).unwrap();

            let result = sut.delete(&entry.id);
            assert!(result.is_ok());
            doom2_wad_file.assert(predicates::path::missing());
        }

        #[test]
        fn should_return_ok_result_for_non_existent_file() {
            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let wad_dir = tmp_dir.child("wads");
            wad_dir.create_dir_all().unwrap();

            let sut = ObjectRepository::new(&wad_dir).unwrap();

            let result = sut.delete("non-existent-id");
            assert!(result.is_ok());
        }
    }
}

#[cfg(test)]
mod app_settings_repository {
    mod new {
        use super::super::AppSettingsRepository;
        use crate::settings::AppSettings;
        use assert_fs::prelude::*;
        use predicates::prelude::*;

        #[test]
        fn should_set_fields_correctly() {
            let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
            settings_file.write_str("json document").unwrap();
            let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
            assert_eq!(
                repo.settings_path.to_str().unwrap(),
                settings_file.path().to_str().unwrap()
            );
        }

        #[test]
        fn should_serialize_default_settings_if_settings_file_does_not_exist() {
            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let settings_file = tmp_dir.child("tdl.json");

            let _ = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

            let settings_contents =
                std::fs::read_to_string(settings_file.path().to_str().unwrap()).unwrap();
            let app_settings: AppSettings = serde_json::from_str(&settings_contents).unwrap();
            assert_eq!(app_settings.source_ports.len(), 0);
            assert_eq!(app_settings.profiles.len(), 0);
        }

        #[test]
        fn should_create_the_release_cache_directory_if_settings_file_does_not_exist() {
            let tmp_dir = assert_fs::TempDir::new().unwrap();
            let settings_file = tmp_dir.child("tdl.json");
            let release_cache_dir = tmp_dir.child("release_cache");

            let _ = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

            release_cache_dir.assert(predicate::path::is_dir());
            let settings_contents =
                std::fs::read_to_string(settings_file.path().to_str().unwrap()).unwrap();
            let app_settings: AppSettings = serde_json::from_str(&settings_contents).unwrap();
            assert_eq!(
                app_settings.release_cache_path.to_str().unwrap(),
                release_cache_dir.path().to_str().unwrap()
            );
        }
    }

    mod save {
        use super::super::AppSettings;
        use super::super::AppSettingsRepository;
        use crate::source_port::InstalledSourcePort;
        use crate::source_port::SourcePort;
        use assert_fs::prelude::*;
        use predicates::prelude::*;
        use std::path::PathBuf;

        #[test]
        fn should_serialize_the_settings_file_to_json() {
            let sp_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            sp_exe.write_binary(b"fake source port code").unwrap();
            let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
            let settings = AppSettings {
                source_ports: vec![InstalledSourcePort::new(
                    SourcePort::PrBoomPlus,
                    sp_exe.to_path_buf(),
                    "2.6",
                )
                .unwrap()],
                profiles: Vec::new(),
                release_cache_path: PathBuf::new(),
            };
            let serialized_settings = serde_json::to_string(&settings).unwrap();

            let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
            let _ = repo.save(settings);

            settings_file.assert(predicate::path::exists());
            let settings_contents =
                std::fs::read_to_string(settings_file.path().to_str().unwrap()).unwrap();
            assert_eq!(settings_contents, serialized_settings);
        }
    }

    mod get {
        use super::super::AppSettings;
        use super::super::AppSettingsRepository;
        use crate::source_port::InstalledSourcePort;
        use crate::source_port::SourcePort;
        use assert_fs::prelude::*;
        use predicates::prelude::*;
        use std::path::PathBuf;

        #[test]
        fn should_deserialize_the_settings_file_and_return_the_settings_registry() {
            let sp_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
            sp_exe.write_binary(b"fake source port code").unwrap();
            let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
            let settings = AppSettings {
                source_ports: vec![InstalledSourcePort::new(
                    SourcePort::PrBoomPlus,
                    sp_exe.to_path_buf(),
                    "2.6",
                )
                .unwrap()],
                profiles: Vec::new(),
                release_cache_path: PathBuf::new(),
            };
            let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
            let _ = repo.save(settings);

            let retrieved_settings = repo.get().unwrap();

            settings_file.assert(predicate::path::exists());
            let sp = &retrieved_settings.source_ports[0];
            matches!(sp.name, SourcePort::PrBoomPlus);
            assert_eq!(sp.path.to_str().unwrap(), sp_exe.path().to_str().unwrap());
            assert_eq!(sp.version, "2.6");
        }

        #[test]
        fn should_return_an_empty_initialised_settings_registry_if_no_settings_have_yet_been_saved()
        {
            let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
            let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
            let retrieved_settings = repo.get().unwrap();
            assert_eq!(retrieved_settings.source_ports.len(), 0);
        }
    }
}
