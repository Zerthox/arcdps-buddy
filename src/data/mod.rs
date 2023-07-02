mod structs;

pub use self::structs::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    mem,
    path::Path,
};

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
            if skill.enabled {
                map.insert(skill.id, index);
                for hit_id in &skill.hit_ids {
                    map.insert(*hit_id, index);
                }
            } else {
                map.remove(&skill.id);
                for hit_id in &skill.hit_ids {
                    map.remove(hit_id);
                }
            }
        }
        map.shrink_to_fit();

        Self { map, data }
    }

    /// Creates new skill data with the defaults.
    pub fn with_defaults() -> Self {
        let skills = include!(concat!(env!("OUT_DIR"), "/skills.rs"));
        Self::new(skills)
    }

    /// Checks whether there is an entry for the skill id.
    ///
    /// This includes disabled definitions.
    pub fn contains(&self, id: u32) -> bool {
        self.map.contains_key(&id)
    }

    /// Retrieves the [`SkillDef`] corresponding to the skill id.
    ///
    /// This includes disabled definitions.
    pub fn get(&self, id: u32) -> Option<&SkillDef> {
        self.map.get(&id).and_then(|index| self.data.get(*index))
    }

    /// Attempts to load data from a given file path.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<usize, LoadError> {
        let file = BufReader::new(File::open(path)?);
        let data: Vec<SkillDef> = serde_yaml::from_reader(file)?;
        let count = data.len();
        *self = Self::new(mem::take(&mut self.data).into_iter().chain(data));
        Ok(count)
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
