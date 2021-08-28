mod commands;
mod find;
mod profile;
mod settings;
mod source_port;
mod storage;
mod wad;

use crate::commands::iwad::run_iwad_cmd;
use crate::commands::play::run_play_cmd;
use crate::commands::profile::run_profile_cmd;
use crate::commands::source_port::run_source_port_cmd;
use crate::commands::wad::run_wad_cmd;
use crate::commands::Command;
use crate::find::select_map_to_play;
use crate::settings::get_app_settings_dir_path;
use crate::settings::FileSettingsRepository;
use color_eyre::{Report, Result};
use env_logger::Env;
use log::info;
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};

#[derive(StructOpt, Debug)]
/// Terminal Doom Launcher
#[structopt(global_settings(&[ColoredHelp]))]
pub struct CmdArgs {
    /// subcommands
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    let mut app_settings_path = get_app_settings_dir_path()?;
    app_settings_path.push("app_settings.json");
    let repository = FileSettingsRepository::new(app_settings_path)?;
    let args = CmdArgs::from_args();
    let result = match args.cmd {
        Some(Command::Play {
            megawad,
            map,
            profile,
        }) => {
            if let Some(wad_to_play) = megawad {
                run_play_cmd(wad_to_play, map, profile, repository)
            } else {
                let selected = select_map_to_play()?;
                let wad_to_play = selected.0;
                let map_to_play = Some(selected.1);
                info!(
                    "Selected {}: {}",
                    &wad_to_play,
                    map_to_play.as_ref().unwrap()
                );
                run_play_cmd(wad_to_play, map_to_play, profile, repository)
            }
        }
        Some(Command::Profile { cmd }) => run_profile_cmd(cmd, repository),
        Some(Command::SourcePort { cmd }) => run_source_port_cmd(cmd, &repository),
        Some(Command::Wad { cmd }) => run_wad_cmd(cmd),
        Some(Command::Iwad { cmd }) => run_iwad_cmd(cmd),
        None => panic!("Eventually go into interactive mode"),
    };
    result
}
