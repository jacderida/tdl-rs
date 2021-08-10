pub mod play;
pub mod profile;
pub mod source_port;

use crate::commands::profile::ProfileCommand;
use crate::commands::source_port::SourcePortCommand;
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(
        name = "play",
        no_version,
        global_settings(&[AppSettings::DisableVersion]),
    )]
    /// Plays a megawad
    Play {
        #[structopt(required = true)]
        /// The megawad to play, e.g. DOOM2
        megawad: String,
    },
    #[structopt(
        name = "profile",
        no_version,
        global_settings(&[AppSettings::DisableVersion]),
    )]
    /// Profile management
    Profile {
        #[structopt(subcommand)]
        cmd: ProfileCommand,
    },
    #[structopt(
        name = "source-port",
        no_version,
        global_settings(&[AppSettings::DisableVersion]),
    )]
    /// Source Port management
    SourcePort {
        #[structopt(subcommand)]
        cmd: SourcePortCommand,
    },
}