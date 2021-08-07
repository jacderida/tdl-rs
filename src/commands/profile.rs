use color_eyre::{Report, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum ProfileCommand {
    #[structopt(name = "add")]
    /// Add a profile
    Add {
        /// The name of the profile
        name: String,
        /// The source port for the profile
        source_port: String,
        #[structopt(short, long)]
        /// Controls whether this profile runs in fullscreen mode
        fullscreen: bool,
    },
}

pub fn run_profile_cmd(cmd: ProfileCommand) -> Result<(), Report> {
    Ok(())
}
