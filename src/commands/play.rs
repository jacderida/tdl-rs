use color_eyre::{eyre::WrapErr, Report, Result};
use std::io::{self, Write};
use std::process::Command;

pub fn run_play_cmd(megawad: String) -> Result<(), Report> {
    let output = Command::new("./prboom-plus")
        .current_dir("/home/chris/doom/source-ports/prboom-2.6um")
        .arg("-iwad")
        .arg(format!("/home/chris/doom/iwads/{}.WAD", megawad))
        .output()
        .wrap_err("failed to run source port")?;
    io::stdout().write_all(&output.stdout).unwrap();
    io::stdout().write_all(&output.stderr).unwrap();
    Ok(())
}
