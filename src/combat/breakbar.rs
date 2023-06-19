use arcdps::Agent;

use crate::skill::Skill;

#[derive(Debug, Clone)]
pub struct BreakbarHit {
    pub time: i32,
    pub skill: Skill,
    pub damage: i32,
}

impl BreakbarHit {
    pub const fn new(skill: Skill, _target: &Agent, damage: i32, time: i32) -> Self {
        // TODO: use target
        Self {
            time,
            skill,
            damage,
        }
    }
}
