use super::{agent::Target, skill::Skill};
use arcdps::Agent;

#[derive(Debug, Clone)]
pub struct BreakbarHit {
    pub time: i32,
    pub skill: Skill,
    pub damage: i32,
    pub target: Target,
}

impl BreakbarHit {
    pub fn new(time: i32, skill: Skill, damage: i32, target: &Agent) -> Self {
        Self {
            time,
            skill,
            damage,
            target: target.into(),
        }
    }
}
