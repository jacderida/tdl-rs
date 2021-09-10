use crate::profile::Profile;
use crate::source_port::{Skill, SourcePortType};
use crate::storage::AppSettingsRepository;
use color_eyre::{eyre::eyre, Help, Report, Result};
use log::{debug, info};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum ProfileCommand {
    #[structopt(name = "add")]
    /// Add a profile
    Add {
        /// The name of the profile
        name: String,
        /// The source port type. This must refer to a Source Port that has been added with the
        /// `source-port add` command.
        source_port_type: SourcePortType,
        /// The source port version. This must refer to a Source Port that has been added with the
        /// `source-port add` command.
        source_port_version: String,
        #[structopt(short, long)]
        /// Controls whether this profile runs in fullscreen mode
        fullscreen: bool,
        #[structopt(short, long)]
        /// Controls whether this profile will use music or not
        music: bool,
        /// Set the default skill level for this profile. Valid values are TooYoungToDie,
        /// HeyNotTooRough, HurtMePlenty, UltraViolence or Nightmare.
        skill: Skill,
        #[structopt(short, long)]
        /// Use this flag to set this profile as the default. Note, only one profile can be set as
        /// the default, so if this flag is used, the current default will be overriden with this
        /// new profile.
        default: bool,
    },
}

pub fn run_profile_cmd(
    cmd: ProfileCommand,
    repository: AppSettingsRepository,
) -> Result<(), Report> {
    match cmd {
        ProfileCommand::Add {
            name,
            source_port_type,
            source_port_version,
            fullscreen,
            music,
            skill,
            default,
        } => {
            debug!("Running add profile command");
            debug!(
                "Using values: name: {}, source_port_type: {:?}, source_port_version: {}, fullscreen: {}, music: {}, skill: {:?}, default: {}",
                &name, source_port_type, source_port_version, fullscreen, music, skill, default
            );
            let mut is_default = default;
            let mut settings = repository.get()?;
            if settings.profiles.is_empty() {
                // If there are no existing profiles, the first one *must* be set as the default,
                // even if the user didn't specify that.
                is_default = true;
            }

            let profile = Profile::new(
                &name,
                source_port_type,
                source_port_version.to_owned(),
                skill,
                fullscreen,
                music,
                is_default,
            )?;
            if !settings.source_ports.iter().any(|sp| {
                sp.source_port_type == source_port_type && sp.version == source_port_version
            }) {
                return Err(eyre!(format!(
                    "The Source Port '{:?}' with version '{}' does not exist",
                    &source_port_type, source_port_version
                ))
                .suggestion("Use the 'source-port list' command to find a valid Source Port"));
            }
            if !settings.profiles.is_empty() && default {
                let mut current = settings.profiles.iter_mut().find(|x| x.default).unwrap();
                current.default = false;
                info!("The current default profile is '{}'", current.name);
                info!("The newly added profile will now be set as the default");
            }
            settings.profiles.push(profile);
            repository.save(settings)?;
            info!("Added new profile '{}'", name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_profile_cmd;
    use super::Profile;
    use super::ProfileCommand;
    use super::Skill;
    use crate::settings::AppSettings;
    use crate::source_port::SourcePort;
    use crate::source_port::SourcePortType;
    use crate::storage::AppSettingsRepository;
    use assert_fs::prelude::*;

    #[test]
    fn add_profile_cmd_should_save_the_first_profile() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "default".to_string(),
            source_port_type: SourcePortType::PrBoom,
            source_port_version: "2.6".to_string(),
            fullscreen: true,
            music: true,
            skill: Skill::UltraViolence,
            default: true,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 1);
        assert_eq!(settings.profiles[0].name, "default");
        matches!(
            settings.profiles[0].source_port_type,
            SourcePortType::PrBoom
        );
        assert_eq!(settings.profiles[0].source_port_version, "2.6");
        assert!(settings.profiles[0].fullscreen);
        assert!(settings.profiles[0].music);
        assert!(settings.profiles[0].default);
        matches!(settings.profiles[0].skill, Skill::UltraViolence);
    }

    #[test]
    fn add_profile_cmd_should_save_the_first_profile_and_ensure_it_is_marked_default() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "default".to_string(),
            source_port_type: SourcePortType::PrBoom,
            source_port_version: "2.6".to_string(),
            fullscreen: true,
            music: true,
            skill: Skill::UltraViolence,
            default: false,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 1);
        assert_eq!(settings.profiles[0].name, "default");
        matches!(
            settings.profiles[0].source_port_type,
            SourcePortType::PrBoom
        );
        assert_eq!(settings.profiles[0].source_port_version, "2.6");
        assert!(settings.profiles[0].fullscreen);
        assert!(settings.profiles[0].music);
        assert!(settings.profiles[0].default);
        matches!(settings.profiles[0].skill, Skill::UltraViolence);
    }

    #[test]
    fn add_profile_cmd_should_add_a_new_profile() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: vec![Profile {
                name: "default".to_string(),
                source_port_type: SourcePortType::PrBoom,
                source_port_version: "2.6".to_string(),
                skill: Skill::UltraViolence,
                fullscreen: true,
                music: true,
                default: true,
            }],
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "prboom-nomusic".to_string(),
            source_port_type: SourcePortType::PrBoom,
            source_port_version: "2.6".to_string(),
            fullscreen: true,
            music: false,
            skill: Skill::UltraViolence,
            default: false,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 2);
        assert_eq!(settings.profiles[1].name, "prboom-nomusic");
        matches!(
            settings.profiles[1].source_port_type,
            SourcePortType::PrBoom
        );
        assert_eq!(settings.profiles[1].source_port_version, "2.6");
        assert!(settings.profiles[1].fullscreen);
        assert!(!settings.profiles[1].music);
        matches!(settings.profiles[1].skill, Skill::UltraViolence);
    }

    #[test]
    fn add_profile_cmd_should_add_a_new_default_profile_and_override_the_current_default() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: vec![Profile {
                name: "default".to_string(),
                source_port_type: SourcePortType::PrBoom,
                source_port_version: "2.6".to_string(),
                skill: Skill::UltraViolence,
                fullscreen: true,
                music: true,
                default: true,
            }],
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "prboom-nomusic".to_string(),
            source_port_type: SourcePortType::PrBoom,
            source_port_version: "2.6".to_string(),
            fullscreen: true,
            music: false,
            skill: Skill::UltraViolence,
            default: true,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 2);
        assert!(!settings.profiles[0].default);
        assert_eq!(settings.profiles[1].name, "prboom-nomusic");
        matches!(
            settings.profiles[1].source_port_type,
            SourcePortType::PrBoom
        );
        assert_eq!(settings.profiles[1].source_port_version, "2.6");
        assert!(settings.profiles[1].fullscreen);
        assert!(!settings.profiles[1].music);
        assert!(settings.profiles[1].default);
        matches!(settings.profiles[1].skill, Skill::UltraViolence);
    }

    #[test]
    fn add_profile_cmd_should_not_allow_invalid_source_port_reference() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = AppSettings {
            source_ports: vec![SourcePort {
                source_port_type: SourcePortType::PrBoom,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "default".to_string(),
            source_port_type: SourcePortType::PrBoom,
            source_port_version: "2.7".to_string(),
            fullscreen: true,
            music: true,
            skill: Skill::UltraViolence,
            default: true,
        };

        let result = run_profile_cmd(cmd, repo);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The Source Port 'PrBoom' with version '2.7' does not exist".to_string()
        )
    }
}
