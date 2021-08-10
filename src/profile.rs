use crate::source_port::Skill;
use color_eyre::{eyre::ensure, Report, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub source_port: String,
    pub skill: Skill,
    pub fullscreen: bool,
    pub music: bool,
    pub default: bool,
}

impl Profile {
    pub fn new(
        name: &str,
        source_port: &str,
        skill: Skill,
        fullscreen: bool,
        music: bool,
        default: bool,
    ) -> Result<Profile, Report> {
        ensure!(!name.is_empty(), "The name of the profile must be set");
        ensure!(
            !source_port.is_empty(),
            "The source port for the profile must be set"
        );
        Ok(Profile {
            name: name.to_string(),
            source_port: source_port.to_string(),
            skill,
            fullscreen,
            music,
            default,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Profile;
    use super::Skill;

    #[test]
    fn constructor_should_set_fields_correctly() {
        let profile =
            Profile::new("default", "prboom", Skill::UltraViolence, true, false, true).unwrap();
        assert_eq!(profile.name, "default");
        assert_eq!(profile.source_port, "prboom");
        matches!(profile.skill, Skill::UltraViolence);
        assert_eq!(profile.fullscreen, true);
        assert_eq!(profile.music, false);
        assert_eq!(profile.default, true);
    }

    #[test]
    fn constructor_should_return_error_for_name_not_set() {
        let result = Profile::new("", "prboom", Skill::UltraViolence, true, false, true);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The name of the profile must be set"
        );
    }

    #[test]
    fn constructor_should_return_error_for_source_port_not_set() {
        let result = Profile::new("default", "", Skill::UltraViolence, true, false, true);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The source port for the profile must be set"
        );
    }
}
