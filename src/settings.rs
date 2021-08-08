use crate::profile::Profile;
use crate::source_port::SourcePort;
use color_eyre::{eyre::eyre, Report, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::FileSettingsRepository;
    use super::SettingsRegistry;
    use super::SettingsRepository;
    use super::SourcePort;
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
            source_ports: vec![SourcePort::new("prboom", sp_exe.to_path_buf(), "2.6").unwrap()],
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
            source_ports: vec![SourcePort::new("prboom", sp_exe.to_path_buf(), "2.6").unwrap()],
            profiles: Vec::new(),
        };
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let _ = repo.save(settings);

        let retrieved_settings = repo.get().unwrap();

        settings_file.assert(predicate::path::exists());
        let sp = &retrieved_settings.source_ports[0];
        assert_eq!(sp.name, "prboom");
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
