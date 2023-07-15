use super::agent::Target;
use arcdps::{evtc::AgentKind, Agent};
use strum::AsRefStr;

/// Information about a buff application.
#[derive(Debug, Clone)]
pub struct BuffApply {
    /// Time of the application.
    pub time: i32,

    /// Buff applied.
    pub buff: Buff,

    /// Duration of buff applied.
    pub duration: i32,

    /// Target the buff was applied to.
    pub target: Target,
}

impl BuffApply {
    /// Creates a new buff apply.
    pub fn new(time: i32, buff: Buff, duration: i32, target: &Agent) -> Self {
        Self {
            buff,
            time,
            duration,
            target: target.into(),
        }
    }

    /// Checks whether the apply target was a player.
    pub fn to_player(&self) -> bool {
        matches!(self.target.kind, AgentKind::Player)
    }
}

/// Applied buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr)]
pub enum Buff {
    #[strum(serialize = "Quick")]
    Quickness,

    #[strum(serialize = "Alac")]
    Alacrity,

    #[strum(serialize = "Arc Power")]
    ArcanePower,

    #[strum(serialize = "Spider")]
    SpiderVenom,

    #[strum(serialize = "Skale")]
    SkaleVenom,

    #[strum(serialize = "Devourer")]
    DevourerVenom,

    #[strum(serialize = "Basi")]
    BasiliskVenom,

    #[strum(serialize = "Soul Stone")]
    SouleStoneVenom,

    #[strum(serialize = "Dwarf")]
    RiteOfTheGreatDwarf,

    #[strum(serialize = "AoJ")]
    AshesOfTheJust,

    #[strum(serialize = "Moa")]
    MoaStance,

    #[strum(serialize = "Vulture")]
    VultureStance,

    #[strum(serialize = "OWP")]
    OneWolfPack,
}

impl TryFrom<u32> for Buff {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1187 => Ok(Self::Quickness),
            30328 => Ok(Self::Alacrity),
            5582 => Ok(Self::ArcanePower),
            13036 => Ok(Self::SpiderVenom),
            13054 => Ok(Self::SkaleVenom),
            13094 => Ok(Self::DevourerVenom),
            13133 => Ok(Self::BasiliskVenom),
            49038 => Ok(Self::SouleStoneVenom),
            26596 | 33330 => Ok(Self::RiteOfTheGreatDwarf),
            41957 => Ok(Self::AshesOfTheJust),
            45038 => Ok(Self::MoaStance),
            44651 => Ok(Self::VultureStance),
            44139 => Ok(Self::OneWolfPack),
            value => Err(value),
        }
    }
}
