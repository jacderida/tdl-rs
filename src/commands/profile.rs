use crate::profile::Profile;
use crate::settings::SettingsRepository;
use crate::source_port::Skill;
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
        /// The source port for the profile
        source_port: String,
        #[structopt(short, long)]
        /// Controls whether this profile runs in fullscreen mode
        fullscreen: bool,
        #[structopt(short, long)]
        /// Controls whether this profile will use music or not
        music: bool,
        /// Set the default skill level for this profile. Valid values are TooYoungToDie,
        /// HeyNotTooRough, HurtMePlenty, UltraViolence or Nightmare.
        skill: Skill,
    },
}

pub fn run_profile_cmd(
    cmd: ProfileCommand,
    repository: impl SettingsRepository,
) -> Result<(), Report> {
    match cmd {
        ProfileCommand::Add {
            name,
            source_port,
            fullscreen,
            music,
            skill,
        } => {
            debug!(
                "Running add profile command: {} {} {} {} {:?}",
                &name, &source_port, fullscreen, music, skill
            );
            let profile = Profile::new(&name, &source_port, skill, fullscreen, music)?;
            let mut settings = repository.get()?;
            if !settings
                .source_ports
                .iter()
                .any(|sp| sp.name == source_port)
            {
                return Err(
                    eyre!(format!("The Source Port '{}' does not exist", source_port)).suggestion(
                        "Use the 'source-port list' command to find a valid Source Port",
                    ),
                );
            }
            settings.profiles.push(profile);
            repository.save(settings)?;
            info!("Added new profile {}", name);
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
    use crate::settings::FileSettingsRepository;
    use crate::settings::SettingsRegistry;
    use crate::settings::SettingsRepository;
    use crate::source_port::SourcePort;
    use assert_fs::prelude::*;

    #[test]
    fn add_profile_cmd_should_save_the_first_profile() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort {
                name: "prboom".to_string(),
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "default".to_string(),
            source_port: "prboom".to_string(),
            fullscreen: true,
            music: true,
            skill: Skill::UltraViolence,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 1);
        assert_eq!(settings.profiles[0].name, "default");
        assert_eq!(settings.profiles[0].source_port, "prboom");
        assert_eq!(settings.profiles[0].fullscreen, true);
        assert_eq!(settings.profiles[0].music, true);
        matches!(settings.profiles[0].skill, Skill::UltraViolence);
    }

    #[test]
    fn add_profile_cmd_should_add_a_new_profile() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort {
                name: "prboom".to_string(),
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: vec![Profile {
                name: "default".to_string(),
                source_port: "prboom".to_string(),
                skill: Skill::UltraViolence,
                fullscreen: true,
                music: true,
            }],
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "prboom-nomusic".to_string(),
            source_port: "prboom".to_string(),
            fullscreen: true,
            music: false,
            skill: Skill::UltraViolence,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 2);
        assert_eq!(settings.profiles[1].name, "prboom-nomusic");
        assert_eq!(settings.profiles[1].source_port, "prboom");
        assert_eq!(settings.profiles[1].fullscreen, true);
        assert_eq!(settings.profiles[1].music, false);
        matches!(settings.profiles[1].skill, Skill::UltraViolence);
    }

    #[test]
    fn add_profile_cmd_should_not_allow_invalid_source_port_reference() {
        let settings_file = assert_fs::NamedTempFile::new("tdl.json").unwrap();
        let repo = FileSettingsRepository::new(settings_file.to_path_buf()).unwrap();

        let prboom_exe = assert_fs::NamedTempFile::new("prboom.exe").unwrap();
        prboom_exe.write_binary(b"fake source port code").unwrap();
        let settings = SettingsRegistry {
            source_ports: vec![SourcePort {
                name: "prboom".to_string(),
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: "default".to_string(),
            source_port: "missing".to_string(),
            fullscreen: true,
            music: true,
            skill: Skill::UltraViolence,
        };

        let result = run_profile_cmd(cmd, repo);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("The Source Port 'missing' does not exist")
        )
    }
}
