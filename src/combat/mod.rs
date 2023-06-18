pub mod boon;
pub mod cast;

use self::boon::BoonApply;
use self::cast::Cast;

#[derive(Debug, Clone)]
pub struct CombatData {
    pub casts: Vec<Cast>,
    pub boons: Vec<BoonApply>,
}

impl CombatData {
    pub const fn new() -> Self {
        Self {
            casts: Vec::new(),
            boons: Vec::new(),
        }
    }
}

impl Default for CombatData {
    fn default() -> Self {
        Self::new()
    }
}
