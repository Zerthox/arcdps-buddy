use serde::{Deserialize, Serialize};

// TODO: allow name override?
// TODO: instead of hits allow counting buff apply?
// TODO: support instant casts with buff apply?

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    /// Skill id.
    pub id: u32,

    /// Whether the definition is active.
    #[serde(default = "default_as_true")]
    pub enabled: bool,

    /// Additional hit skill ids.
    #[serde(default)]
    pub hit_ids: Vec<u32>,

    /// Total amount of hits.
    pub hits: Option<usize>,

    /// Minimum amount of hits expected.
    pub expected: Option<usize>,

    /// Maximum duration (ms) to count as one cast.
    #[serde(default = "default_as_max")]
    pub max_duration: i32,

    /// Whether to include minion hits.
    #[serde(default)]
    pub minion: bool,
}

const fn default_as_true() -> bool {
    true
}

const fn default_as_max() -> i32 {
    i32::MAX
}
