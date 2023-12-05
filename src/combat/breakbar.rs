use super::{agent::Target, skill::Skill};

/// Information about a defiance damage hit.
#[derive(Debug, Clone)]
pub struct BreakbarHit {
    /// Time of the hit.
    pub time: i32,

    /// Skill causing the hit.
    pub skill: Skill,

    /// Defiance damage dealt by the hit.
    ///
    /// Due to decimals this is `10 * damage` compared to regular defiance damage units.
    pub damage: i32,

    /// Target hit.
    pub target: Target,
}

impl BreakbarHit {
    /// Creates a new breakbar hit.
    pub fn new(time: i32, skill: Skill, damage: i32, target: Target) -> Self {
        Self {
            time,
            skill,
            damage,
            target,
        }
    }
}
