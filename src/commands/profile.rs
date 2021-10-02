use crate::profile::Profile;
use crate::source_port::{Skill, SourcePortType};
use crate::storage::AppSettingsRepository;
use color_eyre::{eyre::eyre, eyre::WrapErr, Help, Report, Result};
use log::{debug, info};
use prettytable::{cell, row, Table};
use serde_hjson::{Map, Value};
use std::io::Write;
use structopt::StructOpt;
use tempfile::NamedTempFile;

#[derive(Debug, StructOpt)]
pub enum ProfileCommand {
    #[structopt(name = "ls")]
    /// Lists all the available profiles that have been added
    Ls,
    #[structopt(name = "add")]
    /// Add a profile. If the `--name` argument is not supplied, this command will run in
    /// interactive mode and launch the editor specified by the EDITOR variable.
    Add {
        /// The name of the profile
        #[structopt(short, long)]
        name: Option<String>,
        /// The source port type. This must refer to a Source Port that has been added with the
        /// `source-port add` command.
        #[structopt(name = "type", short, long)]
        source_port_type: Option<SourcePortType>,
        /// The source port version. This must refer to a Source Port that has been added with the
        /// `source-port add` command.
        #[structopt(name = "version", short, long)]
        source_port_version: Option<String>,
        #[structopt(short, long)]
        /// Controls whether this profile runs in fullscreen mode
        fullscreen: bool,
        #[structopt(short, long)]
        /// Controls whether this profile will use music or not
        music: bool,
        /// Set the default skill level for this profile. Valid values are TooYoungToDie,
        /// HeyNotTooRough, HurtMePlenty, UltraViolence or Nightmare.
        #[structopt(name = "skill", short, long)]
        skill: Option<Skill>,
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
        ProfileCommand::Ls => {
            let settings = repository.get()?;
            if settings.profiles.is_empty() {
                info!("No profiles have been added yet.");
                info!("Run the `profile add` command to create a new profile.");
            } else {
                info!("Listing {} profiles", settings.profiles.len());
                let mut table = Table::new();
                table.add_row(row!["Name", "Source Port", "Version", "Is Default?"]);
                for profile in settings.profiles {
                    table.add_row(row![
                        profile.name,
                        profile.source_port_type,
                        profile.source_port_version,
                        profile.default
                    ]);
                }
                table.printstd();
            }
        }
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
            let mut is_default = default;
            let mut settings = repository.get()?;
            if settings.profiles.is_empty() {
                // If there are no existing profiles, the first one *must* be set as the default,
                // even if the user didn't specify that.
                is_default = true;
            }
            let profile = if let Some(name) = name {
                let source_port_type = source_port_type.unwrap();
                let source_port_version = source_port_version.unwrap();
                let skill = skill.unwrap();
                Profile::new(
                    &name,
                    source_port_type,
                    source_port_version,
                    skill,
                    fullscreen,
                    music,
                    is_default,
                )?
            } else {
                get_profile_in_interactive_mode()?
            };

            debug!(
                "Using values: name: {}, type: {:?}, version: {}, fullscreen: {}, music: {},\
                skill: {:?}, default: {}",
                &profile.name,
                profile.source_port_type,
                profile.source_port_version,
                fullscreen,
                music,
                profile.skill,
                default
            );

            if !settings.source_ports.iter().any(|sp| {
                sp.source_port_type == profile.source_port_type
                    && sp.version == profile.source_port_version
            }) {
                return Err(eyre!(format!(
                    "The Source Port '{:?}' with version '{}' does not exist",
                    &profile.source_port_type, profile.source_port_version
                ))
                .suggestion("Use the 'source-port ls' command to find a valid source port"));
            }
            if !settings.profiles.is_empty() && default {
                let mut current = settings.profiles.iter_mut().find(|x| x.default).unwrap();
                current.default = false;
                info!("The current default profile is '{}'", current.name);
                info!("The newly added profile will now be set as the default");
            }
            settings.profiles.push(profile.to_owned());
            repository.save(settings)?;
            info!("Added new profile '{}'", &profile.name);
        }
    }
    Ok(())
}

fn get_profile_in_interactive_mode() -> Result<Profile, Report> {
    info!("The `--name` argument wasn't supplied, so we will use interactive mode to add the profile.");
    let add_profile_template = include_bytes!("../../resources/add_profile_template.hjson");
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(add_profile_template)?;
    let editor = std::env::var("EDITOR")
        .wrap_err("The EDITOR environment variable was not set.")
        .suggestion(
            "Please set the EDITOR variable to e.g. 'vim', 'nvim' or 'nano'.\
             Note: the locations for those must be on PATH.",
        )?;
    if editor == "nvim" || editor == "vim" {
        duct::cmd!(editor, "+set filetype=hjson", temp_file.path()).run()?;
    } else {
        duct::cmd!(editor, temp_file.path()).run()?;
    }

    let profile_as_hjson = std::fs::read_to_string(&temp_file)?;
    let json_profile: Map<String, Value> = serde_hjson::from_str(&profile_as_hjson).unwrap();
    let source_port_type = json_profile
        .get("type")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<SourcePortType>()
        .map_err(|e| eyre!("Error parsing source port type: {}", e))?;
    let skill = json_profile
        .get("skill")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<Skill>()
        .map_err(|e| eyre!("Error parsing skill type: {}", e))?;
    let profile = Profile::new(
        json_profile.get("name").unwrap().as_str().unwrap(),
        source_port_type,
        String::from(json_profile.get("version").unwrap().as_str().unwrap()),
        skill,
        json_profile.get("fullscreen").unwrap().as_bool().unwrap(),
        json_profile.get("music").unwrap().as_bool().unwrap(),
        json_profile.get("default").unwrap().as_bool().unwrap(),
    )?;
    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::run_profile_cmd;
    use super::Profile;
    use super::ProfileCommand;
    use super::Skill;
    use crate::settings::AppSettings;
    use crate::source_port::InstalledSourcePort;
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
            source_ports: vec![InstalledSourcePort {
                source_port_type: SourcePortType::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: Some("default".to_string()),
            source_port_type: Some(SourcePortType::PrBoomPlus),
            source_port_version: Some("2.6".to_string()),
            fullscreen: true,
            music: true,
            skill: Some(Skill::UltraViolence),
            default: true,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 1);
        assert_eq!(settings.profiles[0].name, "default");
        matches!(
            settings.profiles[0].source_port_type,
            SourcePortType::PrBoomPlus
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
            source_ports: vec![InstalledSourcePort {
                source_port_type: SourcePortType::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: Some("default".to_string()),
            source_port_type: Some(SourcePortType::PrBoomPlus),
            source_port_version: Some("2.6".to_string()),
            fullscreen: true,
            music: true,
            skill: Some(Skill::UltraViolence),
            default: false,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 1);
        assert_eq!(settings.profiles[0].name, "default");
        matches!(
            settings.profiles[0].source_port_type,
            SourcePortType::PrBoomPlus
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
            source_ports: vec![InstalledSourcePort {
                source_port_type: SourcePortType::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: vec![Profile {
                name: "default".to_string(),
                source_port_type: SourcePortType::PrBoomPlus,
                source_port_version: "2.6".to_string(),
                skill: Skill::UltraViolence,
                fullscreen: true,
                music: true,
                default: true,
            }],
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: Some("prboom-nomusic".to_string()),
            source_port_type: Some(SourcePortType::PrBoomPlus),
            source_port_version: Some("2.6".to_string()),
            fullscreen: true,
            music: false,
            skill: Some(Skill::UltraViolence),
            default: false,
        };

        run_profile_cmd(cmd, repo).unwrap();

        let repo = AppSettingsRepository::new(settings_file.to_path_buf()).unwrap();
        let settings = repo.get().unwrap();
        assert_eq!(settings.profiles.len(), 2);
        assert_eq!(settings.profiles[1].name, "prboom-nomusic");
        matches!(
            settings.profiles[1].source_port_type,
            SourcePortType::PrBoomPlus
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
            source_ports: vec![InstalledSourcePort {
                source_port_type: SourcePortType::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: vec![Profile {
                name: "default".to_string(),
                source_port_type: SourcePortType::PrBoomPlus,
                source_port_version: "2.6".to_string(),
                skill: Skill::UltraViolence,
                fullscreen: true,
                music: true,
                default: true,
            }],
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: Some("prboom-nomusic".to_string()),
            source_port_type: Some(SourcePortType::PrBoomPlus),
            source_port_version: Some("2.6".to_string()),
            fullscreen: true,
            music: false,
            skill: Some(Skill::UltraViolence),
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
            SourcePortType::PrBoomPlus
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
            source_ports: vec![InstalledSourcePort {
                source_port_type: SourcePortType::PrBoomPlus,
                path: prboom_exe.path().to_path_buf(),
                version: "2.6".to_string(),
            }],
            profiles: Vec::new(),
        };
        repo.save(settings).unwrap();

        let cmd = ProfileCommand::Add {
            name: Some("default".to_string()),
            source_port_type: Some(SourcePortType::PrBoomPlus),
            source_port_version: Some("2.7".to_string()),
            fullscreen: true,
            music: true,
            skill: Some(Skill::UltraViolence),
            default: true,
        };

        let result = run_profile_cmd(cmd, repo);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The Source Port 'PrBoomPlus' with version '2.7' does not exist".to_string()
        )
    }
}
