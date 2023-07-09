pub mod agent;
pub mod breakbar;
pub mod buff;
pub mod cast;
pub mod skill;

use breakbar::BreakbarHit;
use buff::BuffApply;
use cast::Cast;

#[derive(Debug, Clone)]
pub struct CombatData {
    pub casts: Vec<Cast>,
    pub buffs: Vec<BuffApply>,
    pub breakbar: Vec<BreakbarHit>,
}

impl CombatData {
    pub const fn new() -> Self {
        Self {
            casts: Vec::new(),
            buffs: Vec::new(),
            breakbar: Vec::new(),
        }
    }
}

impl Default for CombatData {
    fn default() -> Self {
        Self::new()
    }
}
