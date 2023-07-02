use serde::{Deserialize, Serialize};

// TODO: allow name override?
// TODO: instead of hits allow counting buff apply?
// TODO: support instant casts with buff apply?

/// Skill information.
#[derive(Debug, Clone)]
pub struct SkillInfo {
    /// Skill id.
    pub id: u32,

    /// Hit information.
    pub hits: Option<SkillHits>,

    /// Maximum duration (ms) to count as one cast.
    pub max_duration: i32,

    /// Whether to include minion hits.
    pub minion: bool,
}

impl From<SkillDef> for SkillInfo {
    fn from(def: SkillDef) -> Self {
        let SkillDef {
            id,
            enabled: _,
            hit_ids: _,
            hits,
            expected,
            max_duration,
            minion,
        } = def;
        Self {
            id,
            hits: hits.map(|max| SkillHits {
                max,
                expected: expected.unwrap_or((max + 1) / 2),
            }),
            max_duration,
            minion,
        }
    }
}

/// Skill hit information.
#[derive(Debug, Clone)]
pub struct SkillHits {
    /// Total amount of hits.
    pub max: usize,

    /// Minimum amount of hits expected.
    pub expected: usize,
}

/// Skill definition parsed from a file.
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
