mod cast;

pub use self::cast::*;

use crate::history::History;

pub type CastData = Vec<Cast>;

#[derive(Debug, Clone)]
pub struct Casts {
    pub history: History<CastData>,
}

impl Casts {
    pub const fn new() -> Self {
        Self {
            history: History::new(10),
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
