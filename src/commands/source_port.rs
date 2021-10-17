use crate::source_port::{
    get_latest_source_port_release, InstalledSourcePort, ReleaseRepository, SourcePort,
};
use crate::storage::{AppSettingsRepository, ObjectRepository};
use color_eyre::{eyre::eyre, Help, Report, Result};
use log::{debug, info};
use prettytable::{cell, row, Table};
use std::path::PathBuf;
use structopt::StructOpt;
use strum::IntoEnumIterator;

#[derive(Debug, StructOpt)]
pub enum SourcePortCommand {
    /// Adds a source port from an existing directory
    #[structopt(name = "add")]
    Add {
        /// The name of the source port. Valid values are 'Chocolate', 'Crispy', 'DoomRetro',
        /// 'Dsda', 'EternityEngine', 'GzDoom', 'LzDoom', 'Odamex', 'PrBoomPlus', 'Rude', 'Woof',
        /// 'Zandronum'.
        name: SourcePort,
        /// The path of the source port executable
        path: PathBuf,
        /// The version of the source port
        version: String,
    },
    #[structopt(name = "ls")]
    Ls,
}

pub fn run_source_port_cmd(
    cmd: SourcePortCommand,
    app_settings_repository: &AppSettingsRepository,
    release_repository: &impl ReleaseRepository,
) -> Result<(), Report> {
    match cmd {
        SourcePortCommand::Add {
            name,
            path,
            version,
        } => {
            debug!("Running add source port command");
            debug!(
                "Using values: name: {:?}, path: {}, version: {}",
                name,
                path.display(),
                &version
            );
            let source_port = InstalledSourcePort::new(name, path, &version)?;
            let mut settings = app_settings_repository.get()?;
            if settings
                .source_ports
                .iter()
                .any(|sp| sp.name == name && sp.version == version)
            {
                return Err(eyre!(format!(
                    "There is already a {:?} source port at version {}",
                    name, version
                ))
                .suggestion("Try adding one with a different name or version"));
            }
            settings.source_ports.push(source_port);
            app_settings_repository.save(settings)?;
            info!("Added version {} of {:?}", version, name);
        }
        SourcePortCommand::Ls => {
            info!("Listing all available source ports...");
            let app_settings = app_settings_repository.get()?;
            let object_repo = ObjectRepository::new(&app_settings.release_cache_path)?;
            let mut available_source_ports = Vec::new();
            for sp in SourcePort::iter() {
                let release = get_latest_source_port_release(sp, release_repository, &object_repo)?;
                available_source_ports.push(release);
            }

            let mut table = Table::new();
            table.add_row(row!["Source Port", "Latest Version", "Installed?"]);
            for asp in available_source_ports {
                table.add_row(row![asp.source_port.to_string(), asp.version, "No"]);
            }
            table.printstd();
        }
    }
    Ok(())
}

#[cfg(test)]
mod add {
    use super::run_source_port_cmd;
    use super::SourcePortCommand;
    use crate::settings::AppSettings;
    use crate::source_port::test::FakeReleaseRepository;
    use crate::source_port::{InstalledSourcePort, SourcePort};
    use crate::storage::AppSettingsRepository;
    use assert_fs::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn should_save_the_first_source_port() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let app_settings_repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let fake_release_repo = FakeReleaseRepository {
            response_directory: PathBuf::new(),
        };

        let sp_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        sp_exe.write_binary(b"fake source port code").unwrap();

        let cmd = SourcePortCommand::Add {
            name: SourcePort::PrBoomPlus,
            path: sp_exe.path().to_path_buf(),
            version: "2.6".to_string(),
        };
        run_source_port_cmd(cmd, &app_settings_repo, &fake_release_repo).unwrap();

        let settings = app_settings_repo.get().unwrap();
        assert_eq!(settings.source_ports.len(), 1);
        matches!(settings.source_ports[0].name, SourcePort::PrBoomPlus);
        assert_eq!(
            settings.source_ports[0].path.to_str(),
            sp_exe.path().to_str()
        );
        assert_eq!(settings.source_ports[0].version, "2.6");
    }

    #[test]
    fn should_save_a_new_source_port() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let app_settings_repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let fake_release_repo = FakeReleaseRepository {
            response_directory: PathBuf::new(),
        };

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![InstalledSourcePort {
                name: SourcePort::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
            release_cache_path: PathBuf::new(),
        };
        app_settings_repo.save(settings).unwrap();

        let gzdoom_exe = assert_fs::NamedTempFile::new("gzdoom.exe").unwrap();
        gzdoom_exe.write_binary(b"fake source port code").unwrap();

        let cmd = SourcePortCommand::Add {
            name: SourcePort::GzDoom,
            path: gzdoom_exe.path().to_path_buf(),
            version: "4.6.1".to_string(),
        };
        run_source_port_cmd(cmd, &app_settings_repo, &fake_release_repo).unwrap();

        let settings = app_settings_repo.get().unwrap();
        assert_eq!(settings.source_ports.len(), 2);
        matches!(settings.source_ports[1].name, SourcePort::GzDoom);
        assert_eq!(
            settings.source_ports[1].path.to_str(),
            gzdoom_exe.path().to_str()
        );
        assert_eq!(settings.source_ports[1].version, "4.6.1");
    }

    #[test]
    fn should_return_error_for_duplicate_type_and_version_combination() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let app_settings_repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let fake_release_repo = FakeReleaseRepository {
            response_directory: PathBuf::new(),
        };

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![InstalledSourcePort {
                name: SourcePort::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
            release_cache_path: PathBuf::new(),
        };
        app_settings_repo.save(settings).unwrap();

        let cmd = SourcePortCommand::Add {
            name: SourcePort::PrBoomPlus,
            path: prboom_exe.path().to_path_buf(),
            version: "2.6".to_string(),
        };
        let result = run_source_port_cmd(cmd, &app_settings_repo, &fake_release_repo);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "There is already a PrBoomPlus source port at version 2.6".to_string()
        )
    }

    #[test]
    fn should_save_source_port_with_duplicate_source_port_but_different_version() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let app_settings_repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let fake_release_repo = FakeReleaseRepository {
            response_directory: PathBuf::new(),
        };

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![InstalledSourcePort {
                name: SourcePort::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
            release_cache_path: PathBuf::new(),
        };
        app_settings_repo.save(settings).unwrap();

        let cmd = SourcePortCommand::Add {
            name: SourcePort::PrBoomPlus,
            path: prboom_exe.path().to_path_buf(),
            version: "2.7".to_string(),
        };
        run_source_port_cmd(cmd, &app_settings_repo, &fake_release_repo).unwrap();

        let settings = app_settings_repo.get().unwrap();
        assert_eq!(settings.source_ports.len(), 2);
        matches!(settings.source_ports[1].name, SourcePort::PrBoomPlus);
        assert_eq!(
            settings.source_ports[1].path.to_str(),
            prboom_exe.path().to_str()
        );
        assert_eq!(settings.source_ports[1].version, "2.7");
    }
}
