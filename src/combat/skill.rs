use phf::phf_map;

/// Information about a skill.
#[derive(Debug, Clone)]
pub struct Skill {
    /// Id of the skill.
    pub id: u32,

    /// Name of the skill.
    pub name: String,
}

impl Skill {
    /// Creates a new skill.
    ///
    /// Name will fallback to the skill id if not present or empty.
    pub fn new(id: u32, name: Option<&str>) -> Self {
        Self {
            id,
            name: match OVERRIDES.get(&id) {
                Some(name) => name.to_string(),
                None => match name {
                    Some(name) if !name.is_empty() => name.into(),
                    _ => id.to_string(),
                },
            },
        }
    }
}

/// Skill name overrides.
static OVERRIDES: phf::Map<u32, &'static str> = phf_map! {
    22492u32 => "Basilisk Venom",
};
