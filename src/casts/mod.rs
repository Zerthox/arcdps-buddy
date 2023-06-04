mod cast;
mod skill;

pub use self::cast::*;
pub use self::skill::*;

use crate::history::{Fight, History};
use arcdps::Agent;

pub type CastData = Vec<Cast>;

#[derive(Debug, Clone)]
pub struct Casts {
    history: History<CastData>,
}

impl Casts {
    // TODO: make customizable
    pub const MAX_HISTORY: usize = 10;

    pub const fn new() -> Self {
        Self {
            history: History::new(),
        }
    }

    pub fn fight_count(&self) -> usize {
        self.history.len()
    }

    pub fn fights(&self) -> impl Iterator<Item = &Fight<CastData>> {
        self.history.all_fights()
    }

    pub fn add_fight(&mut self, species: u32, target: Option<Agent>, time: u64) {
        self.history.add_fight(time, Self::MAX_HISTORY);
        self.update_target(species, target);
    }

    pub fn update_target(&mut self, species: u32, target: Option<Agent>) {
        self.history.update_latest_target(species, target)
    }

    pub fn end_fight(&mut self, time: u64) {
        self.history.end_latest_fight(time)
    }

    pub fn casts(&self, index: usize) -> &[Cast] {
        if let Some(fight) = self.history.fight_at(index) {
            &fight.data
        } else {
            &[]
        }
    }

    pub fn latest_cast_mut(&mut self, skill: u32) -> Option<&mut Cast> {
        self.history.latest_fight_mut().and_then(|fight| {
            fight
                .data
                .iter_mut()
                .rev()
                .find(|cast| cast.skill.id == skill)
        })
    }

    pub fn add_cast(&mut self, cast: Cast) {
        if let Some(fight) = self.history.latest_fight_mut() {
            let index = fight
                .data
                .iter()
                .rev()
                .position(|other| other.time <= cast.time)
                .unwrap_or(0);
            fight.data.insert(fight.data.len() - index, cast);
        }
    }
}
