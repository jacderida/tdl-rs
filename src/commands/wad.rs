use crate::wad::WadMetadata;
use color_eyre::{Report, Result};
use log::{debug, info};
use prettytable::{cell, row, Table};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WadCommand {
    #[structopt(name = "lsdir")]
    /// Lists the directories of a given WAD
    LsDir {
        #[structopt(short, long)]
        /// Specify the name of an imported WAD.
        name: Option<String>,
        #[structopt(short, long)]
        /// Specify a path to any WAD file.
        path: Option<PathBuf>,
    },
}

pub fn run_wad_cmd(cmd: WadCommand) -> Result<(), Report> {
    match cmd {
        WadCommand::LsDir { name, path } => {
            if let Some(path) = path {
                let wad = WadMetadata::from_path(path)?;
                debug!(
                    "WAD header information: type: {}, directory entries: {}, directory offset: {} ",
                    wad.header.wad_type, wad.header.directory_entries, wad.header.directory_offset
                );
                info!("Directory has {} entries", wad.directory.len());
                let mut table = Table::new();
                table.add_row(row!["Lump Name", "Lump Size", "Lump Offset"]);
                for entry in wad.directory {
                    table.add_row(row![entry.lump_name, entry.lump_size, entry.lump_offset]);
                }
                table.printstd();
            }
        }
    }
    Ok(())
}
