use crate::source_port::SourcePort;
use color_eyre::{Report, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum SourcePortCommand {
    #[structopt(name = "add")]
    /// Adds a source port from an existing directory
    Add {
        /// The name of the source port
        name: String,
        /// The path of the source port executable
        path: PathBuf,
        /// The version of the source port
        version: String,
    },
}

pub fn run_source_port_cmd(cmd: SourcePortCommand) -> Result<(), Report> {
    match cmd {
        SourcePortCommand::Add {
            name,
            path,
            version,
        } => {
            let source_port = SourcePort::new(&name, path, &version)?;
            // use the settings registry to save the source port
            println!("Adding source port {}", source_port.name);
        }
    }
    Ok(())
}

//#[cfg(test)]
//mod tests {
//use super::run_source_port_cmd;
//use super::SourcePortCommand;
//use std::path::PathBuf;

//#[test]
//fn first_source_port_should_be_saved() {
//let cmd = SourcePortCommand::Add {
//name: "prboom".to_string(),
//path: PathBuf::new(),
//version: "2.6".to_string(),
//};
//}
//}
