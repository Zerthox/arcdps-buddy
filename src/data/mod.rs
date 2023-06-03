mod structs;

pub use self::structs::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    mem,
    path::Path,
};

pub const SKILLS: &[SkillDef] = &include!(concat!(env!("OUT_DIR"), "/skills.rs"));

/// Skill data.
#[derive(Debug)]
pub struct SkillData {
    map: HashMap<u32, usize>,
    data: Vec<SkillDef>,
}

impl SkillData {
    /// Creates new skill data with the given data.
    pub fn new(skills: impl IntoIterator<Item = SkillDef>) -> Self {
        let data = skills.into_iter().collect::<Vec<_>>();
        let mut map = HashMap::with_capacity(data.len());
        for (index, skill) in data.iter().enumerate() {
            map.insert(skill.id, index);
            if let Some(hit_id) = skill.hit_id {
                map.insert(hit_id, index);
            }
        }

        Self { map, data }
    }

    /// Creates new skill data with the defaults.
    pub fn with_defaults() -> Self {
        Self::new(SKILLS.iter().cloned())
    }

    /// Checks whether there is an entry for the skill id.
    pub fn contains(&self, id: u32) -> bool {
        self.map.contains_key(&id)
    }

    /// Retrieves the [`SkillDef`] corresponding to the skill id.
    pub fn get(&self, id: u32) -> Option<&SkillDef> {
        self.map.get(&id).and_then(|index| self.data.get(*index))
    }

    /// Maps a potential hit skill id to the primary skill id.
    pub fn map_hit_id(&self, id: u32) -> u32 {
        match self.get(id) {
            Some(SkillDef {
                id: skill_id,
                hit_id: Some(hit_id),
                ..
            }) if *hit_id == id => *skill_id,
            _ => id,
        }
    }

    /// Attempts to load data from a given file path.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<(), LoadError> {
        let file = BufReader::new(File::open(path)?);
        let data: Vec<SkillDef> = serde_yaml::from_reader(file)?;
        *self = Self::new(mem::take(&mut self.data).into_iter().chain(data));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoadError {
    NotFound,
    FailedToRead,
    Invalid,
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::FailedToRead,
        }
    }
}

impl From<serde_yaml::Error> for LoadError {
    fn from(_: serde_yaml::Error) -> Self {
        Self::Invalid
    }
}
