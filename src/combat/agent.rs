use arcdps::{evtc::AgentKind, Agent};

// TODO: show id settings?

/// Information about a target agent.
#[derive(Debug, Clone)]
pub struct Target {
    /// Kind of agent.
    pub kind: AgentKind,

    /// Agent name.
    pub name: String,
}

impl Target {
    /// Creates a new target agent.
    pub fn new(kind: AgentKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

    /// Checks whether the target matches the given species.
    pub fn matches_species(&self, species: Option<u32>) -> bool {
        match (species, self.kind) {
            (Some(species), AgentKind::Npc(id) | AgentKind::Gadget(id)) => id as u32 == species,
            _ => false,
        }
    }
}

impl From<&Agent<'_>> for Target {
    fn from(agent: &Agent) -> Self {
        let kind = agent.kind();
        let name = match agent.name {
            Some(name) if !name.is_empty() => name.into(),
            _ => match kind {
                AgentKind::Player => format!("Player:{}", agent.id),
                AgentKind::Npc(species) => format!("NPC:{species}",),
                AgentKind::Gadget(species) => format!("Gadget:{species}"),
            },
        };
        Self::new(kind, name)
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
