use super::skill::Skill;
use arcdps::{Activation, Agent};

#[derive(Debug, Clone)]
pub struct Cast {
    /// Casted skill.
    pub skill: Skill,

    /// Time of start event or first registered hit.
    pub time: i32,

    /// Current [`CastState`] of the cast.
    pub state: CastState,

    /// Time spent in animation.
    pub duration: i32,

    /// Related hits.
    pub hits: Vec<Hit>,
}

impl Cast {
    pub const fn start(skill: Skill, state: CastState, time: i32) -> Self {
        Self {
            skill,
            time,
            state,
            duration: 0,
            hits: Vec::new(),
        }
    }

    pub const fn end(skill: Skill, state: CastState, duration: i32, time: i32) -> Self {
        Self {
            skill,
            time,
            state,
            duration,
            hits: Vec::new(),
        }
    }

    pub fn from_hit(skill: Skill, target: &Agent, time: i32) -> Self {
        let mut skill = Self::start(skill, CastState::Pre, time);
        skill.hit(target);
        skill
    }

    pub fn hit(&mut self, target: &Agent) {
        self.hits.push(Hit {
            target: target.prof,
        })
    }

    pub fn complete(&mut self, skill: Skill, result: CastState, duration: i32, time: i32) {
        if let CastState::Pre = self.state {
            self.skill = skill;
            self.time = time - duration;
        }
        self.state = result;
        self.duration = duration;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastState {
    /// Unknown state.
    #[default]
    Unknown,

    /// Cast is ongoing.
    Casting,

    /// Cast start is not registered.
    Pre,

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
