use crate::skill::Skill;
use arcdps::{Activation, Agent};

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

    /// Related hits.
    pub hits: Vec<Hit>,
}

impl Cast {
    pub const fn new(skill: Skill, time: u64) -> Self {
        Self {
            skill,
            time,
            state: CastState::Unknown,
            duration: 0,
            hits: Vec::new(),
        }
    }

    pub fn hit(&mut self, target: &Agent) {
        self.hits.push(Hit {
            target: target.prof,
        })
    }

    pub fn complete(&mut self, result: CastState, duration: i32) {
        self.state = result;
        self.duration = duration;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastState {
    /// Unknown or initial state.
    #[default]
    Unknown,

    /// Completed fully.
    Fire,

    /// Cancelled after fire.
    Cancel,

    /// Interrupted before fire.
    Interrupt,
}

impl From<Activation> for CastState {
    fn from(activation: Activation) -> Self {
        match activation {
            Activation::Reset => Self::Fire,
            Activation::CancelFire => Self::Cancel,
            Activation::CancelCancel => Self::Interrupt,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hit {
    /// Target species.
    pub target: u32,
}
