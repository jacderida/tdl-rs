mod commands;
mod profile;
mod settings;
mod source_port;
mod wad;

use crate::commands::play::run_play_cmd;
use crate::commands::profile::run_profile_cmd;
use crate::commands::source_port::run_source_port_cmd;
use crate::commands::wad::run_wad_cmd;
use crate::commands::Command;
use crate::settings::FileSettingsRepository;
use color_eyre::{Report, Result};
use env_logger::Env;
use std::path::PathBuf;
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
    // TODO: Obviously remove use of hard coded settings reference.
    let settings_path = std::env::var("TDL_SETTINGS_PATH")
        .unwrap_or_else(|_| "/home/chris/.config/tdl/settings.json".to_string());
    let repository = FileSettingsRepository::new(PathBuf::from(settings_path))?;
    let args = CmdArgs::from_args();
    let result = match args.cmd {
        Some(Command::Play { megawad, profile }) => run_play_cmd(megawad, profile, repository),
        Some(Command::Profile { cmd }) => run_profile_cmd(cmd, repository),
        Some(Command::SourcePort { cmd }) => run_source_port_cmd(cmd, &repository),
        Some(Command::Wad { cmd }) => run_wad_cmd(cmd),
        None => panic!("Eventually go into interactive mode"),
    };
    result
}
