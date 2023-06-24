use super::agent::Target;
use arcdps::{evtc::AgentKind, Agent};
use strum::AsRefStr;

#[derive(Debug, Clone)]
pub struct BoonApply {
    pub time: i32,
    pub boon: Boon,
    pub duration: i32,
    pub target: Target,
}

impl BoonApply {
    pub fn new(time: i32, boon: Boon, duration: i32, target: &Agent) -> Self {
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
