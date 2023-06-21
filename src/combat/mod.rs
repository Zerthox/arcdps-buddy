pub mod agent;
pub mod boon;
pub mod breakbar;
pub mod cast;
pub mod skill;

use self::boon::BoonApply;
use self::breakbar::BreakbarHit;
use self::cast::Cast;

#[derive(Debug, Clone)]
pub struct CombatData {
    pub casts: Vec<Cast>,
    pub boons: Vec<BoonApply>,
    pub breakbar: Vec<BreakbarHit>,
}

impl CombatData {
    pub const fn new() -> Self {
        Self {
            casts: Vec::new(),
            boons: Vec::new(),
            breakbar: Vec::new(),
        }
    }
}

impl Default for CombatData {
    fn default() -> Self {
        Self::new()
    }
}
