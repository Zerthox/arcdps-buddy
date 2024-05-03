use serde::{Deserialize, Serialize};

// TODO: allow name override?
// TODO: instead of hits allow counting buff apply?
// TODO: support instant casts with buff apply?

/// Extra error margin for max duration.
const DURATION_EPSILON: i32 = 500;

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
            max_duration: max_duration
                .map(|dur| dur + DURATION_EPSILON)
                .unwrap_or(i32::MAX),
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

#[allow(dead_code)] // TODO: cargo/clippy complain despite these being used in cast log?
impl SkillHits {
    pub fn has_hits(&self) -> bool {
        self.max > 0
    }

    pub fn missed(&self, hits: usize) -> bool {
        if self.has_hits() {
            hits < self.expected
        } else {
            hits == 0
        }
    }

    pub fn categorize(&self, hits: usize) -> SkillHitCount {
        let Self { max, expected } = *self;
        if self.has_hits() {
            if hits > max {
                SkillHitCount::OverMax
            } else if hits == max {
                SkillHitCount::Max
            } else if hits >= expected {
                SkillHitCount::Expected
            } else {
                SkillHitCount::Miss
            }
        } else if hits > 0 {
            SkillHitCount::Expected
        } else {
            SkillHitCount::Miss
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SkillHitCount {
    Miss,
    Expected,
    Max,
    OverMax,
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
    pub max_duration: Option<i32>,

    /// Whether to include minion hits.
    #[serde(default)]
    pub minion: bool,
}

fn default_as_true() -> bool {
    true
}
