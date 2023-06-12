use crate::history::History;
use arcdps::Agent;
use strum::AsRefStr;

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

    pub fn apply(&mut self, time: u64, id: u32, target: &Agent) {
        if let Some(fight) = self.history.latest_fight_mut() {
            if let Ok(boon) = id.try_into() {
                fight.data.push(BoonApply::new(boon, time, target))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr)]
pub enum Boon {
    Quickness,
    Alacrity,
}

impl TryFrom<u32> for Boon {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1187 => Ok(Self::Quickness),
            30328 => Ok(Self::Alacrity),
            value => Err(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoonApply {
    pub boon: Boon,
    pub time: u64,
    pub target: String,
    pub to_player: bool,
}

impl BoonApply {
    pub fn new(boon: Boon, time: u64, target: &Agent) -> Self {
        Self {
            boon,
            time,
            target: target.name.map(Into::into).unwrap_or_default(),
            to_player: target.elite != u32::MAX,
        }
    }
}
