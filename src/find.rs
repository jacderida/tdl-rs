use crate::settings::get_app_settings_dir_path;
use crate::storage::ObjectRepository;
use crate::wad::WadEntry;
use color_eyre::{Report, Result};
#[cfg(target_family = "unix")]
use skim::prelude::*;
use std::io::Cursor;
use std::path::Path;

#[cfg(target_family = "unix")]
pub fn select_map_to_play<'a>() -> Result<(String, String), Report> {
    let mut wads_path = get_app_settings_dir_path()?;
    wads_path.push("wads");

    let mut wad_entries: Vec<WadEntry> = Vec::new();
    let repository = ObjectRepository::new(&wads_path)?;
    for dir_entry in std::fs::read_dir(wads_path)? {
        let file_name = dir_entry?.file_name();
        let id = Path::new(&file_name).file_stem().unwrap().to_str().unwrap();
        let wad: WadEntry = repository.get(&String::from(id))?;
        wad_entries.push(wad);
    }

    let mut search_entries = String::new();
    for entry in wad_entries {
        for map in entry.maps {
            search_entries.push_str(&format!("{} {} {}\n", entry.name, map.number, map.name));
        }
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("70%"))
        .multi(false)
        .prompt(Some("Please select a map to play\n"))
        .build()
        .unwrap();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(search_entries));
    let selected = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap();
    let split: Vec<String> = selected
        .iter()
        .nth(0)
        .unwrap()
        .output()
        .split(" ")
        .map(|s| String::from(s))
        .collect();
    let selected_wad = Path::new(&split[0].clone())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    Ok((String::from(selected_wad), split[1].clone()))
}

#[cfg(target_family = "windows")]
pub fn select_map_to_play<'a>() -> Result<(String, String), Report> {
    unimplemented!();
}
