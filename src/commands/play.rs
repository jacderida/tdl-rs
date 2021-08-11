use crate::profile::Profile;
use crate::settings::SettingsRegistry;
use crate::settings::SettingsRepository;
use crate::source_port::Skill;
use crate::source_port::SourcePort;
use color_eyre::{eyre::eyre, eyre::WrapErr, Report, Result};
use log::info;
use std::path::Path;

pub fn run_play_cmd(
    megawad: String,
    profile: Option<String>,
    repository: impl SettingsRepository,
) -> Result<(), Report> {
    let settings = repository.get()?;
    let selected_profile = get_profile(&settings, profile)?;
    let source_port = get_source_port(&settings, &selected_profile)?;
    let args = get_args(selected_profile, &megawad);
    let mut source_port_path = source_port.path.to_owned();

    print_play_info(&source_port_path, &args);
    source_port_path.pop();
    let output = duct::cmd(&source_port.path, &args)
        .dir(source_port_path.to_str().unwrap())
        .read()?;
    println!("{}", output);
    Ok(())
}

fn get_profile(settings: &SettingsRegistry, profile: Option<String>) -> Result<&Profile, Report> {
    if let Some(p) = profile {
        let selected = settings
            .profiles
            .iter()
            .find(|x| x.name == p)
            .ok_or_else(|| eyre!("Failed to find '{}' profile", p))
            .wrap_err(
                "This could indicate the settings.json file was modified outside the application",
            )?;
        return Ok(selected);
    }
    let default = settings
        .profiles
        .iter()
        .find(|p| p.default)
        .ok_or_else(|| eyre!("Failed to find a default profile"))
        .wrap_err(
            "This could indicate the settings.json file was modified outside the application",
        )?;
    Ok(default)
}

fn get_source_port<'a>(
    settings: &'a SettingsRegistry,
    profile: &'a Profile,
) -> Result<&'a SourcePort, Report> {
    let source_port = settings
        .source_ports
        .iter()
        .find(|sp| sp.source_port_type == profile.source_port_type)
        .ok_or_else(|| {
            eyre!(
                "Failed to find the {:?}-{} source port",
                profile.source_port_type,
                profile.source_port_version
            )
        })
        .wrap_err(
            "This could indicate the settings.json file was modified outside the application",
        )?;
    Ok(source_port)
}

fn get_args(profile: &Profile, megawad: &String) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    args.push("-iwad".to_string());
    args.push(megawad.to_owned());
    args.push("-skill".to_string());
    match profile.skill {
        Skill::Nightmare => args.push("5".to_string()),
        Skill::UltraViolence => args.push("4".to_string()),
        Skill::HurtMePlenty => args.push("3".to_string()),
        Skill::HeyNotTooRough => args.push("2".to_string()),
        Skill::TooYoungToDie => args.push("1".to_string()),
    }
    if !profile.music {
        args.push("-nomusic".to_string());
    }
    if !profile.fullscreen {
        args.push("-nofullscreen".to_string());
    }
    args
}

fn print_play_info(source_port_path: impl AsRef<Path>, args: &Vec<String>) {
    info!("Running play command");
    info!("Launching {}", source_port_path.as_ref().display());
    info!("Using arguments: {}", args.join(" "));
}
