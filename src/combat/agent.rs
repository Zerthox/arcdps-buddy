use super::name_of;
use arc_util::colors::{CYAN, GREEN, RED, YELLOW};
use arcdps::{
    evtc::{self, AgentKind},
    exports::{Colors, CoreColor},
};

// TODO: show id settings?

/// Information about an agent.
#[derive(Debug, Clone)]
pub struct Agent {
    /// Kind of agent.
    pub kind: AgentKind,

    /// Agent name.
    pub name: String,
}

impl Agent {
    /// Creates a new agent.
    pub fn new(kind: AgentKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

    /// Checks whether the agent matches the given species.
    pub fn matches_species(&self, species: Option<u32>) -> bool {
        match (species, self.kind) {
            (Some(species), AgentKind::Npc(id) | AgentKind::Gadget(id)) => id as u32 == species,
            _ => false,
        }
    }

    /// Checks whether the agent is a player.
    pub fn is_player(&self) -> bool {
        matches!(self.kind, AgentKind::Player)
    }

    /// Returns friendly agent color.
    pub fn friendly_color(&self, colors: &Colors) -> [f32; 4] {
        if self.is_player() {
            colors.core(CoreColor::LightTeal).unwrap_or(CYAN)
        } else {
            colors.core(CoreColor::LightGreen).unwrap_or(GREEN)
        }
    }

    /// Returns enemy agent color.
    pub fn enemy_color(&self, colors: &Colors, fight_target: Option<u32>) -> [f32; 4] {
        if self.matches_species(fight_target) {
            colors.core(CoreColor::LightRed).unwrap_or(RED)
        } else {
            colors.core(CoreColor::LightYellow).unwrap_or(YELLOW)
        }
    }
}

impl From<&evtc::Agent> for Agent {
    fn from(agent: &evtc::Agent) -> Self {
        Self::new(agent.kind(), name_of(agent))
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        match (self.kind, other.kind) {
            (AgentKind::Player, AgentKind::Player) => self.name == other.name,
            (AgentKind::Npc(id1), AgentKind::Npc(id2))
            | (AgentKind::Gadget(id1), AgentKind::Gadget(id2)) => id1 == id2,
            _ => false,
        }
    }
}
