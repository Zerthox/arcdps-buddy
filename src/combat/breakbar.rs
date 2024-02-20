use super::{agent::Agent, skill::Skill};

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

    /// Agent causing the hit.
    pub attacker: String,

    /// Whether the attacker is our character.
    pub is_own: bool,

    /// Target hit.
    pub target: Agent,
}

impl BreakbarHit {
    /// Creates a new breakbar hit.
    pub fn new(
        time: i32,
        skill: Skill,
        damage: i32,
        attacker: impl Into<String>,
        is_own: bool,
        target: Agent,
    ) -> Self {
        Self {
            time,
            skill,
            damage,
            attacker: attacker.into(),
            is_own,
            target,
        }
    }
}
