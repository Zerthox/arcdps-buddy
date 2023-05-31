mod structs;

pub use self::structs::*;
use std::collections::HashSet;

pub const SKILLS: &[SkillDef] = &include!(concat!(env!("OUT_DIR"), "/skills.rs"));

/// Plugin data.
#[derive(Debug)]
pub struct Data {
    skills: HashSet<SkillDef>,
}

impl Data {
    /// Creates a new data set with the given parameters.
    pub fn new(skills: impl IntoIterator<Item = SkillDef>) -> Self {
        Self {
            skills: skills.into_iter().collect(),
        }
    }

    /// Creates a new data set with the defaults.
    pub fn with_defaults() -> Self {
        Self::new(SKILLS.iter().cloned())
    }

    pub fn has_skill(&self, id: u32) -> bool {
        self.skills.contains(&id)
    }

    pub fn skill(&self, id: u32) -> Option<&SkillDef> {
        self.skills.get(&id)
    }
}
