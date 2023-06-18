use arcdps::Agent;
use strum::AsRefStr;

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
    pub time: i32,
    pub target: String,
    pub to_player: bool,
}

impl BoonApply {
    pub fn new(boon: Boon, time: i32, target: &Agent) -> Self {
        Self {
            boon,
            time,
            target: target.name.map(Into::into).unwrap_or_default(),
            to_player: target.elite != u32::MAX,
        }
    }
}
