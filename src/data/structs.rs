use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    /// Skill id.
    pub id: u32,

    /// Total amount of hits.
    pub hits: Option<u32>,

    /// Minimum amount of hits expected.
    pub expected: Option<u32>,
}

impl PartialEq for SkillDef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SkillDef {}

impl PartialOrd for SkillDef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for SkillDef {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for SkillDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Borrow<u32> for SkillDef {
    fn borrow(&self) -> &u32 {
        &self.id
    }
}
