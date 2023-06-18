mod apply;

pub use self::apply::*;

use crate::history::History;
use arcdps::Agent;

pub type BoonData = Vec<BoonApply>;

#[derive(Debug, Clone)]
pub struct Boons {
    pub history: History<BoonData>,
}

impl Boons {
    pub fn new() -> Self {
        Self {
            history: History::new(10),
        }
    }

    pub fn apply(&mut self, time: i32, id: u32, target: &Agent) {
        if let Some(fight) = self.history.latest_fight_mut() {
            if let Ok(boon) = id.try_into() {
                fight.data.push(BoonApply::new(boon, time, target))
            }
        }
    }
}
