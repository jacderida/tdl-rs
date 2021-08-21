use crate::settings::get_app_settings_dir_path;
use crate::settings::get_user_settings;
use crate::storage::ObjectRepository;
use crate::wad::{MapInfo, WadEntry, WadMetadata};
use color_eyre::{eyre::eyre, Report, Result};
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum IwadCommand {
    #[structopt(name = "import")]
    /// Import an IWAD into your collection
    Import {
        /// Specify the path to the IWAD file.
        path: PathBuf,
    },
}

lazy_static! {
    static ref DOOM_TITLE: &'static str = "The Ultimate DOOM";
    static ref DOOM_RELEASE_DATE: &'static str = "1995-04-30";
    static ref DOOM_AUTHOR: &'static str = "id Software";
    static ref DOOM_MAP_INFO: HashMap<&'static str, &'static str> = {
        maplit::hashmap! {
            "E1M1" => "Hanger",
            "E1M2" => "Nuclear Plant",
            "E1M3" => "Toxin Refinery",
            "E1M4" => "Command Control",
            "E1M5" => "Phobos Lab",
            "E1M6" => "Central Processing",
            "E1M7" => "Computer Station",
            "E1M8" => "Phobos Anomaly",
            "E1M9" => "Military Base",
            "E2M1" => "Deimos Anomaly",
            "E2M2" => "Containment Area",
            "E2M3" => "Refinery",
            "E2M4" => "Deimos Lab",
            "E2M5" => "Command Center",
            "E2M6" => "Halls of the Damned",
            "E2M7" => "Spawning Vats",
            "E2M8" => "Tower of Babel",
            "E2M9" => "Fortress of Mystery",
            "E3M1" => "Hell Keep",
            "E3M2" => "Slough of Despair",
            "E3M3" => "Pandemonium",
            "E3M4" => "House of Pain",
            "E3M5" => "Unholy Cathedral",
            "E3M6" => "Mt. Erebus",
            "E3M7" => "Limbo",
            "E3M8" => "Dis",
            "E3M9" => "Warrens",
            "E4M1" => "Hell Beneath",
            "E4M2" => "Perfect Hatred",
            "E4M3" => "Sever the Wicked",
            "E4M4" => "Unruly Evil",
            "E4M5" => "They Will Repent",
            "E4M6" => "Against Thee Wickedly",
            "E4M7" => "And Hell Followed",
            "E4M8" => "Unto the Cruel",
            "E4M9" => "Fear",
        }
    };
    static ref DOOM2_TITLE: &'static str = "Doom II: Hell on Earth";
    static ref DOOM2_RELEASE_DATE: &'static str = "1994-09-30";
    static ref DOOM2_AUTHOR: &'static str = "id Software";
    static ref DOOM2_MAP_INFO: HashMap<&'static str, &'static str> = {
        maplit::hashmap! {
            "MAP01" => "Entryway",
            "MAP02" => "Underhalls",
            "MAP03" => "The Gantlet",
            "MAP04" => "The Focus",
            "MAP05" => "The Waste Tunnels",
            "MAP06" => "The Crusher",
            "MAP07" => "Dead Simple",
            "MAP08" => "Tricks and Traps",
            "MAP09" => "The Pit",
            "MAP10" => "Refueling Base",
            "MAP11" => "'O' of Destruction!",
            "MAP12" => "The Factory",
            "MAP13" => "Downtown",
            "MAP14" => "The Inmost Dens",
            "MAP15" => "Industrial Zone",
            "MAP16" => "Suburbs",
            "MAP17" => "Tenements",
            "MAP18" => "The Courtyard",
            "MAP19" => "The Citadel",
            "MAP20" => "Gotcha!",
            "MAP21" => "Nirvana",
            "MAP22" => "The Catacombs",
            "MAP23" => "Barrels o' Fun",
            "MAP24" => "The Chasm",
            "MAP25" => "Bloodfalls",
            "MAP26" => "The Abandoned Mines",
            "MAP27" => "Monster Condo",
            "MAP28" => "The Spirit World",
            "MAP29" => "The Living End",
            "MAP30" => "Icon of Sin",
            "MAP31" => "Wolfenstein",
            "MAP32" => "Grosse",
        }
    };
    static ref PLUTONIA_TITLE: &'static str = "The Plutonia Experiment";
    static ref PLUTONIA_RELEASE_DATE: &'static str = "1996-06-17";
    static ref PLUTONIA_AUTHOR: &'static str = "Dario Casali & Milo Casali";
    static ref PLUTONIA_MAP_INFO: HashMap<&'static str, &'static str> = {
        maplit::hashmap! {
            "MAP01" => "Congo",
            "MAP02" => "Well of Souls",
            "MAP03" => "Aztec",
            "MAP04" => "Caged",
            "MAP05" => "Ghost Town",
            "MAP06" => "Baron's Lair",
            "MAP07" => "Caughtyard",
            "MAP08" => "Realm",
            "MAP09" => "Abattoire",
            "MAP10" => "Onslaught",
            "MAP11" => "Hunted",
            "MAP12" => "Speed",
            "MAP13" => "The Crypt",
            "MAP14" => "Genesis",
            "MAP15" => "The Twilight",
            "MAP16" => "The Omen",
            "MAP17" => "Compound",
            "MAP18" => "Neurosphere",
            "MAP19" => "NME",
            "MAP20" => "The Death Domain",
            "MAP21" => "Slayer",
            "MAP22" => "Impossible Mission",
            "MAP23" => "Tombstone",
            "MAP24" => "The Final Frontier",
            "MAP25" => "The Temple of Darkness",
            "MAP26" => "Bunker",
            "MAP27" => "Anti-Christ",
            "MAP28" => "The Sewers",
            "MAP29" => "Odyssey of Noises",
            "MAP30" => "The Gateway of Hell",
            "MAP31" => "Cyberden",
            "MAP32" => "Go 2 It",
        }
    };
    static ref TNT_TITLE: &'static str = "TNT: Evilution";
    static ref TNT_RELEASE_DATE: &'static str = "1996-06-17";
    static ref TNT_AUTHOR: &'static str = "TeamTNT";
    static ref TNT_MAP_INFO: HashMap<&'static str, &'static str> = {
        maplit::hashmap! {
            "MAP01" => "System Control",
            "MAP02" => "Human BBQ",
            "MAP03" => "Power Control",
            "MAP04" => "Wormhole",
            "MAP05" => "Hanger",
            "MAP06" => "Open Season",
            "MAP07" => "Prison",
            "MAP08" => "Metal",
            "MAP09" => "Stronghold",
            "MAP10" => "Redemption",
            "MAP11" => "Storage Facility",
            "MAP12" => "Crater",
            "MAP13" => "Nukage",
            "MAP14" => "Steel Works",
            "MAP15" => "Dead Zone",
            "MAP16" => "Deepest Reaches",
            "MAP17" => "Processing Area",
            "MAP18" => "Mill",
            "MAP19" => "Shipping/Respawning",
            "MAP20" => "Central Processing",
            "MAP21" => "Administration Center",
            "MAP22" => "Habitat",
            "MAP23" => "Lunar Mining Project",
            "MAP24" => "Quarry",
            "MAP25" => "Baron's Den",
            "MAP26" => "Ballistyx",
            "MAP27" => "Mount Pain",
            "MAP28" => "Heck",
            "MAP29" => "River Styx",
            "MAP30" => "Last Call",
            "MAP31" => "Pharaoh",
            "MAP32" => "Caribbean",
        }
    };
}

pub fn run_iwad_cmd(cmd: IwadCommand) -> Result<(), Report> {
    match cmd {
        IwadCommand::Import { path } => {
            let metadata = WadMetadata::from_path(&path)?;
            let id = get_wad_entry_id(&path)?;
            let file_name = get_wad_file_name(&path)?;
            let maps = get_maps_from_metadata(&file_name, &metadata)?;
            let info = get_additional_wad_info(&file_name);
            let entry = WadEntry::new(id, file_name.to_string(), info.0, info.1, info.2, maps)?;
            print_wad_info(&path, &entry);
            save_wad_entry(&entry)?;
            import_wad_file(&path, &entry)?;
        }
    }
    Ok(())
}

fn get_wad_entry_id(path: &PathBuf) -> Result<String, Report> {
    let temp = path.to_owned();
    let file_name = temp
        .file_name()
        .ok_or_else(|| eyre!("Could not retrieve filename from path"))?;
    let file_name = PathBuf::from(file_name);
    let id = file_name
        .file_stem()
        .ok_or_else(|| eyre!("Could not parse the ID for the WAD"))?;
    Ok(String::from(id.to_str().unwrap()))
}

fn get_wad_file_name(path: &PathBuf) -> Result<&str, Report> {
    let file_name = path
        .file_name()
        .ok_or_else(|| eyre!("Could not retrieve filename from path"))?;
    let file_name = file_name
        .to_str()
        .ok_or_else(|| eyre!("Could not retrieve string"))?;
    Ok(file_name)
}

fn get_maps_from_metadata(
    wad_file_name: &str,
    metadata: &WadMetadata,
) -> Result<Vec<MapInfo>, Report> {
    // I originally used `map_filter` for this, but I actually find the filter, then map, more
    // readable for what I'm trying to do here.
    let map_entries: Vec<String> = metadata
        .directory
        .iter()
        .filter(|x| MapInfo::is_valid_map_number(&x.lump_name) && x.lump_size == 0)
        .map(|x| x.lump_name.clone())
        .collect();
    let map_info: &HashMap<&'static str, &'static str>;
    match wad_file_name {
        "DOOM2.WAD" => map_info = &DOOM2_MAP_INFO,
        "DOOM.WAD" => map_info = &DOOM_MAP_INFO,
        "PLUTONIA.WAD" => map_info = &PLUTONIA_MAP_INFO,
        "TNT.WAD" => map_info = &TNT_MAP_INFO,
        _ => panic!("IWAD not supported"),
    }
    let mut maps: Vec<MapInfo> = Vec::new();
    for map_entry in map_entries {
        let name = map_info.get(&map_entry as &str).unwrap();
        maps.push(MapInfo::new(map_entry.clone(), String::from(name.clone()))?);
    }
    Ok(maps)
}

fn get_additional_wad_info(wad_file_name: &str) -> (String, String, String) {
    let info: (String, String, String);
    match wad_file_name {
        "DOOM2.WAD" => {
            info = (
                DOOM2_TITLE.to_string(),
                DOOM2_RELEASE_DATE.to_string(),
                DOOM2_AUTHOR.to_string(),
            )
        }
        "DOOM.WAD" => {
            info = (
                DOOM_TITLE.to_string(),
                DOOM_RELEASE_DATE.to_string(),
                DOOM_AUTHOR.to_string(),
            )
        }
        "PLUTONIA.WAD" => {
            info = (
                PLUTONIA_TITLE.to_string(),
                PLUTONIA_RELEASE_DATE.to_string(),
                PLUTONIA_AUTHOR.to_string(),
            )
        }
        "TNT.WAD" => {
            info = (
                TNT_TITLE.to_string(),
                TNT_RELEASE_DATE.to_string(),
                TNT_AUTHOR.to_string(),
            )
        }
        _ => panic!("IWAD not supported"),
    }
    info
}

fn print_wad_info(path: &PathBuf, wad_entry: &WadEntry) {
    info!("Importing {}", path.display());
    info!("ID: {}", wad_entry.id);
    info!("WAD Name: {}", wad_entry.name);
    info!("Title: {}", wad_entry.title);
    info!("Released: {}", wad_entry.release_date);
    info!("Author: {}", wad_entry.author);
    for map in &wad_entry.maps {
        info!("{}: {}", map.number, map.name);
    }
}

fn save_wad_entry(wad_entry: &WadEntry) -> Result<(), Report> {
    let mut wads_entry_path = get_app_settings_dir_path()?;
    wads_entry_path.push("wads");
    let repository = ObjectRepository::new(&wads_entry_path)?;
    repository.save(&wad_entry.id, &wad_entry)?;
    Ok(())
}

fn import_wad_file(wad_path: &PathBuf, wad_entry: &WadEntry) -> Result<(), Report> {
    let settings = get_user_settings()?;
    let mut wad_import_path = settings.iwads_path.clone();
    wad_import_path.push(&wad_entry.name);
    std::fs::copy(wad_path, wad_import_path)?;
    Ok(())
}
