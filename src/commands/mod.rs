pub mod iwad;
pub mod play;
pub mod profile;
pub mod source_port;
pub mod wad;

use crate::commands::iwad::IwadCommand;
use crate::commands::profile::ProfileCommand;
use crate::commands::source_port::SourcePortCommand;
use crate::commands::wad::WadCommand;
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
        #[structopt(short, long)]
        /// The megawad to play, e.g. DOOM2
        megawad: Option<String>,
        #[structopt(short, long)]
        /// Specify the profile to play with. If not supplied, the default profile will be used.
        profile: Option<String>,
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
    /// IWAD Management
    Iwad {
        #[structopt(subcommand)]
        cmd: IwadCommand,
    },
    /// WAD Management
    Wad {
        #[structopt(subcommand)]
        cmd: WadCommand,
    },
}
