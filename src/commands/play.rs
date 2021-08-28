use crate::profile::Profile;
use crate::settings::get_app_settings_dir_path;
use crate::settings::get_user_settings;
use crate::settings::SettingsRegistry;
use crate::settings::SettingsRepository;
use crate::source_port::Skill;
use crate::source_port::SourcePort;
use crate::storage::ObjectRepository;
use crate::wad::WadEntry;
use color_eyre::{eyre::eyre, eyre::WrapErr, Report, Result};
use log::info;
use std::path::Path;

pub fn run_play_cmd(
    megawad: String,
    map: Option<String>,
    profile: Option<String>,
    repository: impl SettingsRepository,
) -> Result<(), Report> {
    let settings = repository.get()?;
    let selected_profile = get_profile(&settings, profile)?;
    let source_port = get_source_port(&settings, selected_profile)?;
    let args = get_args(selected_profile, &megawad, &map)?;
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

fn get_args(profile: &Profile, megawad: &str, map: &Option<String>) -> Result<Vec<String>, Report> {
    let mut wads_path = get_app_settings_dir_path()?;
    wads_path.push("wads");

    let repository = ObjectRepository::new(&wads_path)?;
    let wad_entry: WadEntry = repository.get(&String::from(megawad))?;

    let user_settings = get_user_settings()?;
    let mut iwad_entry_pb = user_settings.iwads_path;
    iwad_entry_pb.push(wad_entry.name);

    let mut args: Vec<String> = vec![
        "-iwad".to_string(),
        iwad_entry_pb.as_path().to_str().unwrap().to_string(),
        "-skill".to_string(),
    ];
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
    if map.is_some() {
        let map = wad_entry
            .maps
            .iter()
            .find(|x| x.number == *map.as_ref().unwrap())
            .ok_or_else(|| eyre!("Could not find {} in {}", map.as_ref().unwrap(), megawad))?;
        args.push("-warp".to_string());
        if map.warp.contains(' ') {
            // DOOM format.
            args.push(String::from(map.warp.chars().next().unwrap()));
            args.push(String::from(map.warp.chars().nth(2).unwrap()));
        } else {
            // DOOM2 format.
            args.push(map.warp.clone());
        }
    }
    Ok(args)
}

fn print_play_info(source_port_path: impl AsRef<Path>, args: &[String]) {
    info!("Running play command");
    info!("Launching {}", source_port_path.as_ref().display());
    info!("Using arguments: {}", args.join(" "));
}
