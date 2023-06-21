use arcdps::{evtc::AgentKind, Agent};

// TODO: show id settings?

#[derive(Debug, Clone)]
pub struct Target {
    pub kind: AgentKind,
    pub name: String,
}

impl Target {
    pub fn new(kind: AgentKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

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
