use super::agent::agent_name;
use crate::skill::Skill;
use arcdps::Agent;

#[derive(Debug, Clone)]
pub struct BreakbarHit {
    pub time: i32,
    pub skill: Skill,
    pub damage: i32,
    pub target: String,
}

impl BreakbarHit {
    pub fn new(skill: Skill, target: &Agent, damage: i32, time: i32) -> Self {
        // TODO: use target
        Self {
            time,
            skill,
            damage,
            target: agent_name(target),
        }
    }
}
