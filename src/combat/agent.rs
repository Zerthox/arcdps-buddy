use arcdps::{evtc::AgentKind, Agent};

// TODO: show id settings?

pub fn agent_name(agent: &Agent) -> String {
    match agent.name {
        Some(name) if !name.is_empty() => name.into(),
        _ => match agent.kind() {
            AgentKind::Player => format!("Player:{}", agent.id),
            AgentKind::Npc(species) => format!("NPC:{species}",),
            AgentKind::Gadget(species) => format!("Gadget:{species}"),
        },
    }
}
