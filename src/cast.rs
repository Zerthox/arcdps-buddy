use crate::skill::Skill;
use arcdps::Activation;

#[derive(Debug, Clone)]
pub struct Cast {
    /// Casted [´Skill´].
    pub skill: Skill,

    /// Time of start event or first registered hit.
    pub time: u64,

    /// Current [`CastState`] of the cast.
    pub state: CastState,

    /// Time spent in animation.
    pub duration: i32,

    /// Number of related hits.
    pub hits: u32,
}

impl Cast {
    pub const fn new(skill: Skill, time: u64) -> Self {
        Self {
            skill,
            time,
            state: CastState::Unknown,
            duration: 0,
            hits: 0,
        }
    }

    pub fn hit(&mut self) {
        self.hits += 1;
    }

    pub fn complete(&mut self, result: CastState, duration: i32) {
        self.state = result;
        self.duration = duration;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastState {
    #[default]
    Unknown,
    Fire,
    Cancel,
}

impl From<Activation> for CastState {
    fn from(activation: Activation) -> Self {
        match activation {
            Activation::CancelFire | Activation::Reset => Self::Fire,
            Activation::CancelCancel => Self::Cancel,
            _ => Self::Unknown,
        }
    }
}
