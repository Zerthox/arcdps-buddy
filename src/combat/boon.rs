use arcdps::{evtc::AgentKind, Agent};
use strum::AsRefStr;

use super::agent::Target;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr)]
pub enum Boon {
    #[strum(serialize = "Quick")]
    Quickness,

    #[strum(serialize = "Alac")]
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
    pub time: i32,
    pub duration: i32,
    pub target: Target,
}

impl BoonApply {
    pub fn new(boon: Boon, target: &Agent, duration: i32, time: i32) -> Self {
        Self {
            boon,
            time,
            duration,
            target: target.into(),
        }
    }

    pub fn to_player(&self) -> bool {
        matches!(self.target.kind, AgentKind::Player)
    }
}
