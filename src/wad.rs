use byteorder::{LittleEndian, ReadBytesExt};
use color_eyre::{eyre::ensure, eyre::eyre, Help, Report, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::Path;

const DIRECTORY_ENTRY_SIZE: u32 = 16;
lazy_static! {
    /// These regexes are used in a loop when IWADs or PWADs are being imported, and compiling the
    /// regex each time meant it actually would take a significant amount of time to perform the
    /// import. For that reason, they are compiled once.
    ///
    /// It's reasonable to perform an unwrap here because I know the regex is valid.
    static ref DOOM2_FORMAT_REGEX: Regex = Regex::new("^MAP0{1}[1-9]|MAP[0-9]{2}$").unwrap();
    static ref DOOM_FORMAT_REGEX: Regex = Regex::new("^E[1-9]{1}M[1-9]{1}$").unwrap();
}

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

/// Structure that describes a map in the game.
///
/// The number will either be in MAPxx, for DOOM2, or ExMx for DOOM. For example, MAP03, or E1M3.
/// The map will also have a name. Newer WADs contain MAPINFO lumps in the WAD, but the original
/// IWADs did not contain these.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapInfo {
    /// The map number. Either MAPxx or ExMx.
    pub number: String,
    /// The name of the map.
    pub name: String,
    /// The value to be used with the `-warp` argument on source ports. For DOOM2, this will be the
    /// xx part of the number, e.g., for MAP01, `warp` will be 1, and for MAP12, it will be 12. For
    /// DOOM, E1M1 will turn into "1 1". It's due to the latter that we store this as a string
    /// rather than an integer.
    pub warp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WadEntry {
    pub id: String,
    pub name: String,
    pub title: String,
    pub release_date: String,
    pub author: String,
    pub maps: Vec<MapInfo>,
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

impl MapInfo {
    pub fn new(number: String, name: String) -> Result<MapInfo, Report> {
        ensure!(
            !number.is_empty(),
            "A number must be provided for the map. It should be in the DOOM or DOOM2 format."
        );
        ensure!(
            MapInfo::is_valid_map_number(&number),
            "The map number must be in the DOOM or DOOM2 format. Valid values are ExMx or MAPxx."
        );
        ensure!(!name.is_empty(), "A name must be provided for the map.");

        let warp: String;
        if DOOM2_FORMAT_REGEX.is_match(&number) {
            let num_part = &number[3..];
            if num_part.starts_with("0") {
                warp = String::from(num_part.chars().nth(1).unwrap());
            } else {
                warp = String::from(num_part);
            }
        } else {
            warp = format!(
                "{} {}",
                number.chars().nth(1).unwrap(),
                number.chars().nth(3).unwrap()
            );
        }
        Ok(MapInfo { number, name, warp })
    }

    pub fn is_valid_map_number(number: &String) -> bool {
        DOOM_FORMAT_REGEX.is_match(&number) || DOOM2_FORMAT_REGEX.is_match(&number)
    }
}

impl WadEntry {
    pub fn new(
        id: String,
        name: String,
        title: String,
        release_date: String,
        author: String,
        maps: Vec<MapInfo>,
    ) -> Result<WadEntry, Report> {
        ensure!(!id.is_empty(), "The ID for the WAD entry must be set.");
        ensure!(!name.is_empty(), "The name for the WAD entry must be set.");
        ensure!(
            !title.is_empty(),
            "The title for the WAD entry must be set."
        );
        ensure!(
            !release_date.is_empty(),
            "The release date for the WAD entry must be set."
        );
        ensure!(
            !author.is_empty(),
            "The author for the WAD entry must be set."
        );
        Ok(WadEntry {
            id,
            name,
            title,
            release_date,
            author,
            maps,
        })
    }
}

#[cfg(test)]
mod mapinfo {
    use super::MapInfo;

    #[test]
    fn constructor_should_permit_a_map_in_the_doom2_format() {
        let map = MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap();
        assert_eq!("MAP01", map.number);
        assert_eq!("Entryway", map.name);
        assert_eq!("1", map.warp);

        let map = MapInfo::new("MAP12".to_string(), "The Factory".to_string()).unwrap();
        assert_eq!("MAP12", map.number);
        assert_eq!("The Factory", map.name);
        assert_eq!("12", map.warp);
    }

    #[test]
    fn constructor_should_permit_a_map_in_the_doom_format() {
        let map = MapInfo::new("E1M1".to_string(), "Hanger".to_string()).unwrap();
        assert_eq!("E1M1", map.number);
        assert_eq!("Hanger", map.name);
        assert_eq!("1 1", map.warp);

        let map = MapInfo::new("E4M8".to_string(), "Hanger".to_string()).unwrap();
        assert_eq!("E4M8", map.number);
        assert_eq!("Hanger", map.name);
        assert_eq!("4 8", map.warp);
    }

    #[test]
    fn constructor_should_ensure_the_map_number_is_set() {
        let result = MapInfo::new("".to_string(), "Entryway".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "A number must be provided for the map. It should be in the DOOM or DOOM2 format."
        );
    }

    #[test]
    fn constructor_should_ensure_the_map_name_is_set() {
        let result = MapInfo::new("MAP01".to_string(), "".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "A name must be provided for the map."
        );
    }

    #[test]
    fn constructor_should_reject_map_number_in_wrong_format() {
        let result = MapInfo::new("MAP1".to_string(), "Entryway".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The map number must be in the DOOM or DOOM2 format. Valid values are ExMx or MAPxx."
        );

        let result = MapInfo::new("map01".to_string(), "Entryway".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The map number must be in the DOOM or DOOM2 format. Valid values are ExMx or MAPxx."
        );

        let result = MapInfo::new("MA01".to_string(), "Entryway".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The map number must be in the DOOM or DOOM2 format. Valid values are ExMx or MAPxx."
        );

        let result = MapInfo::new("e1m1".to_string(), "Hanger".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The map number must be in the DOOM or DOOM2 format. Valid values are ExMx or MAPxx."
        );

        // DOOM.WAD contains lumps named `D_ExMx`, which are music data for the maps. The original
        // regex matched on these lumps.
        let result = MapInfo::new("D_E1M1".to_string(), "Hanger".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The map number must be in the DOOM or DOOM2 format. Valid values are ExMx or MAPxx."
        );
    }
}

#[cfg(test)]
mod wadentry {
    use super::MapInfo;
    use super::WadEntry;

    #[test]
    fn constructor_should_set_fields_correctly() {
        let maps = vec![
            MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
            MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
            MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
        ];
        let entry = WadEntry::new(
            "DOOM2".to_string(),
            "DOOM2.WAD".to_string(),
            "Doom II: Hell on Earth".to_string(),
            "1994-09-30".to_string(),
            "id Software".to_string(),
            maps,
        )
        .unwrap();
        assert_eq!(entry.id, "DOOM2");
        assert_eq!(entry.name, "DOOM2.WAD");
        assert_eq!(entry.title, "Doom II: Hell on Earth");
        assert_eq!(entry.release_date, "1994-09-30");
        assert_eq!(entry.author, "id Software");
        assert_eq!(entry.maps.len(), 3);
    }

    #[test]
    fn constructor_should_ensure_the_id_is_set() {
        let maps = vec![
            MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
            MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
            MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
        ];
        let result = WadEntry::new(
            "".to_string(),
            "DOOM2.WAD".to_string(),
            "Doom II: Hell on Earth".to_string(),
            "1994-09-30".to_string(),
            "id Software".to_string(),
            maps,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The ID for the WAD entry must be set."
        );
    }

    #[test]
    fn constructor_should_ensure_the_name_is_set() {
        let maps = vec![
            MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
            MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
            MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
        ];
        let result = WadEntry::new(
            "DOOM2".to_string(),
            "".to_string(),
            "Doom II: Hell on Earth".to_string(),
            "1994-09-30".to_string(),
            "id Software".to_string(),
            maps,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The name for the WAD entry must be set."
        );
    }

    #[test]
    fn constructor_should_ensure_the_title_is_set() {
        let maps = vec![
            MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
            MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
            MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
        ];
        let result = WadEntry::new(
            "DOOM2".to_string(),
            "DOOM2.WAD".to_string(),
            "".to_string(),
            "1994-09-30".to_string(),
            "id Software".to_string(),
            maps,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The title for the WAD entry must be set."
        );
    }

    #[test]
    fn constructor_should_ensure_the_release_date_is_set() {
        let maps = vec![
            MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
            MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
            MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
        ];
        let result = WadEntry::new(
            "DOOM2".to_string(),
            "DOOM2.WAD".to_string(),
            "Doom II: Hell on Earth".to_string(),
            "".to_string(),
            "id Software".to_string(),
            maps,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The release date for the WAD entry must be set."
        );
    }

    #[test]
    fn constructor_should_ensure_the_author_is_set() {
        let maps = vec![
            MapInfo::new("MAP01".to_string(), "Entryway".to_string()).unwrap(),
            MapInfo::new("MAP02".to_string(), "Underhalls".to_string()).unwrap(),
            MapInfo::new("MAP03".to_string(), "The Gantlet".to_string()).unwrap(),
        ];
        let result = WadEntry::new(
            "DOOM2".to_string(),
            "DOOM2.WAD".to_string(),
            "Doom II: Hell on Earth".to_string(),
            "1994-09-30".to_string(),
            "".to_string(),
            maps,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The author for the WAD entry must be set."
        );
    }
}
