use arcdps::evtc::{self, AgentKind};

// TODO: show id settings?

/// Information about an agent.
#[derive(Debug, Clone)]
pub struct Agent {
    /// Kind of agent.
    pub kind: AgentKind,

    /// Whether the character is the local player.
    pub is_self: bool,

    /// Agent name.
    pub name: String,
}

impl Agent {
    /// Creates a new agent.
    pub fn new(kind: AgentKind, is_self: bool, name: impl Into<String>) -> Self {
        Self {
            kind,
            is_self,
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
}

impl From<&evtc::Agent> for Agent {
    fn from(agent: &evtc::Agent) -> Self {
        let kind = agent.kind();
        let name = match agent.name() {
            Some(name) if !name.is_empty() => name.into(),
            _ => match kind {
                AgentKind::Player => format!("Player:{}", agent.id),
                AgentKind::Npc(species) => format!("NPC:{species}",),
                AgentKind::Gadget(species) => format!("Gadget:{species}"),
            },
        };
        Self::new(kind, agent.is_self != 0, name)
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
