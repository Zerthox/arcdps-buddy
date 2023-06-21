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
    pub fn new(skill: Skill, target: &Agent, damage: i32, time: i32) -> Self {
        Self {
            time,
            skill,
            damage,
            target: target.into(),
        }
    }
}
