use serde::{Deserialize, Serialize};

// TODO: allow name override?
// TODO: instead of hits allow counting buff apply?
// TODO: support instant casts with buff apply?
// TODO: support own minions hits?

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    /// Skill id.
    pub id: u32,

    /// Whether the definition is active.
    #[serde(default = "default_as_true")]
    pub enabled: bool,

    /// Additional hit skill id.
    #[serde(default)]
    pub hit_id: Option<u32>,

    /// Total amount of hits.
    pub hits: Option<u32>,

    /// Minimum amount of hits expected.
    pub expected: Option<u32>,

    /// Maximum duration (ms) to count as one cast.
    #[serde(default = "default_as_max")]
    pub max_duration: i32,
}

const fn default_as_true() -> bool {
    true
}

const fn default_as_max() -> i32 {
    i32::MAX
}
