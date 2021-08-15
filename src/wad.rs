use byteorder::{LittleEndian, ReadBytesExt};
use color_eyre::{eyre::eyre, Help, Report, Result};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::Path;

const DIRECTORY_ENTRY_SIZE: u32 = 16;

pub struct WadHeader {
    pub wad_type: String,
    pub directory_entries: u32,
    pub directory_offset: u32,
}

pub struct WadDirectoryEntry {
    pub lump_offset: u32,
    pub lump_size: u32,
    pub lump_name: String,
}

pub struct WadMetadata {
    pub header: WadHeader,
    pub directory: Vec<WadDirectoryEntry>,
}

impl WadMetadata {
    pub fn from_path(wad_path: impl AsRef<Path>) -> Result<WadMetadata, Report> {
        let mut file = File::open(&wad_path)?;
        let header = WadMetadata::read_header(&wad_path, &mut file)?;
        let directory = WadMetadata::read_directory(&mut file, &header)?;
        Ok(WadMetadata { header, directory })
    }

    fn read_header(
        wad_path: &impl AsRef<Path>,
        wad_file: &mut impl Read,
    ) -> Result<WadHeader, Report> {
        let mut header = [0; 12];
        wad_file.read(&mut header)?;
        let wad_type = String::from_utf8(header[..4].to_vec())?;
        if wad_type != "IWAD" && wad_type != "PWAD" {
            return Err(
                eyre!(format!("Failed to parse {}", wad_path.as_ref().display()))
                    .suggestion("This file is likely not a WAD file"),
            );
        }
        let mut cursor = Cursor::new(header[4..].to_vec());
        let directory_entries = cursor.read_u32::<LittleEndian>()?;
        let directory_offset = cursor.read_u32::<LittleEndian>()?;
        Ok(WadHeader {
            wad_type,
            directory_entries,
            directory_offset,
        })
    }

    fn read_directory(
        wad_file: &mut (impl Read + std::io::Seek),
        header: &WadHeader,
    ) -> Result<Vec<WadDirectoryEntry>, Report> {
        let capacity = &header.directory_entries * DIRECTORY_ENTRY_SIZE;
        let mut directory_buf: Vec<u8> = Vec::with_capacity(capacity.try_into()?);
        &wad_file.seek(SeekFrom::Start(header.directory_offset.into()))?;
        &wad_file.read_to_end(&mut directory_buf)?;

        let mut directory: Vec<WadDirectoryEntry> = Vec::new();
        let mut cursor = Cursor::new(directory_buf);
        let mut entries_read = 0;
        while entries_read < header.directory_entries {
            let lump_offset = cursor.read_u32::<LittleEndian>()?;
            let lump_size = cursor.read_u32::<LittleEndian>()?;
            let mut buffer = [0; 8];
            cursor.read(&mut buffer)?;
            let lump_name = String::from_utf8(buffer.to_vec())?;
            let lump_name = lump_name.trim_matches(char::from(0)).to_string(); // Strip UTF-8 terminator.
            directory.push(WadDirectoryEntry {
                lump_offset,
                lump_size,
                lump_name,
            });
            entries_read += 1;
        }
        Ok(directory)
    }
}
