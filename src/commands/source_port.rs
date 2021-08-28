use crate::settings::SettingsRepository;
use crate::source_port::{SourcePort, SourcePortType};
use color_eyre::{eyre::eyre, Help, Report, Result};
use log::{debug, info};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum SourcePortCommand {
    #[structopt(name = "add")]
    /// Adds a source port from an existing directory
    Add {
        /// The type of the source port. Valid values are 'prboom', 'prboomumapinfo', 'dsda',
        /// 'gzdoom', 'doomretro'.
        source_port_type: SourcePortType,
        /// The path of the source port executable
        path: PathBuf,
        /// The version of the source port
        version: String,
    },
}

pub fn run_source_port_cmd(
    cmd: SourcePortCommand,
    repository: &impl SettingsRepository,
) -> Result<(), Report> {
    match cmd {
        SourcePortCommand::Add {
            source_port_type,
            path,
            version,
        } => {
            debug!("Running add source port command");
            debug!(
                "Using values: type: {:?}, path: {}, version: {}",
                source_port_type,
                path.display(),
                &version
            );
            let source_port = SourcePort::new(source_port_type, path, &version)?;
            let mut settings = repository.get()?;
            if settings
                .source_ports
                .iter()
                .any(|sp| sp.source_port_type == source_port_type && sp.version == version)
            {
                return Err(eyre!(format!(
                    "There is already a {:?} Source Port at version {}",
                    source_port_type, version
                ))
                .suggestion("Try adding one with a different name or version"));
            }
            settings.source_ports.push(source_port);
            repository.save(settings)?;
            info!("Added version {} of {:?}", version, source_port_type);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_source_port_cmd;
    use super::SourcePortCommand;
    use crate::settings::FileSettingsRepository;
    use crate::settings::SettingsRegistry;
    use crate::settings::SettingsRepository;
    use crate::source_port::SourcePort;
    use crate::source_port::SourcePortType;
    use assert_fs::prelude::*;

    #[test]
    fn add_source_port_cmd_should_save_the_first_source_port() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let sp_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        sp_exe.write_binary(b"fake source port code").unwrap();

        let cmd = SourcePortCommand::Add {
            source_port_type: SourcePortType::PrBoom,
            path: sp_exe.path().to_path_buf(),
            version: "2.6".to_string(),
        };
        run_source_port_cmd(cmd, &repo).unwrap();

        let settings = repo.get().unwrap();
        assert_eq!(settings.source_ports.len(), 1);
        matches!(
            settings.source_ports[0].source_port_type,
            SourcePortType::PrBoom
        );
        assert_eq!(
            settings.source_ports[0].path.to_str(),
            sp_exe.path().to_str()
        );
        assert_eq!(settings.source_ports[0].version, "2.6");
    }

    #[test]
    fn add_source_port_cmd_should_add_a_new_source_port() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let gzdoom_exe = assert_fs::NamedTempFile::new("gzdoom.exe").unwrap();
        gzdoom_exe.write_binary(b"fake source port code").unwrap();

        let cmd = SourcePortCommand::Add {
            source_port_type: SourcePortType::GzDoom,
            path: gzdoom_exe.path().to_path_buf(),
            version: "4.6.1".to_string(),
        };
        run_source_port_cmd(cmd, &repo).unwrap();

        let settings = repo.get().unwrap();
        assert_eq!(settings.source_ports.len(), 2);
        matches!(
            settings.source_ports[1].source_port_type,
            SourcePortType::GzDoom
        );
        assert_eq!(
            settings.source_ports[1].path.to_str(),
            gzdoom_exe.path().to_str()
        );
        assert_eq!(settings.source_ports[1].version, "4.6.1");
    }

    #[test]
    fn add_source_port_cmd_should_not_allow_duplicate_type_version_combo() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = SourcePortCommand::Add {
            source_port_type: SourcePortType::PrBoom,
            path: prboom_exe.path().to_path_buf(),
            version: "2.6".to_string(),
        };
        let result = run_source_port_cmd(cmd, &repo);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "There is already a PrBoom Source Port at version 2.6".to_string()
        )
    }

    #[test]
    fn add_source_port_cmd_should_allow_duplicate_type_with_different_version() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = SourcePortCommand::Add {
            source_port_type: SourcePortType::PrBoom,
            path: prboom_exe.path().to_path_buf(),
            version: "2.7".to_string(),
        };
        run_source_port_cmd(cmd, &repo).unwrap();

        let settings = repo.get().unwrap();
        assert_eq!(settings.source_ports.len(), 2);
        matches!(
            settings.source_ports[1].source_port_type,
            SourcePortType::PrBoom
        );
        assert_eq!(
            settings.source_ports[1].path.to_str(),
            prboom_exe.path().to_str()
        );
        assert_eq!(settings.source_ports[1].version, "2.7");
    }
}
