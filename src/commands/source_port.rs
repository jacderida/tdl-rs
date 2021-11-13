use crate::settings::{get_user_settings, AppSettings};
use crate::source_port::{
    get_latest_source_port_release, install_source_port_release, InstalledSourcePort,
    ReleaseRepository, SourcePort, SourcePortError, SourcePortRelease,
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
    /// Install a supported source port.
    #[structopt(name = "install")]
    Install {
        /// The source port to install. Valid values are 'Chocolate', 'Crispy', 'DoomRetro',
        /// 'Dsda', 'EternityEngine', 'GzDoom', 'LzDoom', 'Odamex', 'PrBoomPlus', 'Rude', 'Woof',
        /// 'Zandronum'.
        source_port: SourcePort,
        #[structopt(name = "version", short, long)]
        /// The version of the source port to install. If this is not supplied, the latest version
        /// will be installed.
        version: Option<String>,
    },
    /// Lists all supported source ports and their latest versions.
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
            add_source_port(app_settings_repository, name, path, version)?;
        }
        SourcePortCommand::Install {
            source_port,
            version,
        } => {
            run_install_subcommand(
                source_port,
                version,
                app_settings_repository,
                release_repository,
            )?;
        }
        SourcePortCommand::Ls => {
            info!("Listing all available source ports...");
            let app_settings = app_settings_repository.get()?;
            let object_repo = ObjectRepository::new(&app_settings.release_cache_path)?;
            let mut available_source_ports = Vec::new();
            let mut no_release_source_ports: Vec<SourcePort> = Vec::new();
            for sp in SourcePort::iter() {
                match get_latest_source_port_release(sp, release_repository, &object_repo) {
                    Ok(release) => {
                        available_source_ports.push(release);
                    }
                    Err(error) => match error {
                        SourcePortError::NoLatestRelease(sp) => {
                            no_release_source_ports.push(sp);
                        }
                        _ => {
                            return Err(eyre!(error));
                        }
                    },
                }
            }

            let mut table = Table::new();
            table.add_row(row!["Source Port", "Latest Version", "Installed?"]);
            for asp in available_source_ports {
                let installed = if is_source_port_installed(&asp, &app_settings) {
                    "Yes"
                } else {
                    "No"
                };
                table.add_row(row![asp.source_port.to_string(), asp.version, installed]);
            }
            table.printstd();

            if !no_release_source_ports.is_empty() {
                println!("The following source ports had no releases marked as latest:");
                for sp in no_release_source_ports {
                    println!("* {} has no version marked as latest", sp);
                }
            }
        }
    }
    Ok(())
}

fn run_install_subcommand(
    source_port: SourcePort,
    version: Option<String>,
    app_settings_repository: &AppSettingsRepository,
    release_repository: &impl ReleaseRepository,
) -> Result<(), Report> {
    if cfg!(target_family = "unix") {
        return Err(eyre!(
            "The install command is not supported on unix-based operating systems"
        ));
    }
    info!("Installing the {} source port...", source_port);
    let app_settings = app_settings_repository.get()?;
    let object_repo = ObjectRepository::new(&app_settings.release_cache_path)?;
    let user_settings = get_user_settings()?;
    let release = get_latest_source_port_release(source_port, release_repository, &object_repo)?;
    if is_source_port_installed(&release, &app_settings) {
        return Err(eyre!(format!(
            "Version {} of {} is already installed",
            release.version, release.source_port
        )));
    }
    let mut sp_dest_path = user_settings.source_ports_path.join(format!(
        "{}-{}",
        source_port.get_default_install_dir_name(),
        release.version
    ));
    match install_source_port_release(release.clone(), sp_dest_path.clone()) {
        Ok(()) => {
            sp_dest_path.push(source_port.get_bin_name());
            add_source_port(
                app_settings_repository,
                source_port,
                sp_dest_path,
                release.version,
            )
        }
        Err(error) => match error {
            SourcePortError::InstallDestinationExistsError(_) => {
                return Err(eyre!(error)
                    .wrap_err(format!(
                        "Failed to install the latest version of {}",
                        source_port
                    ))
                    .suggestion(format!(
                        "Remove the {} directory and run the command again",
                        sp_dest_path.clone().display().to_string()
                    )));
            }
            SourcePortError::AssetNotFoundError(_, _, _) => {
                return Err(eyre!(error)
                    .wrap_err(format!(
                        "Failed to install the latest version of {}",
                        source_port
                    ))
                    .suggestion(
                        "You can try the command again with the --version argument \
                        to install a specific version",
                    )
                    .suggestion(format!(
                        "Check the Github repository for {} to see what versions are available",
                        source_port
                    )));
            }
            _ => Err(eyre!(error)),
        },
    }
}

fn add_source_port(
    app_settings_repository: &AppSettingsRepository,
    source_port: SourcePort,
    path: PathBuf,
    version: String,
) -> Result<(), Report> {
    let isp = InstalledSourcePort::new(source_port, path, &version)?;
    let mut settings = app_settings_repository.get()?;
    if settings
        .source_ports
        .iter()
        .any(|sp| sp.name == source_port && sp.version == version)
    {
        return Err(eyre!(format!(
            "There is already a {:?} source port at version {}",
            source_port, version
        ))
        .suggestion("Try adding one with a different name or version"));
    }
    settings.source_ports.push(isp);
    app_settings_repository.save(settings)?;
    info!("Added version {} of {:?}", version, source_port);
    Ok(())
}

fn is_source_port_installed(
    source_port_release: &SourcePortRelease,
    settings: &AppSettings,
) -> bool {
    settings.source_ports.iter().any(|sp| {
        sp.name == source_port_release.source_port && sp.version == source_port_release.version
    })
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
