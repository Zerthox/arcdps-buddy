use serde::{Deserialize, Serialize};

// TODO: allow name override?
// TODO: instead of hits allow counting buff apply?
// TODO: support instant casts with buff apply?

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    /// Skill id.
    pub id: u32,

    /// Additional hit skill id.
    pub hit_id: Option<u32>,

    /// Total amount of hits.
    pub hits: Option<u32>,

    /// Minimum amount of hits expected.
    pub expected: Option<u32>,
}
