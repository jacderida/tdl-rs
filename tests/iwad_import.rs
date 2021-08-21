use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn doom2_iwad_import_should_print_the_correct_information() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM2.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("ID: DOOM2"))
        .stderr(predicate::str::contains("WAD Name: DOOM2.WAD"))
        .stderr(predicate::str::contains("Title: Doom II: Hell on Earth"))
        .stderr(predicate::str::contains("Released: 1994-09-30"))
        .stderr(predicate::str::contains("Author: id Software"))
        .stderr(predicate::str::contains("MAP01: Entryway"))
        .stderr(predicate::str::contains("MAP02: Underhalls"))
        .stderr(predicate::str::contains("MAP03: The Gantlet"))
        .stderr(predicate::str::contains("MAP04: The Focus"))
        .stderr(predicate::str::contains("MAP05: The Waste Tunnels"))
        .stderr(predicate::str::contains("MAP06: The Crusher"))
        .stderr(predicate::str::contains("MAP07: Dead Simple"))
        .stderr(predicate::str::contains("MAP08: Tricks and Traps"))
        .stderr(predicate::str::contains("MAP09: The Pit"))
        .stderr(predicate::str::contains("MAP10: Refueling Base"))
        .stderr(predicate::str::contains("MAP11: 'O' of Destruction!"))
        .stderr(predicate::str::contains("MAP12: The Factory"))
        .stderr(predicate::str::contains("MAP13: Downtown"))
        .stderr(predicate::str::contains("MAP14: The Inmost Dens"))
        .stderr(predicate::str::contains("MAP15: Industrial Zone"))
        .stderr(predicate::str::contains("MAP16: Suburbs"))
        .stderr(predicate::str::contains("MAP17: Tenements"))
        .stderr(predicate::str::contains("MAP18: The Courtyard"))
        .stderr(predicate::str::contains("MAP19: The Citadel"))
        .stderr(predicate::str::contains("MAP20: Gotcha!"))
        .stderr(predicate::str::contains("MAP21: Nirvana"))
        .stderr(predicate::str::contains("MAP22: The Catacombs"))
        .stderr(predicate::str::contains("MAP23: Barrels o' Fun"))
        .stderr(predicate::str::contains("MAP24: The Chasm"))
        .stderr(predicate::str::contains("MAP25: Bloodfalls"))
        .stderr(predicate::str::contains("MAP26: The Abandoned Mines"))
        .stderr(predicate::str::contains("MAP27: Monster Condo"))
        .stderr(predicate::str::contains("MAP28: The Spirit World"))
        .stderr(predicate::str::contains("MAP29: The Living End"))
        .stderr(predicate::str::contains("MAP30: Icon of Sin"))
        .stderr(predicate::str::contains("MAP31: Wolfenstein"))
        .stderr(predicate::str::contains("MAP32: Grosse"));

    // For this test, we are going to make the assumption that the contents of this file are
    // correct. This is a reasonable assumption, as the component that saves the imported entry has
    // been tested elsewhere and has been verfied to be working.
    let wad_entry_file = settings_dir.child("wads/DOOM2.json");
    wad_entry_file.assert(predicate::path::is_file());

    let iwad_file = doom_home_dir.child("iwads/DOOM2.WAD");
    iwad_file.assert(predicate::path::is_file());
    iwad_file.assert(predicate::path::eq_file("test_iwads/DOOM2.WAD"));
}

#[test]
fn doom_iwad_import_should_print_the_correct_information() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/DOOM.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("ID: DOOM"))
        .stderr(predicate::str::contains("WAD Name: DOOM.WAD"))
        .stderr(predicate::str::contains("Title: The Ultimate DOOM"))
        .stderr(predicate::str::contains("Released: 1995-04-30"))
        .stderr(predicate::str::contains("Author: id Software"))
        .stderr(predicate::str::contains("E1M1: Hanger"))
        .stderr(predicate::str::contains("E1M2: Nuclear Plant"))
        .stderr(predicate::str::contains("E1M3: Toxin Refinery"))
        .stderr(predicate::str::contains("E1M4: Command Control"))
        .stderr(predicate::str::contains("E1M5: Phobos Lab"))
        .stderr(predicate::str::contains("E1M6: Central Processing"))
        .stderr(predicate::str::contains("E1M7: Computer Station"))
        .stderr(predicate::str::contains("E1M8: Phobos Anomaly"))
        .stderr(predicate::str::contains("E1M9: Military Base"))
        .stderr(predicate::str::contains("E2M1: Deimos Anomaly"))
        .stderr(predicate::str::contains("E2M2: Containment Area"))
        .stderr(predicate::str::contains("E2M3: Refinery"))
        .stderr(predicate::str::contains("E2M4: Deimos Lab"))
        .stderr(predicate::str::contains("E2M5: Command Center"))
        .stderr(predicate::str::contains("E2M6: Halls of the Damned"))
        .stderr(predicate::str::contains("E2M7: Spawning Vats"))
        .stderr(predicate::str::contains("E2M8: Tower of Babel"))
        .stderr(predicate::str::contains("E2M9: Fortress of Mystery"))
        .stderr(predicate::str::contains("E3M1: Hell Keep"))
        .stderr(predicate::str::contains("E3M2: Slough of Despair"))
        .stderr(predicate::str::contains("E3M3: Pandemonium"))
        .stderr(predicate::str::contains("E3M4: House of Pain"))
        .stderr(predicate::str::contains("E3M5: Unholy Cathedral"))
        .stderr(predicate::str::contains("E3M6: Mt. Erebus"))
        .stderr(predicate::str::contains("E3M7: Limbo"))
        .stderr(predicate::str::contains("E3M8: Dis"))
        .stderr(predicate::str::contains("E3M9: Warrens"))
        .stderr(predicate::str::contains("E4M1: Hell Beneath"))
        .stderr(predicate::str::contains("E4M2: Perfect Hatred"))
        .stderr(predicate::str::contains("E4M3: Sever the Wicked"))
        .stderr(predicate::str::contains("E4M4: Unruly Evil"))
        .stderr(predicate::str::contains("E4M5: They Will Repent"))
        .stderr(predicate::str::contains("E4M6: Against Thee Wickedly"))
        .stderr(predicate::str::contains("E4M7: And Hell Followed"))
        .stderr(predicate::str::contains("E4M8: Unto the Cruel"))
        .stderr(predicate::str::contains("E4M9: Fear"));

    // For this test, we are going to make the assumption that the contents of this file are
    // correct. This is a reasonable assumption, as the component that saves the imported entry has
    // been tested elsewhere and has been verfied to be working.
    let wad_entry_file = settings_dir.child("wads/DOOM.json");
    wad_entry_file.assert(predicate::path::is_file());

    let iwad_file = doom_home_dir.child("iwads/DOOM.WAD");
    iwad_file.assert(predicate::path::is_file());
}

#[test]
fn plutonia_iwad_import_should_print_the_correct_information() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/PLUTONIA.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("ID: PLUTONIA"))
        .stderr(predicate::str::contains("WAD Name: PLUTONIA.WAD"))
        .stderr(predicate::str::contains("Title: The Plutonia Experiment"))
        .stderr(predicate::str::contains("Released: 1996-06-17"))
        .stderr(predicate::str::contains(
            "Author: Dario Casali & Milo Casali",
        ))
        .stderr(predicate::str::contains("MAP01: Congo"))
        .stderr(predicate::str::contains("MAP02: Well of Souls"))
        .stderr(predicate::str::contains("MAP03: Aztec"))
        .stderr(predicate::str::contains("MAP04: Caged"))
        .stderr(predicate::str::contains("MAP05: Ghost Town"))
        .stderr(predicate::str::contains("MAP06: Baron's Lair"))
        .stderr(predicate::str::contains("MAP07: Caughtyard"))
        .stderr(predicate::str::contains("MAP08: Realm"))
        .stderr(predicate::str::contains("MAP09: Abattoire"))
        .stderr(predicate::str::contains("MAP10: Onslaught"))
        .stderr(predicate::str::contains("MAP11: Hunted"))
        .stderr(predicate::str::contains("MAP12: Speed"))
        .stderr(predicate::str::contains("MAP13: The Crypt"))
        .stderr(predicate::str::contains("MAP14: Genesis"))
        .stderr(predicate::str::contains("MAP15: The Twilight"))
        .stderr(predicate::str::contains("MAP16: The Omen"))
        .stderr(predicate::str::contains("MAP17: Compound"))
        .stderr(predicate::str::contains("MAP18: Neurosphere"))
        .stderr(predicate::str::contains("MAP19: NME"))
        .stderr(predicate::str::contains("MAP20: The Death Domain"))
        .stderr(predicate::str::contains("MAP21: Slayer"))
        .stderr(predicate::str::contains("MAP22: Impossible Mission"))
        .stderr(predicate::str::contains("MAP23: Tombstone"))
        .stderr(predicate::str::contains("MAP24: The Final Frontier"))
        .stderr(predicate::str::contains("MAP25: The Temple of Darkness"))
        .stderr(predicate::str::contains("MAP26: Bunker"))
        .stderr(predicate::str::contains("MAP27: Anti-Christ"))
        .stderr(predicate::str::contains("MAP28: The Sewers"))
        .stderr(predicate::str::contains("MAP29: Odyssey of Noises"))
        .stderr(predicate::str::contains("MAP30: The Gateway of Hell"))
        .stderr(predicate::str::contains("MAP31: Cyberden"))
        .stderr(predicate::str::contains("MAP32: Go 2 It"));

    // For this test, we are going to make the assumption that the contents of this file are
    // correct. This is a reasonable assumption, as the component that saves the imported entry has
    // been tested elsewhere and has been verfied to be working.
    let wad_entry_file = settings_dir.child("wads/PLUTONIA.json");
    wad_entry_file.assert(predicate::path::is_file());

    let iwad_file = doom_home_dir.child("iwads/PLUTONIA.WAD");
    iwad_file.assert(predicate::path::is_file());
}

#[test]
fn tnt_iwad_import_should_print_the_correct_information() {
    let settings_dir = assert_fs::TempDir::new().unwrap();
    let doom_home_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("tdl").unwrap();
    cmd.arg("iwad")
        .arg("import")
        .arg("test_iwads/TNT.WAD")
        .env("RUST_LOG", "debug")
        .env("TDL_SETTINGS_PATH", settings_dir.path().to_str().unwrap())
        .env("TDL_DOOM_HOME_PATH", doom_home_dir.path().to_str().unwrap())
        .assert()
        .success()
        .stderr(predicate::str::contains("ID: TNT"))
        .stderr(predicate::str::contains("WAD Name: TNT.WAD"))
        .stderr(predicate::str::contains("Title: TNT: Evilution"))
        .stderr(predicate::str::contains("Released: 1996-06-17"))
        .stderr(predicate::str::contains("Author: TeamTNT"))
        .stderr(predicate::str::contains("MAP01: System Control"))
        .stderr(predicate::str::contains("MAP02: Human BBQ"))
        .stderr(predicate::str::contains("MAP03: Power Control"))
        .stderr(predicate::str::contains("MAP04: Wormhole"))
        .stderr(predicate::str::contains("MAP05: Hanger"))
        .stderr(predicate::str::contains("MAP06: Open Season"))
        .stderr(predicate::str::contains("MAP07: Prison"))
        .stderr(predicate::str::contains("MAP08: Metal"))
        .stderr(predicate::str::contains("MAP09: Stronghold"))
        .stderr(predicate::str::contains("MAP10: Redemption"))
        .stderr(predicate::str::contains("MAP11: Storage Facility"))
        .stderr(predicate::str::contains("MAP12: Crater"))
        .stderr(predicate::str::contains("MAP13: Nukage"))
        .stderr(predicate::str::contains("MAP14: Steel Works"))
        .stderr(predicate::str::contains("MAP15: Dead Zone"))
        .stderr(predicate::str::contains("MAP16: Deepest Reaches"))
        .stderr(predicate::str::contains("MAP17: Processing Area"))
        .stderr(predicate::str::contains("MAP18: Mill"))
        .stderr(predicate::str::contains("MAP19: Shipping/Respawning"))
        .stderr(predicate::str::contains("MAP20: Central Processing"))
        .stderr(predicate::str::contains("MAP21: Administration Center"))
        .stderr(predicate::str::contains("MAP22: Habitat"))
        .stderr(predicate::str::contains("MAP23: Lunar Mining Project"))
        .stderr(predicate::str::contains("MAP24: Quarry"))
        .stderr(predicate::str::contains("MAP25: Baron's Den"))
        .stderr(predicate::str::contains("MAP26: Ballistyx"))
        .stderr(predicate::str::contains("MAP27: Mount Pain"))
        .stderr(predicate::str::contains("MAP28: Heck"))
        .stderr(predicate::str::contains("MAP29: River Styx"))
        .stderr(predicate::str::contains("MAP30: Last Call"))
        .stderr(predicate::str::contains("MAP31: Pharaoh"))
        .stderr(predicate::str::contains("MAP32: Caribbean"));

    // For this test, we are going to make the assumption that the contents of this file are
    // correct. This is a reasonable assumption, as the component that saves the imported entry has
    // been tested elsewhere and has been verfied to be working.
    let wad_entry_file = settings_dir.child("wads/TNT.json");
    wad_entry_file.assert(predicate::path::is_file());

    let iwad_file = doom_home_dir.child("iwads/TNT.WAD");
    iwad_file.assert(predicate::path::is_file());
}
