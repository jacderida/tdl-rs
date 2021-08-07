use crate::source_port::Skill;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,
    source_port: String,
    skill: Skill,
    fullscreen: bool,
    music: bool,
    autoload_wads: Vec<String>,
}
