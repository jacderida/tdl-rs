use crate::source_port::Skill;
use crate::source_port::SourcePortType;
use color_eyre::{eyre::ensure, Report, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub source_port_type: SourcePortType,
    pub source_port_version: String,
    pub skill: Skill,
    pub fullscreen: bool,
    pub music: bool,
    pub default: bool,
}

impl Profile {
    pub fn new(
        name: &str,
        source_port_type: SourcePortType,
        source_port_version: String,
        skill: Skill,
        fullscreen: bool,
        music: bool,
        default: bool,
    ) -> Result<Profile, Report> {
        ensure!(!name.is_empty(), "The name of the profile must be set");
        ensure!(
            !source_port_version.is_empty(),
            "The source port version for the profile must be set"
        );
        Ok(Profile {
            name: name.to_string(),
            source_port_type,
            source_port_version,
            skill,
            fullscreen,
            music,
            default,
        })
    }
}

#[cfg(test)]
mod new {
    use super::Profile;
    use super::Skill;
    use crate::source_port::SourcePortType;

    #[test]
    fn should_set_fields() {
        let profile = Profile::new(
            "default",
            SourcePortType::PrBoomPlus,
            "2.6um".to_string(),
            Skill::UltraViolence,
            true,
            false,
            true,
        )
        .unwrap();
        assert_eq!(profile.name, "default");
        matches!(profile.source_port_type, SourcePortType::PrBoomPlus);
        assert_eq!(profile.source_port_version, "2.6um".to_string());
        matches!(profile.skill, Skill::UltraViolence);
        assert!(profile.fullscreen);
        assert!(!profile.music);
        assert!(profile.default);
    }

    #[test]
    fn should_return_error_for_empty_name() {
        let result = Profile::new(
            "",
            SourcePortType::PrBoomPlus,
            "2.6um".to_string(),
            Skill::UltraViolence,
            true,
            false,
            true,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The name of the profile must be set"
        );
    }

    #[test]
    fn should_return_error_for_empty_version() {
        let result = Profile::new(
            "default",
            SourcePortType::PrBoomPlus,
            "".to_string(),
            Skill::UltraViolence,
            true,
            false,
            true,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The source port version for the profile must be set"
        );
    }
}
