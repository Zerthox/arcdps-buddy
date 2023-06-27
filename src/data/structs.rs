use serde::{Deserialize, Serialize};

// TODO: allow name override?
// TODO: instead of hits allow counting buff apply?
// TODO: support instant casts with buff apply?
// TODO: support own minions hits?

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    /// Skill id.
    pub id: u32,

    #[serde(default = "default_as_true")]
    pub enabled: bool,

    /// Additional hit skill id.
    pub hit_id: Option<u32>,

    /// Total amount of hits.
    pub hits: Option<u32>,

    /// Minimum amount of hits expected.
    pub expected: Option<u32>,
}

const fn default_as_true() -> bool {
    true
}
