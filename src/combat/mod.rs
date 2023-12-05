pub mod agent;
pub mod breakbar;
pub mod buff;
pub mod cast;
pub mod skill;
pub mod transfer;

use self::breakbar::BreakbarHit;
use self::buff::BuffApply;
use self::cast::Cast;
use self::transfer::TransferTracker;

#[derive(Debug, Clone)]
pub struct CombatData {
    pub casts: Vec<Cast>,
    pub buffs: Vec<BuffApply>,
    pub breakbar: Vec<BreakbarHit>,
    pub transfers: TransferTracker,
}

impl CombatData {
    pub const fn new() -> Self {
        Self {
            casts: Vec::new(),
            buffs: Vec::new(),
            breakbar: Vec::new(),
            transfers: TransferTracker::new(),
        }
    }
}

impl Default for CombatData {
    fn default() -> Self {
        Self::new()
    }
}
