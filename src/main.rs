mod commands;
mod profile;
mod settings;
mod source_port;

use crate::commands::play::run_play_cmd;
use crate::commands::profile::run_profile_cmd;
use crate::commands::source_port::run_source_port_cmd;
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
    let args = CmdArgs::from_args();
    let result = match args.cmd {
        Some(Command::Play { megawad }) => run_play_cmd(megawad),
        Some(Command::Profile { cmd }) => run_profile_cmd(cmd),
        Some(Command::SourcePort { cmd }) => {
            let repository = FileSettingsRepository::new(PathBuf::from(
                "/home/chris/.config/tdl/settings.json",
            ))?;
            run_source_port_cmd(cmd, &repository)
        }
        None => panic!("Eventually go into interactive mode"),
    };
    result
}
