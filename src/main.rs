mod commands;
mod profile;
mod settings;
mod source_port;

use crate::commands::play::run_play_cmd;
use crate::commands::profile::run_profile_cmd;
use crate::commands::source_port::run_source_port_cmd;
use crate::commands::Command;

use color_eyre::{Report, Result};
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
    let args = CmdArgs::from_args();
    let result = match args.cmd {
        Some(Command::Play { megawad }) => run_play_cmd(megawad),
        Some(Command::Profile { cmd }) => run_profile_cmd(cmd),
        Some(Command::SourcePort { cmd }) => run_source_port_cmd(cmd),
        None => panic!("Eventually go into interactive mode"),
    };
    result
}
